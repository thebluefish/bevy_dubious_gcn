use bevy_dubious_gcn::prelude::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GcnPlugin)
        .add_system(test.system())
        .run();
}

fn test(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}
