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

- `-m`, `--monitor <MONITOR>`: Specifies what to monitor. Available options are:
  - `latency` | `l`: Monitor network latency.
  - `availability` | `a`: Monitor network availability.
  - `all`: Monitor both latency and availability. (default)(can ommit)

- `-a`, `--addr <IP_ADDRS>`: Specifies the IP addresses to monitor. This option is required. You can provide multiple IP addresses separated by commas.

- `-t`, `--threshold <THRESHOLD>`: Specifies the threshold for latency. Latency values above this threshold will be considered as high latency. The threshold value should be specified in seconds. (default: 10)

- `-o`, `--timeout <TIMEOUT>`: Specifies the timeout for latency monitoring. If the latency measurement exceeds this timeout value, it will be considered as a timeout. The timeout value should be specified in seconds. (default: 10)

- `-i`, `--interval <INTERVAL>`: Specifies the monitoring interval. This determines how frequently the network will be monitored. The interval value should be specified in seconds. (default: 10)

- `--stop`: Stops the lnetm daemon if it is currently running.

Note: If you start multiple instances of lnetm with different monitoring options simultaneously, each instance will run independently and monitor the network based on its specific configuration.

Example usage:

- To monitor network latency for a single IP address with a threshold of 20 seconds and a monitoring interval of 5 seconds:

    ```plaintext
    lnetm -m l -a 192.168.0.1 -t 20 -i 5
    ```

- To monitor network availability for multiple IP addresses with the default threshold and interval:

    ```plaintext
    lnetm -m a -a 192.168.0.1,192.168.0.2,192.168.0.3
    ```

- To monitor both latency and availability for a single IP address with the default threshold and interval:

    ```plaintext
    lnetm -a 192.168.0.1
    ```

- To stop the lnetm daemon if it is running:

    ```plaintext
    lnetm -a 192.168.0.1 --stop
    ```

## Contributing

Contributions to lnetm are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

Contributions
-------------

- [ ] **Allow multiple address monitoring**: Enable monitoring of multiple IP addresses instead of just one.
- [ ] **Implement logging to file**: Integrate a logging to a file to improve visibility into the monitoring process.
- [ ] **Improve error handling**: Enhance error messages and handle errors more gracefully.
- [ ] **Add additional monitoring metrics**: Include metrics like packet loss rate, or bandwidth usage.

