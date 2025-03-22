use bevy::{log::{Level, LogPlugin}, prelude::*};
use lightyear::client::{config::ClientConfig, plugin::ClientPlugins};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, replication::ReplicationPlugin, ui::UiPlugin};

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Connecting, // Connection request sent to the server
    Loading,    // Connected and server told us to load something
    Spawning,   // Loaded the assets, now wait for the Player to be replicated
    Playing     // Player exists and we can give control to the client
}

pub fn build_client_app(client_config: ClientConfig, asset_path: String) -> App {
    let mut app = App::new();
    
    app
        .add_plugins((
            DefaultPlugins.build().set(
                AssetPlugin {
                    file_path: asset_path,
                    ..default()
                }
            ),
            ClientPlugins {
                config: client_config
            },
            CommonPlugin,
            UiPlugin,
            NetworkPlugin,
            RenderPlugin,
            ReplicationPlugin,
        ));

    app.init_state::<GameState>();
    
    app
}
