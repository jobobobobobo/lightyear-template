use lightyear::prelude::{LinkConditionerConfig, TickConfig, server::ServerTransport};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLinkConditionerConfig {
    pub incoming_latency_ms: u64,
    pub incoming_jitter_ms: u64,
    pub incoming_loss: f32,
}

impl From<LinkConditionerConfig> for SerializableLinkConditionerConfig {
    fn from(config: LinkConditionerConfig) -> Self {
        Self {
            incoming_latency_ms: config.incoming_latency.as_millis() as u64,
            incoming_jitter_ms: config.incoming_jitter.as_millis() as u64,
            incoming_loss: config.incoming_loss,
        }
    }
}

impl From<SerializableLinkConditionerConfig> for LinkConditionerConfig {
    fn from(config: SerializableLinkConditionerConfig) -> Self {
        Self {
            incoming_latency: Duration::from_millis(config.incoming_latency_ms),
            incoming_jitter: Duration::from_millis(config.incoming_jitter_ms),
            incoming_loss: config.incoming_loss,
        }
    }
}

pub struct SharedLaunchOptions {
    pub protocol_id: u64,
    pub key: [u8; 32],
    pub simulation_update_frequency: Duration,
    pub server_replication_send_interval: Duration,
    pub client_replication_send_interval: Duration,
}

