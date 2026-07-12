pub type Meter = f32;

const DATA_SIZE: usize = 162;

pub struct QUSt {
    inch: Vec<Meter>,    
    auxch: Vec<Meter>,
    outch: Vec<Meter>,
    opt1: u8,
    opt2: u8,
    version: String,
}

impl QUSt {
    pub fn new(inch: Vec<Meter>, auxch: Vec<Meter>, outch: Vec<Meter>, opt1: u8, opt2: u8, version: String) -> Self {
        assert!(inch.len() == 16);
        assert!(auxch.len() == 10);
        assert!(outch.len() == 10);
        Self {
            inch: inch,
            auxch: auxch,
            outch: outch,
            opt1: opt1,
            opt2: opt2,
            version: version,
        }
    }
    
    pub fn placeholder() -> Vec<u8> {
        vec![0u8; DATA_SIZE]
    }
    
    pub fn to_bytes(self) -> [u8; DATA_SIZE] {
        let mut buf = [0u8; DATA_SIZE];
        
        let mut offset: usize = 0;
        for ch in self.inch {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        for ch in self.auxch {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        for ch in self.outch {
            buf[offset .. offset + 4].copy_from_slice(&ch.to_le_bytes());
            offset += 4;
        }
        offset = buf.len() - 18;
        buf[offset] = self.opt1;
        buf[offset + 1] = self.opt2;
        buf[offset + 2 .. offset + 14].copy_from_slice(self.version.to_string().as_bytes());
        
        buf
    }
    
    pub fn parse(data: &[u8], size: u32) -> Option<Self> {
        if size < DATA_SIZE as u32 {
            return None;
        }
        let mut inch: Vec<Meter> = vec![];
        let mut auxch: Vec<Meter> = vec![];
        let mut outch: Vec<Meter> = vec![];
        for ch in 0 .. 16 {
            let offset = ch * 4;
            inch.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        for ch in 16 .. 26 {
            let offset = ch * 4;
            auxch.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        for ch in 26 .. 36 {
            let offset = ch * 4;
            outch.push(f32::from_le_bytes(data[offset .. offset + 4].try_into().unwrap()));
        }
        let opt1 = data[DATA_SIZE - 18];
        let opt2 = data[DATA_SIZE - 17];
        let version = String::from_utf8(data[DATA_SIZE - 16 .. DATA_SIZE - 4].to_vec()).unwrap();
        
        Some(Self::new(
            inch,
            auxch,
            outch,
            opt1,
            opt2,
            version,
        ))
    }
}

#[test]
fn test_qust() {
    let bytes = QUSt::new(
        [0f32; 16].to_vec(),
        [0f32; 10].to_vec(),
        [0f32; 10].to_vec(),
        0,
        0,
        "this.is.test".to_string(),
    ).to_bytes();
    QUSt::parse(&bytes, bytes.len() as u32).unwrap();
}