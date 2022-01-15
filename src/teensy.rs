//! Provides an interface to reading/writing data to a Teensy board over raw HID.

use hidapi;

pub struct Teensy {
    handle: hidapi::HidDevice
}

impl Teensy {
    const VENDOR: u16 = 0x16C0;
    const PRODUCT: u16 = 0x0486;
    const PACKET_SIZE: usize = 64;

    pub fn new() -> Result<Self, hidapi::HidError> {
        let hidapi = hidapi::HidApi::new()?;
        let teensy = hidapi.open(Teensy::VENDOR, Teensy::PRODUCT)?;
        
        Ok(Self { handle: teensy })
    }

    pub fn write(&self, buffer: &[u8; Teensy::PACKET_SIZE]) -> hidapi::HidResult<usize> {
        Ok(self.handle.write(buffer)?)
    }

    pub fn read(&self, buffer: &mut [u8; Teensy::PACKET_SIZE]) -> hidapi::HidResult<usize> {
        Ok(self.handle.read(buffer)?)
    }
}