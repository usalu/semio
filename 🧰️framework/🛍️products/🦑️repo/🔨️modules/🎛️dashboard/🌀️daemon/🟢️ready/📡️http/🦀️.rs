//! 🌐️ Bounded HTTP readiness of the exact announced local address.
//!
//! @see ../../../🧪️tests/🧭️journeys/🥒️.feature

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const BUDGET: Duration = Duration::from_millis(500);
const SLICE: Duration = Duration::from_millis(50);
const CONNECT: Duration = Duration::from_millis(100);
const HEADER_LIMIT: usize = 8192;

/// 🟢️ Probes a local HTTP handler without extending its deadline or ignoring cancellation.
pub(crate) fn probe(url: &str, cancelled: &AtomicBool) -> bool {
    let Some(rest) = url.strip_prefix("http://") else { return false };
    let boundary = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let Some((host, port)) = rest[..boundary].split_once(':') else { return false };
    let Some(port) = port.parse::<u16>().ok().filter(|port| *port != 0) else { return false };
    let addresses: &[IpAddr] = match host {
        "127.0.0.1" | "0.0.0.0" => &[IpAddr::V4(Ipv4Addr::LOCALHOST)],
        "localhost" => &[IpAddr::V6(Ipv6Addr::LOCALHOST), IpAddr::V4(Ipv4Addr::LOCALHOST)],
        _ => return false,
    };
    let tail = rest[boundary..].split('#').next().unwrap_or_default();
    let target = if tail.starts_with('/') { tail.to_string() } else { format!("/{tail}") };
    if target.bytes().any(|byte| byte <= b' ' || byte == 0x7f) { return false; }
    let request = format!("GET {target} HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\n\r\n");
    let deadline = Instant::now() + BUDGET;
    for address in addresses {
        if cancelled.load(Ordering::Acquire) { return false; }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() { return false; }
        let Ok(mut stream) = TcpStream::connect_timeout(&SocketAddr::new(*address, port), CONNECT.min(remaining)) else { continue };
        if stream.set_write_timeout(Some(SLICE)).is_err() || stream.write_all(request.as_bytes()).is_err() || stream.set_read_timeout(Some(SLICE)).is_err() { continue; }
        let mut header = Vec::new();
        let mut chunk = [0u8; 1024];
        while Instant::now() < deadline && !cancelled.load(Ordering::Acquire) && header.len() < HEADER_LIMIT {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() || stream.set_read_timeout(Some(SLICE.min(remaining))).is_err() { break; }
            match stream.read(&mut chunk) {
                Ok(0) => break,
                Ok(count) => {
                    header.extend_from_slice(&chunk[..count]);
                    if let Some(end) = header.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                        let line = std::str::from_utf8(&header[..end]).ok().and_then(|text| text.lines().next()).unwrap_or_default();
                        let mut fields = line.split_whitespace();
                        let version = fields.next();
                        let status = fields.next().and_then(|status| status.parse::<u16>().ok());
                        return matches!(version, Some("HTTP/1.0" | "HTTP/1.1")) && status.is_some_and(|status| (200..400).contains(&status));
                    }
                }
                Err(error) if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut | std::io::ErrorKind::Interrupted) => {}
                Err(_) => break,
            }
        }
    }
    false
}