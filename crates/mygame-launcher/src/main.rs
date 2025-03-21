use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use clap::{Parser, ValueEnum};
use launch_options::{ClientLaunchOptions, ServerLaunchOptions, SharedLaunchOptions};
use lightyear::{
    client::config::{ClientConfig, NetcodeConfig as ClientNetcodeConfig},
    connection::client::NetConfig as ClientNetConfig,
    prelude::{
        LinkConditionerConfig, SharedConfig, TickConfig,
        client::{
            Authentication, ClientTransport, InterpolationConfig, IoConfig as ClientIoConfig,
            PredictionConfig,
        },
        server::{IoConfig as ServerIoConfig, NetConfig as ServerNetConfig, ServerTransport},
    },
    server::config::{NetcodeConfig as ServerNetcodeConfig, ServerConfig},
};
use mygame_client::app::build_client_app;
use mygame_server::app::build_server_app;

mod launch_options;

#[derive(Parser)]
#[command(name = "mygame")]
#[command(version = "0.1")]
#[command(about = "Manages and launches various server and client configurations for mygame.")]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(long, default_value_t = false)]
    headless: bool,

    #[arg(short, long, default_value_t = 0)]
    client_id: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Client,
    Server,
    Combined,
}

fn main() {
    let cli = Cli::parse();

    let shared_launch_options = SharedLaunchOptions::default();

    let shared_config = SharedConfig {
        server_replication_send_interval: shared_launch_options.server_replication_send_interval,
        client_replication_send_interval: shared_launch_options.client_replication_send_interval,
        tick: TickConfig {
            tick_duration: shared_launch_options.simulation_update_frequency,
        },
    };

    match cli.mode {
        Mode::Client => {
            if cli.client_id == 0 {
                warn!(
                    "To connect with multiple clients, specify a unique client id with --client_id <id>"
                )
            }

            let client_launch_options = ClientLaunchOptions {
                listen_addr: Ipv4Addr::LOCALHOST.into(),
                listen_port: 0,
                server_addr: Ipv4Addr::LOCALHOST.into(),
                server_port: 12025,
                conditioner: LinkConditionerConfig {
                    incoming_latency: Duration::from_millis(50),
                    incoming_jitter: Duration::ZERO,
                    incoming_loss: 0.0,
                },
                correction_ticks_factor: 2.0,
                min_delay: Duration::from_millis(25),
            };

            let transport_config =
                ClientIoConfig::from_transport(ClientTransport::UdpSocket(SocketAddr::new(
                    IpAddr::V4(client_launch_options.listen_addr),
                    client_launch_options.listen_port,
                )));

            let auth = Authentication::Manual {
                server_addr: SocketAddr::new(
                    IpAddr::V4(client_launch_options.server_addr),
                    client_launch_options.server_port,
                ),
                client_id: cli.client_id,
                private_key: shared_launch_options.key,
                protocol_id: shared_launch_options.protocol_id,
            };

            let client_config = ClientConfig {
                shared: shared_config,
                net: ClientNetConfig::Netcode {
                    auth,
                    config: ClientNetcodeConfig {
                        token_expire_secs: -1,
                        client_timeout_secs: 5,
                        ..default()
                    },
                    io: transport_config,
                },
                prediction: PredictionConfig::default()
                    .with_correction_ticks_factor(client_launch_options.correction_ticks_factor),
                interpolation: InterpolationConfig {
                    min_delay: client_launch_options.min_delay,
                    send_interval_ratio: 0.,
                },
                ..default()
            };

            build_client_app(client_config).run();
        }
        Mode::Server => {
            let server_launch_options = ServerLaunchOptions {
                listen_addr: Ipv4Addr::LOCALHOST,
                listen_port: 12025,
                headless: false,
                conditioner: LinkConditionerConfig {
                    incoming_latency: Duration::from_millis(50),
                    incoming_jitter: Duration::ZERO,
                    incoming_loss: 0.0,
                },
            };

            let io_config = ServerIoConfig::from_transport(ServerTransport::UdpSocket(
                (
                    server_launch_options.listen_addr,
                    server_launch_options.listen_port,
                )
                    .into(),
            ))
            .with_conditioner(server_launch_options.conditioner);

            let net_configs = vec![ServerNetConfig::Netcode {
                config: ServerNetcodeConfig {
                    protocol_id: shared_launch_options.protocol_id,
                    private_key: shared_launch_options.key,
                    ..default()
                },
                io: io_config,
            }];

            let server_config = ServerConfig {
                shared: shared_config,
                net: net_configs,
                ..default()
            };

            build_server_app(server_config, cli.headless).run();
        }
        Mode::Combined => todo!(),
    }
}
