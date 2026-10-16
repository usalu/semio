//! 📤️ Emit: the coordinator emit path that turns an interaction into an event.

use super::Event;
use serde::Serialize;
use std::io::{Read, Write};
use std::net::TcpStream;

//#region 📤️Emit

/// 🛰️ Delivers an envelope to the coordinator.
pub trait Emitter {
    /// 📤️ Sends one envelope. Delivery failures are swallowed, exactly as in the Go twin.
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value);
}

/// 🌐️ Posts envelopes to `COMPOSE_SERVER_ADDR` over a hand-rolled HTTP/1.1 request.
#[derive(Debug, Clone, Default)]
pub struct HttpEmitter;

/// 🔗️ The coordinator endpoint for an address, empty when the address is empty.
pub fn emit_url(addr: &str) -> String {
    let addr = addr.trim();
    if addr.is_empty() {
        return String::new();
    }
    let url = if addr.starts_with("http://") || addr.starts_with("https://") {
        addr.to_string()
    } else {
        format!("http://{addr}")
    };
    format!("{}/api/v1/events", url.strip_suffix('/').unwrap_or(&url))
}

/// 🧭️ The host, port and path a request target decomposes into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestTarget {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub secure: bool,
}

/// 🔍️ Splits an absolute URL into its connection target.
pub fn parse_target(url: &str) -> Option<RequestTarget> {
    let (secure, rest) = match url.strip_prefix("http://") {
        Some(rest) => (false, rest),
        None => (true, url.strip_prefix("https://")?),
    };
    let (authority, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };
    if authority.is_empty() {
        return None;
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, port.parse().ok()?),
        None => (authority, if secure { 443 } else { 80 }),
    };
    Some(RequestTarget { host: host.to_string(), port, path: path.to_string(), secure })
}

impl Emitter for HttpEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
        let url = emit_url(&std::env::var("COMPOSE_SERVER_ADDR").unwrap_or_default());
        if url.is_empty() {
            return;
        }
        let Some(target) = parse_target(&url) else { return };
        if target.secure {
            return;
        }
        let envelope = Event { kind: kind.to_string(), source: source.to_string(), payload: payload.clone() };
        let Ok(body) = serde_json::to_vec(&envelope) else { return };
        let token = std::env::var("COMPOSE_SERVER_TOKEN").unwrap_or_default();
        let token = token.trim();
        let mut request = format!(
            "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
            target.path,
            target.host,
            target.port,
            body.len()
        );
        if !token.is_empty() {
            request.push_str(&format!("Authorization: Bearer {token}\r\n"));
        }
        request.push_str("\r\n");
        let Ok(mut stream) = TcpStream::connect((target.host.as_str(), target.port)) else { return };
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(5)));
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
        if stream.write_all(request.as_bytes()).is_err() {
            return;
        }
        if stream.write_all(&body).is_err() {
            return;
        }
        let _ = stream.flush();
        let mut discard = Vec::new();
        let _ = stream.read_to_end(&mut discard);
    }
}

/// 📤️ Posts an event to the repo coordinator. No-operation when `COMPOSE_SERVER_ADDR` is unset.
pub fn emit<T: Serialize>(kind: &str, source: &str, payload: &T) {
    let Ok(value) = serde_json::to_value(payload) else { return };
    HttpEmitter.emit(kind, source, &value);
}

//#endregion 📤️Emit
