use bevy::asset::AssetMetaCheck;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use lightyear::{
    client::{config::ClientConfig, plugin::ClientPlugins},
    server::config::ServerConfig,
};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, replication::ReplicationPlugin, ui::UiPlugin};

#[cfg(feature = "host")]
use crate::host::HostPlugin;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    #[cfg(feature = "host")]
    Hosting, // Prepping the local server
    Connecting, // Connection request sent to the server
    Loading,    // Connected and server told us to load something
    Spawning,   // Loaded the assets, now wait for the Player to be replicated
    Playing,    // Player exists and we can give control to the client
}

/// The root asset path is preserved here by the client at startup so it can be forwarded
/// to the client server, should they choose to host.
#[derive(Resource)]
pub struct AssetPath(pub String);

#[derive(Resource)]
pub struct ClientHostConfig {
    pub server_config: ServerConfig,
}

#[cfg(not(feature = "host"))]
pub fn build_client_app(client_config: ClientConfig, asset_path: String) -> App {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.build().set(AssetPlugin {
            file_path: asset_path.clone(),
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),
        ClientPlugins {
            config: client_config,
        },
        CommonPlugin,
        UiPlugin,
        NetworkPlugin,
        RenderPlugin,
        ReplicationPlugin,
    ));

    app.init_state::<GameState>();
    app.insert_resource(AssetPath(asset_path));

    app
}

/// The "host" feature has its own signature for build_client_app so it may
/// obtain a ServerConfig to configure the server with.
#[cfg(feature = "host")]
pub fn build_client_app(
    client_config: ClientConfig,
    asset_path: String,
    server_config: ServerConfig,
) -> App {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.build().set(AssetPlugin {
            file_path: asset_path.clone(),
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),
        ClientPlugins {
            config: client_config,
        },
        CommonPlugin,
        UiPlugin,
        NetworkPlugin,
        RenderPlugin,
        ReplicationPlugin,
        HostPlugin,
    ));

    app.init_state::<GameState>();
    app.insert_resource(AssetPath(asset_path));

    app.insert_resource(ClientHostConfig { server_config });

    app
}
