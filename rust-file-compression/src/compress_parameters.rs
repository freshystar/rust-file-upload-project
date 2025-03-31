use std::io;

use crate::{database::insert_user, file_compress::compress_file};
use sqlx::postgres::PgPoolOptions;

pub async fn compression_prm() {
    let mut input: Vec<String> = Vec::new();
    let mut methods: Vec<String> = Vec::new();
    let mut output_files: Vec<String> = Vec::new();

    loop {
        let mut response = String::new();
        let mut files = String::new();
        let mut method = String::new();
        let mut output = String::new();
        println!("Do you want to compress files? [yes/no]");

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.trim() {
            "no" => {
                break;
            }
            "yes" => {
                println!("Enter the file you want to compress: ");
                io::stdin()
                    .read_line(&mut files)
                    .expect("Failed to read file");
                input.push(files.trim().to_string());

                println!("Which compression method do you want to use? [best,fast,default]");

                io::stdin()
                    .read_line(&mut method)
                    .expect("Failed to read line");
                methods.push(method.trim().to_string());
                println!("Status: pending...");

                println!("Where do you want the compressed file to go?");
                io::stdin()
                    .read_line(&mut output)
                    .expect("Failed to read line");
                output_files.push(output.trim().to_string());
            }
            _ => {}
        }
    }

    let mut index = 0;

    for _ in 0..output_files.len() {
        let (input, output, method) = (&input[index], &output_files[index], &methods[index]);
        let result = compress_file(input, output, method);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres:mysecretpassword@10.39.78.149:5432/upload")
            .await
            .unwrap();

        let response_id = insert_user(&pool, &pool, &input, &output).await;

        match result {
            Ok(_) => {
                let status = format!(
                    "UPDATE files SET file_status= 'completed' WHERE id={}",
                    response_id
                );
                let status_result = sqlx::query(status.as_str()).execute(&pool).await.unwrap();

                println!("Status: In process...");
                println!("The file  number {} was compressed successfully", index + 1);
            }
            Err(error) => {
                eprint!("Error: {}", error)
            }
        }

        index += 1;
    }
}
