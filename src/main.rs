mod control;
mod device;

use std::result::Result;
use rusb::{Context, DeviceHandle, UsbContext};
use structopt::StructOpt;

// GIGABYTE G27Q USB
const VID: u16 = 0x2109;
const PID: u16 = 0x8883;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gbmoncli",
    about = "A CLI to control your Gigabyte G27Q monitor.",
    author = "Thilo Billerbeck <thilo.billerbeck@officerent.de>"
)]
struct Opt {
    #[structopt(short, long, help("Change monitor brightness"), value_name("0-100"))]
    brightness: Option<u8>,

    #[structopt(short, long, help("Change monitor contrast"), value_name("0-100"))]
    contrast: Option<u8>,

    #[structopt(short, long, help("Change monitor sharpness"), value_name("0-10"))]
    sharpness: Option<u8>,

    #[structopt(short, long, help("Turn AMD Freesync on/off"), value_name("true|false"))]
    freesync: Option<bool>,

    #[structopt(short("e"), long, help("Change monitor black equalizer"), value_name("0-20"))]
    black_equalizer: Option<u8>,

    #[structopt(long, help("Set the monitors OSD transparency"), value_name("0-4"))]
    osd_transparency: Option<u8>,

    #[structopt(long, help("Set the monitors OSD disappearing time (5 second increments)"), value_name("0-6"))]
    osd_time: Option<u8>,

    #[structopt(short, long, help("Change the monitor profile"), value_name("standard|fps|rtsrpg|movie|reader|srgb|custom1|custom2|custom3"))]
    profile: Option<String>,

    #[structopt(long, value_name("0-20"))]
    color_vibrance: Option<u8>,

    #[structopt(long, value_name("0-10"))]
    low_blue_light: Option<u8>,

    #[structopt(long, value_name("0-4"))]
    super_resolution: Option<u8>,

    #[structopt(short, long, value_name("0-5"))]
    gamma: Option<u8>,

    #[structopt(short("t"), long, value_name("0-3"))]
    color_temperature: Option<u8>,

    #[structopt(short, long, value_name("0-2"))]
    overdrive: Option<u8>,

    #[structopt(long, value_name("0-100"))]
    red: Option<u8>,

    #[structopt(long, value_name("0-100"))]
    green: Option<u8>,

    #[structopt(long, value_name("0-100"))]
    blue: Option<u8>,

    #[structopt(short, long, value_name("0-2"))]
    input: Option<u8>,

    #[structopt(long, value_name("0-2"))]
    qm_up: Option<u8>,

    #[structopt(long, value_name("0-2"))]
    qm_left: Option<u8>,

    #[structopt(long, value_name("0-2"))]
    qm_down: Option<u8>,

    #[structopt(long, value_name("0-2"))]
    qm_right: Option<u8>,

    #[structopt(long, help("Print monitors values"), value_name("true|false"))]
    print: Option<bool>,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    let mut context = Context::new().expect("Error creating context");
    let (mut _device, mut handle) =
        device::open_device(&mut context, VID, PID).expect("Failed to open USB device");    

    #[cfg(debug_assertions)]
    let bufbefore = control::read_values(&mut handle)?;

    process_args(opt, &mut handle)?;

    #[cfg(debug_assertions)]
    let bufafter = control::read_values(&mut handle)?;
    
    #[cfg(debug_assertions)]
    compare_monitor_output(bufbefore, bufafter);

    Ok(())
}

fn process_args<T: UsbContext>(opt: Opt, handle: &mut DeviceHandle<T>) -> Result<(), String> {
    match opt.print {
        Some(_) => { control::print_values(handle)?; },
        None => (),
    }

    match opt.brightness {
        Some(value) => { control::set_brightness(handle, value)?; },
        None => (),
    }

    match opt.contrast {
        Some(value) => { control::set_contrast(handle, value)?; },
        None => (),
    }

    match opt.sharpness {
        Some(value) => { control::set_sharpness(handle, value)?; },
        None => (),
    }

    match opt.freesync {
        Some(value) => { control::set_freesync(handle, value as u8)?; },
        None => (),
    }

    match opt.black_equalizer {
        Some(value) => { control::set_black_equalizer(handle, value)?; },
        None => (),
    }

    match opt.osd_transparency {
        Some(value) => { control::set_osd_transparency(handle, value)?; },
        None => (),
    }

    match opt.osd_time {
        Some(value) => { control::set_osd_time(handle, value)?; },
        None => (),
    }

    match opt.color_vibrance {
        Some(value) => { control::set_color_vibrance(handle, value)?; },
        None => (),
    }

    match opt.low_blue_light {
        Some(value) => { control::set_low_blue_light(handle, value)?; },
        None => (),
    }

    match opt.super_resolution {
        Some(value) => { control::set_super_resolution(handle, value)?; },
        None => (),
    }

    match opt.gamma {
        Some(value) => { control::set_gamma(handle, value)?; },
        None => (),
    }

    match opt.color_temperature {
        Some(value) => { control::set_color_temperature(handle, value)?; },
        None => (),
    }

    match opt.overdrive {
        Some(value) => { control::set_overdrive(handle, value)?; },
        None => (),
    }

    match opt.red {
        Some(value) => { control::set_red(handle, value)?; },
        None => (),
    }

    match opt.green {
        Some(value) => { control::set_green(handle, value)?; },
        None => (),
    }

    match opt.blue {
        Some(value) => { control::set_blue(handle, value)?; },
        None => (),
    }

    match opt.input {
        Some(value) => { control::set_input(handle, value)?; },
        None => (),
    }

    match opt.qm_up {
        Some(value) => { control::set_qm_up(handle, value)?; },
        None => (),
    }

    match opt.qm_left {
        Some(value) => { control::set_qm_left(handle, value)?; },
        None => (),
    }

    match opt.qm_down {
        Some(value) => { control::set_qm_down(handle, value)?; },
        None => (),
    }

    match opt.qm_right {
        Some(value) => { control::set_qm_right(handle, value)?; },
        None => (),
    }

    match opt.profile {
        Some(value) => { 
            match value.as_str() {
                "standard" => control::switch_profile(handle, 0)?,
                "rtsrpg" => control::switch_profile(handle, 2)?,
                "fps" => control::switch_profile(handle, 1)?,
                "movie" => control::switch_profile(handle, 3)?,
                "reader" => control::switch_profile(handle, 4)?,
                "srgb" => control::switch_profile(handle, 5)?,
                "custom1" => control::switch_profile(handle, 6)?,
                "custom2" => control::switch_profile(handle, 7)?,
                "custom3" => control::switch_profile(handle, 8)?,
                _ => return Err("Please use a valid profile name".to_string()),
            };
        },
        None => (),
    }

    Ok(())
}

fn compare_monitor_output(bufbefore: ([u8; 36], [u8; 36]), bufafter: ([u8; 36], [u8; 36])) {
    for (i, x) in bufbefore.0.iter().enumerate() {
        if *x != bufafter.0[i] {
            println!("0 : {} CHANGED FROM {} TO {}", i, x, bufafter.0[i]);
        }
    }

    for (i, x) in bufbefore.1.iter().enumerate() {
        if *x != bufafter.1[i] {
            println!("1 : {} CHANGED FROM {} TO {}", i, x, bufafter.1[i]);
        }
    }
}