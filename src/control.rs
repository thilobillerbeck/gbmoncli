use core::time;
use std::result::Result;
use std::{thread, time::Duration};
use rusb::{DeviceHandle, UsbContext};

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
const CMD_COLOR_VIBRANCE: [u8; 2] = [0xe0, 0x08];
const CMD_LOW_BLUE_LIGHT: [u8; 2] = [0xe0, 0x0b];
const CMD_SUPER_RESOLUTION: [u8; 2] = [0xe0, 0x0d];
const CMD_GAMMA: [u8; 2] = [0xe0, 0x07];
const CMD_COLOR_TEMPERATURE: [u8; 2] = [0xe0, 0x03];
const CMD_OVERDRIVE: [u8; 2] = [0xe0, 0x09];
const CMD_RED: [u8; 2] = [0xe0, 0x04];
const CMD_GREEN: [u8; 2] = [0xe0, 0x05];
const CMD_BLUE: [u8; 2] = [0xe0, 0x06];
const CMD_INPUT: [u8; 2] = [0xe0, 0x2d];
const CMD_QM_UP: [u8; 2] = [0xe0, 0x32];
const CMD_QM_DOWN: [u8; 2] = [0xe0, 0x33];
const CMD_QM_RIGHT: [u8; 2] = [0xe0, 0x34];
const CMD_QM_LEFT: [u8; 2] = [0xe0, 0x35];

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

pub fn set_color_vibrance<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 20 {
        write_value_control(handle, CMD_COLOR_VIBRANCE, value)
    } else {
        Err("Color Vibrance must be between 0 and 20".to_string())
    }
}

pub fn set_low_blue_light<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 10 {
        write_value_control(handle, CMD_LOW_BLUE_LIGHT, value)
    } else {
        Err("Low Blue Light must be between 0 and 10".to_string())
    }
}

pub fn set_super_resolution<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 4 {
        write_value_control(handle, CMD_SUPER_RESOLUTION, value)
    } else {
        Err("Super Resolution must be between 0 and 4".to_string())
    }
}

pub fn set_gamma<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 5 {
        write_value_control(handle, CMD_GAMMA, value)
    } else {
        Err("Gamma must be between 0 and 5".to_string())
    }
}

pub fn set_color_temperature<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 3 {
        write_value_control(handle, CMD_COLOR_TEMPERATURE, value)
    } else {
        Err("Color Temperature must be between 0 and 3".to_string())
    }
}

pub fn set_overdrive<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 2 {
        write_value_control(handle, CMD_OVERDRIVE, value)
    } else {
        Err("Overdrive must be between 0 and 2".to_string())
    }
}

pub fn set_red<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 100 {
        write_value_control(handle, CMD_RED, value)
    } else {
        Err("Red must be between 0 and 2".to_string())
    }
}

pub fn set_green<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 100 {
        write_value_control(handle, CMD_GREEN, value)
    } else {
        Err("Green must be between 0 and 2".to_string())
    }
}

pub fn set_blue<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 100 {
        write_value_control(handle, CMD_BLUE, value)
    } else {
        Err("Blue must be between 0 and 2".to_string())
    }
}

pub fn set_input<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 2 {
        write_value_control(handle, CMD_INPUT, value)
    } else {
        Err("Input be between 0 and 2".to_string())
    }
}

pub fn set_qm_up<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 7 {
        write_value_control(handle, CMD_QM_UP, value)
    } else {
        Err("Quick Menu Up must be between 0 and 7".to_string())
    }
}

pub fn set_qm_left<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 7 {
        write_value_control(handle, CMD_QM_LEFT, value)
    } else {
        Err("Quick Menu Left must be between 0 and 7".to_string())
    }
}

pub fn set_qm_down<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 7 {
        write_value_control(handle, CMD_QM_DOWN, value)
    } else {
        Err("Quick Menu Down must be between 0 and 7".to_string())
    }
}

pub fn set_qm_right<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    if value <= 7 {
        write_value_control(handle, CMD_QM_RIGHT, value)
    } else {
        Err("Quick Menu Right must be between 0 and 7".to_string())
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

/*
BLACK EQUALIZER: 0 : 13
BRIGHTNESS: 0 : 14
COLOR_VIBRANCE: 0 : 16
COLOR_TEMPERATURE: 0 : 17
CONTRAST: 0 : 18
SHARPNESS: 0 : 22
GAMMA: 0 : 25
LOW BLUE LIGHT: 1 : 10
SUPER RESOLUTION: 1 : 11
OVERDRIVE: 1 : 12
OSD TIME (IN SECS): 1 : 13
QM UP: 1 : 18
QM DOWN: 1 : 19 
QM RIGHT: 1 : 20
QM LEFT: 1 : 21
OSD TRANSPARENCY: 1 : 23 

// RGB NOT IN THESE TWO ARRAYS
// INPUT EITHER
*/

pub fn print_values<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<(), String> {
    let buffers = read_values(handle)?;

    println!("BRIGHTNESS: {}\nCONTRAST: {}\nGAMMA: {}", buffers.0[14], buffers.0[18], buffers.0[25]);

    Ok(())
}

pub fn read_values<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<([u8; 36], [u8; 36]), String> {
    wait(); // leave a bit of time between commands
    let mut buf1: [u8; 36] = [0; 36];
    let mut buf2: [u8; 36] = [0; 36];

    // TODO: Error handling
    let mut result = handle.write_control(CMD_HEADER_REQUEST_TYPE, CMD_HEADER_REQUEST, CMD_HEADER_VALUE, CMD_HEADER_INDEX, &[0x6E, 0x51, 0x83, 0x01, 0xE0, 0x50], Duration::from_millis(100));
    wait();
    result = handle.read_control(0xc0, 162, 0x0000, 111,  &mut buf1, Duration::from_millis(1000));
    wait();
    result = handle.write_control(CMD_HEADER_REQUEST_TYPE, CMD_HEADER_REQUEST, CMD_HEADER_VALUE, CMD_HEADER_INDEX, &[0x6E, 0x51, 0x83, 0x01, 0xE0, 0x51], Duration::from_millis(100));
    wait();
    result = handle.read_control(0xc0, 162, 0x0000, 111,  &mut buf2, Duration::from_millis(1000));
    wait();

    #[cfg(debug_assertions)] {
        println!("{:?}", buf1);
        println!("{:?}", buf2);
    }

    match result {
        Ok(_) => Ok((buf1, buf2)),
        Err(e) => Err(e.to_string())
    }
}

fn wait() {
    thread::sleep(time::Duration::from_millis(50));
}