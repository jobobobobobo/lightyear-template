use bevy::prelude::*;
use lightyear::prelude::{
    FromClients, ReplicationGroup, ServerConnectionManager,
    server::{ServerCommandsExt, ServerConnection},
};
use mygame_assets::{CurrentLevel, LevelState};
use mygame_protocol::message::{ClientHostRequestShutdown, Level};

use crate::app::ServerMode;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_server);
        app.add_systems(Update, on_host_request_shutdown);
    }
}

pub(crate) const REPLICATION_GROUP_PREDICTED: ReplicationGroup = ReplicationGroup::new_id(42);

fn start_server(mut commands: Commands, mut current_level: ResMut<CurrentLevel>) {
    commands.start_server();

    current_level.0 = Level::Example;
}

fn on_host_request_shutdown(
    mut commands: Commands,
    mut ev_host_request_shutdown: ResMut<Events<FromClients<ClientHostRequestShutdown>>>,
    server_mode: Res<ServerMode>,
) {
    let owner = match *server_mode {
        ServerMode::Windowed => return,
        ServerMode::Headless => return,
        ServerMode::ClientHost(client_id) => client_id,
    };

    for ev in ev_host_request_shutdown.drain() {
        println!("from: {} but owner: {}", ev.from, owner);

        if ev.from.to_bits() == owner.to_bits() {
            println!("NOT HELLO...");
            commands.stop_server();
        }
    }
}
