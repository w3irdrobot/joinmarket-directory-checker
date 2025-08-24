use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub proxy: ProxyConfig,
    pub monitoring: MonitoringConfig,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub check_interval_seconds: u64,
    pub connection_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointStatus {
    Unknown,
    Checking,
    Online { response_time_ms: u64 },
    Offline { error: String },
}

impl EndpointStatus {
    pub fn status_text(&self) -> &'static str {
        match self {
            EndpointStatus::Unknown => "Unknown",
            EndpointStatus::Checking => "Checking",
            EndpointStatus::Online { .. } => "Online",
            EndpointStatus::Offline { .. } => "Offline",
        }
    }

    pub fn status_emoji(&self) -> &'static str {
        match self {
            EndpointStatus::Unknown => "⚪",
            EndpointStatus::Checking => "🟡",
            EndpointStatus::Online { .. } => "🟢",
            EndpointStatus::Offline { .. } => "🔴",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            EndpointStatus::Unknown => "status-unknown",
            EndpointStatus::Checking => "status-checking",
            EndpointStatus::Online { .. } => "status-online",
            EndpointStatus::Offline { .. } => "status-offline",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EndpointInfo {
    pub endpoint: Endpoint,
    pub status: EndpointStatus,
    pub last_check: Option<DateTime<Utc>>,
}

pub type StatusStore = HashMap<String, EndpointInfo>;
