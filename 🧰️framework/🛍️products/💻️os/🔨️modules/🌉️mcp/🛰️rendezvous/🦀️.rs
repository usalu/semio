//! 🛰️ The local bridge rendezvous — how a `semio-os-mcp` process launched over **stdio** (the one
//! shape every client `.mcp.json`/`mcp.json`/`config.toml` in this repo actually launches) and a
//! **live os session** find each other, without either one being told the other's address out of
//! band.
//!
//! Topology is unchanged from `run_http`: the gateway is the `/bridge` WebSocket SERVER and a shell
//! DIALS it (`🧵️bridge`, `🚚️transport`). What stdio mode lacked was (a) a listener at all and (b)
//! anywhere to publish its address. Both live here:
//!
//! - `~/.semio/agent/bridge/sessions/<pid>.json` — one record per **live os session**, written by the
//!   session itself. Its presence is the only "a `dev s` is running" signal this crate trusts; a
//!   record whose pid is gone is stale and is swept on the next read, never believed.
//! - `~/.semio/agent/bridge/offers/<pid>.json` — one record per **live gateway**, written here,
//!   carrying the loopback `ws://…/bridge` url and the single-process admission proof a dialer must
//!   present as the second websocket subprotocol. Mode `0600` (owner-only) on unix, in the user's own
//!   home: the same trust boundary the agent audit lane (`📒️audit`) already stands on, and strictly
//!   stronger than an argv/environment carrier, which `🏗️bootstrap`'s process-entry seal forbids
//!   outright. The offer is removed when the gateway exits.
//!
//! The admission proof is minted per process and never derived from the hub credential: a stdio
//! gateway has no fd-3 credential to delegate, and inventing a path that would copy one into a file
//! is exactly what the credential seal exists to prevent.

use crate::errors::{GatewayError, GatewayErrorCode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

//#region 🔖️Paths
/// 🏷️ The carrier that points one gateway and one os session at a rendezvous of their own instead of
/// the per-user default. It is a **directory path, never a credential** (the admission proof stays in
/// the owner-only offer file), and it is spelled with the `S_` prefix the process-entry seal admits.
///
/// Why it exists: the default rendezvous is per-user, and `newestLiveAgentBridgeOffer` hands a shell
/// whichever gateway published last. With several gateways and several dev sessions alive at once —
/// an agent fleet, or one developer running two products — a shell dials a gateway that is not the
/// one its own agent launched. Pointing both halves of one pair at one directory is the only way to
/// make the attachment deterministic.
pub const RENDEZVOUS_DIR_ENV: &str = "S_AGENT_BRIDGE_DIR";

/// 🏠️ [`RENDEZVOUS_DIR_ENV`] when it is set, else `~/.semio/agent/bridge` — sibling of `📒️audit`'s
/// `~/.semio/agent/audit`, same cross-platform `HOME`/`USERPROFILE` lookup, no new dependency.
pub fn rendezvous_dir() -> PathBuf {
    match std::env::var(RENDEZVOUS_DIR_ENV) {
        Ok(value) if !value.is_empty() => PathBuf::from(value),
        _ => home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".semio").join("agent").join("bridge"),
    }
}

pub fn sessions_dir() -> PathBuf {
    rendezvous_dir().join("sessions")
}

pub fn offers_dir() -> PathBuf {
    rendezvous_dir().join("offers")
}

/// 📂️ The two leaves of an arbitrary rendezvous root — every function in this facet takes the root
/// explicitly in its `_in` form, so a test drives the REAL layout under its own temporary directory
/// without touching a process-global `HOME` other tests read concurrently.
pub fn sessions_dir_in(root: &Path) -> PathBuf {
    root.join("sessions")
}

pub fn offers_dir_in(root: &Path) -> PathBuf {
    root.join("offers")
}

fn home_dir() -> Option<PathBuf> {
    for variable in ["HOME", "USERPROFILE"] {
        if let Ok(value) = std::env::var(variable) {
            if !value.is_empty() {
                return Some(PathBuf::from(value));
            }
        }
    }
    None
}
//#endregion 🔖️Paths

//#region 🔖️Records
/// 🖥️ One live os session, as the session itself published it. `pid` is the authority: this crate
/// never believes a record whose process is gone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsSessionRecord {
    pub schema_version: u32,
    pub session_id: String,
    pub pid: u32,
    pub shell_kind: String,
    pub started_at_ms: u64,
}

/// 📨️ One live gateway's standing invitation to dial its `/bridge`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeOffer {
    pub schema_version: u32,
    pub url: String,
    pub admission_proof: String,
    pub principal: String,
    pub pid: u32,
    pub published_at_ms: u64,
}

pub const RENDEZVOUS_SCHEMA_VERSION: u32 = 1;
//#endregion 🔖️Records

//#region 🔖️Liveness
/// 🫀️ Whether `pid` still names a live process. `kill(pid, 0)` on unix; on Windows a process whose
/// pid is gone cannot be opened, and the conservative answer there is "alive" only when the record is
/// young — an unreachable process's stale offer is harmless (a dialer simply fails to connect),
/// while wrongly sweeping a LIVE session's record would blind discovery.
#[cfg(unix)]
pub fn process_is_alive(pid: u32) -> bool {
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    match i32::try_from(pid) {
        Ok(pid) => (unsafe { kill(pid, 0) }) == 0,
        Err(_) => false,
    }
}

