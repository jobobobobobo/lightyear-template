use bevy::prelude::*;
use lightyear::prelude::{
    ReplicationGroup, ServerConnectionManager,
    server::{ServerCommandsExt, ServerConnection},
};
use mygame_assets::AssetState;
use mygame_common::level::LoadLevelRequest;
use mygame_protocol::component::Level;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_server);
    }
}

pub(crate) const REPLICATION_GROUP_PREDICTED: ReplicationGroup = ReplicationGroup::new_id(42);

fn start_server(mut commands: Commands) {
    commands.start_server();

    commands.trigger(LoadLevelRequest {
        level: Level::Example,
    });
}
