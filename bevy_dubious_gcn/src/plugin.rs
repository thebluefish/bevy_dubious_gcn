use crate::gcn::*;
use bevy::{prelude::*, asset::LoadState};

pub struct GcnPlugin;

impl Plugin for GcnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<GcnEvent>()
            .init_resource::<GcnSettings>()
            .init_resource::<Input<GcnButton>>()
            .init_resource::<Axis<GcnAxis>>()
        ;
    }
}
