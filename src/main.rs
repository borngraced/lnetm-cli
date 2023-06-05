use clap::*;
use std::{
    net::{IpAddr, Ipv4Addr},
    thread,
    time::Duration,
};

mod availability;
use availability::check_network_availability;

mod latency;
use latency::check_network_latency;

const MONITOR_KIND: &str = "all";
const THRESHOLD: u32 = 100;
const TIMEOUT: u64 = 10;
const INTERVAL: u64 = 10;
const PING_DATA: &str = "hello";

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
    env_logger::init();

    let netm = NetMCli::new();
    match netm.kind() {
        MonitorKind::Latency => check_network_latency(&netm),
        MonitorKind::Availability => check_network_availability(&netm),
        MonitorKind::All => thread::scope(|sc| {
            sc.spawn(|| {
                check_network_availability(&netm);
            });

            sc.spawn(|| {
                check_network_latency(&netm);
            });
        }),
    }
}
#[derive(Debug, Clone)]
pub struct NetMCli {
    kind: String,
    addrs: String,
    data: String,
    interval: u64,
    timeout: u64,
    threshold: u32,
}

impl NetMCli {
    fn new() -> Self {
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

        let monitor_kind = matches
            .get_one::<String>("monitor")
            .map(|e| e.clone())
            .unwrap_or(MONITOR_KIND.to_string());
        let addrs = matches
            .get_one::<String>("addr")
            .map(|e| e.clone())
            .expect("required");
        let data = matches
            .get_one::<String>("data")
            .map(|e| e.clone())
            .unwrap_or(PING_DATA.to_string());
        let interval = matches
            .get_one::<u64>("interval")
            .map(|t| t.clone())
            .unwrap_or(INTERVAL);
        let timeout = matches
            .get_one::<u64>("timeout")
            .map(|t| t.clone())
            .unwrap_or(TIMEOUT);
        let threshold = matches
            .get_one::<u32>("threshold")
            .map(|t| t.clone())
            .unwrap_or(THRESHOLD);

        Self {
            kind: monitor_kind,
            addrs,
            data,
            interval,
            timeout,
            threshold,
        }
    }

    pub(crate) fn kind(&self) -> MonitorKind {
        MonitorKind::from(self.kind.clone())
    }

    pub(crate) fn addrs(&self) -> String {
        self.addrs.clone()
    }

    pub(crate) fn data(&self) -> &[u8] {
        self.data.as_bytes()
    }

    pub(crate) fn interval(&self) -> Duration {
        Duration::from_secs(self.interval)
    }

    pub(crate) fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    pub(crate) fn threshold(&self) -> u32 {
        self.threshold
    }
}
