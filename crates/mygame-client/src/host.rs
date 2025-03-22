#![cfg(feature = "host")]

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use lightyear::client::config::ClientConfig;
use lightyear::prelude::server::ServerTransport;
use lightyear::server::config::ServerConfig;
use std::{
    net::{Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};

use crate::app::{AssetPath, ClientHostConfig, GameState};
use mygame_server::app::build_server_app;

pub struct HostPlugin;

impl Plugin for HostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Hosting), on_client_begin_hosting);
    }
}

struct SendApp(App);

unsafe impl Send for SendApp {}
impl SendApp {
    fn run(&mut self) {
        self.0.run();
    }
}

fn on_client_begin_hosting(
    mut commands: Commands,
    asset_path: Res<AssetPath>,
    host_config: Res<ClientHostConfig>,
) {
    {
        let server_app = build_server_app(host_config.server_config.clone(), asset_path.0.clone(), true);

        let mut send_server_app = SendApp(server_app);
        std::thread::spawn(move || send_server_app.run());
    }

    commands.set_state(GameState::Connecting);
}
