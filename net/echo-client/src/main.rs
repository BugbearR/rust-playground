use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8080";

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to server: {}", server_address);

            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin.lock());

            loop {
                print!("=>");
                io::stdout().flush()?;

                let mut input_buffer = String::new();
                reader.read_line(&mut input_buffer)?;

                let message_to_send = input_buffer.trim_end();
                if message_to_send.is_empty() {
                    println!("Close connection.");
                    break;
                }

                stream.write_all(message_to_send.as_bytes())?;
                stream.write_all(b"\n")?;
                stream.flush()?;

                let mut reader_stream = BufReader::new(&stream);
                let mut response_buffer = Vec::new();

                match reader_stream.read_until(b'\n', &mut response_buffer) {
                    Ok(0) => {
                        println!("Connection closed by server.");
                        break;
                    }
                    Ok(_) => {
                        match str::from_utf8(&response_buffer) {
                            Ok(response_str) => {
                                print!("<={}", response_str);
                                if !response_str.ends_with('\n') {
                                    println!();
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to decode (UTF-8): {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Receive error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Connect error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
