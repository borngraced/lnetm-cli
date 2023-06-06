#[macro_use]
extern crate log;

mod availability;
use std::thread;

use availability::check_network_availability;

mod latency;
use latency::check_network_latency;

mod netm_cli;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use netm_cli::*;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("/lnetm.log")
        .expect("logging init failed");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("logging init failed");

    log4rs::init_config(config).expect("logging init failed");

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
