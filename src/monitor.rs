use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::connection::socks5_connect;
use crate::types::{Config, Endpoint, EndpointInfo, EndpointStatus, StatusStore};

pub type SharedStatusStore = Arc<RwLock<StatusStore>>;

pub struct MonitorService {
    config: Config,
    status_store: SharedStatusStore,
}

impl MonitorService {
    pub fn new(config: Config) -> Self {
        let status_store = Arc::new(RwLock::new(HashMap::new()));

        // Initialize status store with all endpoints
        let mut initial_store = HashMap::new();
        for endpoint in &config.endpoints {
            let key = format!("{}:{}", endpoint.address, endpoint.port);
            let endpoint_info = EndpointInfo {
                endpoint: endpoint.clone(),
                status: EndpointStatus::Unknown,
                last_check: None,
            };
            initial_store.insert(key, endpoint_info);
        }

        // Set the initial store
        let store_clone = status_store.clone();
        tokio::spawn(async move {
            let mut store = store_clone.write().await;
            *store = initial_store;
        });

        Self {
            config,
            status_store,
        }
    }

    pub fn get_status_store(&self) -> SharedStatusStore {
        self.status_store.clone()
    }

    pub async fn start_monitoring(&self) {
        let proxy_addr = format!("{}:{}", self.config.proxy.host, self.config.proxy.port)
            .parse::<SocketAddr>()
            .expect("Invalid proxy address");

        let check_interval = Duration::from_secs(self.config.monitoring.check_interval_seconds);
        let connection_timeout =
            Duration::from_secs(self.config.monitoring.connection_timeout_seconds);

        println!("🔍 Starting endpoint monitoring...");
        println!(
            "   Check interval: {}s",
            self.config.monitoring.check_interval_seconds
        );
        println!(
            "   Connection timeout: {}s",
            self.config.monitoring.connection_timeout_seconds
        );
        println!("   Endpoints to monitor: {}", self.config.endpoints.len());

        loop {
            let start_time = Instant::now();

            // Check all endpoints concurrently
            let mut tasks = Vec::new();

            for endpoint in &self.config.endpoints {
                let endpoint = endpoint.clone();
                let status_store = self.status_store.clone();

                let task = tokio::spawn(async move {
                    Self::check_endpoint(endpoint, proxy_addr, connection_timeout, status_store)
                        .await;
                });

                tasks.push(task);
            }

            // Wait for all checks to complete
            for task in tasks {
                let _ = task.await;
            }

            let check_duration = start_time.elapsed();
            println!(
                "✅ Completed monitoring cycle in {:.2}s",
                check_duration.as_secs_f64()
            );

            // Sleep until next check interval
            sleep(check_interval).await;
        }
    }

    async fn check_endpoint(
        endpoint: Endpoint,
        proxy_addr: SocketAddr,
        connection_timeout: Duration,
        status_store: SharedStatusStore,
    ) {
        let key = format!("{}:{}", endpoint.address, endpoint.port);
        let check_time = Utc::now();

        // Update status to "Checking"
        {
            let mut store = status_store.write().await;
            if let Some(endpoint_info) = store.get_mut(&key) {
                endpoint_info.status = EndpointStatus::Checking;
            }
        }

        println!("🔗 Checking {}", key);

        // Perform the actual connection check with timeout
        let status = match tokio::time::timeout(
            connection_timeout,
            Self::test_connection(proxy_addr, &endpoint.address, endpoint.port),
        )
        .await
        {
            Ok(Ok(response_time)) => {
                println!("✅ {} - Online ({}ms)", key, response_time);
                EndpointStatus::Online {
                    response_time_ms: response_time,
                }
            }
            Ok(Err(error)) => {
                println!("❌ {} - Offline: {}", key, error);
                EndpointStatus::Offline {
                    error: error.to_string(),
                }
            }
            Err(_) => {
                let timeout_msg = format!("Connection timeout ({}s)", connection_timeout.as_secs());
                println!("⏰ {} - {}", key, timeout_msg);
                EndpointStatus::Offline { error: timeout_msg }
            }
        };

        // Update the status store
        {
            let mut store = status_store.write().await;
            if let Some(endpoint_info) = store.get_mut(&key) {
                endpoint_info.status = status;
                endpoint_info.last_check = Some(check_time);
            }
        }
    }

    async fn test_connection(
        proxy_addr: SocketAddr,
        target_host: &str,
        target_port: u16,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = Instant::now();

        // Attempt to connect via SOCKS5
        let _stream = socks5_connect(proxy_addr, target_host, target_port).await?;

        let response_time = start_time.elapsed().as_millis() as u64;
        Ok(response_time)
    }
}

pub async fn start_background_monitoring(config: Config) -> SharedStatusStore {
    let monitor = MonitorService::new(config);
    let status_store = monitor.get_status_store();

    // Start monitoring in background task
    tokio::spawn(async move {
        monitor.start_monitoring().await;
    });

    status_store
}
