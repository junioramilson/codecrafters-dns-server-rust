use crate::{header::DnsHeader, question::DnsQuestion};

#[derive(Debug)]
pub struct DNSMessage {
    pub header: DnsHeader,
    pub question: DnsQuestion,
}

impl DNSMessage {
    pub fn new(buffer: &mut [u8]) -> DNSMessage {
        let header = DnsHeader::new(&mut buffer[..12]);
        let question = DnsQuestion::new(&mut buffer[12..]);

        println!("HEADER SIZE: {:?}", header.to_bytes().len());
        println!("QUESTION: {:?}", question);

        DNSMessage { header, question }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.header.to_bytes());
        bytes.extend(self.question.to_bytes());

        bytes
    }
}
