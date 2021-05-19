use rusb::constants::{LIBUSB_ENDPOINT_OUT, LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_TYPE_CLASS, LIBUSB_RECIPIENT_INTERFACE};

pub use rusb::constants::LIBUSB_REQUEST_GET_DESCRIPTOR as GetDescriptor;
pub use rusb::constants::LIBUSB_DT_REPORT as ReportDescriptor;

/// Supported HID requests
/// See 7.2 of https://www.usb.org/sites/default/files/hid1_11.pdf
#[derive(Debug)]
#[repr(u8)]
pub enum Request {
    GetReport   = 0x1,
    GetIdle     = 0x2,
    GetProtocol = 0x3,
    SetReport   = 0x9,
    SetIdle     = 0xA,
    SetProtocol = 0xB,
}

/// HID requests can only be one of these two
#[repr(u8)]
pub enum RequestType {
    Write = LIBUSB_ENDPOINT_OUT | LIBUSB_REQUEST_TYPE_CLASS | LIBUSB_RECIPIENT_INTERFACE, // Host to Device
    Read  = LIBUSB_ENDPOINT_IN  | LIBUSB_REQUEST_TYPE_CLASS | LIBUSB_RECIPIENT_INTERFACE, // Device to Host
}

/// If a device supports Boot, will start in Boot; otherwise Report
#[derive(Debug)]
#[repr(u8)]
pub enum Protocol {
    Boot    = 0,
    Report  = 1,
}

/// GetReport / SetReport type
#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ReportType {
    Unused  = 0x0,
    Input   = 0x1,
    Output  = 0x2,
    Feature = 0x3,
}

/// Boot+Report or Report-only device
/// Boot mode is limited and practically useless - you should usually set to Report ASAP
#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum HidSubClass {
    None = 0x0,
    Boot = 0x1,
}