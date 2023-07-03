use tokio::net::{TcpListener};
use tokio::io::{BufReader};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("socket: {:?}", socket);
        let buf_reader = BufReader::new(socket);
        println!("buf: {:?}", buf_reader);
        println!("buf: {:?}", buf_reader.buffer());
    }
}