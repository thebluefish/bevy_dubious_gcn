use super::{Endpoint, HidError, types::*};
use rusb::{DeviceHandle, Direction, Context, Device, UsbContext, constants::{LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_TYPE_STANDARD, LIBUSB_RECIPIENT_INTERFACE}, InterfaceDescriptor, Interface, Interfaces, ConfigDescriptor};
use std::{sync::{Arc, RwLock}, time::Duration};

/*
The HID descriptor hierarchy:
    Config
        Interface
            HID
                Endpoint In
                Endpoint Out (optional)
See 7.1 of https://www.usb.org/sites/default/files/hid1_11.pdf
*/

pub trait HidConfigDescriptor {
    /// HID devices have exactly 1 interface
    /// If this returns an error, you are not working with an HID
    fn get_hid_interface(&self) -> Result<Interface, HidError>;
    fn extract_endpoints(&self) -> Result<(Endpoint, Option<Endpoint>), HidError>;
}

impl HidConfigDescriptor for ConfigDescriptor {
    fn get_hid_interface(&self) -> Result<Interface, HidError> {
        for iface in self.interfaces() {
            for descriptor in iface.descriptors() {
                if descriptor.class_code() == 3 {
                    return Ok(iface);
                }
            }
        }

        Err(HidError::NoInterface)
    }

    fn extract_endpoints(&self) -> Result<(Endpoint, Option<Endpoint>), HidError> {
        let interface_descriptor = self.get_hid_interface()?.descriptors().next().ok_or(HidError::NoInterface)?;
        let mut descriptors = interface_descriptor.endpoint_descriptors();
        Ok((descriptors.next().ok_or(HidError::Invalid)?.into(), descriptors.next().map(|o| Endpoint::from(o))))
    }
}