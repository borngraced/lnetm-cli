mod avail;
pub use avail::check_network_availability;
mod latency;
pub use latency::check_network_latency;
mod cli;
pub use cli::{MonitorKind, NetMCli};
