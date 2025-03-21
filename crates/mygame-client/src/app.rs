use bevy::prelude::*;
use lightyear::client::{config::ClientConfig, plugin::ClientPlugins};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, ui::UiPlugin};

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Connecting
}

pub fn build_client_app(client_config: ClientConfig) -> App {
    let mut app = App::new();
    
    app
        .add_plugins((
            DefaultPlugins.build(),
            ClientPlugins {
                config: client_config
            },
            CommonPlugin,
            UiPlugin,
            NetworkPlugin,
            RenderPlugin
        ));

    app.init_state::<GameState>();

    app
}
