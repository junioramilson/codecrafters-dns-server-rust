use std::{io::Bytes, net::UdpSocket};

#[derive(Debug)]
struct DnsHeader {
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

impl DnsHeader {
    pub fn new(buffer: &mut [u8]) -> DnsHeader {
        let id = ((buffer[0] as u16) << 8) | (buffer[1] as u16);

        // TODO: parse the remaining fields

        DnsHeader {
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

// impl From<&str> for &[u8] {
//     fn from(domain: &str) -> Self {

//     }
// }

#[derive(Debug)]
struct DnsQuestion {
    name: String,
    r#type: u16,
    class: u16,
}

impl DnsQuestion {
    pub fn new(buffer: &mut [u8]) -> DnsQuestion {
        let mut name = String::new();
        let r#type = 0;
        let class = 0;

        let index = buffer.iter().position(|&x| x == b'\x00').unwrap();
        // let size_of_content = buffer[0];

        name = String::from_utf8(buffer[1..index].to_vec()).unwrap();

        println!("NAME: {:?}", name);

        DnsQuestion {
            name,
            r#type,
            class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes
    }
}

#[derive(Debug)]
struct DNSMessage {
    pub header: DnsHeader,
    pub question: DnsQuestion,
}

impl DNSMessage {
    pub fn new(buffer: &mut [u8]) -> DNSMessage {
        let header = DnsHeader::new(&mut buffer[..12]);
        let question = DnsQuestion::new(&mut buffer[12..]);

        DNSMessage { header, question }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.header.to_bytes());
        bytes.extend(self.question.to_bytes());

        bytes
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                let dns_message = DNSMessage::new(&mut buf);

                let msg = &dns_message.to_bytes();

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
