use std::net::{UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

const NTP_TIMESTAMP_DELTA: u64 = 2208988800;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <address> <port>", args[0]);
        std::process::exit(1);
    }

    let address = &args[1];
    let port = args[2].parse::<u16>().expect("Invalid port number");

    let bind_addr = format!("{}:{}", address, port);
    let socket = UdpSocket::bind(&bind_addr)?;
    println!("SNTP server listening on {}", bind_addr);

    let mut buf = [0u8; 48];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        if amt != 48 {
            continue;
        }

        let response = create_sntp_response(&buf);
        socket.send_to(&response, &src)?;
    }
}

fn create_sntp_response(request: &[u8]) -> [u8; 48] {
    let mut response = [0u8; 48];

    // Copy the first 8 bytes from the request (including the LI, VN, and Mode fields)
    response[0..8].copy_from_slice(&request[0..8]);

    // Set the Stratum to 1 (primary reference)
    response[1] = 1;

    // Set the Mode to 4 (server)
    response[0] = (response[0] & 0b11111000) | 0b100;

    // Set the current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ntp_seconds = now.as_secs() + NTP_TIMESTAMP_DELTA;
    let ntp_fraction = (now.subsec_nanos() as u64 * (1u64 << 32)) / 1_000_000_000;

    // Transmit Timestamp
    response[40..44].copy_from_slice(&(ntp_seconds as u32).to_be_bytes());
    response[44..48].copy_from_slice(&(ntp_fraction as u32).to_be_bytes());

    response
}
