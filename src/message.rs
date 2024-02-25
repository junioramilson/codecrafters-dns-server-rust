use std::net::Ipv4Addr;

use crate::{answer::DnsAnswer, header::DnsHeader, question::DnsQuestion};

#[derive(Debug)]
pub struct DNSMessage {
    pub header: Option<DnsHeader>,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>,
}

impl DNSMessage {
    pub fn new() -> DNSMessage {
        DNSMessage {
            header: None,
            questions: Vec::<DnsQuestion>::new(),
            answers: Vec::<DnsAnswer>::new(),
        }
    }

    pub fn parse(&self, buffer: &mut [u8]) -> DNSMessage {
        let mut header = DnsHeader::new(&mut buffer[..12]);
        let questions = self.parse_questions(&mut buffer[12..]);
        let answers = self.parse_answers(&questions);

        header.qdcount = questions.len() as u16;
        header.ancount = answers.len() as u16;

        DNSMessage {
            header: Some(header),
            questions,
            answers,
        }
    }

    fn parse_questions(&self, buffer: &mut [u8]) -> Vec<DnsQuestion> {
        let mut questions = Vec::<DnsQuestion>::new();

        println!("Buffer: {:?}", String::from_utf8(buffer.to_vec()));

        let mut name = Vec::<u8>::new();
        let mut curr_index_label_len = 0;
        loop {
            if buffer[curr_index_label_len] == 0 {
                break;
            }

            let label_len = buffer[curr_index_label_len] as usize;
            let label_content = &buffer[curr_index_label_len + 1..label_len + curr_index_label_len];

            name.push((label_len - 1) as u8);
            name.extend_from_slice(label_content);

            curr_index_label_len = curr_index_label_len + label_len + 1;
        }

        name.push(0);

        // let label_len_index = 0;

        // let label_1_len = buffer[label_len_index] as usize + 1;
        // let label_1 = &buffer[label_len_index + 1..label_1_len];
        // let label_2_len = buffer[label_1_len] as usize;
        // let label_2 = &buffer[(label_1_len + 1)..(label_1_len + label_2_len + 1) as usize];

        // let mut name = Vec::<u8>::new();
        // name.push((label_1_len - 1) as u8);
        // name.extend_from_slice(label_1);
        // name.push(label_2_len as u8);
        // name.extend_from_slice(label_2);
        // name.push(0);

        questions.push(DnsQuestion {
            name,
            r#type: 1,
            class: 1,
        });

        questions
    }

    fn parse_answers(&self, questions: &Vec<DnsQuestion>) -> Vec<DnsAnswer> {
        let mut answers = Vec::<DnsAnswer>::new();

        for question in questions {
            let name = question.name.clone();
            answers.push(DnsAnswer {
                name,
                r#type: 1,
                class: 1,
                ttl: 60,
                rdlength: 4,
                rdata: Ipv4Addr::new(8, 8, 8, 8).octets().to_vec(),
            })
        }

        answers
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.as_ref().unwrap().to_bytes());

        let questions: Vec<u8> = self.questions.iter().flat_map(|q| q.to_bytes()).collect();
        let answers: Vec<u8> = self.answers.iter().flat_map(|a| a.to_bytes()).collect();

        bytes.extend_from_slice(&questions);
        bytes.extend_from_slice(&answers);

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut buf = vec![
            12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let dns_message = DNSMessage::new();
        let dns_message = dns_message.parse(&mut buf);
    }
}
