#[repr(C, packed)]
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
        
        buf[0..1].copy_from_slice(&self.channel.to_le_bytes());
        buf[2] = self.bus;
        buf[3] = self.position;
        buf[4..7].copy_from_slice(&self.value.to_le_bytes());
        
        return buf;
    }
}

pub struct Mprm {
    mixers: Vec<Mixer>,
}

impl<'a> Mprm {
    pub fn new(mixers: Vec<Mixer>) -> Self {
        Self { mixers: mixers }
    }

    pub fn to_bytes(&self) -> [u8; 488] {
        let mut buf = [0u8; 488];
        let offset: usize = 4;
        for m in &self.mixers {
            buf[offset..offset+3].copy_from_slice(&m.to_bytes());
        }
        buf[484..487].copy_from_slice(&((self.mixers.len() as u32).to_le_bytes()));
        
        return buf;
    }
}