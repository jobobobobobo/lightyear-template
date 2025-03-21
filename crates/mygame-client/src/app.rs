use bevy::{log::{Level, LogPlugin}, prelude::*};
use lightyear::client::{config::ClientConfig, plugin::ClientPlugins};
use mygame_assets::CurrentLevel;
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, replication::ReplicationPlugin, ui::UiPlugin};

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Connecting,
    Loading,
    Playing,
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
    app.init_resource::<CurrentLevel>();
    
    app
}
