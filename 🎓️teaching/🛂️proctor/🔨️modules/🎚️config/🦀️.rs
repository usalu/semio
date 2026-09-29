//! 🎚️ The proctor's configuration — environment only, validated before anything is opened.
//!
//! | Variable | Meaning | Default |
//! |---|---|---|
//! | `PROCTOR_PORT` | TCP port | `8791` |
//! | `PROCTOR_BIND` | IP address to bind | `127.0.0.1` |
//! | `PROCTOR_DATA` | directory of `proctor.sqlite` | required |
//! | `PROCTOR_CATALOG` | catalog `🔣️.json` | required |
//! | `PROCTOR_MODE` | `development` or `production` | loopback bind → development, else production |
//! | `PROCTOR_ALLOWED_ORIGINS` | comma-separated `scheme://host[:port]` origins granted CORS | loopback bind → any loopback origin, else none |
//! | `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` (a TLS-terminating proxy is the only client) | `none` |
//! | `PROCTOR_PRESENCE_TICK_MS` | how often a presence room publishes its coalesced batch, 10…1000 ms | `100` |
//!
//! A production proctor on a network interface requires all three statements together: the mode,
//! an explicit origin allowlist (the origin the site is served from, e.g.
//! `https://quizzes.architektur-und-technologie.de`) and `PROCTOR_TRUSTED_FORWARDING=proxy`, exactly
//! like the hub. A development proctor binds loopback only.
//!
//! @see ../../../../🌎️hub/README.md — the production gating this mirrors

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::time::Duration;

use server::gateway::PresenceSettings;

/// 🔌️ `PROCTOR_PORT`.
pub const PORT: &str = "PROCTOR_PORT";
/// 📡️ `PROCTOR_BIND`.
pub const BIND: &str = "PROCTOR_BIND";
/// 🗄️ `PROCTOR_DATA`.
pub const DATA: &str = "PROCTOR_DATA";
/// 📚️ `PROCTOR_CATALOG`.
pub const CATALOG: &str = "PROCTOR_CATALOG";
/// 🏭️ `PROCTOR_MODE`.
pub const MODE: &str = "PROCTOR_MODE";
/// 🌍️ `PROCTOR_ALLOWED_ORIGINS`.
pub const ALLOWED_ORIGINS: &str = "PROCTOR_ALLOWED_ORIGINS";
/// 🛡️ `PROCTOR_TRUSTED_FORWARDING`.
pub const TRUSTED_FORWARDING: &str = "PROCTOR_TRUSTED_FORWARDING";
/// 👥️ `PROCTOR_PRESENCE_TICK_MS`.
pub const PRESENCE_TICK_MS: &str = "PROCTOR_PRESENCE_TICK_MS";

/// 🔢️ The port a proctor listens on when `PROCTOR_PORT` is unset.
pub const DEFAULT_PORT: u16 = 8791;

const ALLOWED_ORIGINS_MAX: usize = 32;

/// ⏱️ The bounds of `PROCTOR_PRESENCE_TICK_MS`: below, the batches cost more than they save; above, the
/// cursors of others visibly lag.
pub const PRESENCE_TICK_MS_RANGE: std::ops::RangeInclusive<u64> = 10..=1000;

/// 🏭️ Which posture the proctor runs in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProctorMode {
    Development,
    Production,
}

/// 🌍️ Which browser origins receive a credentialed CORS grant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrossOriginPolicy {
    LoopbackDevelopment,
    Allowlist(Vec<String>),
    Closed,
}

/// 🛡️ Whether forwarding headers are trusted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forwarding {
    Untrusted,
    TerminatingProxy,
}

/// 🎚️ Everything the proctor reads from its environment.
#[derive(Clone, Debug, PartialEq)]
pub struct ProctorConfig {
    pub bind: IpAddr,
    pub port: u16,
    pub data: PathBuf,
    pub catalog: PathBuf,
    pub mode: ProctorMode,
    pub origins: CrossOriginPolicy,
    pub forwarding: Forwarding,
    pub presence_tick: Duration,
}

/// 🧯️ A configuration the proctor refuses to boot with, naming the variable at fault.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

/// 🧭️ The gate every request passes before the gateway: cross-origin policy and transport trust.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gate {
    pub origins: CrossOriginPolicy,
    pub forwarding: Forwarding,
}

