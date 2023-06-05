use crate::NetMCli;
use log::info;
use notify_rust::Notification;
use ping_rs::PingOptions;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{net::IpAddr, thread::sleep};

const FAILURE_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn check_network_latency(netm: &NetMCli) {
    info!("Network availability is running");

    let optons = PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    let parsed_addr = IpAddr::from(Ipv4Addr::from_str(&netm.addrs()).expect("A valid ip addr"));
    let addr = &parsed_addr;
    let data = netm.data();
    let timeout = netm.timeout();
    let threshold = netm.threshold();
    let interval = netm.interval();
    let ping_result = ping_rs::send_ping(addr, timeout, data, Some(&optons));

    match ping_result {
        Ok(reply) => {
            info!("RTT: {:.2} ms", reply.rtt,);
            if reply.rtt > threshold {
                let msg = "High latency detected!";
                Notification::new()
                    .summary(msg)
                    .body("High latency was detected while monitoring {addr}")
                    .show()
                    .unwrap();
                info!(
                    "{msg}, FAILURES: {}",
                    FAILURE_COUNT
                        .fetch_add(FAILURE_COUNT.load(Ordering::Relaxed), Ordering::Relaxed)
                );
            }
        }
        Err(err) => println!("Error sending ping request: {err:?}"),
    }

    sleep(interval)
}
