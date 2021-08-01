use core::time;
use std::result::Result;
use std::{thread, time::Duration};
use rusb::{DeviceHandle, Error, UsbContext};

// VALUE COMMAND ARGS
const CMD_HEADER_REQUEST_TYPE: u8 = 0x40;
const CMD_HEADER_REQUEST: u8 = 178;
const CMD_HEADER_VALUE: u16 = 0x0000;
const CMD_HEADER_INDEX: u16 = 0x0000;

const CMD_PREFIX: [u8; 4] = [0x6e, 0x51, 0x84, 0x03];

const CMD_BRIGHTNESS: [u8; 2] = [0x10, 0x00];
const CMD_CONTRAST: [u8; 2] = [0x12, 0x00];
const CMD_SHARPNESS: [u8; 2] = [0x87, 0x00];
const CMD_FREESYNC: [u8; 2] = [0xe0, 0x0c];
const CMD_BLACK_EQUALIZER: [u8; 2] = [0xe0, 0x02];
const CMD_SWITCH_PROFILE: [u8; 2] = [0xe0, 0x2c];
const CMD_OSD_TRANSPARENCY: [u8; 2] = [0xe0, 0x2f];
const CMD_OSD_TIME: [u8; 2] = [0xe0, 0x30];

pub fn set_brightness<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 100 {
        write_value_control(handle, CMD_BRIGHTNESS, value)
    } else {
        Err("Brightness must be between 0 and 100".to_string())
    }
}

pub fn set_contrast<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 100 {
        write_value_control(handle, CMD_CONTRAST, value)
    } else {
        Err("Contrast must be between 0 and 100".to_string())
    }
}

pub fn set_sharpness<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 10 {
        write_value_control(handle, CMD_SHARPNESS, value)
    } else {
        Err("Sharpness must be between 0 and 10".to_string())
    }
}

pub fn set_freesync<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 1 {
        write_value_control(handle, CMD_FREESYNC, value)
    } else {
        Err("Freesync must be between 0 and 1".to_string())
    }
}

pub fn set_black_equalizer<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 20 {
        write_value_control(handle, CMD_BLACK_EQUALIZER, value)
    } else {
        Err("Black Equalizer must be between 0 and 20".to_string())
    }
}

pub fn set_osd_transparency<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 4 {
        write_value_control(handle, CMD_OSD_TRANSPARENCY, value)
    } else {
        Err("OSD transparency must be between 0 and 4".to_string())
    }
}

pub fn set_osd_time<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 6 {
        write_value_control(handle, CMD_OSD_TIME, value)
    } else {
        Err("OSD display time must be between 0 and 6".to_string())
    }
}

pub fn switch_profile<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    write_value_control(handle, CMD_SWITCH_PROFILE, value)
}

fn write_value_control<T: UsbContext>(handle: &mut DeviceHandle<T>, cmd: [u8; 2], value: u8) -> Result<(), String> {
    wait(); // leave a bit of time between commands
    let result = handle.write_control(CMD_HEADER_REQUEST_TYPE, CMD_HEADER_REQUEST, CMD_HEADER_VALUE, CMD_HEADER_INDEX, &[CMD_PREFIX[0], CMD_PREFIX[1], CMD_PREFIX[2], CMD_PREFIX[3], cmd[0], cmd[1], value], Duration::from_millis(100));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn wait() {
    thread::sleep(time::Duration::from_millis(50));
}