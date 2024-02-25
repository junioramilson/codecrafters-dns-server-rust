#[derive(Debug)]
pub struct DnsQuestion {
    pub name: Vec<u8>,
    pub r#type: u16,
    pub class: u16,
}

impl DnsQuestion {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.r#type.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());

        bytes
    }
}