impl Default for SharedLaunchOptions {
    fn default() -> Self {
        Self {
            protocol_id: Default::default(),
            key: Default::default(),
            simulation_update_frequency: Duration::from_millis(16),
            server_replication_send_interval: Duration::from_millis(0),
            client_replication_send_interval: Duration::from_millis(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSharedLaunchOptions {
    pub protocol_id: u64,
    pub key: [u8; 32],
    pub simulation_update_frequency_ms: u64,
    pub server_replication_send_interval_ms: u64,
    pub client_replication_send_interval_ms: u64,
}

impl From<SharedLaunchOptions> for SerializableSharedLaunchOptions {
    fn from(options: SharedLaunchOptions) -> Self {
        Self {
            protocol_id: options.protocol_id,
            key: options.key,
            simulation_update_frequency_ms: options.simulation_update_frequency.as_millis() as u64,
            server_replication_send_interval_ms: options
                .server_replication_send_interval
                .as_millis() as u64,
            client_replication_send_interval_ms: options
                .client_replication_send_interval
                .as_millis() as u64,
        }
    }
}

impl From<SerializableSharedLaunchOptions> for SharedLaunchOptions {
    fn from(options: SerializableSharedLaunchOptions) -> Self {
        Self {
            protocol_id: options.protocol_id,
            key: options.key,
            simulation_update_frequency: Duration::from_millis(
                options.simulation_update_frequency_ms,
            ),
            server_replication_send_interval: Duration::from_millis(
                options.server_replication_send_interval_ms,
            ),
            client_replication_send_interval: Duration::from_millis(
                options.client_replication_send_interval_ms,
            ),
        }
    }
}

pub struct ClientLaunchOptions {
    pub listen_addr: Ipv4Addr,
    pub listen_port: u16,
    pub server_addr: Ipv4Addr,
    pub server_port: u16,
    pub conditioner: LinkConditionerConfig,
    pub correction_ticks_factor: f32,
    pub min_delay: Duration,
    pub certificate_digest: Option<String>,
    pub asset_path: String,
}

impl Default for ClientLaunchOptions {
    fn default() -> Self {
        Self {
            listen_addr: Ipv4Addr::LOCALHOST,
            listen_port: 0,
            server_addr: Ipv4Addr::LOCALHOST,
            server_port: 0,
            conditioner: LinkConditionerConfig {
                incoming_latency: Duration::from_millis(50),
                incoming_jitter: Duration::ZERO,
                incoming_loss: 0.0,
            },
            correction_ticks_factor: 2.0,
            min_delay: Duration::from_millis(25),
            certificate_digest: None,
            asset_path: String::from("../mygame-assets/assets"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableClientLaunchOptions {
    pub listen_addr: String,
    pub listen_port: u16,
    pub server_addr: String,
    pub server_port: u16,
    pub conditioner: SerializableLinkConditionerConfig,
    pub correction_ticks_factor: f32,
    pub min_delay_ms: u64,
    pub certificate_digest: Option<String>,
    pub asset_path: String,
}

impl From<ClientLaunchOptions> for SerializableClientLaunchOptions {
    fn from(options: ClientLaunchOptions) -> Self {
        Self {
            listen_addr: options.listen_addr.to_string(),
            listen_port: options.listen_port,
            server_addr: options.server_addr.to_string(),
            server_port: options.server_port,
            conditioner: SerializableLinkConditionerConfig::from(options.conditioner),
            correction_ticks_factor: options.correction_ticks_factor,
            min_delay_ms: options.min_delay.as_millis() as u64,
            certificate_digest: options.certificate_digest,
            asset_path: options.asset_path,
        }
    }
}

impl From<SerializableClientLaunchOptions> for ClientLaunchOptions {
    fn from(serializable: SerializableClientLaunchOptions) -> Self {
        Self {
            listen_addr: serializable
                .listen_addr
                .parse()
                .unwrap_or(Ipv4Addr::LOCALHOST),
            listen_port: serializable.listen_port,
            server_addr: serializable
                .server_addr
                .parse()
                .unwrap_or(Ipv4Addr::LOCALHOST),
            server_port: serializable.server_port,
            conditioner: LinkConditionerConfig::from(serializable.conditioner),
            correction_ticks_factor: serializable.correction_ticks_factor,
            min_delay: Duration::from_millis(serializable.min_delay_ms),
            certificate_digest: serializable.certificate_digest,
            asset_path: serializable.asset_path,
        }
    }
}

pub struct ServerLaunchOptions {
    pub headless: bool,
    pub listen_addr: Ipv4Addr,
    pub udp_listen_port: u16,
    pub webtransport_listen_port: u16,
    pub conditioner: LinkConditionerConfig,
    pub webtransport_cert_path: String,
    pub webtransport_key_path: String,
    pub asset_path: String,
}

impl Default for ServerLaunchOptions {
    fn default() -> Self {
        Self {
            headless: false,
            listen_addr: Ipv4Addr::LOCALHOST,
            udp_listen_port: 12025,
            webtransport_listen_port: 12026,
            conditioner: LinkConditionerConfig {
                incoming_latency: Duration::from_millis(50),
                incoming_jitter: Duration::ZERO,
                incoming_loss: 0.0,
            },
            webtransport_cert_path: String::from("./crates/mygame-launcher/web/certs/cert.pem"),
            webtransport_key_path: String::from("./crates/mygame-launcher/web/certs/key.pem"),
            asset_path: String::from("../mygame-assets/assets"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableServerLaunchOptions {
    pub headless: bool,
    pub listen_addr: String,
    pub udp_listen_port: u16,
    pub webtransport_listen_port: u16,
    pub conditioner: SerializableLinkConditionerConfig,
    pub webtransport_cert_path: String,
    pub webtransport_key_path: String,
    pub asset_path: String,
}

impl From<ServerLaunchOptions> for SerializableServerLaunchOptions {
    fn from(options: ServerLaunchOptions) -> Self {
        Self {
            headless: options.headless,
            listen_addr: options.listen_addr.to_string(),
            udp_listen_port: options.udp_listen_port,
            webtransport_listen_port: options.webtransport_listen_port,
            conditioner: SerializableLinkConditionerConfig::from(options.conditioner),
            webtransport_cert_path: options.webtransport_cert_path,
            webtransport_key_path: options.webtransport_key_path,
            asset_path: options.asset_path,
        }
    }
}

impl From<SerializableServerLaunchOptions> for ServerLaunchOptions {
    fn from(serializable: SerializableServerLaunchOptions) -> Self {
        Self {
            headless: serializable.headless,
            listen_addr: serializable
                .listen_addr
                .parse()
                .unwrap_or(Ipv4Addr::LOCALHOST),
            udp_listen_port: serializable.udp_listen_port,
            webtransport_listen_port: serializable.webtransport_listen_port,
            conditioner: LinkConditionerConfig::from(serializable.conditioner),
            webtransport_cert_path: serializable.webtransport_cert_path,
            webtransport_key_path: serializable.webtransport_key_path,
            asset_path: serializable.asset_path,
        }
    }
}
