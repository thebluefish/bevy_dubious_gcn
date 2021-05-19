mod events;
mod gcn;
mod plugin;
mod wup_device;

use usb_shenanigans::prelude::*;

pub mod prelude {
    pub use crate::{
        events::*,
        plugin::GcnPlugin,
    };
}

/*
We want to provide direct integration into bevy for controller support

We want to:
 - Spawn a thread for device scanner, storing handles in a WupScanner Resource
 - Spawn a thread for each device, storing handles in a GcnDevices Resource
 */