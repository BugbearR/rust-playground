use std::net::UdpSocket;
use std::time::{Duration, UNIX_EPOCH};
use std::env;

const NTP_TIMESTAMP_DELTA: u64 = 2208988800; // delta of unix_epock 1970-01-01 00:00:00 and ntp_epoch 1900-01-01 00:00:00

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <address> <port>", args[0]);
        std::process::exit(1);
    }

    let address = &args[1];
    let port = args[2].parse::<u16>().expect("Invalid port number");

    let ntp_server = format!("{}:{}", address, port);

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::new(5, 0)))?;

    let mut buffer = [0u8; 48];
    buffer[0] = 0x1B; // version 3 + mode 3

    socket.send_to(&buffer, ntp_server)?;
    socket.recv_from(&mut buffer)?;

    let seconds = u32::from_be_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]);
    let fraction = u32::from_be_bytes([buffer[44], buffer[45], buffer[46], buffer[47]]);

    let ntp_time = seconds as f64 + (fraction as f64) / 2f64.powi(32);
    let unix_time = ntp_time - NTP_TIMESTAMP_DELTA as f64;

    let system_time = UNIX_EPOCH + Duration::from_secs_f64(unix_time);
    let datetime = system_time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("seconds: {:?}", seconds);
    println!("fraction: {:?}", fraction);
    println!("ntp_time: {:?}", ntp_time);
    println!("system_time: {:?}", system_time);
    println!("unix time: {}", datetime.as_secs());

    Ok(())
}
