use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use launch_options::SharedLaunchOptions;
use lightyear::prelude::{LinkConditionerConfig, SharedConfig, TickConfig};

mod launch_options;

mod native;

fn main() {
    native::run();
}
