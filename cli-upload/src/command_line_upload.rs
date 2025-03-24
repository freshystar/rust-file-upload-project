use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Read};

pub fn cli_uploader() {
    let server_url = "http://localhost:3000/cli-upload";
    let mut files: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        let mut response = String::new();

        println!("Do you want to upload another file?: yes/no");

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.trim() {
            "no" => {
                break;
            }
            "yes" => {
                println!("Input the path to the file:");

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input: String = input.to_lowercase().trim().to_string();
                files.push(input);
            }
            _ => {}
        }
    }
    for file_path in &files {
        // Read the file
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        // Create a multipart form
        let part = Part::bytes(buffer).file_name(file_path.clone());
        let form = Form::new().part("file", part);

        // Send the file
        let client = Client::new();
        let response = client
            .post(server_url)
            .multipart(form)
            .send()
            .expect("Failed to send request");

        println!("Server Response: {:?}", response.text().unwrap());
    }

    println!("{:?}", files);
}
