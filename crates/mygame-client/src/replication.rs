use bevy::prelude::*;
use lightyear::prelude::*;
use mygame_assets::AssetState;
use mygame_common::level::LoadLevelRequest;
use mygame_protocol::message::ServerWelcome;

use crate::app::GameState;

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_server_welcome);
        
        app.add_systems(OnEnter(AssetState::Loaded), |mut commands: Commands| {
            commands.set_state(GameState::Playing)
        });
    }
}

fn on_server_welcome(
    mut commands: Commands,
    mut server_welcome_events: ResMut<Events<ClientReceiveMessage<ServerWelcome>>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in server_welcome_events.drain() {
        if *game_state != GameState::Connecting {
            warn!("Received a ServerWelcome message at an unexpected time.");
            return;
        }

        next_state.set(GameState::Loading);

        commands.trigger(LoadLevelRequest {
            level: ev.message.current_level,
        });
    }
}
