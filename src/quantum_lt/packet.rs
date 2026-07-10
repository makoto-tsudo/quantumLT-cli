use self::CmdInner::*;
use self::SubCmdInner::*;

pub struct CMD(CmdInner);
#[allow(dead_code)]
enum CmdInner {
    ApplGetP,
    ApplSetP,
    ApplRply,
    SetP,
    Rply,
}

impl AsRef<str> for CMD {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(dead_code)]
impl CMD {
    pub const APPL_GETP: CMD = CMD(ApplGetP);
    pub const APPL_SETP: CMD = CMD(ApplSetP);
    pub const APPL_RPLY: CMD = CMD(ApplRply);
    pub const SET_P: CMD = CMD(SetP);
    pub const RPLY: CMD = CMD(Rply);
    
    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            ApplGetP => "ApplGetP",
            ApplSetP => "ApplSetP",
            ApplRply => "ApplRply",
            SetP => "SetP",
            Rply => "Rply",
        }
    }
    
    #[inline]
    pub fn as_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        
        for (i, b) in self.as_str().bytes().rev().enumerate() {
            buf[i] = b;
        }
        
        return buf;
    }
}

pub struct SUBCMD(SubCmdInner);
#[allow(dead_code)]
enum SubCmdInner {
    QUSt,
    Pari,
    Mprm,    
}

impl AsRef<str> for SUBCMD {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(dead_code)]
impl SUBCMD {
    pub const QUST: SUBCMD = SUBCMD(QUSt);
    pub const PARI: SUBCMD = SUBCMD(Pari);
    pub const MPRM: SUBCMD = SUBCMD(Mprm);

    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            QUSt => "QUst",
            Pari => "Pari",
            Mprm => "mprm",
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        
        for (i, b) in self.as_str().bytes().rev().enumerate() {
            buf[i] = b;
        }
        
        return buf;
    }
}

pub struct Packet<'a> {
    id: u16,
    seq: u32,
    cmd: CMD,    
    bank: u32,
    subcmd: SUBCMD,
    payload: &'a [u8],
}
impl<'a> Packet<'a> {
    pub fn new(id: u16, seq: u32, cmd: CMD, bank: u32, subcmd: SUBCMD, payload: &'a [u8]) -> Self {
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
        let len = self.payload.len() as u16;

        buf.extend_from_slice(&(len + 28).to_le_bytes());
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.seq.to_le_bytes());
        buf.extend_from_slice(&self.cmd.as_bytes());
        buf.extend_from_slice(&self.bank.to_le_bytes());
        buf.extend_from_slice(&self.subcmd.as_bytes());
        buf.extend_from_slice(&(len as u32 + 8).to_le_bytes());
        buf.extend_from_slice(&self.payload);
        
        return buf
    }
}