use bevy::prelude::*;
use lightyear::prelude::server::ServerCommandsExt;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            start_server,
        );
    }
}

fn start_server(
    mut commands: Commands,
) {
    commands.start_server();
}
