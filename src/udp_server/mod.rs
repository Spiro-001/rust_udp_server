use colored::*;
use std::net::UdpSocket;

pub fn start_server(addr: &str) {
    // Initialize New Server
    println!(
        "{} {}",
        "Initializing UDP server on".yellow(),
        addr.yellow()
    );
    const BUFFER_LIMIT: usize = 2400;
    let socket = UdpSocket::bind(addr).expect("Couldn't bind to address");
    println!("{} {}", "Success! Now listening on".green(), addr.green());
    let mut buf = [0; BUFFER_LIMIT];

    loop {
        match socket.recv(&mut buf) {
            Ok(r) => {
                let str = String::from_utf8_lossy(&buf[..r]);
                println!("received {r} bytes {str}");
            }
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
}
