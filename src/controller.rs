//! Provides an abstraction of the HID communication with the Teensy hardware.
//! 
//! This module handles everything from opening the device, to sending and receiving packets.


/// Teensy vendor ID in "raw HID" mode
const VENDOR_ID: u16 = 0x16C0;
/// Teensy product ID in "raw HID" mode
const PRODUCT_ID: u16 = 0x486;