/// 🫀️ See the unix twin — Windows keeps a record until it ages out rather than probing the process
/// table through a new dependency.
#[cfg(not(unix))]
pub fn process_is_alive(_pid: u32) -> bool {
    true
}

/// ⌛️ How long a session record is believed on a platform with no process probe (the Windows path
/// above). Where the probe is real, the pid is the ONLY authority: a session that has been up for
/// three days is still up, and ageing it out would blind discovery for exactly the long-running
/// sessions an agent most wants to attach to.
pub const SESSION_RECORD_MAX_AGE_MS: u64 = 12 * 60 * 60 * 1000;

/// 🫀️ Whether one record still describes a live session.
#[cfg(unix)]
fn record_is_live(record: &OsSessionRecord) -> bool {
    process_is_alive(record.pid)
}

/// 🫀️ See the unix twin — with no process probe, age is the only signal available.
#[cfg(not(unix))]
fn record_is_live(record: &OsSessionRecord) -> bool {
    now_ms().saturating_sub(record.started_at_ms) <= SESSION_RECORD_MAX_AGE_MS
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}
//#endregion 🔖️Liveness

//#region 🔖️Discovery
/// 🔎️ Every live os session, newest first, sweeping stale records as it goes. An empty result is the
/// honest "no `dev s` session is running on this machine for this user" — it is never an error, and
/// callers turn it into a typed, retryable diagnostic rather than a silent no-op.
pub fn live_os_sessions() -> Vec<OsSessionRecord> {
    live_os_sessions_in(&rendezvous_dir())
}

/// 🔎️ [`live_os_sessions`] against an explicit rendezvous root.
pub fn live_os_sessions_in(root: &Path) -> Vec<OsSessionRecord> {
    let mut records = Vec::new();
    let Ok(entries) = std::fs::read_dir(sessions_dir_in(root)) else { return records };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("json") {
            continue;
        }
        let Some(record) = std::fs::read(&path).ok().and_then(|bytes| serde_json::from_slice::<OsSessionRecord>(&bytes).ok()) else {
            let _ = std::fs::remove_file(&path);
            continue;
        };
        if !record_is_live(&record) {
            let _ = std::fs::remove_file(&path);
            continue;
        }
        records.push(record);
    }
    records.sort_by(|left, right| right.started_at_ms.cmp(&left.started_at_ms));
    records
}
//#endregion 🔖️Discovery

//#region 🔖️Offer
/// 🧹️ A published offer that removes its own file on drop — the gateway's exit, clean or not, must
/// not leave a dead `ws://` address behind for a shell to retry forever.
pub struct PublishedBridgeOffer {
    path: PathBuf,
    offer: BridgeOffer,
}

impl PublishedBridgeOffer {
    pub fn offer(&self) -> &BridgeOffer {
        &self.offer
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for PublishedBridgeOffer {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// 📤️ Writes `offer` into `~/.semio/agent/bridge/offers/<pid>.json`, owner-only, atomically
/// (write-temp-then-rename, so a dialer never reads a half-written record).
pub fn publish_offer(offer: BridgeOffer) -> Result<PublishedBridgeOffer, GatewayError> {
    publish_offer_in(&rendezvous_dir(), offer)
}

/// 📤️ [`publish_offer`] against an explicit rendezvous root.
pub fn publish_offer_in(root: &Path, offer: BridgeOffer) -> Result<PublishedBridgeOffer, GatewayError> {
    let directory = offers_dir_in(root);
    std::fs::create_dir_all(&directory).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot create bridge rendezvous directory `{}`: {error}", directory.display())))?;
    let path = directory.join(format!("{}.json", offer.pid));
    let temporary = directory.join(format!("{}.json.partial", offer.pid));
    let bytes = serde_json::to_vec_pretty(&offer).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
    write_owner_only(&temporary, &bytes)?;
    std::fs::rename(&temporary, &path).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot publish bridge offer `{}`: {error}", path.display())))?;
    Ok(PublishedBridgeOffer { path, offer })
}

#[cfg(unix)]
fn write_owner_only(path: &Path, bytes: &[u8]) -> Result<(), GatewayError> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot write `{}`: {error}", path.display())))?;
    file.write_all(bytes).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
    file.flush().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))
}

#[cfg(not(unix))]
fn write_owner_only(path: &Path, bytes: &[u8]) -> Result<(), GatewayError> {
    std::fs::write(path, bytes).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot write `{}`: {error}", path.display())))
}
//#endregion 🔖️Offer

//#region 🔖️AdmissionProof
/// 🔑️ A fresh per-process admission proof — the value a dialer must present as the SECOND websocket
/// subprotocol (`semio.mcp.bridge.v1`, `<proof>`). Entropy comes from `std`'s own OS-seeded
/// `RandomState` (the same source `HashMap`'s DoS resistance stands on), mixed with the process id
/// and a high-resolution instant, then run through the repo's own hash — no external dependency, and
/// cross-platform by construction.
pub fn mint_admission_proof() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut material = Vec::with_capacity(48);
    for _ in 0..3 {
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write_u64(now_ms());
        material.extend_from_slice(&hasher.finish().to_le_bytes());
    }
    material.extend_from_slice(&std::process::id().to_le_bytes());
    material.extend_from_slice(&std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos().to_le_bytes());
    framework_hash::hash_bytes(&material)[..32].to_owned()
}
//#endregion 🔖️AdmissionProof

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
