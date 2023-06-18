use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use nm::NetMCli;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

mod nm;

fn main() -> Result<(), Box<dyn Error>> {
    let netm = NetMCli::new();
    if netm.stop {
        stop_daemon()?;
        info!("Daemon stopped successfully.");
        process::exit(0);
    }

    // initialize logging
    init_logging()?;

    // run program
    netm.run();

    Ok(())
}

fn init_logging() -> Result<(), Box<dyn Error>> {
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

fn stop_daemon() -> std::io::Result<()> {
    let pid = fs::read_to_string("/tmp/lnetm.pid")?;
    let pid = pid.trim().parse::<i32>().unwrap();

    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }

    fs::remove_file("/tmp/lnetm.pid")?;

    Ok(())
}
