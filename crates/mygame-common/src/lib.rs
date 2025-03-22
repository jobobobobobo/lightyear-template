use avian3d::{prelude::PhysicsInterpolationPlugin, PhysicsPlugins};
use bevy::prelude::*;
use mygame_protocol::ProtocolPlugin;
use mygame_assets::AssetPlugin;

pub mod level;
pub mod player;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetPlugin,
            ProtocolPlugin,
            PhysicsPlugins::new(FixedPostUpdate)
                .build()
                .disable::<PhysicsInterpolationPlugin>(),
            level::LevelPlugin,
            player::PlayerPlugin,
        ));
    }
}
