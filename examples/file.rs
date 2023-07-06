use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;
use std::str;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("file.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    println!("buffer: {:?}", buffer);
    println!("string: {:?}", str::from_utf8(&buffer));
    Ok(())
}