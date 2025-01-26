use serde;
use serde_json;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

struct BingoWords {
    bingoWords: Vec<String>
}

#[tokio::main]
async fn main() {
    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    loop {
        let (mut socket, _) = listner.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read form Socket; err = {:?}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buffer[..n]).await {
                    eprintln!("Failed to write form Socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

// fn readData() -> Vec<BingoWords> {
//
//     return;
// }

fn writeData() {

}
