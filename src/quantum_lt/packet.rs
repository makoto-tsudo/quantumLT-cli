use crate::quantum_lt::{Mprm, Pari, QUSt};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum CMD {
    ApplGetP,
    ApplSetP,
    ApplRply,
    SetP,
    Rply,
}

#[allow(dead_code)]
impl CMD {
    const ALL: [CMD; 5] = [
        CMD::ApplGetP,
        CMD::ApplSetP,
        CMD::ApplRply,
        CMD::SetP,
        CMD::Rply,
    ];

    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            CMD::ApplGetP => "ApplGetP",
            CMD::ApplSetP => "ApplSetP",
            CMD::ApplRply => "ApplRply",
            CMD::SetP => "SetP",
            CMD::Rply => "Rply",
        }
    }
    
    #[inline]
    pub fn as_bytes(self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        
        for (i, b) in self.as_str().bytes().rev().enumerate() {
            buf[i] = b;
        }
        
        return buf;
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let bytes: [u8; 8] = bytes.get(..8)?.try_into().ok()?;
        
        Self::ALL.into_iter().find(|cmd| cmd.as_bytes() == bytes)
    }
}

#[derive(Clone, Copy)]
pub enum SUBCMD {
    QUSt,
    Pari,
    Mprm,    
}

#[allow(dead_code)]
impl SUBCMD {
    const ALL: [SUBCMD; 3] = [
        SUBCMD::QUSt,
        SUBCMD::Pari,
        SUBCMD::Mprm,
    ];

    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            SUBCMD::QUSt => "QUst",
            SUBCMD::Pari => "Pari",
            SUBCMD::Mprm => "mprm",
        }
    }

    #[inline]
    pub fn as_bytes(self) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        
        for (i, b) in self.as_str().bytes().rev().enumerate() {
            buf[i] = b;
        }
        
        return buf;
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let bytes: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
        
        Self::ALL.into_iter().find(|cmd| cmd.as_bytes() == bytes)
    }
}

const HEADER_SIZE: usize = 8;

pub struct Packet {
    id: u16,
    seq: u32,
    cmd: Option< CMD>,
    bank: Option<u32>,
    subcmd: Option<SUBCMD>,
    payload: Option<Vec<u8>>,
}
impl Packet {
    fn new(id: u16, seq: u32, cmd: Option<CMD>, bank: Option<u32>, subcmd: Option<SUBCMD>, payload: Option<Vec<u8>>) -> Self {
        Self {
            id: id,
            seq: seq,
            cmd: cmd,
            bank: bank,
            subcmd: subcmd,
            payload: payload,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut len = 0u16;
        
        buf.extend_from_slice(&[0u8; 2]);   // place holder
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.seq.to_le_bytes());
        if let Some(cmd) = self.cmd {
            len += 8;
            buf.extend_from_slice(&cmd.as_bytes());
        }
        if let Some(bank) = self.bank {
            len += 4;
            buf.extend_from_slice(&bank.to_le_bytes());
        }
        if let Some(subcmd) = self.subcmd {
            len += 4;
            buf.extend_from_slice(&subcmd.as_bytes());
        }
        if let Some(payload) = &self.payload {
            let size = (4 + payload.len()) as u16;
            len += size;
            buf.extend_from_slice(&(size as u32 + 8).to_le_bytes());
            buf.extend_from_slice(&payload);
        }
        buf[0 .. 2].copy_from_slice(&len.to_le_bytes());
        
        return buf
    }
    