impl ProctorConfig {
    /// 📥️ Read and validate the configuration through `read` (the process environment in
    /// production, a map in tests).
    pub fn from_environment(read: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let value = |name: &str| read(name).map(|value| value.trim().to_string()).filter(|value| !value.is_empty());
        let bind: IpAddr = match value(BIND) {
            Some(text) => text.parse().map_err(|_| ConfigError(format!("{BIND} must be an IP address, got {text:?}")))?,
            None => IpAddr::V4(Ipv4Addr::LOCALHOST),
        };
        let port = match value(PORT) {
            Some(text) => text.parse().map_err(|_| ConfigError(format!("{PORT} must be a port number, got {text:?}")))?,
            None => DEFAULT_PORT,
        };
        let data = value(DATA).map(PathBuf::from).ok_or_else(|| ConfigError(format!("{DATA} must name the directory of proctor.sqlite")))?;
        let catalog = value(CATALOG).map(PathBuf::from).ok_or_else(|| ConfigError(format!("{CATALOG} must name the catalog 🔣️.json")))?;
        let mode = match value(MODE).as_deref() {
            Some("production") => ProctorMode::Production,
            Some("development") => ProctorMode::Development,
            Some(other) => return Err(ConfigError(format!("{MODE} must be development or production, got {other:?}"))),
            None if bind.is_loopback() => ProctorMode::Development,
            None => ProctorMode::Production,
        };
        let origins = cross_origin_policy(value(ALLOWED_ORIGINS), bind)?;
        let forwarding = match value(TRUSTED_FORWARDING).as_deref() {
            None | Some("none") => Forwarding::Untrusted,
            Some("proxy") => Forwarding::TerminatingProxy,
            Some(other) => return Err(ConfigError(format!("{TRUSTED_FORWARDING} must be none or proxy, got {other:?}"))),
        };
        let presence_tick = match value(PRESENCE_TICK_MS) {
            Some(text) => text.parse().ok().filter(|millis| PRESENCE_TICK_MS_RANGE.contains(millis)).map(Duration::from_millis).ok_or_else(|| ConfigError(format!("{PRESENCE_TICK_MS} must be a whole number of milliseconds in {}..={}, got {text:?}", PRESENCE_TICK_MS_RANGE.start(), PRESENCE_TICK_MS_RANGE.end())))?,
            None => PresenceSettings::default().tick,
        };
        let config = Self { bind, port, data, catalog, mode, origins, forwarding, presence_tick };
        config.validate()?;
        Ok(config)
    }

    /// 🛂️ The three-statement production rule and the loopback-only development rule.
    pub fn validate(&self) -> Result<(), ConfigError> {
        match self.mode {
            ProctorMode::Development if !self.bind.is_loopback() => Err(ConfigError(format!("development mode must bind loopback; set {MODE}=production to bind {}", self.bind))),
            ProctorMode::Production if !self.bind.is_loopback() && !matches!(self.origins, CrossOriginPolicy::Allowlist(_)) => Err(ConfigError(format!("a production proctor on a network interface requires {ALLOWED_ORIGINS} to name the origins its browsers are served from"))),
            ProctorMode::Production if !self.bind.is_loopback() && self.forwarding != Forwarding::TerminatingProxy => Err(ConfigError(format!("a production proctor on a network interface speaks cleartext HTTP and requires {TRUSTED_FORWARDING}=proxy, stating that a TLS-terminating reverse proxy is the only thing that reaches it"))),
            _ => Ok(()),
        }
    }

    /// 🧭️ The request gate this configuration implies.
    pub fn gate(&self) -> Gate {
        Gate { origins: self.origins.clone(), forwarding: self.forwarding }
    }

    /// 👥️ The presence socket's settings: the protocol's defaults at this configuration's tick.
    pub fn presence(&self) -> PresenceSettings {
        PresenceSettings { tick: self.presence_tick, ..PresenceSettings::default() }
    }
}

