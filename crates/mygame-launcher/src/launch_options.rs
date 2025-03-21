use std::{net::Ipv4Addr, time::Duration};

use lightyear::prelude::{server::ServerTransport, LinkConditionerConfig, TickConfig};

pub struct ServerLaunchOptions {
    pub headless: bool,

    pub listen_addr: Ipv4Addr,
    pub listen_port: u16,

    pub conditioner: LinkConditionerConfig
}

pub struct ClientLaunchOptions {
    pub listen_addr: Ipv4Addr,
    pub listen_port: u16,

    pub server_addr: Ipv4Addr,
    pub server_port: u16,

    pub conditioner: LinkConditionerConfig,

    pub correction_ticks_factor: f32,
    pub min_delay: Duration,
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
