use bevy::prelude::*;
use mygame_protocol::ProtocolPlugin;
use mygame_assets::AssetPlugin;

pub mod level;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetPlugin,
            ProtocolPlugin,
            level::LevelPlugin
        ));
    }
}
