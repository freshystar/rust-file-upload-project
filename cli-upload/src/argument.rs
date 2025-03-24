use clap::Parser;

/// Simple file uploader CLI
#[derive(Parser)]
#[command(version = "1.0", about = "Uploads a file to the server")]
pub struct Args {
    /// Path to the file to upload
    file_path: String,
}
