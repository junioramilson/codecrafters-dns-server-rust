#[derive(Debug)]
pub struct DnsAnswer {
    pub name: String,
    pub r#type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: String,
}

impl DnsAnswer {
    pub fn new() -> DnsAnswer {
        DnsAnswer {
            name: String::from("\x0ccodecrafters\x02io"),
            r#type: 1,
            class: 1,
            ttl: 60,
            rdlength: 4,
            rdata: String::from("\x08\x08\x08\x08"),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.name.as_bytes());
        bytes.extend_from_slice(&self.r#type.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        bytes.extend_from_slice(&self.rdlength.to_be_bytes());
        bytes.extend_from_slice(&self.rdata.as_bytes());

        bytes
    }
}
