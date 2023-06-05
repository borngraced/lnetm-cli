use clap::*;
use core::str::FromStr;
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

mod availability;
use availability::check_network_availability;

mod latency;
use latency::check_network_latency;

#[derive(Debug, Clone)]
enum MonitorKind {
    All,
    Availability,
    Latency,
}

impl From<String> for MonitorKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "latency" | "l" => MonitorKind::Latency,
            "availability" | "a" => MonitorKind::Availability,
            _ => MonitorKind::All,
        }
    }
}

fn main() {
    let matches = command!()
        .version("0.0.1")
        .author("Samuel Onoja")
        .name("netM")
        .about("Rust network monitoring cli tool.")
        .arg(
            arg!(-m --monitor <MONITOR> "What to monitor e.g latency or availability(All is default)")
            .id("monitor")
            .required(false)
            .default_value("all")
            .value_parser(value_parser!(String))
        )
        .arg(
            arg!(
                -a --addr <IP_ADDRS> "Set IP Addr to monitor"
            )
            .id("addr")
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(-d --data <DATA> "Set data to ping to server")
            .id("data")
            .required(false)
            .value_parser(value_parser!(String))
            .default_value("hey"
        ))
        .arg(
            arg!(
                -t --threshold <THRESHOLD> "Set threshold for latency (in secs. (100 is default))"
            )
            .id("threshold")
            .required(false)
            .value_parser(value_parser!(u32))
            .default_value("10")
            .action(ArgAction::Set)
            )
        .arg(
            arg!(
                -o --timeout <TIMEOUT> "Set timeout for latency monitoring (in secs. (10 is default))"
            )
            .id("timeout")
            .required(false)
            .value_parser(value_parser!(u64))
            .default_value("10")
            .action(ArgAction::Set)
            )
         .arg(
            arg!(
                -i --interval <INTERVAL> "Set monitoring interval (in secs. (10 is default))"
            )
            .id("interval")
            .required(false)
            .value_parser(value_parser!(u64))
            .default_value("10")
            .action(ArgAction::Set)
            )
        .get_matches();

    let monitor_kind = matches.get_one::<String>("monitor").map(|e| e.clone());
    let addr = matches.get_one::<String>("addr").expect("required");
    let data = matches
        .get_one::<String>("data")
        .map(|e| e.clone())
        .unwrap_or("hey!".to_string());
    let interval = matches.get_one::<u64>("interval").map(|t| t.clone());
    let timeout = matches.get_one::<u64>("timeout").map(|t| t.clone());
    let threshold = matches.get_one::<u32>("threshold").map(|t| t.clone());

    let timeout = Duration::from_secs(timeout.unwrap_or(10));

    match MonitorKind::from(monitor_kind.unwrap_or("All".to_string())) {
        MonitorKind::Latency => check_network_latency(
            &IpAddr::V4(Ipv4Addr::from_str(addr).unwrap()),
            data.as_bytes(),
            timeout,
            threshold,
            Duration::from_secs(interval.unwrap_or(10)),
        ),
        MonitorKind::Availability => loop {
            check_network_availability(&addr, timeout);
        },
        MonitorKind::All => todo!(),
    }
}
