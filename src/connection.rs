use std::net::{IpAddr, SocketAddr};

use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum Socks5Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SOCKS5 authentication failed")]
    AuthenticationFailed,
    #[error("SOCKS5 connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid SOCKS5 response")]
    InvalidResponse,
    #[error("Unsupported address type")]
    UnsupportedAddressType,
}

pub type Result<T> = std::result::Result<T, Socks5Error>;

/// SOCKS5 command types
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Connect = 0x01,
}

/// SOCKS5 address types
#[derive(Debug, Clone, Copy)]
pub enum AddressType {
    Ipv4 = 0x01,
    DomainName = 0x03,
    Ipv6 = 0x04,
}

/// Connect to a target through a SOCKS5 proxy (no authentication)
///
/// # Arguments
/// * `proxy_addr` - The SOCKS5 proxy server address
/// * `target_host` - Target hostname or IP address
/// * `target_port` - Target port number
///
/// # Returns
/// A connected TcpStream that can be used to communicate with the target
pub async fn socks5_connect(
    proxy_addr: SocketAddr,
    target_host: &str,
    target_port: u16,
) -> Result<TcpStream> {
    // Connect to the SOCKS5 proxy
    let mut stream = TcpStream::connect(proxy_addr).await?;

    // Step 1: Authentication negotiation (no auth only)
    negotiate_no_auth(&mut stream).await?;

    // Step 2: Send connection request
    send_connect_request(&mut stream, target_host, target_port).await?;

    // Step 3: Read connection response
    read_connect_response(&mut stream).await?;

    Ok(stream)
}

/// Negotiate no authentication with SOCKS5 proxy
async fn negotiate_no_auth(stream: &mut TcpStream) -> Result<()> {
    // Send authentication methods - only no auth
    let request = vec![
        0x05, // SOCKS version 5
        0x01, // Number of methods
        0x00, // No authentication
    ];

    stream.write_all(&request).await?;

    // Read response
    let mut response = [0u8; 2];
    stream.read_exact(&mut response).await?;

    if response[0] != 0x05 {
        return Err(Socks5Error::InvalidResponse);
    }

    let selected_method = response[1];
    if selected_method != 0x00 {
        return Err(Socks5Error::AuthenticationFailed);
    }

    Ok(())
}

/// Send CONNECT request to SOCKS5 proxy
async fn send_connect_request(
    stream: &mut TcpStream,
    target_host: &str,
    target_port: u16,
) -> Result<()> {
    let mut request = vec![
        0x05,                   // SOCKS version
        Command::Connect as u8, // Command
        0x00,                   // Reserved
    ];

    // Determine address type and encode address
    if let Ok(ip) = target_host.parse::<IpAddr>() {
        match ip {
            IpAddr::V4(ipv4) => {
                request.push(AddressType::Ipv4 as u8);
                request.extend_from_slice(&ipv4.octets());
            }
            IpAddr::V6(ipv6) => {
                request.push(AddressType::Ipv6 as u8);
                request.extend_from_slice(&ipv6.octets());
            }
        }
    } else {
        // Domain name
        if target_host.len() > 255 {
            return Err(Socks5Error::UnsupportedAddressType);
        }
        request.push(AddressType::DomainName as u8);
        request.push(target_host.len() as u8);
        request.extend_from_slice(target_host.as_bytes());
    }

    // Add port
    request.extend_from_slice(&target_port.to_be_bytes());

    stream.write_all(&request).await?;
    Ok(())
}

/// Read and parse CONNECT response from SOCKS5 proxy
async fn read_connect_response(stream: &mut TcpStream) -> Result<()> {
    // Read fixed part of response
    let mut response = [0u8; 4];
    stream.read_exact(&mut response).await?;

    if response[0] != 0x05 {
        return Err(Socks5Error::InvalidResponse);
    }

    // Check reply code
    match response[1] {
        0x00 => {} // Success
        0x01 => {
            return Err(Socks5Error::ConnectionFailed(
                "General SOCKS server failure".to_string(),
            ));
        }
        0x02 => {
            return Err(Socks5Error::ConnectionFailed(
                "Connection not allowed by ruleset".to_string(),
            ));
        }
        0x03 => {
            return Err(Socks5Error::ConnectionFailed(
                "Network unreachable".to_string(),
            ));
        }
        0x04 => {
            return Err(Socks5Error::ConnectionFailed(
                "Host unreachable".to_string(),
            ));
        }
        0x05 => {
            return Err(Socks5Error::ConnectionFailed(
                "Connection refused".to_string(),
            ));
        }
        0x06 => return Err(Socks5Error::ConnectionFailed("TTL expired".to_string())),
        0x07 => {
            return Err(Socks5Error::ConnectionFailed(
                "Command not supported".to_string(),
            ));
        }
        0x08 => {
            return Err(Socks5Error::ConnectionFailed(
                "Address type not supported".to_string(),
            ));
        }
        _ => return Err(Socks5Error::ConnectionFailed("Unknown error".to_string())),
    }

    // Read bound address (we don't use it, but need to consume it)
    let address_type = response[3];
    match address_type {
        0x01 => {
            // IPv4: 4 bytes + 2 bytes port
            let mut addr = [0u8; 6];
            stream.read_exact(&mut addr).await?;
        }
        0x03 => {
            // Domain name: 1 byte length + domain + 2 bytes port
            let mut len_buf = [0u8; 1];
            stream.read_exact(&mut len_buf).await?;
            let len = len_buf[0] as usize;
            let mut addr = vec![0u8; len + 2];
            stream.read_exact(&mut addr).await?;
        }
        0x04 => {
            // IPv6: 16 bytes + 2 bytes port
            let mut addr = [0u8; 18];
            stream.read_exact(&mut addr).await?;
        }
        _ => return Err(Socks5Error::InvalidResponse),
    }

    Ok(())
}
