use bevy::prelude::*;
use lightyear::prelude::{
    ReplicationGroup, ServerConnectionManager,
    server::{ServerCommandsExt, ServerConnection},
};
use mygame_assets::{CurrentLevel, LevelState};
use mygame_protocol::message::Level;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_server);
    }
}

pub(crate) const REPLICATION_GROUP_PREDICTED: ReplicationGroup = ReplicationGroup::new_id(42);

fn start_server(mut commands: Commands, mut current_level: ResMut<CurrentLevel>) {
    commands.start_server();

    current_level.0 = Level::Example;
}
