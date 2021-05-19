use super::{HidConfigDescriptor, HidError, types::*};
use rusb::{DeviceHandle, Direction, Context, Device, UsbContext, constants::{LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_TYPE_STANDARD, LIBUSB_RECIPIENT_INTERFACE}, InterfaceDescriptor, Interface, Interfaces};
use std::{sync::{Arc, RwLock}, time::Duration};

pub trait HidDevice {
    /// Supported by all HID devices
    fn get_hid_report(&self, report_type: ReportType, report_id: u8, interface: u16, buffer_len: usize, timeout: Duration) -> Result<Vec<u8>, HidError>;
    fn get_hid_descriptor(&self, length: usize) -> Result<Vec<u8>, HidError>;
    fn get_hid_descriptor_ext(&self, interface: u16, length: usize, timeout: Duration) -> Result<Vec<u8>, HidError>;
    // fn extract_endpoints(&self) -> Result<(Endpoint, Option<Endpoint>), HidError>;
}

impl<T: UsbContext> HidDevice for DeviceHandle<T> {

    fn get_hid_report(&self, report_type: ReportType, report_id: u8, interface: u16, buffer_len: usize, timeout: Duration) -> Result<Vec<u8>, HidError> {
        let mut buf = vec![0; buffer_len];

        let len = self.read_control(
            RequestType::Read as u8,
            Request::GetReport as u8,
            u16::from_be_bytes([report_type as u8, report_id]),
            interface,
            &mut buf,
            timeout,
        )?;
        buf.resize(len, 0);

        Ok(buf)
    }

    fn get_hid_descriptor(&self, length: usize) -> Result<Vec<u8>, HidError> {
        self.get_hid_descriptor_ext(self.device().active_config_descriptor()?.get_hid_interface()?.number() as u16, length, Duration::from_secs(1))
    }

    fn get_hid_descriptor_ext(&self, interface: u16, length: usize, timeout: Duration) -> Result<Vec<u8>, HidError> {
        let mut buf = vec![0; length];
        let len = self.read_control(
            LIBUSB_ENDPOINT_IN | LIBUSB_REQUEST_TYPE_STANDARD | LIBUSB_RECIPIENT_INTERFACE,
            GetDescriptor as u8,
            u16::from_be_bytes([ReportDescriptor as u8, 0]),
            interface,
            &mut buf,
            timeout,
        )?;
        println!("read {}", len);
        buf.resize(len, 0);

        Ok(buf)
    }
}