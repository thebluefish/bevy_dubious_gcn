#[derive(Debug, thiserror::Error)]
pub enum HidError {
    #[error(transparent)]
    UsbError(#[from] rusb::Error),
    #[error("Invalid HID")]
    Invalid, // Catch-all error, ideally use something more specific
    #[error("Device has no valid HID interface")]
    NoInterface,
    #[error("Unknown or invalid subclass")]
    UnknownSubclass,
    #[error("Unsupported request")]
    UnsupportedRequest,

}