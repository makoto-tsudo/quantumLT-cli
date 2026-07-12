#[derive(Clone, Copy)]
pub struct Mixer {
    channel: u16,
    bus: u8,
    position: u8,
    value: f32,
}
impl Mixer {
    pub fn new(channel: u16, bus: u8, position: u8, value: f32) -> Self {
        Self {
            channel: channel,
            bus: bus,
            position: position,
            value: value,
        }
    }

    fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        
        buf[0..2].copy_from_slice(&self.channel.to_le_bytes());
        buf[2] = self.bus;
        buf[3] = self.position;
        buf[4..8].copy_from_slice(&self.value.to_le_bytes());
        
        return buf;
    }
    
    pub fn parse(data: &[u8], size: usize) -> Option<Self> {
        if size < 8 {
            return None;
        }
        
        let channel = u16::from_le_bytes(data[0 .. 2].try_into().unwrap());
        let bus = data[2];
        let pos = data[3];
        let value = f32::from_le_bytes(data[4 .. 8].try_into().unwrap());
        
        Some(Self::new(channel, bus, pos, value))
    }
}

const DATA_SIZE: usize = 488;
const MAX_MIXSERS: usize = 60;
pub struct Mprm {
    mixers: Vec<Mixer>,
}

impl Mprm {
    pub fn new(mixers: Vec<Mixer>) -> Self {
        assert!(mixers.len() <= MAX_MIXSERS);
        Self { mixers: mixers }
    }

    pub fn to_bytes(&self) -> [u8; DATA_SIZE] {
        let mut buf = [0u8; DATA_SIZE];
        let mut offset: usize = 4;
        for m in &self.mixers {
            buf[offset..offset+8].copy_from_slice(&m.to_bytes());
            offset += 8;
        }
        offset = buf.len() - 4;
        buf[offset .. ].copy_from_slice(&((self.mixers.len() as u32).to_le_bytes()));
        
        return buf;
    }
    
    pub fn parse(data: &[u8], size: u32) -> Option<Self> {
        if size < DATA_SIZE as u32 {
            return None;
        }
        let mut mixers: Vec<Mixer> = vec![];
        
        for idx in 0 .. MAX_MIXSERS {
            let offset = idx * 8;
            mixers.push(Mixer::parse(&data[offset .. offset + 8], 8).unwrap());
        }
        
        Some(Self::new(mixers))
    }
}

#[test]
fn test_mprm() {
    let mut mixers: Vec<Mixer> = vec![];
    mixers.push(Mixer::new(0, 0, 0, -145f32));
    let mprm = Mprm::new(mixers);
    let bytes = mprm.to_bytes();
    Mprm::parse(&bytes, bytes.len() as u32).unwrap();
}