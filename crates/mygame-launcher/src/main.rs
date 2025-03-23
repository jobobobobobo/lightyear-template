use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use launch_options::SharedLaunchOptions;
use lightyear::prelude::{LinkConditionerConfig, SharedConfig, TickConfig};

mod launch_options;

#[cfg(not(target_family = "wasm"))]
mod load_certs;

#[cfg(target_family = "wasm")]
mod wasm_impl {
    use crate::launch_options::{ClientLaunchOptions, SharedLaunchOptions};
    use bevy::prelude::*;
    use lightyear::{
        client::config::{ClientConfig, NetcodeConfig as ClientNetcodeConfig},
        connection::client::NetConfig as ClientNetConfig,
        prelude::{
            LinkConditionerConfig, SharedConfig,
            client::{
                Authentication, ClientTransport, InterpolationConfig, IoConfig as ClientIoConfig,
                PredictionConfig,
            },
        },
    };
    use mygame_client::app::build_client_app;
    use std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        time::Duration,
    };
    use wasm_bindgen::prelude::*;
    use web_sys::{console, window};

    fn extract_certificate_digest() -> Option<String> {
        let window = web_sys::window()?;

        let search = window.location().search().ok()?;

        let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;

        let digest = params.get("digest")?;

        if digest.is_empty() {
            return None;
        }

        return Some(digest);
    }

    pub fn run(shared_launch_options: SharedLaunchOptions, shared_config: SharedConfig) {
        console_error_panic_hook::set_once();
        console::log_1(&"Wasm success! Starting client...".into());

        let wasm_asset_path = String::from("./assets");

        let certificate_digest = match extract_certificate_digest() {
            Some(digest) => digest,
            None => {
                console::log_1(&"No certificate digest found in URL path. Expected format: localhost:8080/DEF1AC...".into());
                return;
            }
        };

        console::log_2(
            &"Got certificate digest".into(),
            &JsValue::from_str(&certificate_digest),
        );

        let client_launch_options = ClientLaunchOptions {
            listen_addr: Ipv4Addr::LOCALHOST.into(),
            listen_port: 0,
            server_addr: Ipv4Addr::LOCALHOST.into(),
            server_port: 12026, // webtransport port
            conditioner: LinkConditionerConfig {
                incoming_latency: Duration::from_millis(50),
                incoming_jitter: Duration::ZERO,
                incoming_loss: 0.0,
            },
            correction_ticks_factor: 2.0,
            min_delay: Duration::from_millis(25),
            client_id: 293857, // TODO
        };

        let transport_config =
            ClientIoConfig::from_transport(ClientTransport::WebTransportClient {
                client_addr: SocketAddr::new(
                    IpAddr::V4(client_launch_options.listen_addr),
                    client_launch_options.listen_port,
                ),
                server_addr: SocketAddr::new(
                    IpAddr::V4(client_launch_options.server_addr),
                    client_launch_options.server_port,
                ),
                certificate_digest: certificate_digest,
            });

        let auth = Authentication::Manual {
            server_addr: SocketAddr::new(
                IpAddr::V4(client_launch_options.server_addr),
                client_launch_options.server_port,
            ),
            client_id: client_launch_options.client_id,
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

        build_client_app(client_config, wasm_asset_path).run();
    }
}

