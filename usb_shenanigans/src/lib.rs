// Goals:
//    Implement common USB & HID calls as traits over rusb Device and DeviceHandle

pub mod hid;
pub mod partition_results;

pub mod prelude {
    pub use crate::{
        hid::*,
        partition_results::PartitionResults,
    };
}

/// This crate *should* work on macos, but adding the required exception requires disabling SIP which is very, very bad.
/// Seriously, don't do it. If you think you know the average user and still consider doing it, you don't know the average user.
/// If you're absolutely certain that you **really** want this, and swear to me that you won't bitch at me when one of your users gets inevitably pwnd, feel free to fork and remove this block.
/// You are 100% responsible for whatever shenanigans happens next should you do so.
#[cfg(target_os = "macos")]
compile_error!("MacOS is unsupported!");