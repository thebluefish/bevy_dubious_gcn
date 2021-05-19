use usb_shenanigans::prelude::*;
use log::*;
pub use rusb::{Context, UsbContext};
use rusb::Device;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Debug).init()?;

    // println!("{:x}\n{:x}\n{:x}", 0x0200u16, u16::from_le_bytes([0x00, 0x02]), u16::from_be_bytes([0x02, 0x00]));

    let ctx = Context::new()?;
    let devices = ctx.devices()?;

    for device in devices.iter() {
        let desc = device.device_descriptor()?;
        device.active_config_descriptor()?;
        if desc.vendor_id() == 0x057e {
            match desc.product_id() {
                0x0337 => {
                    handle_wup28(device)?;
                    break;
                },
                0x2009 => {
                    // handle_switch_pro(device)?;
                },
                _ => debug!("unknown nintendo device {:x}", desc.product_id()),
            }
        }
    }

    Ok(())
}

fn handle_wup28(device: Device<Context>) -> anyhow::Result<()> {
    let handle = device.open()?;

    // println!("WUP: {:02x?}", handle.get_configuration()?);
    // println!("{:02x?}", handle.get_hid_descriptor(0, 0xd6)?);
    println!("gcn: {:02x?}", handle.get_hid_descriptor(256)?);
    // let iface = handle.device().active_config_descriptor()?.get_hid_interface()?;
    println!("iface");
    let (ep_in, ep_out) = handle.device().active_config_descriptor()?.extract_endpoints()?;
    println!("in: {:?}\nout: {:?}", ep_in, ep_out);


    Ok(())
}
