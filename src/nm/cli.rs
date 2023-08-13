use clap::*;
use daemonize::Daemonize;
use log::info;
use std::{process, thread};

use crate::nm::{check_network_availability, check_network_latency};

const MONITOR_KIND: &str = "all";
const THRESHOLD: u32 = 100;
const TIMEOUT: u64 = 10;
const INTERVAL: u64 = 10;

#[derive(Debug, Clone)]
pub enum MonitorKind {
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

#[derive(Debug, Clone)]
pub struct NetMCli {
    pub(crate) kind: MonitorKind,
    pub(crate) addrs: String,
    pub(crate) interval: u64,
    pub(crate) timeout: u64,
    pub(crate) threshold: u32,
    pub(crate) daemon: bool,
    pub(crate) stop: bool,
}

impl NetMCli {
    pub fn new() -> Self {
        let matches = command!()
            .version("0.0.1")
            .author("Samuel Onoja")
            .name("lnetm")
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
            .arg(
                arg!(
                --daemon <DAEMON> "Run program as daemon service"
            )
                    .id("daemon")
                    .required(false)
                    .action(ArgAction::SetTrue)
            )
            .arg(
                arg!(
               --stop <STOP> "Stop the daemonized process"
            )
                    .id("stop")
                    .required(false)
                    .action(ArgAction::SetTrue)
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
        let stop = matches.get_flag("stop");
        let daemon = matches.get_flag("daemon");

        Self {
            kind: MonitorKind::from(monitor_kind),
            addrs,
            interval,
            timeout,
            threshold,
            daemon,
            stop,
        }
    }

    pub fn run(&self) {
        if self.daemon {
            self.run_as_daemon()
        } else {
            self.run_no_daemon()
        }
    }

    pub fn run_no_daemon(&self) {
        info!("Starting lnetm daemon... ");

        info!("... lnetm daemon started successfuly");
        match self.kind {
            MonitorKind::Latency => check_network_latency(&self),
            MonitorKind::Availability => check_network_availability(&self),
            MonitorKind::All => {
                let lnetm_clone = self.clone();
                thread::spawn(move || check_network_availability(&lnetm_clone));
                check_network_latency(&self);
            }
        }
    }

    pub fn run_as_daemon(&self) {
        info!("Starting lnetm daemon... ");
        let start_daemon = Daemonize::new()
            .pid_file("/tmp/lnetm.pid")
            .chown_pid_file(false)
            .working_directory("/tmp")
            .group("daemon")
            .start();

        match start_daemon {
            Ok(_) => {
                info!("... lnetm daemon started successfuly");
                match self.kind {
                    MonitorKind::Latency => check_network_latency(&self),
                    MonitorKind::Availability => check_network_availability(&self),
                    MonitorKind::All => {
                        let lnetm_clone = self.clone();
                        thread::spawn(move || check_network_availability(&lnetm_clone));
                        check_network_latency(&self);
                    }
                }
            }
            Err(err) => {
                log::error!("Failed to daemonize: {}", err);
                process::exit(1);
            }
        }
    }
}
