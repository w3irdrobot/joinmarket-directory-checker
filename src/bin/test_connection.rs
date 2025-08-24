use std::net::SocketAddr;

use joinmarket_directory_checker::connection::socks5_connect;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SOCKS5 Connect Test (No Authentication)");

    let proxy_port = std::env::args().nth(1).unwrap_or("1080".to_string());
    let proxy_addr: SocketAddr = format!("127.0.0.1:{}", proxy_port).parse()?;

    // Test 1: Connect to a web server via SOCKS5
    println!("\n=== Test 1: HTTP Connection ===");
    match socks5_connect(proxy_addr, "httpbin.org", 80).await {
        Ok(mut stream) => {
            println!("✓ Successfully connected to httpbin.org:80 via SOCKS5 proxy");

            // Send a simple HTTP request to verify the connection works
            let request = "GET /ip HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
            stream.write_all(request.as_bytes()).await?;

            // Read first part of response
            let mut buffer = [0u8; 1024];
            let bytes_read = stream.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);

            if response.contains("HTTP/1.1 200 OK") {
                println!("✓ HTTP request successful - connection is working!");
                println!(
                    "Response preview: {}",
                    response.lines().take(3).collect::<Vec<_>>().join(" | ")
                );
            } else {
                println!("Response: {}", response);
            }
        }
        Err(e) => {
            println!("✗ Failed to connect: {}", e);
            println!("  Note: This requires a SOCKS5 proxy running on 127.0.0.1:1080");
        }
    }

    // Test 2: Connect to an IP address
    println!("\n=== Test 2: IP Address Connection ===");
    match socks5_connect(proxy_addr, "8.8.8.8", 53).await {
        Ok(_stream) => {
            println!("✓ Successfully connected to 8.8.8.8:53 (Google DNS) via SOCKS5");
        }
        Err(e) => {
            println!("✗ Failed to connect to IP: {}", e);
        }
    }

    // Test 3: Connect to a domain name
    println!("\n=== Test 3: Domain Name Connection ===");
    match socks5_connect(proxy_addr, "example.com", 80).await {
        Ok(_stream) => {
            println!("✓ Successfully connected to example.com:80 via SOCKS5");
        }
        Err(e) => {
            println!("✗ Failed to connect to domain: {}", e);
        }
    }

    // Test 4: Connect to a tor address
    println!("\n=== Test 4: Tor Address Connection ===");
    match socks5_connect(
        proxy_addr,
        "robosatsy56bwqn56qyadmcxkx767hnabg4mihxlmgyt6if5gnuxvzad.onion",
        80,
    )
    .await
    {
        Ok(_stream) => {
            println!(
                "✓ Successfully connected to robosatsy56bwqn56qyadmcxkx767hnabg4mihxlmgyt6if5gnuxvzad.onion via SOCKS5"
            );
        }
        Err(e) => {
            println!("✗ Failed to connect to tor: {}", e);
        }
    }
    Ok(())
}
