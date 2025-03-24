use axum::{extract::Multipart, response::Html, routing::{get, post}, Router};
use std::{env, fs::File, io::Write};

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        // Grab the name
        let file_name = field.file_name().unwrap();

        // Create a path for the soon-to-be file
        let file_path = format!("files/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");
        println!("Got file!");
    }
}

pub async fn send_file(mut multipart: Multipart) -> &'static str {
    let current_dir = env::current_dir().unwrap();

    let path = current_dir.join("uploads"); // Use absolute path

    if !path.exists() {
        std::fs::create_dir(&path).expect("Failed to create 'uploads' directory!");
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("uploaded_file").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = path.join(&file_name);
        println!("Saving file to: {}", file_path.display());

        let mut file = File::create(&file_path).unwrap();
        file.write_all(&data).unwrap();
    }
    "File uploaded successfully!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload)).route("/cli-upload", post(send_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}
