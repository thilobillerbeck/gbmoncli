use core::time;
use std::result::Result;
use std::{thread, time::Duration};
use rusb::{DeviceHandle, UsbContext};
use serde::{Deserialize, Serialize};

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
    set_value(handle, CMD_BRIGHTNESS, value, 100, "Brightness")
}

pub fn set_contrast<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_CONTRAST, value, 100, "Contrast")
}

pub fn set_sharpness<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_SHARPNESS, value, 10, "Sharpness")
}

pub fn set_freesync<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_FREESYNC, value, 1, "Freesync")
}

pub fn set_black_equalizer<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_BLACK_EQUALIZER, value, 20, "Black Equalizer")
}

pub fn set_osd_transparency<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_OSD_TRANSPARENCY, value, 4, "OSD transparency")
}

pub fn set_osd_time<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_OSD_TIME, value, 6, "OSD display time")
}

pub fn set_color_vibrance<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_COLOR_VIBRANCE, value, 20, "Color Vibrance")
}

pub fn set_low_blue_light<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_LOW_BLUE_LIGHT, value, 10, "Low Blue Light")
}

pub fn set_super_resolution<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_SUPER_RESOLUTION, value, 4, "Super Resolution")
}

pub fn set_gamma<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_GAMMA, value, 5, "Gamma")
}

pub fn set_color_temperature<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_COLOR_TEMPERATURE, value, 3, "Color Temperature")
}

pub fn set_overdrive<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_OVERDRIVE, value, 2, "Overdrive")
}

pub fn set_red<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_RED, value, 100, "Red")
}

pub fn set_green<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_GREEN, value, 100, "Green")
}

pub fn set_blue<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_BLUE, value, 100, "Blue")
}

pub fn set_input<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_INPUT, value, 2, "Input")
}

pub fn set_qm_up<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_QM_UP, value, 7, "Quick Menu Up")
}

pub fn set_qm_left<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_QM_LEFT, value, 7, "Quick Menu Left")
}

pub fn set_qm_down<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_QM_DOWN, value, 7, "Quick Menu Down")
}

pub fn set_qm_right<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    set_value(handle, CMD_QM_RIGHT, value, 7, "Quick Menu Right")
}

pub fn switch_profile<T: UsbContext>(handle: &mut DeviceHandle<T>, value: u8) -> Result<(), String> {
    write_value_control(handle, CMD_SWITCH_PROFILE, value)
}

fn set_value<T: UsbContext>(handle: &mut DeviceHandle<T>, cmd: [u8; 2], value: u8, max_value: u8, option_name: &str) -> Result<(), String> {
    if value <= max_value {
        write_value_control(handle, cmd, value)
    } else {
        Err(format!("{} must be between 0 and {}", option_name, max_value))
    }
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
#[derive(Serialize, Deserialize)]
struct Settings {
    brightness: u8,
    contrast: u8,
    gamma: u8,
    black_equalizer: u8,
    color_vibrance: u8,
    color_temperature: u8,
    sharpness: u8,
    low_blue_light: u8,
    super_resolution: u8,
    overdrive: u8,
}

pub fn read_values_json<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<String, String> {
    let buffers = read_values(handle)?;

    let settings = Settings {
        brightness: buffers.0[14],
        color_vibrance: buffers.0[16],
        color_temperature: buffers.0[17],
        contrast: buffers.0[18],
        sharpness: buffers.1[22],
        gamma: buffers.0[25],
        low_blue_light: buffers.1[10],
        super_resolution: buffers.1[11],
        overdrive: buffers.1[12],
        black_equalizer: buffers.0[13],
    };

    Ok(serde_json::to_string_pretty(&settings).unwrap())
}

pub fn print_values<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<(), String> {
    println!("{}", read_values_json(handle).unwrap());

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