use std::{collections::HashMap, net::Ipv4Addr};

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

        println!("Message: {:?}", buffer);

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

        println!("Buffer: {:?}", String::from_utf8_lossy(&buffer.to_vec()));

        let offset: usize = 12;
        let mut offset_counter: usize = offset;

        let mut compress_map = HashMap::<usize, Vec<u8>>::new();

        let mut domain_name_parser = |initial_index: usize| {
            let mut name = Vec::<u8>::new();
            let mut curr_index_label_len = initial_index;

            loop {
                if buffer[curr_index_label_len] == b'\x00' {
                    break;
                }

                let pointer_found = buffer[curr_index_label_len] == 192
                    || (curr_index_label_len > 1 && buffer[curr_index_label_len - 1] == 192);

                if pointer_found {
                    let target_offset = buffer[curr_index_label_len + 1] as usize - 1 as usize;

                    match compress_map.get(&(target_offset)) {
                        Some(content) => {
                            let label_len = buffer[target_offset];

                            name.push(content.len() as u8);
                            name.extend_from_slice(content);

                            offset_counter += label_len as usize;
                            curr_index_label_len += label_len as usize + 1;
                            continue;
                        }
                        None => {
                            break;
                        }
                    };
                }

                let label_len = buffer[curr_index_label_len];
                let label_content = &buffer[curr_index_label_len + 1
                    ..(label_len as usize + curr_index_label_len as usize + 1)];

                compress_map.insert(offset_counter, label_content.to_vec());
                offset_counter += label_len as usize;

                name.push(label_len);
                name.extend_from_slice(label_content);

                curr_index_label_len += label_len as usize + 1;
            }

            name.push(0);

            questions.push(DnsQuestion {
                name,
                r#type: 1,
                class: 1,
            });
        };

        let mut parsing_index: usize = 0;

        loop {
            if buffer.len() < parsing_index || buffer[parsing_index] == b'\x00' {
                break;
            }

            let window_size: usize = 4;
            let splitted = &buffer
                .windows(window_size)
                .enumerate()
                .find(|(_, w)| matches!(*w, b"\x00\x01\x00\x01"))
                .map(|(index, _)| index);

            match splitted {
                Some(splitted) => {
                    domain_name_parser(parsing_index);
                    parsing_index += splitted + window_size;
                }
                None => {
                    break;
                }
            }
        }

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
            232, 226, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115,
            115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3,
            100, 101, 102, 192, 16, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
            0, 0, 0, 0, 0, 0, 0,
        ];
        let dns_message = DNSMessage::new();
        let dns_message = dns_message.parse(&mut buf);

        assert_eq!(dns_message.questions.len(), 2);
        assert_eq!(dns_message.answers.len(), 2);
    }
}
