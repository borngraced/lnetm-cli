use crate::NetMCli;
use log::{info, warn};
use notify_rust::Notification;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{process::Command, thread::sleep};

const FAILURE_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn check_network_availability(netm: &NetMCli) {
    info!("Latency monitoring is running");
    let ip_address = &netm.addrs();
    let interval = netm.timeout();
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip_address)
        .output()
        .expect("failed to execute process");

    info!("{}", String::from_utf8_lossy(&output.stdout));

    if output.status.success() == false {
        let msg = "Network device is unavailable";
        Notification::new()
            .summary(msg)
            .body("The network device is not responding to pings")
            .show()
            .unwrap();
        warn!(
            "{msg}, FAILURES: {}",
            FAILURE_COUNT.fetch_add(FAILURE_COUNT.load(Ordering::Relaxed), Ordering::Relaxed)
        );
    }

    sleep(interval)
}
