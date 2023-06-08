mod avail;
pub use avail::check_network_availability;
mod latency;
pub use latency::check_network_latency;
mod netm_cli;
pub use netm_cli::{MonitorKind, NetMCli};
