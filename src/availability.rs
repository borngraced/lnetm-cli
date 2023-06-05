use notify_rust::Notification;
use std::{process::Command, thread::sleep, time::Duration};

pub fn check_network_availability(ip_address: &str, interval: Duration) {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip_address)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    if output.status.success() == false {
        Notification::new()
            .summary("Network device is unavailable")
            .body("The network device is not responding to pings")
            .show()
            .unwrap();
    }

    sleep(interval)
}
