use std::net::UdpSocket;

#[derive(Debug)]
struct DNSHeader {
    id: u16,
    qr: u8,
    op_code: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DNSHeader {
    pub fn new(buffer: &mut [u8]) -> DNSHeader {
        let id = ((buffer[0] as u16) << 8) | (buffer[1] as u16);

        DNSHeader {
            id,
            qr: 1 << 7,
            op_code: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.id.to_be_bytes());
        bytes.push(self.qr);
        bytes.extend(self.op_code.to_be_bytes());
        bytes.extend(self.aa.to_be_bytes());
        bytes.extend(self.tc.to_be_bytes());
        bytes.extend(self.rd.to_be_bytes());
        bytes.extend(self.ra.to_be_bytes());
        bytes.extend(self.z.to_be_bytes());
        bytes.extend(self.rcode.to_be_bytes());
        bytes.extend(self.qdcount.to_be_bytes());
        bytes.extend(self.ancount.to_be_bytes());
        bytes.extend(self.nscount.to_be_bytes());
        bytes.extend(self.arcount.to_be_bytes());

        bytes
    }
}

#[derive(Debug)]
struct DNSMessage {
    pub header: DNSHeader,
}

impl DNSMessage {
    pub fn new(buffer: &mut [u8]) -> DNSMessage {
        let header = DNSHeader::new(&mut buffer[..12]);

        DNSMessage { header }
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                let dns_message = DNSMessage::new(&mut buf);

                let msg = &&dns_message.header.to_bytes();

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
