use crate::NetMCli;
use fastping_rs::Pinger;
use notify_rust::Notification;
use std::thread::sleep;

static mut FAILURE_COUNT: u64 = 0;

pub fn check_network_latency(netm: &NetMCli) {
    info!("Network availability is running");
    let threshold = netm.threshold();
    let interval = netm.interval();

    let (pinger, ping_result) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };

    pinger.add_ipaddr(&netm.addrs());
    pinger.run_pinger();

    loop {
        match ping_result.recv() {
            Ok(result) => match result {
                fastping_rs::PingResult::Idle { addr } => {
                    error!("Idle Address {}.", addr);
                }
                fastping_rs::PingResult::Receive { addr, rtt } => {
                    info!("Receive from Address {} in {:?}. ms", addr, rtt);
                    if rtt.as_millis() > threshold as u128 {
                        let msg = "High latency detected!";
                        Notification::new()
                            .summary(msg)
                            .body("High latency was detected while monitoring {addr}")
                            .show()
                            .unwrap();
                        unsafe {
                            FAILURE_COUNT += 1;
                            info!("{msg}, no. times detected: {}", FAILURE_COUNT);
                        }
                    }
                }
            },
            Err(_) => warn!("Worker threads disconnected before the solution was found!"),
        }

        sleep(interval)
    }
}
