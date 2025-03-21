use bevy::{prelude::*};
use lightyear::{prelude::*, server::{config::ServerConfig, plugin::ServerPlugins}};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::network::NetworkPlugin;

pub fn build_server_app(server_config: ServerConfig, headless: bool) -> App {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins.build())
        .add_plugins(ServerPlugins {
            config: server_config
        })
        .add_plugins(CommonPlugin)
        .add_plugins(NetworkPlugin);

    if !headless {
        // TODO: all the other plugin-fu
        app.add_plugins(RenderPlugin);
    }

    app
}
