use bevy::prelude::*;
use lightyear::prelude::{server::ServerCommandsExt, MessageSend, NetworkTarget, Replicating, ServerConnectEvent, ServerConnectionManager, ServerDisconnectEvent};
use mygame_assets::CurrentLevel;
use mygame_protocol::{component::Level, message::{ServerWelcome, UnorderedReliable}};

pub struct ReplicationPlugin;
impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_client_connect_success);
        app.add_observer(on_client_disconnect);
    }
}

fn on_client_connect_success(
    trigger: Trigger<ServerConnectEvent>,
    mut commands: Commands,
    mut server: ResMut<ServerConnectionManager>,
    current_level: Res<CurrentLevel>,
) {
    let client_id = trigger.event().client_id;

    if let Err(e) = server.send_message_to_target::<UnorderedReliable, ServerWelcome>(
        &ServerWelcome {
            current_level: current_level.0,
        },
        NetworkTarget::Single(client_id),
    ) {
        error!(
            "unable to replicate level to client id {}, had error {}",
            client_id, e
        );
        commands.disconnect(client_id);

        return;
    }

    info!("connected client ${}", trigger.event().client_id);
}

fn on_client_disconnect(trigger: Trigger<ServerDisconnectEvent>) {
    info!("disconnected client ${}", trigger.event().client_id);
}
