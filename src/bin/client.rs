use mini_redis::client;
use bytes::{self, Bytes};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: Bytes,
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        let _ = tx.send("sending value from first handle").await;
    });

    tokio::spawn(async move {
        let _ = tx2.send("sending value from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}