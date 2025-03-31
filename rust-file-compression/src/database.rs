pub async fn insert_user(
    pool1: &sqlx::PgPool,
    pool2: &sqlx::PgPool,
    file_path: &str,
    compressed_file: &str,
) -> u64 {
    // Blocking database operation inside an async function (inefficient)
    let query = "INSERT INTO files (file_path, compressed_file) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(file_path)
        .bind(compressed_file)
        .execute(pool1)
        .await
        .unwrap();
    let id = "SELECT id FROM files WHERE file_path=$1";

    let result = sqlx::query(id)
        .bind(file_path)
        .execute(pool2)
        .await
        .unwrap();
    let id = result.rows_affected();
    println!("task_id: {}", id);
    id

    // let  status = format!("SELECT file_status FROM files WHERE id={}", result.rows_affected());
    // let status_result = sqlx::query(status.as_str())
    //     .bind(file_path)
    //     .execute(pool3)
    //     .await
    //     .unwrap();
}
