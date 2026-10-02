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
//! | `PROCTOR_PRESENCE_TICK_MS` | how often, at most, a presence socket is sent what changed in its room, 10…1000 ms | `100` |
//! | `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES` | the most one presence frame carries — per socket and tick, and per socket and watch interval over everything it watches —, 4096…65536 bytes | `4096` |
//! | `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` | the most all presence rooms together send per second, 65536…4294967296 bytes | `16777216` |
//! | `PROCTOR_LIMIT_BODY_BYTES` | largest request body, 1024…1048576 bytes | `16384` |
//! | `PROCTOR_LIMIT_COMMANDS_PER_SECOND`, `…_COMMANDS_BURST` | per client address: sustained and burst `POST /commands` | `300`, `900` |
//! | `PROCTOR_LIMIT_QUERIES_PER_SECOND`, `…_QUERIES_BURST` | per client address: sustained and burst reads (`POST /queries`, every `GET`) | `600`, `1800` |
//! | `PROCTOR_LIMIT_UPGRADES_PER_SECOND`, `…_UPGRADES_BURST` | per client address: sustained and burst WebSocket upgrades | `100`, `1800` |
//! | `PROCTOR_LIMIT_SIGNUPS_PER_HOUR`, `…_SIGNUPS_BURST` | per client address: registrations (anonymous learners and claimed handles) accepted per hour and at once; one beyond is `429` naming the allowance `sign-up` | `100`, `1200` |
//! | `PROCTOR_LIMIT_SOCKETS_PER_ADDRESS` | WebSockets one client address may hold open | `2048` |
//! | `PROCTOR_LIMIT_SOCKETS` | WebSockets open in total | `8192` |
//! | `PROCTOR_LIMIT_IN_FLIGHT` | requests served at once in total | `2048` |
//! | `PROCTOR_MAX_LEARNERS` | registrations in total (anonymous learners and claimed handles); one beyond it is `roster-full` | `100000` |
//! | `PROCTOR_MAX_RUNS` | submitted runs per learner over all quizzes; a start beyond it is `runs-exhausted` | `1000` |
//!
//! The two caps are the configurable totals of the quiz core's `Limits`; its other two — 200
//! submitted runs per learner and quiz, 2000 recorded answers per run — are fixed. None is reached by
//! a class of three hundred playing every quiz dozens of times; they bound what one client can make
//! the proctor store ([`ProctorConfig::caps`]).
//!
//! The limits are sized so that a lecture hall of three hundred behind one address is never
//! throttled and one script cannot take the proctor down ([`Limits::default`]); the client address is
//! the last `X-Forwarded-For` entry under `PROCTOR_TRUSTED_FORWARDING=proxy` and the peer otherwise.
//!
//! **Sign-ups are counted apart** ([`SIGN_UP`], [`SIGN_UPS`]). A registration is the one command
//! that takes from a finite store — the registration cap —, so the requests' own rate is no bound
//! for it: at 300 commands per second one address would fill any cap the disk can hold within
//! minutes, and every learner after it would be refused `roster-full`. The allowance counts the
//! registrations of an address that were **accepted** (a refused handle, a replay, a malformed
//! command hand their token back): 1200 at once is a lecture hall of three hundred four times over —
//! everybody signing up within a minute, the next hall right after, students who cleared their
//! browser signing up again —, and 100 per hour hands a hall's worth back every three hours, so the
//! same hall is never refused the week after. One address thus registers at most 1200 and 100 more
//! per hour: the default cap of 100 000 registrations takes it (100 000 − 1200) / 100 = 988 hours,
//! 41 days, where it took 34 seconds before.
//!
//! A production proctor on a network interface requires all three statements together: the mode,
//! an explicit origin allowlist (the origin the site is served from, e.g.
//! `https://quizze.architektur-und-technologie.de`) and `PROCTOR_TRUSTED_FORWARDING=proxy`, exactly
//! like the hub. A development proctor binds loopback only.
//!
//! @see ../../../../🌎️hub/README.md — the production gating this mirrors

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::time::Duration;

