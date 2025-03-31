mod compress_parameters;
mod database;
mod file_compress;

use compress_parameters::compression_prm;

#[tokio::main]
async fn main() {
    compression_prm().await;
}
