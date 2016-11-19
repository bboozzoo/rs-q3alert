#[macro_use] extern crate log;
extern crate env_logger;

use std::net::UdpSocket;
use std::time::Duration;
use std::io::ErrorKind;
use std::env;
use std::process::exit;

fn usage() {
    println!("Usage: ");
    println!("   q3alert <dest>")
}

fn main()
{
    env_logger::init().unwrap();

    let addr = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            error!("no arguments provided");
            usage();
            exit(1);
        },
    };

    info!("starting up, host addr {}", addr);

    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    loop {
        match sock.send_to("foo".as_bytes(), "nas:9999") {
            Err(e) => panic!("failed {}", e),
            Ok(_) => info!("sent"),
        }

        let mut buf: [u8; 4096] = [0; 4096];
        match sock.recv_from(&mut buf) {
            Ok((size, src)) => info!("got {} bytes from {}", size, src),
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => info!("timeout"),
                _ => panic!("recv failed {}", e),
            }
        }
    }
}
