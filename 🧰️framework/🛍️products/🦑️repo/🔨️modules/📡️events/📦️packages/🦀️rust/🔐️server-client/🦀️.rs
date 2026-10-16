//! 🔐️ Server client: the hand-rolled HTTP/1.1 request to the coordinator.

use super::parse_target;
use std::io::{Read, Write};
use std::net::TcpStream;

//#region 🔐️ServerClient

/// 🌐️ The coordinator address `COMPOSE_SERVER_ADDR` names, normalised the way the Go twin does:
/// a bare authority gains `http://` and a trailing slash is dropped. Empty when unset.
pub fn server_addr() -> String {
    let addr = std::env::var("COMPOSE_SERVER_ADDR").unwrap_or_default();
    let addr = addr.trim();
    if addr.is_empty() {
        return String::new();
    }
    let addr = if addr.starts_with("http://") || addr.starts_with("https://") { addr.to_string() } else { format!("http://{addr}") };
    addr.strip_suffix('/').unwrap_or(&addr).to_string()
}

/// 🎟️ The API key `COMPOSE_SERVER_TOKEN` carries, trimmed.
pub fn server_token() -> String {
    std::env::var("COMPOSE_SERVER_TOKEN").unwrap_or_default().trim().to_string()
}

/// 📨️ One authenticated `GET` against the coordinator, answered as `(status line, body)`.
///
/// Only `http://` is reachable: this crate hand-rolls HTTP/1.1 over [`TcpStream`] and owns no TLS.
fn server_get(path: &str) -> Result<(u16, String, String), String> {
    let addr = server_addr();
    if addr.is_empty() {
        return Err("COMPOSE_SERVER_ADDR not set".to_string());
    }
    let target = parse_target(&format!("{addr}{path}")).ok_or_else(|| format!("invalid server address {addr}"))?;
    if target.secure {
        return Err("https coordinator addresses are not supported".to_string());
    }
    let mut request = format!("GET {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nConnection: close\r\n", target.path, target.host, target.port);
    let token = server_token();
    if !token.is_empty() {
        request.push_str(&format!("Authorization: Bearer {token}\r\n"));
    }
    request.push_str("\r\n");
    let mut stream = TcpStream::connect((target.host.as_str(), target.port)).map_err(|error| error.to_string())?;
    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(10)));
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(10)));
    stream.write_all(request.as_bytes()).map_err(|error| error.to_string())?;
    stream.flush().map_err(|error| error.to_string())?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|error| error.to_string())?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").ok_or_else(|| "malformed response".to_string())?;
    let status_line = head.lines().next().unwrap_or_default().trim().to_string();
    let mut fields = status_line.splitn(3, ' ');
    let _ = fields.next();
    let code: u16 = fields.next().unwrap_or_default().parse().map_err(|_| format!("malformed status line {status_line:?}"))?;
    let reason = fields.next().unwrap_or_default().to_string();
    Ok((code, if reason.is_empty() { code.to_string() } else { format!("{code} {reason}") }, body.to_string()))
}

/// 🔐️ The authenticated developer the coordinator reports, or the refusal the Go twin words.
///
/// Twin of `ServerWhoami` in `📦️packages/🐹️go/🐹️.go`: `GET /api/v1/auth`, a non-200 becoming
/// `auth failed: <status>`.
pub fn server_whoami() -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let (code, status, body) = server_get("/api/v1/auth")?;
    if code != 200 {
        return Err(format!("auth failed: {status}"));
    }
    match serde_json::from_str(&body) {
        Ok(serde_json::Value::Object(fields)) => Ok(fields),
        Ok(_) => Err("auth response is not an object".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

//#endregion 🔐️ServerClient
