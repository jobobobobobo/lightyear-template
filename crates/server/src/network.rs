use assets::{CurrentLevel, LevelState};
use bevy::prelude::*;
use lightyear::prelude::{
    server::{NetworkingState, ServerCommandsExt, ServerConnection},
    FromClients, ReplicationGroup, ServerConnectionManager,
};
use protocol::message::Level;

use crate::app::ServerMode;

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
