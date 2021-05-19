mod config_descriptor;
mod device_handle;
mod endpoint;
mod error;
mod types;

pub use config_descriptor::HidConfigDescriptor;
pub use device_handle::HidDevice;
pub use endpoint::Endpoint;
pub use error::HidError;
pub use types::*;

/*
The general process for a new device should go:
- get config descriptor
- Set interface to the HID interface


 */