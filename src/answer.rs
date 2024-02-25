use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct DnsAnswer {
    pub name: Vec<u8>,
    pub r#type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl DnsAnswer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.r#type.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be().to_be_bytes());
        bytes.extend_from_slice(&self.rdlength.to_be_bytes());
        bytes.extend_from_slice(&self.rdata);

        bytes
    }
}
