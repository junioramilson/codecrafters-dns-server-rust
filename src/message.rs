use crate::{answer::DnsAnswer, header::DnsHeader, question::DnsQuestion};

#[derive(Debug)]
pub struct DNSMessage {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub answer: DnsAnswer,
}

impl DNSMessage {
    pub fn new(buffer: &mut [u8]) -> DNSMessage {
        let header = DnsHeader::new(&mut buffer[..12]);
        let question = DnsQuestion::new(&mut buffer[12..]);
        let answer = DnsAnswer::new();

        DNSMessage {
            header,
            question,
            answer,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.question.to_bytes());
        bytes.extend_from_slice(&self.answer.to_bytes());

        bytes
    }
}
