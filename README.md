# lnetm - Linux Network Monitoring Tool

lnetm is a command-line tool built in Rust for monitoring network latency and availability. It provides functionality to measure network latency and check the availability of network connections.

## Features

- Network latency monitoring: Measure the latency (response time) of network connections.
- Network availability monitoring: Check the availability (reachability) of network connections.
- Flexible monitoring options: Customize monitoring settings such as timeout, interval, and threshold.
- Simple command-line interface: Use command-line arguments to configure the monitoring parameters.

## Installation

To use lnetm, make sure you have Rust installed on your Linux system. Then, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/lnetm.git`
2. Navigate to the project directory: `cd lnetm`
3. Build the project: `cargo build --release`
4. Run the lnetm executable: `./target/release/lnetm [OPTIONS]`

## Usage

To use lnetm, run the executable with the desired options and parameters. Here are the available options:

lnetm [OPTIONS]

OPTIONS:
-m, --monitor <MONITOR>      What to monitor (latency, availability, or all) [default: all]
-a, --addr <IP_ADDRS>        Set IP addresses to monitor [required]
-d, --data <DATA>            Set data to ping to server [default: hello]
-t, --threshold <THRESHOLD>  Set threshold for latency (in seconds) [default: 10]
-o, --timeout <TIMEOUT>      Set timeout for latency monitoring (in seconds) [default: 10]
-i, --interval <INTERVAL>    Set monitoring interval (in seconds) [default: 10]


Example usage:

- Monitor latency: `lnetm -m latency -a 192.168.0.1`
- Monitor availability: `lnetm -m availability -a 192.168.0.1`
- Monitor both latency and availability: `lnetm -m all -a 192.168.0.1`

## Contributing

Contributions to lnetm are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

Contributions
-------------

- [ ] **Allow multiple address monitoring**: Enable monitoring of multiple IP addresses instead of just one.
- [ ] **Implement logging to file**: Integrate a logging to a file to improve visibility into the monitoring process.
- [ ] **Improve error handling**: Enhance error messages and handle errors more gracefully.
- [ ] **Add additional monitoring metrics**: Include metrics like packet loss rate, or bandwidth usage.

