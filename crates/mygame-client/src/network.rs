use bevy::prelude::*;
use lightyear::{
    client::config::ClientConfig,
    prelude::{ClientConnectEvent, ClientDisconnectEvent, client::ClientCommandsExt},
};

use crate::app::{ClientHostConfig, GameState};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::ConnectingRemote),
            connect_to_remote_server,
        );
        app.add_systems(OnEnter(GameState::ConnectingSelf), connect_to_local_server);
    }
}

fn connect_to_remote_server(
    mut commands: Commands,
    host_config: ResMut<ClientHostConfig>,
    mut client_config: ResMut<ClientConfig>,
) {
    *client_config = host_config.client_remote_config.clone();
    commands.connect_client();
}

fn connect_to_local_server(
    mut commands: Commands,
    host_config: ResMut<ClientHostConfig>,
    mut client_config: ResMut<ClientConfig>,
) {
    *client_config = host_config.client_local_config.clone();
    commands.connect_client();
}

fn on_client_connect_success(_trigger: Trigger<ClientConnectEvent>) {
    // No need to do anything, we are waiting for a ServerWelcome message
    info!("successful client connection");
}

fn on_client_disconnect(
    _trigger: Trigger<ClientDisconnectEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // TODO: Cleanup existing state?

    game_state.set(GameState::MainMenu);
}
