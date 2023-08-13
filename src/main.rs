use log::info;
use nm::NetMCli;
use std::error::Error;
use std::process;

use crate::nm::{init_logging, stop_daemon};

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
