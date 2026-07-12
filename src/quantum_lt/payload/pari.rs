#[derive(Clone, Copy)]
pub struct Pari {
    channel: u32,
    bus: u32,
    value: u32,
}

impl Pari {
    pub fn new(channel: u32, bus: u32, value: u32) -> Self {
        Self { channel, bus, value }
    }
    
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        
        buf[0 .. 4].copy_from_slice(&self.channel.to_le_bytes());
        buf[4 .. 8].copy_from_slice(&self.bus.to_le_bytes());
        buf[8 .. 12].copy_from_slice(&self.value.to_le_bytes());
        
        return buf;
    }
    
    pub fn parse(data: &[u8], size: u32) -> Option<Self> {
        if size < 12 {
            return None;
        }
        let channel = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let bus = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let value = u32::from_le_bytes(data[8..12].try_into().unwrap());
        
        Some(Self::new(channel, bus, value))
    }
}

#[test]
fn test_pari() {
    let bytes = Pari::new(0, 0, 0).to_bytes();
    Pari::parse(&bytes, bytes.len() as u32).unwrap();
}