impl ProctorMode {
    /// 🏷️ The word the startup line reports.
    pub fn label(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

impl CrossOriginPolicy {
    /// ✅️ Whether `origin` earns the CORS grant.
    pub fn admits(&self, origin: &str) -> bool {
        match self {
            Self::LoopbackDevelopment => is_loopback_origin(origin),
            Self::Allowlist(origins) => origins.iter().any(|allowed| allowed.eq_ignore_ascii_case(origin)),
            Self::Closed => false,
        }
    }

    /// 🏷️ The word the startup line reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::LoopbackDevelopment => "loopback-development",
            Self::Allowlist(_) => "allowlist",
            Self::Closed => "closed",
        }
    }

    /// 📜️ The startup line's account of the policy: its label, and the admitted origins of an allowlist.
    pub fn describe(&self) -> String {
        match self {
            Self::Allowlist(origins) => format!("{} {}", self.label(), origins.join(", ")),
            _ => self.label().to_string(),
        }
    }
}

impl Forwarding {
    /// 🔐️ Whether a request whose first `X-Forwarded-Proto` value is `proto` reached the client
    /// securely, as far as this proctor may know. Untrusted forwarding never reads the header.
    pub fn secure(self, proto: Option<&str>) -> bool {
        match self {
            Self::Untrusted => true,
            Self::TerminatingProxy => proto.and_then(|value| value.split(',').next()).is_some_and(|scheme| scheme.trim().eq_ignore_ascii_case("https")),
        }
    }

    /// 🏷️ The word the startup line reports.
    pub fn label(self) -> &'static str {
        match self {
            Self::Untrusted => "none",
            Self::TerminatingProxy => "proxy",
        }
    }
}

fn cross_origin_policy(encoded: Option<String>, bind: IpAddr) -> Result<CrossOriginPolicy, ConfigError> {
    let Some(encoded) = encoded else {
        return Ok(if bind.is_loopback() { CrossOriginPolicy::LoopbackDevelopment } else { CrossOriginPolicy::Closed });
    };
    let mut origins: Vec<String> = Vec::new();
    for origin in encoded.split(',').map(str::trim).filter(|origin| !origin.is_empty()) {
        if !is_browser_origin(origin) {
            return Err(ConfigError(format!("{ALLOWED_ORIGINS} entry {origin:?} is not a scheme://host[:port] origin")));
        }
        if !origins.iter().any(|existing| existing.eq_ignore_ascii_case(origin)) {
            origins.push(origin.to_string());
        }
        if origins.len() > ALLOWED_ORIGINS_MAX {
            return Err(ConfigError(format!("{ALLOWED_ORIGINS} exceeds {ALLOWED_ORIGINS_MAX} entries")));
        }
    }
    if origins.is_empty() {
        return Err(ConfigError(format!("{ALLOWED_ORIGINS} names no origin")));
    }
    Ok(CrossOriginPolicy::Allowlist(origins))
}

/// 🌐️ A serialized origin `scheme://host[:port]` without userinfo, path, query, fragment or wildcard.
pub fn is_browser_origin(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once("://") else { return false };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    if rest.is_empty() || rest.contains(['/', '?', '#', '@', '*', ' ', '\t']) {
        return false;
    }
    let (host, port) = match rest.strip_prefix('[') {
        Some(bracketed) => match bracketed.split_once(']') {
            Some((inside, port)) if inside.parse::<Ipv6Addr>().is_ok() => (inside, port),
            _ => return false,
        },
        None => match rest.find(':') {
            Some(index) => (&rest[..index], &rest[index..]),
            None => (rest, ""),
        },
    };
    !host.is_empty() && (port.is_empty() || port.strip_prefix(':').is_some_and(|digits| !digits.is_empty() && digits.parse::<u16>().is_ok()))
}

/// 🏠️ Whether an origin's host is this machine — `localhost`, `127.0.0.0/8` or `[::1]`.
pub fn is_loopback_origin(value: &str) -> bool {
    if !is_browser_origin(value) {
        return false;
    }
    let Some((_, rest)) = value.split_once("://") else { return false };
    match rest.strip_prefix('[') {
        Some(bracketed) => bracketed.split_once(']').is_some_and(|(inside, _)| inside.parse::<Ipv6Addr>().is_ok_and(|address| address.is_loopback())),
        None => {
            let host = rest.split_once(':').map_or(rest, |(name, _)| name);
            host.eq_ignore_ascii_case("localhost") || host.parse::<IpAddr>().is_ok_and(|address| address.is_loopback())
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
