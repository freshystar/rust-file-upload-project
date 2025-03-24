

// pub async fn upload_chunk(mut multipart: Multipart) -> impl IntoResponse {
//     let mut file_name = String::new();
//     let mut chunk_number = 0;
//     let mut total_chunks = 0;
//     let mut chunk_data = Vec::new();

//     while let Some(field) = match multipart.next_field().await {
//         Ok(f) => f,
//         Err(err) => {
//             eprintln!("Error reading multipart field: {:?}", err);
//             return StatusCode::BAD_REQUEST;
//         }
//     } {
//         let field_name = field.name().unwrap_or_default().to_string();
//         match field_name.as_str() {
//             "fileName" => file_name = sanitize_filename(&field.text().await.unwrap_or_default()),
//             "chunkNumber" => chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0),
//             "totalChunks" => total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0),
//             "chunk" => chunk_data = field.bytes().await.unwrap_or_else(|_| Vec::new()).to_vec(),
//             _ => {}
//         }
//     }

//     if file_name.is_empty() || chunk_data.is_empty() {
//         return StatusCode::BAD_REQUEST;
//     }

//     let temp_dir = format!("./uploads/temp/{}", file_name);
//     fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {});
//     let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
//     let mut file = File::create(&chunk_path).unwrap();
//     file.write_all(&chunk_data).unwrap();

//     if is_upload_complete(&temp_dir, total_chunks) {
//         assemble_file(&temp_dir, &file_name, total_chunks).unwrap();
//     }

//     StatusCode::OK
// }