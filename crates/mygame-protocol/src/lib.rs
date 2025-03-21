use bevy::prelude::*;

pub mod component;
pub mod input;
pub mod message;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        component::register_components(app);
        message::register_messages(app);
        input::register_input(app);
    }
}
