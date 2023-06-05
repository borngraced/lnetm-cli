use notify_rust::Notification;
use ping_rs::PingOptions;
use std::{net::IpAddr, thread::sleep, time::Duration};

pub fn check_network_latency(
    addr: &IpAddr,
    data: &[u8],
    timeout: Duration,
    threshold: Option<u32>,
    interval: Duration,
) {
    let optons = PingOptions {
        ttl: 128,
        dont_fragment: true,
    };
    let ping_result = ping_rs::send_ping(addr, timeout, data, Some(&optons));

    match ping_result {
        Ok(reply) => {
            println!("RTT: {:.2} ms", reply.rtt);
            if reply.rtt > threshold.unwrap_or(100) {
                let msg = "High latency detected!";
                Notification::new()
                    .summary(msg)
                    .body("High latency was detected while monitoring {addr}")
                    .show()
                    .unwrap();
            }
        }
        Err(err) => println!("Error sending ping request: {err:?}"),
    }

    sleep(interval)
}
