use crate::NetMCli;
use notify_rust::Notification;
use std::{process::Command, thread::sleep};

static mut FAILURE_COUNT: u64 = 0;

pub fn check_network_availability(netm: &NetMCli) {
    info!("Network Availability monitoring is running");
    let ip_address = &netm.addrs();
    let interval = netm.timeout();

    loop {
        let output = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(ip_address)
            .output()
            .expect("failed to execute process");

        if output.status.success() == false {
            let msg = "Network device is unavailable";
            unsafe {
                FAILURE_COUNT += 1;
                info!("{msg}, FAILURES: {}", FAILURE_COUNT);
            }
            // Notification::new()
            //   .summary(msg)
            // .body("The network device is not responding to pings - failure count: {FAILURE_COUNT}")
            // .show()
            // .unwrap();
        }

        sleep(interval)
    }
}
