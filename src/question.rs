#[derive(Debug)]
pub struct DnsQuestion {
    pub name: String,
    pub r#type: u16,
    pub class: u16,
}

impl DnsQuestion {
    pub fn new(buffer: &mut [u8]) -> DnsQuestion {
        let end_of_labels_index = buffer.iter().position(|&x| x == b'\x00').unwrap();
        let first_label_len = (buffer[0] as u16) as usize;
        let mut current_index = first_label_len + 1;

        loop {
            let label_len = (buffer[current_index] as u16) as usize;
            current_index += label_len + 1;
            if buffer[current_index] == b'\x00' {
                break;
            }
        }

        let name = String::from_utf8(buffer[0..end_of_labels_index + 1].to_vec()).unwrap();
        let r#type = buffer[current_index + 2] as u16;
        let class = buffer[current_index + 2] as u16;

        DnsQuestion {
            name,
            r#type,
            class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.name.as_bytes());
        bytes.extend_from_slice(&self.r#type.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());

        bytes
    }
}
