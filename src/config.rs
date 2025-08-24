use std::fs;
use std::path::Path;

use crate::types::Config;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn create_default_config() -> Config {
    use crate::types::{Endpoint, MonitoringConfig, ProxyConfig, ServerConfig};

    Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        proxy: ProxyConfig {
            host: "127.0.0.1".to_string(),
            port: 9050,
        },
        monitoring: MonitoringConfig {
            check_interval_seconds: 30,
            connection_timeout_seconds: 10,
        },
        endpoints: vec![
            Endpoint {
                name: "Example Hidden Service".to_string(),
                address: "example1234567890abcdef1234567890abcdef12345678.onion".to_string(),
                port: 80,
            },
            Endpoint {
                name: "Another Service".to_string(),
                address: "another1234567890abcdef1234567890abcdef12345678.onion".to_string(),
                port: 8080,
            },
            Endpoint {
                name: "HTTPS Service".to_string(),
                address: "secure1234567890abcdef1234567890abcdef12345678.onion".to_string(),
                port: 443,
            },
            Endpoint {
                name: "RoboSats".to_string(),
                address: "robosatsy56bwqn56qyadmcxkx767hnabg4mihxlmgyt6if5gnuxvzad.onion"
                    .to_string(),
                port: 80,
            },
        ],
    }
}
