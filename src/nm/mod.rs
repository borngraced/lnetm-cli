mod avail;
use std::{error::Error, fs, path::PathBuf};

pub use avail::check_network_availability;
mod latency;
pub use latency::check_network_latency;
mod cli;
pub use cli::{MonitorKind, NetMCli};
use log::{info, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

const PID_PATH: &str = "/tmp/lnetm.pid";

pub fn init_logging() -> Result<(), Box<dyn Error>> {
    // Initialize log path
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let mut log_dir = dirs::home_dir().expect("Failed to get home directory");
    log_dir.push("lnetm");

    if !log_dir.exists() {
        std::fs::create_dir(&log_dir).expect("Failed to create log directory");
    }

    let mut log_path = PathBuf::from(&log_dir);
    log_path.push("lnetm.log");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}\n")))
        .build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}\n")))
        .build(log_path.clone())?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;
    info!(
        "... log initialized successfuly at: {:?}",
        log_path.as_path()
    );

    Ok(())
}

pub fn stop_daemon() -> std::io::Result<()> {
    let pid = fs::read_to_string(PID_PATH)?;
    let pid = pid.trim().parse::<i32>().unwrap();
    println!("{}", pid);

    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }

    fs::remove_file(PID_PATH)?;

    Ok(())
}
