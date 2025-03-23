use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use launch_options::SharedLaunchOptions;
use lightyear::prelude::{LinkConditionerConfig, SharedConfig, TickConfig};

mod launch_options;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(not(target_family = "wasm"))]
mod native;

fn main() {
    #[cfg(target_family = "wasm")]
    wasm::run();

    #[cfg(not(target_family = "wasm"))]
    native::run();
}