use server::gateway::PresenceSettings;
use server::throttle::{Allowance, Limits, Rate};

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
/// 🖼️ `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES`.
pub const LIMIT_PRESENCE_FRAME_BYTES: &str = "PROCTOR_LIMIT_PRESENCE_FRAME_BYTES";
/// 🚰️ `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND`.
pub const LIMIT_PRESENCE_BYTES_PER_SECOND: &str = "PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND";
/// 📦️ `PROCTOR_LIMIT_BODY_BYTES`.
pub const LIMIT_BODY_BYTES: &str = "PROCTOR_LIMIT_BODY_BYTES";
/// ✍️ `PROCTOR_LIMIT_COMMANDS_PER_SECOND`.
pub const LIMIT_COMMANDS_PER_SECOND: &str = "PROCTOR_LIMIT_COMMANDS_PER_SECOND";
/// 💥️ `PROCTOR_LIMIT_COMMANDS_BURST`.
pub const LIMIT_COMMANDS_BURST: &str = "PROCTOR_LIMIT_COMMANDS_BURST";
/// 📖️ `PROCTOR_LIMIT_QUERIES_PER_SECOND`.
pub const LIMIT_QUERIES_PER_SECOND: &str = "PROCTOR_LIMIT_QUERIES_PER_SECOND";
/// 📚️ `PROCTOR_LIMIT_QUERIES_BURST`.
pub const LIMIT_QUERIES_BURST: &str = "PROCTOR_LIMIT_QUERIES_BURST";
/// 🔌️ `PROCTOR_LIMIT_UPGRADES_PER_SECOND`.
pub const LIMIT_UPGRADES_PER_SECOND: &str = "PROCTOR_LIMIT_UPGRADES_PER_SECOND";
/// 🌩️ `PROCTOR_LIMIT_UPGRADES_BURST`.
pub const LIMIT_UPGRADES_BURST: &str = "PROCTOR_LIMIT_UPGRADES_BURST";
/// 🪪️ `PROCTOR_LIMIT_SIGNUPS_PER_HOUR`.
pub const LIMIT_SIGNUPS_PER_HOUR: &str = "PROCTOR_LIMIT_SIGNUPS_PER_HOUR";
/// 🏟️ `PROCTOR_LIMIT_SIGNUPS_BURST`.
pub const LIMIT_SIGNUPS_BURST: &str = "PROCTOR_LIMIT_SIGNUPS_BURST";
/// 🧵️ `PROCTOR_LIMIT_SOCKETS_PER_ADDRESS`.
pub const LIMIT_SOCKETS_PER_ADDRESS: &str = "PROCTOR_LIMIT_SOCKETS_PER_ADDRESS";
/// 🧶️ `PROCTOR_LIMIT_SOCKETS`.
pub const LIMIT_SOCKETS: &str = "PROCTOR_LIMIT_SOCKETS";
/// 🌊️ `PROCTOR_LIMIT_IN_FLIGHT`.
pub const LIMIT_IN_FLIGHT: &str = "PROCTOR_LIMIT_IN_FLIGHT";
/// 🧑‍🤝‍🧑️ `PROCTOR_MAX_LEARNERS`.
pub const MAX_LEARNERS: &str = "PROCTOR_MAX_LEARNERS";
/// 🏃️ `PROCTOR_MAX_RUNS`.
pub const MAX_RUNS: &str = "PROCTOR_MAX_RUNS";

/// 🔢️ The port a proctor listens on when `PROCTOR_PORT` is unset.
pub const DEFAULT_PORT: u16 = 8791;

/// 🎟️ The allowance a registration is counted against at the edge: one token of its client address
/// per accepted registration, an anonymous learner or a claimed handle. The name a `429` states.
pub const SIGN_UP: &str = "sign-up";

/// 💰️ The sign-up allowance of one client address unless `PROCTOR_LIMIT_SIGNUPS_*` state another:
/// 1200 registrations at once — a lecture hall of three hundred four times over — and 100 more
/// per hour.
pub const SIGN_UPS: Rate = Rate::per_hour(100, 1200);

const ALLOWED_ORIGINS_MAX: usize = 32;

