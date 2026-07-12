use std::{time::Duration};
use rusb::{Device, DeviceHandle, Error, GlobalContext, UsbContext};

mod packet;
pub use packet::*;

mod payload;
pub use payload::*;

const VENDOR_ID: u16  = 0x1ed8;
const PRODUCT_ID: u16 = 0x0206;
const IFACE: u8 = 0x05;
const EP: u8 = 0x01;
const ALT_SETTING: u8 = 0x01;
const TIMEOUT: Duration = Duration::from_millis(20);

pub fn search(serial: String) -> Result<Device<GlobalContext>, Error> {
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;
        if desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID {
            if let Ok(handle) = dev.open() {
                if let Ok(s) = handle.read_serial_number_string_ascii(&desc) {
                    if s == serial {
                        return Ok(dev)
                    }
                }
            }
        }
    }
    return Err(Error::NoDevice)
}

fn send_packet<T: UsbContext>(handle: &mut DeviceHandle<T>, pkt: &Packet) -> Result<Packet, Error> {
    let wbuf = pkt.to_bytes();
    let mut rbuf: Vec<u8> = vec![0; wbuf.len()];
    let wsize = handle.write_bulk(EP, wbuf.as_slice(), TIMEOUT)?;
    println!("write packet: {} bytes", wsize);
    let rsize = handle.read_bulk(EP, rbuf.as_mut_slice(), TIMEOUT)?;
    println!("read packet: {} bytes", rsize);
    let pkt = Packet::parse(&rbuf, rsize).unwrap();
    
    Ok(pkt)
}

fn send_init_packets<T: UsbContext>(handle: &mut DeviceHandle<T>, seq: &mut u32) -> Result<(), Error> {
    let mut mixers: Vec<Mixer> = vec![];
    {   // Pari enable mixer
        *seq += 1;
        let payload = Pari::new(0, 0, 4).to_bytes();
        let pkt = PacketBuilder::pari(*seq, 0, payload.to_vec())
            .build().unwrap();
        send_packet(handle, &pkt)?;
    }

    for bus in 0 .. 4u8 {
        for pos in 0..1u8 {
            for ch in 0 .. 25u8 {
                let mut value: f32 = -145f32;    // Mute
                if ch < 16 {
                    // Input 
                } else {
                    // DAW
                    if bus == 4 || bus == ((ch - 16) >> 1) {
                        // stereo bus or loop
                        if pos == (ch & 1) {
                            // L / R panned
                            value = 0f32;
                        }
                    }
                }
                mixers.push(Mixer::new(ch.into(), bus, pos, value));
                if mixers.len() == 60 {
                    *seq += 1;
                    let payload = Mprm::new(mixers.clone()).to_bytes();
                    let pkt = PacketBuilder::mprm(*seq, payload.to_vec())
                        .build().unwrap();
                    send_packet(handle, &pkt)?;
                    mixers.clear();
                }
            }
        }
    }
    if mixers.len() > 0 {
        let payload = Mprm::new(mixers.clone()).to_bytes();
        *seq += 1;
        let mut pkt = PacketBuilder::mprm(*seq, payload.to_vec())
            .build().unwrap();
        send_packet(handle, &mut pkt)?;
        mixers.clear();
    }

    {   // Pari disable startup
        *seq += 1;
        let payload = Pari::new(0, 1, 0).to_bytes();
        let pkt = PacketBuilder::pari(*seq, 0, payload.to_vec())
            .build().unwrap();
        send_packet(handle, &pkt)?;
    }
    for ch in 0..15u32 {
        *seq += 1;
        let payload = Pari::new(ch, 0, 0).to_bytes();
        let pkt = PacketBuilder::pari(*seq, 1, payload.to_vec())
            .build().unwrap();
        send_packet(handle, &pkt)?;
    }
        
    Ok(())
}

pub fn init<T: UsbContext>(dev: Device<T>) -> Result<(), Error> {
    let mut handle = dev.open()?;
    let mut seq = 0u32;
    handle.claim_interface(IFACE)?;
    handle.set_alternate_setting(IFACE, ALT_SETTING)?;

    send_init_packets(&mut handle, &mut seq)?;
    
    return Ok(());
}