#[cfg(not(target_family = "wasm"))]
mod native_impl {
    use crate::{
        launch_options::{ClientLaunchOptions, ServerLaunchOptions, SharedLaunchOptions},
        load_certs,
    };
    use bevy::prelude::*;
    use clap::{Parser, ValueEnum};
    use lightyear::{
        client::config::{ClientConfig, NetcodeConfig as ClientNetcodeConfig},
        connection::client::NetConfig as ClientNetConfig,
        prelude::{
            LinkConditionerConfig, SharedConfig,
            client::{
                Authentication, ClientTransport, InterpolationConfig, IoConfig as ClientIoConfig,
                PredictionConfig,
            },
            server::{
                Identity, IoConfig as ServerIoConfig, NetConfig as ServerNetConfig, ServerTransport,
            },
        },
        server::config::{NetcodeConfig as ServerNetcodeConfig, ServerConfig},
    };
    use mygame_client::app::build_client_app;
    use mygame_server::app::{build_server_app, ServerMode};
    use std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        path::Path,
        time::Duration,
    };

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
    }

    pub fn run(shared_launch_options: SharedLaunchOptions, shared_config: SharedConfig) {
        let cli = Cli::parse();

        let development_asset_path = String::from("../mygame-assets/assets");

        match cli.mode {
            Mode::Client => {
                if cli.client_id == 0 {
                    panic!(
                        "No --client_id specified. To connect with multiple clients, specify a unique client id with --client_id <id>"
                    )
                }

                let client_launch_options = ClientLaunchOptions {
                    listen_addr: Ipv4Addr::LOCALHOST.into(),
                    listen_port: 0,
                    server_addr: Ipv4Addr::LOCALHOST.into(),
                    server_port: 0, // external: 12025, internal: 0
                    conditioner: LinkConditionerConfig {
                        incoming_latency: Duration::from_millis(50),
                        incoming_jitter: Duration::ZERO,
                        incoming_loss: 0.0,
                    },
                    correction_ticks_factor: 2.0,
                    min_delay: Duration::from_millis(25),
                };

                // Everything but wasm always has the option to host on the client
                let server_launch_options = ServerLaunchOptions {
                    listen_addr: Ipv4Addr::LOCALHOST,
                    udp_listen_port: 12025,
                    webtransport_listen_port: 12026,
                    headless: true,
                    conditioner: LinkConditionerConfig {
                        incoming_latency: Duration::from_millis(50),
                        incoming_jitter: Duration::ZERO,
                        incoming_loss: 0.0,
                    },
                    webtransport_cert_path: String::from(
                        "./crates/mygame-launcher/web/certs/cert.pem",
                    ),
                    webtransport_key_path: String::from(
                        "./crates/mygame-launcher/web/certs/key.pem",
                    ),
                };

                let (from_server_send, from_server_recv) = crossbeam_channel::unbounded();
                let (to_server_send, to_server_recv) = crossbeam_channel::unbounded();

                let transport_config =
                    ClientIoConfig::from_transport(ClientTransport::LocalChannel {
                        recv: from_server_recv.clone(),
                        send: to_server_send.clone(),
                    });

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
                    prediction: PredictionConfig::default().with_correction_ticks_factor(
                        client_launch_options.correction_ticks_factor,
                    ),
                    interpolation: InterpolationConfig {
                        // launch option?
                        min_delay: client_launch_options.min_delay,
                        send_interval_ratio: 0.,
                    },
                    ..default()
                };

                let server_netcode_config = ServerNetcodeConfig::default()
                    .with_protocol_id(shared_launch_options.protocol_id)
                    .with_key(shared_launch_options.key);

                let webtransport_identity = load_certs::load_certificate_from_files(
                    Path::new(&server_launch_options.webtransport_cert_path),
                    Path::new(&server_launch_options.webtransport_key_path),
                )
                .unwrap();

                let net_configs = vec![
                    ServerNetConfig::Netcode {
                        // normal udp sockets for desktop
                        config: server_netcode_config.clone(),
                        io: ServerIoConfig::from_transport(ServerTransport::UdpSocket(
                            (
                                server_launch_options.listen_addr,
                                server_launch_options.udp_listen_port,
                            )
                                .into(),
                        ))
                        .with_conditioner(server_launch_options.conditioner.clone()),
                    },
                    ServerNetConfig::Netcode {
                        // channels, for client host
                        config: server_netcode_config.clone(),
                        io: ServerIoConfig::from_transport(ServerTransport::Channels {
                            channels: vec![(
                                SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 12027), // port doesn't matter?
                                to_server_recv,
                                from_server_send,
                            )],
                        })
                        .with_conditioner(server_launch_options.conditioner.clone()),
                    },
                    ServerNetConfig::Netcode {
                        // webtransport
                        config: server_netcode_config.clone(),
                        io: ServerIoConfig::from_transport(ServerTransport::WebTransportServer {
                            server_addr: SocketAddr::new(
                                IpAddr::V4(server_launch_options.listen_addr),
                                server_launch_options.webtransport_listen_port,
                            ),
                            certificate: webtransport_identity,
                        })
                        .with_conditioner(server_launch_options.conditioner.clone()),
                    },
                ];

                let server_config = ServerConfig {
                    shared: shared_config,
                    net: net_configs,
                    ..default()
                };

                build_client_app(client_config, development_asset_path, server_config).run();
            }
            Mode::Server => {
                let server_launch_options = ServerLaunchOptions {
                    listen_addr: Ipv4Addr::LOCALHOST,
                    udp_listen_port: 12025,
                    webtransport_listen_port: 12026,
                    headless: false,
                    conditioner: LinkConditionerConfig {
                        incoming_latency: Duration::from_millis(50),
                        incoming_jitter: Duration::ZERO,
                        incoming_loss: 0.0,
                    },
                    webtransport_cert_path: String::from(
                        "./crates/mygame-launcher/web/certs/cert.pem",
                    ),
                    webtransport_key_path: String::from(
                        "./crates/mygame-launcher/web/certs/key.pem",
                    ),
                };

                let server_netcode_config = ServerNetcodeConfig::default()
                    .with_protocol_id(shared_launch_options.protocol_id)
                    .with_key(shared_launch_options.key);

                let webtransport_identity = load_certs::load_certificate_from_files(
                    Path::new(&server_launch_options.webtransport_cert_path),
                    Path::new(&server_launch_options.webtransport_key_path),
                )
                .unwrap();

                println!(
                    "Launching Server with Certificate Digest: {}",
                    webtransport_identity.certificate_chain().as_slice()[0]
                        .hash()
                        .to_string()
                        .replace(":", "")
                );

                let net_configs = vec![
                    ServerNetConfig::Netcode {
                        // normal udp sockets for desktop
                        config: server_netcode_config.clone(),
                        io: ServerIoConfig::from_transport(ServerTransport::UdpSocket(
                            (
                                server_launch_options.listen_addr,
                                server_launch_options.udp_listen_port,
                            )
                                .into(),
                        ))
                        .with_conditioner(server_launch_options.conditioner.clone()),
                    },
                    ServerNetConfig::Netcode {
                        // webtransport
                        config: server_netcode_config.clone(),
                        io: ServerIoConfig::from_transport(ServerTransport::WebTransportServer {
                            server_addr: SocketAddr::new(
                                IpAddr::V4(server_launch_options.listen_addr),
                                server_launch_options.webtransport_listen_port,
                            ),
                            certificate: webtransport_identity,
                        })
                        .with_conditioner(server_launch_options.conditioner.clone()),
                    },
                ];

                let server_config = ServerConfig {
                    shared: shared_config,
                    net: net_configs,
                    ..default()
                };

                let mode = if cli.headless {
                    ServerMode::Headless
                } else {
                    ServerMode::Windowed
                };

                build_server_app(server_config, development_asset_path, mode).run();
            }
        }
    }
}

fn main() {
    // Common shared configuration setup
    let shared_launch_options = SharedLaunchOptions::default();
    let shared_config = SharedConfig {
        server_replication_send_interval: shared_launch_options.server_replication_send_interval,
        client_replication_send_interval: shared_launch_options.client_replication_send_interval,
        tick: TickConfig {
            tick_duration: shared_launch_options.simulation_update_frequency,
        },
    };

    // Run the appropriate implementation based on target architecture
    #[cfg(target_family = "wasm")]
    wasm_impl::run(shared_launch_options, shared_config);

    #[cfg(not(target_family = "wasm"))]
    native_impl::run(shared_launch_options, shared_config);
}
