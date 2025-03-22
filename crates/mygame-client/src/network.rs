use bevy::prelude::*;
use lightyear::prelude::{client::ClientCommandsExt, ClientConnectEvent, ClientDisconnectEvent};

use crate::app::GameState;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Connecting),
            connect_to_server,
        );
    }
}


fn connect_to_server(
    mut commands: Commands,
) {
    commands.connect_client();
}

fn on_client_connect_success(
    _trigger: Trigger<ClientConnectEvent>,
) {
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
