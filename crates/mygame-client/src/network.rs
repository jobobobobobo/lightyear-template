use bevy::prelude::*;
use lightyear::prelude::client::ClientCommandsExt;

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