/// ⏱️ The bounds of `PROCTOR_PRESENCE_TICK_MS`: below, the batches cost more than they save; above, the
/// cursors of others visibly lag.
pub const PRESENCE_TICK_MS_RANGE: std::ops::RangeInclusive<u64> = 10..=1000;

/// 🖼️ The bounds of `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES`: below, a frame no longer holds one state of
/// the largest size next to another; above, a socket that stopped reading holds too much.
pub const LIMIT_PRESENCE_FRAME_BYTES_RANGE: std::ops::RangeInclusive<u64> = 4096..=65_536;

/// 🚰️ The bounds of `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND`: below, a handful of sockets already
/// wait; above, the bound is none on any link a proctor is served over.
pub const LIMIT_PRESENCE_BYTES_PER_SECOND_RANGE: std::ops::RangeInclusive<u64> = 65_536..=4_294_967_296;

/// 📦️ The bounds of `PROCTOR_LIMIT_BODY_BYTES`: below, a legitimate answer no longer fits; above, the
/// limit is no protection.
pub const LIMIT_BODY_BYTES_RANGE: std::ops::RangeInclusive<u64> = 1024..=1_048_576;

/// 🔢️ The bounds of every other `PROCTOR_LIMIT_*`: at least one, and nothing a counter cannot hold.
pub const LIMIT_RANGE: std::ops::RangeInclusive<u64> = 1..=1_000_000;

/// 🏭️ Which posture the proctor runs in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProctorMode {
    Development,
    Production,
}

/// 🌍️ Which browser origins receive the CORS grant.
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
    pub presence_frame_bytes: usize,
    pub presence_bytes_per_second: u64,
    pub limits: Limits,
    pub caps: quiz::Limits,
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

/// 🧭️ The gate every request passes before the gateway: cross-origin policy, transport trust and
/// the edge limits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gate {
    pub origins: CrossOriginPolicy,
    pub forwarding: Forwarding,
    pub limits: Limits,
}

impl ProctorConfig {
    /// 📥️ Read and validate the configuration through `read` (the process environment in
    /// production, a map in tests).
    pub fn from_environment(read: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let value = |name: &str| setting(&read, name);
        let (bind, port) = (bind(&read)?, port(&read)?);
        let data = Self::data_directory(&read)?;
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
        let presence = PresenceSettings::default();
        let presence_frame_bytes = bounded(&read, LIMIT_PRESENCE_FRAME_BYTES, presence.max_frame_bytes as u64, &LIMIT_PRESENCE_FRAME_BYTES_RANGE)? as usize;
        let presence_bytes_per_second = bounded(&read, LIMIT_PRESENCE_BYTES_PER_SECOND, presence.max_bytes_per_second.unwrap_or(*LIMIT_PRESENCE_BYTES_PER_SECOND_RANGE.end()), &LIMIT_PRESENCE_BYTES_PER_SECOND_RANGE)?;
        let limits = limits(&read)?;
        let caps = quiz::Limits { learners: bounded(&read, MAX_LEARNERS, quiz::DEFAULT_LIMITS.learners, &LIMIT_RANGE)?, runs: bounded(&read, MAX_RUNS, quiz::DEFAULT_LIMITS.runs, &LIMIT_RANGE)?, ..quiz::DEFAULT_LIMITS };
        let config = Self { bind, port, data, catalog, mode, origins, forwarding, presence_tick, presence_frame_bytes, presence_bytes_per_second, limits, caps };
        config.validate()?;
        Ok(config)
    }

    /// 🗄️ `PROCTOR_DATA` alone: all that `backup` and `restore` need of the environment.
    pub fn data_directory(read: impl Fn(&str) -> Option<String>) -> Result<PathBuf, ConfigError> {
        setting(&read, DATA).map(PathBuf::from).ok_or_else(|| ConfigError(format!("{DATA} must name the directory of proctor.sqlite")))
    }

    /// 🩺️ Where a probe on the same machine reaches the listener of this environment: the bind address
    /// and port, or the loopback address of the same family when every interface is bound.
    pub fn probe_address(read: impl Fn(&str) -> Option<String>) -> Result<SocketAddr, ConfigError> {
        let address = match bind(&read)? {
            IpAddr::V4(address) if address.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(address) if address.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
            address => address,
        };
        Ok(SocketAddr::new(address, port(&read)?))
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
        Gate { origins: self.origins.clone(), forwarding: self.forwarding, limits: self.limits.clone() }
    }