    pub fn parse(data: &[u8], size: usize) -> Result<Self, String> {
        let mut b = PacketBuilder::new();        
        
        if size < HEADER_SIZE {
            return Err("Invalid format.".to_string())
        }
        { // Header
            let len = u16::from_le_bytes(data[0 .. 2].try_into().unwrap());
            b = b.id(u16::from_le_bytes(data[2..4].try_into().unwrap()));
            b = b.seq(u32::from_le_bytes(data[4 .. 8].try_into().unwrap()));
            if usize::from(len) == HEADER_SIZE {
                return Ok(b.build().unwrap());
            }
        }
        {
            b = b.cmd(CMD::from_bytes(&data[8 .. 16]).unwrap());
            b = b.bank(u32::from_le_bytes(data[16 .. 20].try_into().unwrap()));
            b = b.subcmd(SUBCMD::from_bytes(&data[20 .. 24]).unwrap());
            let len: u32 = u32::from_le_bytes(data[24 .. 28].try_into().unwrap());
            let payload: Vec<u8> = match b.subcmd {
                Some(SUBCMD::QUSt) =>
                    QUSt::parse(&data[28..], len - 8).unwrap().to_bytes().to_vec(),
                Some(SUBCMD::Pari) => 
                    Pari::parse(&data[28..], len - 8).unwrap().to_bytes().to_vec(),
                Some(SUBCMD::Mprm) =>
                    Mprm::parse(&data[28..], len - 8).unwrap().to_bytes().to_vec(),
                _ => vec![],
            };
            b = b.payload(payload);
        }
        
        b.build()
    }
}

const ID: u16 = 0x0101;
pub struct PacketBuilder {
    id: Option<u16>,
    seq: Option<u32>,
    cmd: Option<CMD>,
    bank: Option<u32>,
    subcmd: Option<SUBCMD>,
    payload: Option<Vec<u8>>,
}
impl PacketBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            seq: None,
            cmd: None,
            bank: None,
            subcmd: None,
            payload: None
        }
    }
    
    #[allow(dead_code)]
    pub fn qust(seq: u32) -> Self {
        Self::new()
            .id(ID)
            .seq(seq)
            .cmd(CMD::ApplGetP)
            .bank(0)
            .subcmd(SUBCMD::QUSt)
            .payload(QUSt::placeholder())
    }
    
    pub fn pari(seq: u32, bank: u32, payload: Vec<u8>) -> Self {
        Self::new()
            .id(ID)
            .seq(seq)
            .cmd(CMD::ApplSetP)
            .bank(bank)
            .subcmd(SUBCMD::Pari)
            .payload(payload)
    }
    
    pub fn mprm(seq: u32, payload: Vec<u8>) -> Self {
        Self::new()
            .id(ID)
            .seq(seq)
            .cmd(CMD::SetP)
            .bank(0)
            .subcmd(SUBCMD::Mprm)
            .payload(payload)
    }
    
    pub fn id(mut self, value: u16) -> Self {
        self.id = Some(value);
        self
    }

    pub fn seq(mut self, value: u32) -> Self {
        self.seq = Some(value);        
        self
    }

    pub fn cmd(mut self, value: CMD) -> Self {
        self.cmd = Some(value);        
        self
    }

    pub fn bank(mut self, value: u32) -> Self {
        self.bank = Some(value);        
        self
    }

    pub fn subcmd(mut self, value: SUBCMD) -> Self {
        self.subcmd = Some(value);        
        self
    }
    
    pub fn payload(mut self, value: Vec<u8>) -> Self {
        self.payload = Some(value);
        self
    }
    
    pub fn build(self) -> Result<Packet, String> {
        Ok(Packet::new(
            self.id.ok_or("id")?,
            self.seq.ok_or("seq")?,
            self.cmd,
            self.bank,
            self.subcmd,
            self.payload,
        ))
    }
}

#[test]
fn test_packet() {
    let payload = [0u8; 12].to_vec();
    let pkt = PacketBuilder::new()
        .id(0x0101)
        .seq(2)
        .cmd(CMD::ApplSetP)
        .bank(3)
        .subcmd(SUBCMD::Pari)
        .payload(payload)
        .build().unwrap();
    let bytes = pkt.to_bytes();
    Packet::parse(&bytes, bytes.len()).unwrap();
}