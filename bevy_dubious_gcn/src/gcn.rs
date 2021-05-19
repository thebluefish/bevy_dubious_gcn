use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GcnGamepad(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    WupConnected,
    WupDisconnected,
    GcnConnected,
    GcnDisconnected,
    ButtonChanged(GcnButtonType, bool),
    AxisChanged(GamepadAxisType, f32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GcnEvent(pub Gamepad, pub EventType);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GcnButtonType {
    StickUp,
    StickDown,
    StickLeft,
    StickRight,
    CUp,
    CDown,
    CLeft,
    CRight,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    A,
    B,
    X,
    Y,
    Z,
    LeftTrigger,
    RightTrigger,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GcnButton(pub GcnGamepad, pub GamepadButtonType);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GcnAxisType {
    StickX,
    StickY,
    CX,
    CY,
    LeftTrigger,
    RightTrigger,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GcnAxis(pub GcnGamepad, pub GamepadAxisType);


#[derive(Default, Debug)]
pub struct GcnSettings {
    pub default_button_settings: ButtonSettings,
    pub default_axis_settings: AxisSettings,
    pub default_button_axis_settings: ButtonAxisSettings,
    pub button_settings: HashMap<GamepadButton, ButtonSettings>,
    pub axis_settings: HashMap<GamepadAxis, AxisSettings>,
    pub button_axis_settings: HashMap<GamepadButton, ButtonAxisSettings>,
}