    /// 🧧️ The sign-up allowance of one client address in force.
    pub fn sign_ups(&self) -> Rate {
        self.limits.allowances.iter().find(|allowance| allowance.name == SIGN_UP).map_or(SIGN_UPS, |allowance| allowance.rate)
    }

    /// 👥️ The presence socket's settings: the protocol's defaults at this configuration's tick,
    /// frame size and total flow.
    pub fn presence(&self) -> PresenceSettings {
        PresenceSettings { tick: self.presence_tick, max_frame_bytes: self.presence_frame_bytes, max_bytes_per_second: Some(self.presence_bytes_per_second), ..PresenceSettings::default() }
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

/// 🔤️ One variable, trimmed; unset and blank are the same.
fn setting(read: &impl Fn(&str) -> Option<String>, name: &str) -> Option<String> {
    read(name).map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
}

fn bind(read: &impl Fn(&str) -> Option<String>) -> Result<IpAddr, ConfigError> {
    match setting(read, BIND) {
        Some(text) => text.parse().map_err(|_| ConfigError(format!("{BIND} must be an IP address, got {text:?}"))),
        None => Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)),
    }
}

fn port(read: &impl Fn(&str) -> Option<String>) -> Result<u16, ConfigError> {
    match setting(read, PORT) {
        Some(text) => text.parse().map_err(|_| ConfigError(format!("{PORT} must be a port number, got {text:?}"))),
        None => Ok(DEFAULT_PORT),
    }
}

/// 🚦️ The edge limits: the production sizing ([`Limits::default`]) and the sign-up allowance
/// ([`SIGN_UPS`]), with every stated `PROCTOR_LIMIT_*` in its place.
fn limits(read: &impl Fn(&str) -> Option<String>) -> Result<Limits, ConfigError> {
    let defaults = Limits::default();
    let count = |name: &str, default: usize| bounded(read, name, default as u64, &LIMIT_RANGE).map(|value| value as usize);
    let rate = |tokens: &str, burst: &str, default: Rate| Ok::<Rate, ConfigError>(Rate { tokens: bounded(read, tokens, u64::from(default.tokens), &LIMIT_RANGE)? as u32, burst: bounded(read, burst, u64::from(default.burst), &LIMIT_RANGE)? as u32, ..default });
    Ok(Limits {
        writes: rate(LIMIT_COMMANDS_PER_SECOND, LIMIT_COMMANDS_BURST, defaults.writes)?,
        reads: rate(LIMIT_QUERIES_PER_SECOND, LIMIT_QUERIES_BURST, defaults.reads)?,
        upgrades: rate(LIMIT_UPGRADES_PER_SECOND, LIMIT_UPGRADES_BURST, defaults.upgrades)?,
        allowances: vec![Allowance { name: SIGN_UP, rate: rate(LIMIT_SIGNUPS_PER_HOUR, LIMIT_SIGNUPS_BURST, SIGN_UPS)? }],
        sockets_per_client: count(LIMIT_SOCKETS_PER_ADDRESS, defaults.sockets_per_client)?,
        sockets: count(LIMIT_SOCKETS, defaults.sockets)?,
        in_flight: count(LIMIT_IN_FLIGHT, defaults.in_flight)?,
        body_bytes: Some(bounded(read, LIMIT_BODY_BYTES, defaults.body_bytes.unwrap_or(16 * 1024) as u64, &LIMIT_BODY_BYTES_RANGE)? as usize),
        ..defaults
    })
}

/// 🔢️ One whole number inside `range`, or `default` when the variable is unset.
fn bounded(read: &impl Fn(&str) -> Option<String>, name: &str, default: u64, range: &std::ops::RangeInclusive<u64>) -> Result<u64, ConfigError> {
    match setting(read, name) {
        Some(text) => text.parse().ok().filter(|value| range.contains(value)).ok_or_else(|| ConfigError(format!("{name} must be a whole number in {}..={}, got {text:?}", range.start(), range.end()))),
        None => Ok(default),
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
