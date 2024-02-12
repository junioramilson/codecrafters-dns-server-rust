#[derive(Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub qr: u8,
    pub op_code: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new(buffer: &mut [u8]) -> DnsHeader {
        let id = ((buffer[0] as u16) << 8) | (buffer[1] as u16);
        DnsHeader {
            id,
            qr: 1,
            op_code: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 1,
            ancount: 1,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let byte: u8 =
            ((self.qr << 7) | (self.op_code << 6) | (self.aa << 2) | (self.tc << 1) | self.rd);
        let rest: u8 = (self.ra << 7) | (self.z << 6) | (self.rcode << 3);

        bytes.extend(self.id.to_be_bytes());
        bytes.push(byte);
        bytes.push(rest);
        bytes.extend(self.qdcount.to_be_bytes());
        bytes.extend(self.ancount.to_be_bytes());
        bytes.extend(self.nscount.to_be_bytes());
        bytes.extend(self.arcount.to_be_bytes());

        bytes
    }
}
