use bevy::asset::AssetMetaCheck;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use lightyear::prelude::client::{NetConfig, VisualInterpolationPlugin};
use lightyear::{
    client::{config::ClientConfig, plugin::ClientPlugins},
    server::config::ServerConfig,
};
use common::CommonPlugin;
use render::RenderPlugin;

use crate::game_state::{GameLifecyclePlugin, GameState};
use crate::input::InputPlugin;
use crate::{
    interpolation::InterpolationPlugin, network::NetworkPlugin, replication::ReplicationPlugin,
    ui::UiPlugin,
};


// TODO: remove this?
/// The root asset path is preserved here by the client at startup so it can be forwarded
/// to the client server, should they choose to host.
#[derive(Resource)]
pub struct AssetPath(pub String);

#[derive(Resource)]
pub struct LaunchConfigurations {
    pub server_config: Option<ServerConfig>,
    pub client_local_config: Option<ClientConfig>,
    pub client_remote_config: Option<ClientConfig>,
}

fn build_core_client_app(
    app: &mut App,
    client_remote_config: ClientConfig,
    asset_path: String,
) -> &mut App {
    app.add_plugins((
        DefaultPlugins.build().set(AssetPlugin {
            file_path: asset_path.clone(),
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),
        ClientPlugins {
            config: client_remote_config.clone(),
        },
        CommonPlugin,
        GameLifecyclePlugin,
        UiPlugin,
        NetworkPlugin,
        RenderPlugin,
        ReplicationPlugin,
        InputPlugin,
        InterpolationPlugin,
    ));

    app.insert_resource(AssetPath(asset_path));

    app
}

pub fn build_client_app(client_config: ClientConfig, asset_path: String) -> App {
    let mut app = App::new();

    build_core_client_app(&mut app, client_config.clone(), asset_path);

    app.insert_resource(LaunchConfigurations {
        server_config: None,
        client_local_config: None,
        client_remote_config: Some(client_config),
    });

    app
}
