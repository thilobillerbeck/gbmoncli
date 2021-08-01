mod control;
mod device;

use std::result::Result;
use rusb::{Context};
use structopt::StructOpt;

// GIGABYTE G27Q USB
const VID: u16 = 0x2109;
const PID: u16 = 0x8883;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gbmoncli",
    about = "A CLI to control your Gigabyte G27Q monitor.",
    after_help = "Can only execute one attribute at a time",
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
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    let mut context = Context::new().expect("Error creating context");
    let (mut _device, mut handle) =
        device::open_device(&mut context, VID, PID).expect("Failed to open USB device");    

    match opt.brightness {
        Some(value) => { control::set_brightness(&mut handle, value)?; },
        None => (),
    }

    match opt.contrast {
        Some(value) => { control::set_contrast(&mut handle, value)?; },
        None => (),
    }

    match opt.sharpness {
        Some(value) => { control::set_sharpness(&mut handle, value)?; },
        None => (),
    }

    match opt.freesync {
        Some(value) => { control::set_freesync(&mut handle, value as u8)?; },
        None => (),
    }

    match opt.black_equalizer {
        Some(value) => { control::set_black_equalizer(&mut handle, value)?; },
        None => (),
    }

    match opt.osd_transparency {
        Some(value) => { control::set_osd_transparency(&mut handle, value)?; },
        None => (),
    }

    match opt.osd_time {
        Some(value) => { control::set_osd_time(&mut handle, value)?; },
        None => (),
    }

    match opt.profile {
        Some(value) => { 
            match value.as_str() {
                "standard" => { control::switch_profile(&mut handle, 0)? },
                "fps" => { control::switch_profile(&mut handle, 1)? },
                "rtsrpg" => { control::switch_profile(&mut handle, 2)? },
                "movie" => { control::switch_profile(&mut handle, 3)? },
                "reader" => { control::switch_profile(&mut handle, 4)? },
                "srgb" => { control::switch_profile(&mut handle, 5)? },
                "custom1" => { control::switch_profile(&mut handle, 6)? },
                "custom2" => { control::switch_profile(&mut handle, 7)? },
                "custom3" => { control::switch_profile(&mut handle, 8)? },
                _ => return Err("Please use a valid profile name".to_string()),
            };
        },
        None => (),
    }

    Ok(())
}