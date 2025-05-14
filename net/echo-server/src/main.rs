use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    println!("client connected: {}", stream.peer_addr().unwrap());

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("client {} disconnected.", stream.peer_addr().unwrap());
                    break;
                }
                if let Err(e) = stream.write_all(&buffer[0..size]) {
                    eprintln!("write error: {}", e);
                    break;
                }
                println!("Received {} bytes, echo to client {}.", size, stream.peer_addr().unwrap());
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connnect error: {}", e);
            }
        }
    }
    Ok(())
}
