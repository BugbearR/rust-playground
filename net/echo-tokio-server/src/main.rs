use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 1024];
    let peer_addr = stream.peer_addr()?;
    println!("Client connection: {}", peer_addr);

    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("client {} is disconnected.", peer_addr);
                return Ok(());
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error from {}: {}", peer_addr, e);
                return Err(e.into());
            }
        };

        if let Err(e) = stream.write_all(&buffer[0..n]).await {
            eprintln!("Write error to {}: {}", peer_addr, e);
            return Err(e.into());
        }
        println!("Received {} bytes, echo to client {}", n, peer_addr);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("hadle_client error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("accept error: {}", e);
            }
        }
    }
}
