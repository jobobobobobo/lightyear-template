use bevy::{prelude::*, asset::AssetPlugin};
use lightyear::{prelude::*, server::{config::ServerConfig, plugin::ServerPlugins}};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, replication::ReplicationPlugin};

pub fn build_server_app(server_config: ServerConfig, asset_path: String, headless: bool) -> App {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins.build().set(
            AssetPlugin {
                file_path: asset_path,
                ..default()
            }
        ))
        .add_plugins(ServerPlugins {
            config: server_config
        })
        .add_plugins((
            CommonPlugin,
            NetworkPlugin,
            ReplicationPlugin
        ));

    if !headless {
        // TODO: all the other plugin-fu
        app.add_plugins(RenderPlugin);
    }

    app
}
