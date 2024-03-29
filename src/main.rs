use std::net::UdpSocket;
mod answer;
mod header;
mod message;
mod question;

use crate::message::DNSMessage;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                let dns_message = DNSMessage::new();
                let dns_message = dns_message.parse(&mut buf);

                let msg = &dns_message.to_bytes();

                println!("Dispatching message...");

                udp_socket
                    .send_to(msg, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
