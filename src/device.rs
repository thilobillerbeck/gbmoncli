use rusb::{Context, Device, DeviceHandle, UsbContext};

// GIGABYTE G27Q USB
const VID: u16 = 0x2109;
const PID: u16 = 0x8883;

pub fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => {
                    return Some((device, handle))
                },
                Err(_) => continue,
            }
        }
    }
    return None
}

pub fn init_handle() -> DeviceHandle<Context> {
    let mut context = Context::new().expect("Error creating context");
    let (mut _device, mut handle) =
        open_device(&mut context, VID, PID).expect("Failed to open USB device");
    handle
}