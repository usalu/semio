//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The repo coordinator in Rust: the owned append-only event store and its crash-recovery protocol,
// the event-sourced repository with replayed projections, and a hand-rolled HTTP/1.1 surface.
// Twin of `📦️packages/🐹️go/🐹️.go`; both are judged by the cases under `🧪️tests`.

//#endregion 🧲️Header

use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use semio_framework_repo_events::{digest, Sha256};
pub use serde_json;

//#region ⏱️Config

/// ⚙️ Every coordinator setting, read from the environment with the same defaults as the Go twin.
#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub database_path: String,
    pub repo_root: String,
    pub token: String,
    pub github_secret: String,
    pub discord_webhook: String,
    pub request_body_limit: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8787".to_string(),
            database_path: "compose-server.db".to_string(),
            repo_root: String::new(),
            token: String::new(),
            github_secret: String::new(),
            discord_webhook: String::new(),
            request_body_limit: 10 * 1024 * 1024,
        }
    }
}

impl Config {
    /// 🌱️ Reads `COMPOSE_SERVER_*` with the documented fallbacks.
    pub fn from_env() -> Self {
        let cwd = std::env::current_dir().map(|path| path.display().to_string()).unwrap_or_default();
        Self {
            address: env_or("COMPOSE_SERVER_ADDR", "127.0.0.1:8787"),
            database_path: env_or("COMPOSE_SERVER_DB", "compose-server.db"),
            repo_root: env_or("COMPOSE_SERVER_REPO_ROOT", &cwd),
            token: env_or("COMPOSE_SERVER_TOKEN", ""),
            github_secret: env_or("COMPOSE_SERVER_GITHUB_SECRET", ""),
            discord_webhook: env_or("COMPOSE_SERVER_DISCORD_WEBHOOK", ""),
            request_body_limit: env_or("COMPOSE_SERVER_BODY_LIMIT", "")
                .parse()
                .unwrap_or(10 * 1024 * 1024),
        }
    }
}

fn env_or(key: &str, fallback: &str) -> String {
    match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => fallback.to_string(),
    }
}

//#endregion ⏱️Config

//#region 🔢️Encoding

/// 📜️ The persisted schema name every stage file carries.
pub const STAGE_SCHEMA: &str = "semio.coordinator.event-stage/1";

/// 📜️ The persisted schema name of one event envelope.
pub const EVENT_SCHEMA: &str = "semio.coordinator.event/1";

/// 🧾️ One committed record of the append-only log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEnvelope {
    pub stream: String,
    pub sequence: u64,
    pub id: String,
    pub generation: u64,
    pub kind: String,
    pub payload: Value,
    pub checksum: String,
}

/// 📨️ A sequence-free event proposed for an append command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventInput {
    pub stream: String,
    pub id: String,
    pub generation: u64,
    pub kind: String,
    pub payload: Value,
}

/// 📏️ The bounds that keep one append or replay finite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreLimits {
    pub max_payload_bytes: usize,
    pub max_append_bytes: usize,
    pub max_append_events: usize,
    pub max_replay_events: usize,
    pub max_json_depth: usize,
    pub max_log_bytes: u64,
}

impl Default for StoreLimits {
    fn default() -> Self {
        Self {
            max_payload_bytes: 1 << 20,
            max_append_bytes: 8 << 20,
            max_append_events: 4096,
            max_replay_events: 1_000_000,
            max_json_depth: 64,
            max_log_bytes: 1 << 30,
        }
    }
}

/// ✍️ Encodes one JSON value exactly the way Go's `encoding/json` does.
///
/// The log is compared byte for byte across implementations, so the escaping rules must match Go's
/// and not `serde_json`'s: Go escapes `<`, `>`, `&` and U+2028/U+2029, and spells `\b`/`\f` as
/// ``/``. Object keys are emitted in sorted order, which is what Go's map marshalling
/// produces and what `canonical_payload` has already normalised every payload to.
pub fn encode_go_json(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => out.push_str(&number.to_string()),
        Value::String(text) => encode_go_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                encode_go_json(item, out);
            }
            out.push(']');
        }
        Value::Object(entries) => {
            out.push('{');
            for (index, (key, item)) in entries.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                encode_go_string(key, out);
                out.push(':');
                encode_go_json(item, out);
            }
            out.push('}');
        }
    }
}

fn encode_go_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other if (other as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
}

/// 🧮️ The Go-compatible text of one JSON value.
pub fn to_go_json(value: &Value) -> String {
    let mut out = String::new();
    encode_go_json(value, &mut out);
    out
}

/// ♻️ Normalises a payload the way the store persists it: decoded, then re-encoded with sorted keys.
pub fn canonical_payload(payload: &Value, max_depth: usize) -> Result<Value, StoreError> {
    validate_payload(payload, max_depth)?;
    Ok(payload.clone())
}

/// ✅️ Refuses an empty payload and one nested past the configured depth.
pub fn validate_payload(payload: &Value, max_depth: usize) -> Result<(), StoreError> {
    fn depth_of(value: &Value, depth: usize, max_depth: usize) -> Result<(), StoreError> {
        if depth > max_depth {
            return Err(StoreError::Limit(format!("JSON depth {depth} > {max_depth}")));
        }
        match value {
            Value::Array(items) => items.iter().try_for_each(|item| depth_of(item, depth + 1, max_depth)),
            Value::Object(entries) => entries.values().try_for_each(|item| depth_of(item, depth + 1, max_depth)),
            _ => Ok(()),
        }
    }
    match payload {
        Value::Array(_) | Value::Object(_) => depth_of(payload, 1, max_depth),
        Value::Null => Err(StoreError::Corrupt("event payload must be valid JSON".to_string())),
        _ => Ok(()),
    }
}

/// 🔏️ The SHA-256 identity of a record over its NUL-separated header and canonical payload.
///
/// `sha256(stream+nul+sequence+nul+id+nul+generation+nul+type+nul+canonical-payload)`, exactly the
/// formula frozen in `🧫️fixtures/🧬️g3-event-schema.json`.
pub fn event_checksum(event: &EventEnvelope) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        format!(
            "{}\u{0}{}\u{0}{}\u{0}{}\u{0}{}\u{0}",
            event.stream, event.sequence, event.id, event.generation, event.kind
        )
        .as_bytes(),
    );
    hasher.update(to_go_json(&event.payload).as_bytes());
    hasher.finish()
}

/// 🧾️ The canonical single line of one envelope, terminated by the record separator.
pub fn encode_event(event: &EventEnvelope) -> String {
    let mut out = String::from("{\"stream\":");
    encode_go_string(&event.stream, &mut out);
    out.push_str(&format!(",\"sequence\":{},\"id\":", event.sequence));
    encode_go_string(&event.id, &mut out);
    out.push_str(&format!(",\"generation\":{},\"type\":", event.generation));
    encode_go_string(&event.kind, &mut out);
    out.push_str(",\"payload\":");
    encode_go_json(&event.payload, &mut out);
    out.push_str(",\"checksum\":");
    encode_go_string(&event.checksum, &mut out);
    out.push_str("}\n");
    out
}

/// 🔎️ Reads one canonical line back into an envelope, refusing anything the encoder would not emit.
pub fn decode_event(line: &str) -> Result<EventEnvelope, StoreError> {
    let value: Value = serde_json::from_str(line).map_err(|_| StoreError::Corrupt("malformed event".to_string()))?;
    let object = value.as_object().ok_or_else(|| StoreError::Corrupt("malformed event".to_string()))?;
    let text = |key: &str| object.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let number = |key: &str| object.get(key).and_then(Value::as_u64).unwrap_or_default();
    Ok(EventEnvelope {
        stream: text("stream"),
        sequence: number("sequence"),
        id: text("id"),
        generation: number("generation"),
        kind: text("type"),
        payload: object.get("payload").cloned().unwrap_or(Value::Null),
        checksum: text("checksum"),
    })
}

fn valid_scalar(value: &str) -> bool {
    !value.is_empty() && !value.contains('\u{0}')
}

//#endregion 🔢️Encoding

//#region 🛡️Durability

/// 🚫️ The failure a store armed for fault injection raises.
pub const INJECTED_FAULT: &str = "injected durability fault";

/// 💥️ Fails the armed durable mutation and passes every other call through.
#[derive(Debug, Default)]
struct Fault {
    armed: usize,
    seen: usize,
}

impl Fault {
    fn mutate(&mut self, path: &Path) -> Result<(), StoreError> {
        if self.armed == 0 || path.extension().is_some_and(|value| value == "lock") {
            return Ok(());
        }
        self.seen += 1;
        if self.seen == self.armed {
            return Err(StoreError::Unavailable(format!("{INJECTED_FAULT} at mutation {} on {}", self.seen, path.display())));
        }
        Ok(())
    }
}

/// 💾️ Flushes the directory entry itself, so a rename or a removal survives a power loss.
#[cfg(unix)]
pub fn sync_parent(path: &Path) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    std::fs::File::open(parent)?.sync_all()
}

/// 💾️ Flushes the directory entry itself.
///
/// A directory handle on Windows only exists with `FILE_FLAG_BACKUP_SEMANTICS`, and
/// `FlushFileBuffers` additionally refuses a handle that was not opened for writing — the same two
/// conditions the Go twin's `🪟️.go` states.
#[cfg(windows)]
pub fn sync_parent(path: &Path) -> std::io::Result<()> {
    use std::os::windows::fs::OpenOptionsExt;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    let parent = path.parent().unwrap_or(Path::new("."));
    OpenOptions::new().write(true).custom_flags(FILE_FLAG_BACKUP_SEMANTICS).open(parent)?.sync_all()
}

fn remove_durably(path: &Path) -> Result<(), StoreError> {
    match std::fs::remove_file(path) {
        Ok(()) => sync_parent(path).map_err(|failure| StoreError::Unavailable(failure.to_string())),
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
    }
}

fn rename_durably(source: &Path, destination: &Path) -> Result<(), StoreError> {
    std::fs::rename(source, destination).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
    sync_parent(destination).map_err(|failure| StoreError::Unavailable(failure.to_string()))
}

fn write_synced_exclusive(path: &Path, chunks: &[&[u8]]) -> Result<(), StoreError> {
    remove_durably(path)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
    for chunk in chunks {
        if let Err(failure) = file.write_all(chunk) {
            drop(file);
            let _ = std::fs::remove_file(path);
            return Err(StoreError::Unavailable(failure.to_string()));
        }
    }
    if let Err(failure) = file.sync_all() {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(StoreError::Unavailable(failure.to_string()));
    }
    drop(file);
    sync_parent(path).map_err(|failure| StoreError::Unavailable(failure.to_string()))
}

//#endregion 🛡️Durability

//#region 🗄️EventStore

/// 🚨️ Every way an append, a replay or a recovery refuses to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Unavailable(String),
    Corrupt(String),
    Duplicate(String),
    SequenceConflict(String),
    Limit(String),
    NotFound(String),
    PendingCleanup(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(detail) => write!(formatter, "event store unavailable: {detail}"),
            Self::Corrupt(detail) => write!(formatter, "event store corrupt: {detail}"),
            Self::Duplicate(detail) => write!(formatter, "duplicate event: {detail}"),
            Self::SequenceConflict(detail) => write!(formatter, "expected sequence conflict: {detail}"),
            Self::Limit(detail) => write!(formatter, "event store limit exceeded: {detail}"),
            Self::NotFound(detail) => write!(formatter, "projection not found: {detail}"),
            Self::PendingCleanup(detail) => write!(formatter, "append committed with pending cleanup: {detail}"),
        }
    }
}

impl std::error::Error for StoreError {}

/// 📦️ What one append committed, or observed as already committed.
#[derive(Debug, Clone, Default)]
pub struct AppendResult {
    pub events: Vec<EventEnvelope>,
    pub duplicate: bool,
    pub committed: bool,
    pub pending_cleanup: bool,
}

/// 🛟️ The last completed artifact-recovery action.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecoveryStatus {
    pub recovered: bool,
    pub action: String,
}

#[derive(Debug, Clone, Default)]
struct Stage {
    prior_exists: bool,
    prior_size: u64,
    prior_checksum: String,
    next_size: u64,
    next_checksum: String,
}

impl Stage {
    fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("schema".to_string(), Value::String(STAGE_SCHEMA.to_string()));
        object.insert("prior_exists".to_string(), Value::Bool(self.prior_exists));
        object.insert("prior_size".to_string(), Value::from(self.prior_size));
        object.insert("prior_checksum".to_string(), Value::String(self.prior_checksum.clone()));
        object.insert("next_size".to_string(), Value::from(self.next_size));
        object.insert("next_checksum".to_string(), Value::String(self.next_checksum.clone()));
        Value::Object(object)
    }

    fn from_value(value: &Value) -> Option<Self> {
        let object = value.as_object()?;
        if object.get("schema").and_then(Value::as_str)? != STAGE_SCHEMA {
            return None;
        }
        let stage = Self {
            prior_exists: object.get("prior_exists").and_then(Value::as_bool)?,
            prior_size: object.get("prior_size").and_then(Value::as_u64)?,
            prior_checksum: object.get("prior_checksum").and_then(Value::as_str)?.to_string(),
            next_size: object.get("next_size").and_then(Value::as_u64)?,
            next_checksum: object.get("next_checksum").and_then(Value::as_str)?.to_string(),
        };
        if stage.next_size <= stage.prior_size || stage.prior_checksum.is_empty() || stage.next_checksum.is_empty() {
            return None;
        }
        Some(stage)
    }
}

#[derive(Debug, Default, Clone)]
struct PersistedFile {
    exists: bool,
    size: u64,
    checksum: String,
}

/// 🔐️ The in-process half of the store lock: one holder per absolute log path.
///
/// The Go twin keeps a `sync.Map` of semaphores next to the on-disk `.lock` file for exactly this
/// reason — two goroutines of one process would otherwise both find the lock file absent.
fn process_locks() -> &'static (Mutex<std::collections::BTreeSet<PathBuf>>, Condvar) {
    static LOCKS: OnceLock<(Mutex<std::collections::BTreeSet<PathBuf>>, Condvar)> = OnceLock::new();
    LOCKS.get_or_init(|| (Mutex::new(std::collections::BTreeSet::new()), Condvar::new()))
}

/// 🗄️ One append-only log and the crash-recovery artifacts that belong to it.
#[derive(Debug)]
pub struct EventStore {
    path: PathBuf,
    limits: StoreLimits,
    fault: Mutex<Fault>,
    recovery: Mutex<RecoveryStatus>,
}

impl EventStore {
    /// 🆕️ Opens, recovers and validates an event log.
    pub fn open(path: impl AsRef<Path>, limits: StoreLimits) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        if path.as_os_str().is_empty() {
            return Err(StoreError::Unavailable("path is required".to_string()));
        }
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            }
        }
        let store = Self { path, limits, fault: Mutex::new(Fault::default()), recovery: Mutex::new(RecoveryStatus::default()) };
        let guard = store.lock()?;
        store.recover_locked()?;
        store.replay_locked(true)?;
        drop(guard);
        Ok(store)
    }

    /// 💥️ Makes the store's next armed durable mutation fail; `0` disarms.
    pub fn arm_fault(&self, at: usize) {
        let mut fault = self.fault.lock().expect("fault");
        fault.armed = at;
        fault.seen = 0;
    }

    /// 🛟️ The last completed recovery action.
    pub fn recovery_status(&self) -> RecoveryStatus {
        self.recovery.lock().expect("recovery").clone()
    }

    /// 📍️ The log's own path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn stage_path(&self) -> PathBuf {
        self.suffixed("stage")
    }

    fn next_path(&self) -> PathBuf {
        self.suffixed("next")
    }

    fn backup_path(&self) -> PathBuf {
        self.suffixed("backup")
    }

    fn suffixed(&self, suffix: &str) -> PathBuf {
        PathBuf::from(format!("{}.{suffix}", self.path.display()))
    }

    fn mutate(&self, path: &Path) -> Result<(), StoreError> {
        self.fault.lock().expect("fault").mutate(path)
    }

    fn remove(&self, path: &Path) -> Result<(), StoreError> {
        match std::fs::symlink_metadata(path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(failure) => return Err(StoreError::Unavailable(failure.to_string())),
            Ok(_) => {}
        }
        self.mutate(path)?;
        remove_durably(path)
    }

    fn rename(&self, source: &Path, destination: &Path) -> Result<(), StoreError> {
        self.mutate(destination)?;
        rename_durably(source, destination)
    }

    fn write(&self, path: &Path, chunks: &[&[u8]]) -> Result<(), StoreError> {
        self.remove(path)?;
        self.mutate(path)?;
        write_synced_exclusive(path, chunks)
    }

    fn lock(&self) -> Result<StoreGuard, StoreError> {
        let absolute = std::fs::canonicalize(self.path.parent().unwrap_or(Path::new(".")))
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(self.path.file_name().unwrap_or_default());
        let (holders, condition) = process_locks();
        let mut held = holders.lock().expect("locks");
        while !held.insert(absolute.clone()) {
            held = condition.wait(held).expect("locks");
        }
        drop(held);
        let lock_path = self.suffixed("lock");
        loop {
            match OpenOptions::new().create_new(true).write(true).open(&lock_path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "{}", std::process::id());
                    let _ = file.sync_all();
                    return Ok(StoreGuard { holder: absolute, lock_path });
                }
                Err(failure) if failure.kind() == std::io::ErrorKind::AlreadyExists => {
                    if let Ok(metadata) = std::fs::metadata(&lock_path) {
                        let stale = metadata
                            .modified()
                            .ok()
                            .and_then(|when| SystemTime::now().duration_since(when).ok())
                            .is_some_and(|age| age > Duration::from_secs(30));
                        if stale {
                            let _ = std::fs::remove_file(&lock_path);
                            continue;
                        }
                    }
                    std::thread::sleep(Duration::from_millis(2));
                }
                Err(failure) => {
                    let (holders, condition) = process_locks();
                    holders.lock().expect("locks").remove(&absolute);
                    condition.notify_all();
                    return Err(StoreError::Unavailable(format!("acquire lock: {failure}")));
                }
            }
        }
    }

    /// ➕️ Stages and atomically commits events at the expected stream sequence.
    pub fn append(&self, expected_sequence: u64, inputs: &[EventInput]) -> Result<AppendResult, StoreError> {
        if inputs.is_empty() {
            return Ok(AppendResult::default());
        }
        if inputs.len() > self.limits.max_append_events {
            return Err(StoreError::Limit(format!("events {} > {}", inputs.len(), self.limits.max_append_events)));
        }
        let guard = self.lock()?;
        let outcome = self.append_locked(expected_sequence, inputs);
        drop(guard);
        outcome
    }

    fn append_locked(&self, expected_sequence: u64, inputs: &[EventInput]) -> Result<AppendResult, StoreError> {
        self.recover_locked()?;
        let existing = self.replay_locked(true)?;
        let (created, duplicate) = self.prepare(expected_sequence, &existing, inputs)?;
        if duplicate {
            return Ok(AppendResult { events: created, duplicate: true, committed: true, pending_cleanup: false });
        }
        let encoded: String = created.iter().map(encode_event).collect();
        let prior = self.read_optional()?;
        let prior_bytes = prior.clone().unwrap_or_default();
        if (prior_bytes.len() + encoded.len()) as u64 > self.limits.max_log_bytes {
            return Err(StoreError::Limit(format!("log bytes exceed {}", self.limits.max_log_bytes)));
        }
        let mut combined = prior_bytes.clone();
        combined.extend_from_slice(encoded.as_bytes());
        let stage = Stage {
            prior_exists: prior.is_some(),
            prior_size: prior_bytes.len() as u64,
            prior_checksum: digest(&prior_bytes),
            next_size: combined.len() as u64,
            next_checksum: digest(&combined),
        };
        self.commit(&stage, &prior_bytes, encoded.as_bytes())?;
        Ok(AppendResult { events: created, duplicate: false, committed: true, pending_cleanup: false })
    }

    fn prepare(&self, expected_sequence: u64, existing: &[EventEnvelope], inputs: &[EventInput]) -> Result<(Vec<EventEnvelope>, bool), StoreError> {
        let mut normalized = Vec::with_capacity(inputs.len());
        for input in inputs {
            if !valid_scalar(&input.stream) || !valid_scalar(&input.id) || !valid_scalar(&input.kind) || input.generation == 0 {
                return Err(StoreError::Corrupt("event stream, id, generation, and type are required".to_string()));
            }
            let encoded = to_go_json(&input.payload);
            if encoded.len() > self.limits.max_payload_bytes {
                return Err(StoreError::Limit(format!("payload bytes {} > {}", encoded.len(), self.limits.max_payload_bytes)));
            }
            let mut input = input.clone();
            input.payload = canonical_payload(&input.payload, self.limits.max_json_depth)?;
            normalized.push(input);
        }
        let mut by_id: HashMap<&str, &EventEnvelope> = HashMap::new();
        let mut sequences: HashMap<&str, u64> = HashMap::new();
        for event in existing {
            by_id.insert(event.id.as_str(), event);
            sequences.insert(event.stream.as_str(), event.sequence);
        }
        let mut duplicates = Vec::new();
        let mut all_duplicate = true;
        for input in &normalized {
            match by_id.get(input.id.as_str()) {
                None => all_duplicate = false,
                Some(event) => {
                    if event.stream != input.stream || event.generation != input.generation || event.kind != input.kind || event.payload != input.payload {
                        return Err(StoreError::Duplicate(format!("id {:?} has different content", input.id)));
                    }
                    duplicates.push((*event).clone());
                }
            }
        }
        if all_duplicate {
            return Ok((duplicates, true));
        }
        if !duplicates.is_empty() {
            return Err(StoreError::Duplicate("mixed duplicate and new batch".to_string()));
        }
        let stream = normalized[0].stream.clone();
        let actual = sequences.get(stream.as_str()).copied().unwrap_or(0);
        if actual != expected_sequence {
            return Err(StoreError::SequenceConflict(format!("stream {stream:?} expected {expected_sequence} actual {actual}")));
        }
        let mut created = Vec::with_capacity(normalized.len());
        let mut seen: Vec<String> = Vec::new();
        let mut encoded_bytes = 0usize;
        for (index, input) in normalized.iter().enumerate() {
            if input.stream != stream {
                return Err(StoreError::Corrupt("one append batch must target one stream".to_string()));
            }
            if seen.contains(&input.id) {
                return Err(StoreError::Duplicate(input.id.clone()));
            }
            let mut event = EventEnvelope {
                stream: stream.clone(),
                sequence: expected_sequence + index as u64 + 1,
                id: input.id.clone(),
                generation: input.generation,
                kind: input.kind.clone(),
                payload: input.payload.clone(),
                checksum: String::new(),
            };
            event.checksum = event_checksum(&event);
            encoded_bytes += encode_event(&event).len();
            if encoded_bytes > self.limits.max_append_bytes {
                return Err(StoreError::Limit(format!("append bytes {encoded_bytes} > {}", self.limits.max_append_bytes)));
            }
            seen.push(event.id.clone());
            created.push(event);
        }
        Ok((created, false))
    }

    fn commit(&self, stage: &Stage, prior: &[u8], appended: &[u8]) -> Result<(), StoreError> {
        let next = self.next_path();
        if let Err(failure) = self.write(&next, &[prior, appended]) {
            return Err(self.rollback_after(failure));
        }
        if let Err(failure) = self.write_stage(stage) {
            return Err(self.rollback_after(failure));
        }
        if stage.prior_exists {
            if let Err(failure) = self.remove(&self.backup_path()) {
                return Err(self.rollback_after(failure));
            }
            if let Err(failure) = self.rename(&self.path, &self.backup_path()) {
                return Err(self.rollback_after(failure));
            }
        }
        if let Err(failure) = self.rename(&next, &self.path) {
            return Err(self.rollback_after(failure));
        }
        self.cleanup().map_err(|failure| StoreError::PendingCleanup(failure.to_string()))
    }

    fn rollback_after(&self, failure: StoreError) -> StoreError {
        let _ = self.rollback();
        failure
    }

    fn write_stage(&self, stage: &Stage) -> Result<(), StoreError> {
        let staged = format!("{}\n", to_go_json(&stage.to_value()));
        let next = self.suffixed("stage.next");
        self.write(&next, &[staged.as_bytes()])?;
        self.rename(&next, &self.stage_path())
    }

    fn rollback(&self) -> Result<(), StoreError> {
        let stage = match std::fs::read_to_string(self.stage_path()) {
            Err(_) => return remove_durably(&self.next_path()),
            Ok(text) => match serde_json::from_str::<Value>(&text).ok().as_ref().and_then(Stage::from_value) {
                None => return remove_durably(&self.next_path()),
                Some(stage) => stage,
            },
        };
        let backup = self.inspect(&self.backup_path())?;
        if matches(&backup, stage.prior_size, &stage.prior_checksum) {
            remove_durably(&self.path)?;
            rename_durably(&self.backup_path(), &self.path)?;
        } else if !stage.prior_exists {
            remove_durably(&self.path)?;
        }
        self.cleanup_unfaulted()
    }

    fn cleanup(&self) -> Result<(), StoreError> {
        for path in [self.next_path(), self.backup_path(), self.stage_path(), self.suffixed("stage.next")] {
            self.remove(&path)?;
        }
        Ok(())
    }

    fn cleanup_unfaulted(&self) -> Result<(), StoreError> {
        for path in [self.next_path(), self.backup_path(), self.stage_path(), self.suffixed("stage.next")] {
            remove_durably(&path)?;
        }
        Ok(())
    }

    fn recover_locked(&self) -> Result<(), StoreError> {
        self.set_recovery(RecoveryStatus::default());
        remove_durably(&self.suffixed("stage.next"))?;
        let staged = match std::fs::read_to_string(self.stage_path()) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
                remove_durably(&self.next_path())?;
                let backup = self.inspect(&self.backup_path())?;
                if backup.exists {
                    return Err(StoreError::Corrupt("orphan backup".to_string()));
                }
                return Ok(());
            }
            Err(failure) => return Err(StoreError::Unavailable(format!("read stage: {failure}"))),
            Ok(text) => text,
        };
        let stage = serde_json::from_str::<Value>(&staged)
            .ok()
            .as_ref()
            .and_then(Stage::from_value)
            .ok_or_else(|| StoreError::Corrupt("invalid append stage".to_string()))?;
        let current = self.inspect(&self.path)?;
        let backup = self.inspect(&self.backup_path())?;
        if matches(&current, stage.next_size, &stage.next_checksum) {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "committed-cleanup".to_string() });
            return Ok(());
        }
        if matches(&backup, stage.prior_size, &stage.prior_checksum) {
            remove_durably(&self.path)?;
            rename_durably(&self.backup_path(), &self.path)?;
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "prior-restored".to_string() });
            return Ok(());
        }
        if stage.prior_exists && matches(&current, stage.prior_size, &stage.prior_checksum) {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "prior-cleanup".to_string() });
            return Ok(());
        }
        if !stage.prior_exists && !current.exists {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "empty-cleanup".to_string() });
            return Ok(());
        }
        Err(StoreError::Corrupt("neither committed nor prior log is valid".to_string()))
    }

    fn set_recovery(&self, status: RecoveryStatus) {
        *self.recovery.lock().expect("recovery") = status;
    }

    fn inspect(&self, path: &Path) -> Result<PersistedFile, StoreError> {
        match std::fs::read(path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(PersistedFile { exists: false, size: 0, checksum: digest(&[]) }),
            Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
            Ok(data) => Ok(PersistedFile { exists: true, size: data.len() as u64, checksum: digest(&data) }),
        }
    }

    fn read_optional(&self) -> Result<Option<Vec<u8>>, StoreError> {
        match std::fs::read(&self.path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
            Ok(data) => {
                if data.len() as u64 > self.limits.max_log_bytes {
                    return Err(StoreError::Limit(format!("log bytes {} > {}", data.len(), self.limits.max_log_bytes)));
                }
                Ok(Some(data))
            }
        }
    }

    /// ⏪️ Recovers and returns a deterministic copy of every valid event.
    pub fn replay(&self) -> Result<Vec<EventEnvelope>, StoreError> {
        let guard = self.lock()?;
        let outcome = self.recover_locked().and_then(|()| self.replay_locked(true));
        drop(guard);
        outcome
    }

    fn replay_locked(&self, recover_tail: bool) -> Result<Vec<EventEnvelope>, StoreError> {
        let data = match self.read_optional()? {
            None => return Ok(Vec::new()),
            Some(data) if data.is_empty() => return Ok(Vec::new()),
            Some(data) => data,
        };
        let mut data = data;
        if data.last() != Some(&b'\n') {
            if !recover_tail {
                return Err(StoreError::Corrupt("partial tail".to_string()));
            }
            let keep = data.iter().rposition(|byte| *byte == b'\n').map_or(0, |index| index + 1);
            let file = OpenOptions::new().write(true).open(&self.path).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            file.set_len(keep as u64).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            file.sync_all().map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            data.truncate(keep);
        }
        let text = String::from_utf8(data).map_err(|_| StoreError::Corrupt("log is not UTF-8".to_string()))?;
        let mut events: Vec<EventEnvelope> = Vec::new();
        let mut sequences: HashMap<String, u64> = HashMap::new();
        for (index, line) in text.lines().enumerate() {
            if events.len() >= self.limits.max_replay_events {
                return Err(StoreError::Limit(format!("replay events exceed {}", self.limits.max_replay_events)));
            }
            if line.len() > self.limits.max_append_bytes {
                return Err(StoreError::Limit(format!("encoded event bytes {} > {}", line.len(), self.limits.max_append_bytes)));
            }
            let event = decode_event(line).map_err(|_| StoreError::Corrupt(format!("malformed event {}", index + 1)))?;
            if encode_event(&event) != format!("{line}\n") {
                return Err(StoreError::Corrupt(format!("non-canonical event {}", index + 1)));
            }
            let expected = sequences.get(&event.stream).copied().unwrap_or(0) + 1;
            if !valid_scalar(&event.stream) || !valid_scalar(&event.id) || event.generation == 0 || !valid_scalar(&event.kind) || event.sequence != expected || event.checksum != event_checksum(&event) {
                return Err(StoreError::Corrupt(format!("invalid event {}", index + 1)));
            }
            validate_payload(&event.payload, self.limits.max_json_depth).map_err(|_| StoreError::Corrupt(format!("invalid payload {}", index + 1)))?;
            if events.iter().any(|seen| seen.id == event.id) {
                return Err(StoreError::Duplicate(event.id));
            }
            sequences.insert(event.stream.clone(), event.sequence);
            events.push(event);
        }
        Ok(events)
    }
}

fn matches(file: &PersistedFile, size: u64, checksum: &str) -> bool {
    file.exists && file.size == size && file.checksum == checksum
}

/// 🔓️ Releases the on-disk lock file and the in-process holder together.
struct StoreGuard {
    holder: PathBuf,
    lock_path: PathBuf,
}

impl Drop for StoreGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_path);
        let (holders, condition) = process_locks();
        holders.lock().expect("locks").remove(&self.holder);
        condition.notify_all();
    }
}

//#endregion 🗄️EventStore

//#region 🧠️Projection

/// 🎫️ A tracked work item with its lifecycle status.
#[derive(Debug, Clone, Default)]
pub struct Ticket {
    pub id: String,
    pub status: String,
    pub title: String,
    pub emoji: String,
    pub prompt: String,
    pub summary: String,
    pub llm: String,
    pub client: String,
    pub author: String,
    pub github_issue: String,
    pub created_at: String,
    pub closed_at: Option<String>,
}

impl Ticket {
    fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("status".to_string(), Value::String(self.status.clone()));
        object.insert("title".to_string(), Value::String(self.title.clone()));
        object.insert("emoji".to_string(), Value::String(self.emoji.clone()));
        object.insert("prompt".to_string(), Value::String(self.prompt.clone()));
        object.insert("summary".to_string(), Value::String(self.summary.clone()));
        object.insert("llm".to_string(), Value::String(self.llm.clone()));
        object.insert("client".to_string(), Value::String(self.client.clone()));
        object.insert("author".to_string(), Value::String(self.author.clone()));
        object.insert("github_issue".to_string(), Value::String(self.github_issue.clone()));
        object.insert("created_at".to_string(), Value::String(self.created_at.clone()));
        object.insert("closed_at".to_string(), self.closed_at.clone().map_or(Value::Null, Value::String));
        Value::Object(object)
    }

    fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            status: text("status"),
            title: text("title"),
            emoji: text("emoji"),
            prompt: text("prompt"),
            summary: text("summary"),
            llm: text("llm"),
            client: text("client"),
            author: text("author"),
            github_issue: text("github_issue"),
            created_at: text("created_at"),
            closed_at: value.get("closed_at").and_then(Value::as_str).map(str::to_string),
        }
    }
}

/// 📖️ A code region — a file, a section or a definition — with its line range.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub id: String,
    pub kind: String,
    pub file_path: String,
    pub section_path: String,
    pub definition: String,
    pub start_line: i64,
    pub end_line: i64,
    pub updated_at: String,
}

impl Scope {
    fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("kind".to_string(), Value::String(self.kind.clone()));
        object.insert("file_path".to_string(), Value::String(self.file_path.clone()));
        object.insert("section_path".to_string(), Value::String(self.section_path.clone()));
        object.insert("definition_name".to_string(), Value::String(self.definition.clone()));
        object.insert("start_line".to_string(), Value::from(self.start_line));
        object.insert("end_line".to_string(), Value::from(self.end_line));
        object.insert("updated_at".to_string(), Value::String(self.updated_at.clone()));
        Value::Object(object)
    }

    fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            kind: text("kind"),
            file_path: text("file_path"),
            section_path: text("section_path"),
            definition: text("definition_name"),
            start_line: value.get("start_line").and_then(Value::as_i64).unwrap_or_default(),
            end_line: value.get("end_line").and_then(Value::as_i64).unwrap_or_default(),
            updated_at: text("updated_at"),
        }
    }
}

/// 🔭️ A detected issue, such as a scope conflict between tickets.
#[derive(Debug, Clone, Default)]
pub struct Warning {
    pub id: String,
    pub kind: String,
    pub severity: String,
    pub message: String,
    pub ticket_id: String,
    pub scope_id: String,
    pub created_at: String,
}

impl Warning {
    fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("kind".to_string(), Value::String(self.kind.clone()));
        object.insert("severity".to_string(), Value::String(self.severity.clone()));
        object.insert("message".to_string(), Value::String(self.message.clone()));
        object.insert("ticket_id".to_string(), Value::String(self.ticket_id.clone()));
        object.insert("scope_id".to_string(), Value::String(self.scope_id.clone()));
        object.insert("created_at".to_string(), Value::String(self.created_at.clone()));
        object.insert("acknowledged_at".to_string(), Value::Null);
        object.insert("ack_by".to_string(), Value::String(String::new()));
        Value::Object(object)
    }

    fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            kind: text("kind"),
            severity: text("severity"),
            message: text("message"),
            ticket_id: text("ticket_id"),
            scope_id: text("scope_id"),
            created_at: text("created_at"),
        }
    }
}

const COORDINATOR_STREAM: &str = "coordinator";
const EVENT_PUBLISHED: &str = "event.published";
const EVENT_TICKET_RECORDED: &str = "ticket.recorded";
const EVENT_SCOPES_RECORDED: &str = "scopes.recorded";
const EVENT_CLAIM_RECORDED: &str = "claim.recorded";
const EVENT_WARNINGS_RECORDED: &str = "warnings.recorded";
const EVENT_CONTRIBUTOR_RECORDED: &str = "contributor.recorded";
const EVENT_CONTRIBUTOR_RELEASED: &str = "contributor.released";
const EVENT_CHECKPOINT_RECORDED: &str = "checkpoint.recorded";

/// 📊️ Everything a coordinator query answers, rebuilt from the log alone.
#[derive(Debug, Default, Clone)]
pub struct Projection {
    pub sequence: u64,
    pub tickets: std::collections::BTreeMap<String, Ticket>,
    pub scopes: std::collections::BTreeMap<String, std::collections::BTreeMap<String, Scope>>,
    pub claims: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    pub warnings: std::collections::BTreeMap<String, Warning>,
    pub contributors: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
}

impl Projection {
    fn apply(&mut self, event: &EventEnvelope) -> Result<(), StoreError> {
        if event.stream != COORDINATOR_STREAM {
            return Ok(());
        }
        match event.kind.as_str() {
            EVENT_PUBLISHED => {}
            EVENT_TICKET_RECORDED => {
                let ticket = Ticket::from_value(&event.payload);
                self.tickets.insert(ticket.id.clone(), ticket);
            }
            EVENT_SCOPES_RECORDED => {
                let file_path = event.payload.get("file_path").and_then(Value::as_str).unwrap_or_default().to_string();
                let mut by_id = std::collections::BTreeMap::new();
                if let Some(items) = event.payload.get("scopes").and_then(Value::as_array) {
                    for item in items {
                        let scope = Scope::from_value(item);
                        by_id.insert(scope.id.clone(), scope);
                    }
                }
                self.scopes.insert(file_path, by_id);
            }
            EVENT_CLAIM_RECORDED => {
                let ticket = event.payload.get("ticket_id").and_then(Value::as_str).unwrap_or_default().to_string();
                let scope = event.payload.get("scope_id").and_then(Value::as_str).unwrap_or_default().to_string();
                let kind = event.payload.get("claim_type").and_then(Value::as_str).unwrap_or_default().to_string();
                self.claims.entry(ticket).or_default().insert(scope, kind);
            }
            EVENT_WARNINGS_RECORDED => {
                self.warnings.retain(|_, warning| warning.kind != "conflict");
                if let Some(items) = event.payload.get("warnings").and_then(Value::as_array) {
                    for item in items {
                        let warning = Warning::from_value(item);
                        self.warnings.insert(warning.id.clone(), warning);
                    }
                }
            }
            EVENT_CONTRIBUTOR_RECORDED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                let kind = event.payload.get("kind").and_then(Value::as_str).unwrap_or_default();
                let item = event.payload.get("item_id").and_then(Value::as_str).unwrap_or_default();
                self.contributors.entry(github).or_default().insert(contributor_key(kind, item));
            }
            EVENT_CONTRIBUTOR_RELEASED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(items) = event.payload.get("items").and_then(Value::as_array) {
                    for item in items {
                        let kind = item.get("kind").and_then(Value::as_str).unwrap_or_default();
                        let id = item.get("id").and_then(Value::as_str).unwrap_or_default();
                        self.contributors.entry(github.clone()).or_default().remove(&contributor_key(kind, id));
                    }
                }
            }
            EVENT_CHECKPOINT_RECORDED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(items) = event.payload.get("files").and_then(Value::as_array) {
                    for item in items {
                        let file = item.as_str().unwrap_or_default();
                        self.contributors.entry(github.clone()).or_default().remove(&contributor_key("file", file));
                    }
                }
            }
            other => return Err(StoreError::Corrupt(format!("unknown coordinator event type {other:?}"))),
        }
        self.sequence = event.sequence;
        Ok(())
    }
}

fn contributor_key(kind: &str, id: &str) -> String {
    format!("{kind}\u{0}{id}")
}

//#endregion 🧠️Projection

//#region 📚️Repository

/// 🗄️ Executes commands against the log and derives every query by replay.
#[derive(Debug)]
pub struct Repository {
    store: EventStore,
    commands: Mutex<()>,
    projection: Mutex<Projection>,
    closed: AtomicBool,
}

impl Repository {
    /// 🆕️ Opens the owned coordinator event repository at a path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let store = EventStore::open(path, StoreLimits::default())?;
        let repository = Self { store, commands: Mutex::new(()), projection: Mutex::new(Projection::default()), closed: AtomicBool::new(false) };
        repository.reload()?;
        Ok(repository)
    }

    /// 📪️ Makes repository operations explicitly unavailable without discarding durable state.
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    /// 🔌️ Recovers a short storage shortage and rebuilds the projection.
    pub fn reopen(&self) -> Result<(), StoreError> {
        self.reload()?;
        self.closed.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// 📊️ A snapshot of the replayed projection.
    pub fn projection(&self) -> Projection {
        self.projection.lock().expect("projection").clone()
    }

    fn reload(&self) -> Result<(), StoreError> {
        let events = self.store.replay()?;
        let mut projection = Projection::default();
        for event in &events {
            projection.apply(event)?;
        }
        *self.projection.lock().expect("projection") = projection;
        Ok(())
    }

    fn execute(&self, kind: &str, payload: Value) -> Result<(), StoreError> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(StoreError::Unavailable("repository closed".to_string()));
        }
        let guard = self.commands.lock().expect("commands");
        let input = EventInput { stream: COORDINATOR_STREAM.to_string(), id: command_id(kind, 1, &payload), generation: 1, kind: kind.to_string(), payload };
        for _ in 0..3 {
            let expected = self.projection.lock().expect("projection").sequence;
            match self.store.append(expected, std::slice::from_ref(&input)) {
                Ok(result) => {
                    let mut projection = self.projection.lock().expect("projection");
                    for event in &result.events {
                        if event.sequence > projection.sequence {
                            projection.apply(event)?;
                        }
                    }
                    drop(guard);
                    return Ok(());
                }
                Err(StoreError::SequenceConflict(_)) => self.reload()?,
                Err(failure) => {
                    drop(guard);
                    return Err(failure);
                }
            }
        }
        drop(guard);
        Err(StoreError::SequenceConflict("retries exhausted".to_string()))
    }

    /// 📨️ Records one published in-process event.
    pub fn record_published_event(&self, event: &Value) -> Result<(), StoreError> {
        self.execute(EVENT_PUBLISHED, event.clone())
    }

    /// 🎫️ Records the current state of one ticket.
    pub fn record_ticket(&self, ticket: &Ticket) -> Result<(), StoreError> {
        self.execute(EVENT_TICKET_RECORDED, ticket.to_value())
    }

    /// 📖️ Records the scopes of one file.
    pub fn record_scopes(&self, file_path: &str, scopes: &[Scope]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("file_path".to_string(), Value::String(file_path.to_string()));
        object.insert("scopes".to_string(), Value::Array(scopes.iter().map(Scope::to_value).collect()));
        self.execute(EVENT_SCOPES_RECORDED, Value::Object(object))
    }

    /// 🗡️ Records one ticket's claim on one scope.
    pub fn record_claim(&self, ticket_id: &str, scope_id: &str, claim_type: &str, at: &str) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("ticket_id".to_string(), Value::String(ticket_id.to_string()));
        object.insert("scope_id".to_string(), Value::String(scope_id.to_string()));
        object.insert("claim_type".to_string(), Value::String(claim_type.to_string()));
        object.insert("at".to_string(), Value::String(at.to_string()));
        self.execute(EVENT_CLAIM_RECORDED, Value::Object(object))
    }

    /// ⚠️ Replaces every conflict warning with the current set.
    pub fn record_warnings(&self, warnings: &[Warning]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("warnings".to_string(), Value::Array(warnings.iter().map(Warning::to_value).collect()));
        self.execute(EVENT_WARNINGS_RECORDED, Value::Object(object))
    }

    /// 🧑️ Records that a contributor is working on one item.
    pub fn record_contributor_work(&self, github: &str, kind: &str, item_id: &str) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("github".to_string(), Value::String(github.to_string()));
        object.insert("kind".to_string(), Value::String(kind.to_string()));
        object.insert("item_id".to_string(), Value::String(item_id.to_string()));
        self.execute(EVENT_CONTRIBUTOR_RECORDED, Value::Object(object))
    }

    /// 🏁️ Records a checkpoint, releasing every file it touched.
    pub fn record_checkpoint(&self, github: &str, files: &[String]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("github".to_string(), Value::String(github.to_string()));
        object.insert("files".to_string(), Value::Array(files.iter().map(|file| Value::String(file.clone())).collect()));
        self.execute(EVENT_CHECKPOINT_RECORDED, Value::Object(object))
    }

    /// 🎫️ Every ticket, optionally filtered by status, in id order.
    pub fn project_tickets(&self, status: &str) -> Vec<Ticket> {
        self.projection
            .lock()
            .expect("projection")
            .tickets
            .values()
            .filter(|ticket| status.is_empty() || ticket.status == status)
            .cloned()
            .collect()
    }

    /// 🎫️ One ticket by id.
    pub fn project_ticket(&self, ticket_id: &str) -> Result<Ticket, StoreError> {
        self.projection
            .lock()
            .expect("projection")
            .tickets
            .get(ticket_id)
            .cloned()
            .ok_or_else(|| StoreError::NotFound(format!("ticket {ticket_id:?}")))
    }

    /// 📖️ Every scope recorded for one file, in id order.
    pub fn project_scopes_by_file(&self, file_path: &str) -> Vec<Scope> {
        self.projection
            .lock()
            .expect("projection")
            .scopes
            .get(file_path)
            .map(|scopes| scopes.values().cloned().collect())
            .unwrap_or_default()
    }

    /// 🗡️ Every scope one ticket has claimed, in id order.
    pub fn project_claims_by_ticket(&self, ticket_id: &str) -> Vec<Scope> {
        let projection = self.projection.lock().expect("projection");
        let mut by_id: std::collections::BTreeMap<&String, &Scope> = std::collections::BTreeMap::new();
        for scopes in projection.scopes.values() {
            for (id, scope) in scopes {
                by_id.insert(id, scope);
            }
        }
        projection
            .claims
            .get(ticket_id)
            .map(|claims| claims.keys().filter_map(|id| by_id.get(id).map(|scope| (*scope).clone())).collect())
            .unwrap_or_default()
    }

    /// ⚠️ Every warning, optionally filtered by ticket, in id order.
    pub fn project_warnings(&self, ticket_id: &str) -> Vec<Warning> {
        self.projection
            .lock()
            .expect("projection")
            .warnings
            .values()
            .filter(|warning| ticket_id.is_empty() || warning.ticket_id == ticket_id)
            .cloned()
            .collect()
    }

    /// 🔷️ Breaches are not yet recorded by any command, so the projection is always empty.
    pub fn project_breachs(&self, _ticket_id: &str) -> Vec<Value> {
        Vec::new()
    }

    /// 💥️ Every scope claimed by more than one open ticket, in scope order.
    pub fn project_conflicts(&self) -> Vec<(String, Vec<String>)> {
        let projection = self.projection.lock().expect("projection");
        let mut by_scope: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        for (ticket_id, claims) in &projection.claims {
            if projection.tickets.get(ticket_id).map(|ticket| ticket.status.as_str()) != Some("open") {
                continue;
            }
            for scope_id in claims.keys() {
                by_scope.entry(scope_id.clone()).or_default().push(ticket_id.clone());
            }
        }
        by_scope
            .into_iter()
            .filter(|(_, tickets)| tickets.len() > 1)
            .map(|(scope, mut tickets)| {
                tickets.sort();
                (scope, tickets)
            })
            .collect()
    }

    /// 🧑️ Every contributor working on one item, in login order.
    pub fn project_contributors_on_item(&self, kind: &str, item_id: &str) -> Vec<String> {
        let key = contributor_key(kind, item_id);
        self.projection
            .lock()
            .expect("projection")
            .contributors
            .iter()
            .filter(|(_, items)| items.contains(&key))
            .map(|(github, _)| github.clone())
            .collect()
    }
}

fn command_id(kind: &str, generation: u64, payload: &Value) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{kind}\u{0}{generation}\u{0}").as_bytes());
    hasher.update(to_go_json(payload).as_bytes());
    format!("command-{}", hasher.finish())
}

//#endregion 📚️Repository

//#region 🌐️Http

/// 📨️ One parsed HTTP/1.1 request.
#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    fn header(&self, name: &str) -> String {
        self.headers.get(&name.to_ascii_lowercase()).cloned().unwrap_or_default()
    }

    fn json(&self) -> Option<Value> {
        serde_json::from_slice(&self.body).ok()
    }
}

/// 📩️ One HTTP/1.1 response.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

impl Response {
    fn empty(status: u16) -> Self {
        Self { status, content_type: None, body: Vec::new() }
    }

    fn text(status: u16, body: &str) -> Self {
        Self { status, content_type: None, body: body.as_bytes().to_vec() }
    }

    fn json(status: u16, value: &Value) -> Self {
        Self { status, content_type: Some("application/json".to_string()), body: format!("{}\n", to_go_json(value)).into_bytes() }
    }

    fn error(status: u16, message: &str) -> Self {
        let mut object = Map::new();
        object.insert("error".to_string(), Value::String(message.to_string()));
        Self::json(status, &Value::Object(object))
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Status",
    }
}

/// 🗄️ The coordinator's request-scoped state.
#[derive(Debug)]
pub struct Server {
    pub config: Config,
    pub repository: Repository,
    github_cache: Mutex<HashMap<String, (String, u64)>>,
}

impl Server {
    /// ⚙️ Wires a server over an already opened repository.
    pub fn new(config: Config, repository: Repository) -> Self {
        Self { config, repository, github_cache: Mutex::new(HashMap::new()) }
    }

    fn authorized(&self, request: &Request) -> bool {
        if self.config.token.is_empty() {
            return true;
        }
        let authorization = request.header("authorization");
        match authorization.split_once(' ') {
            Some(("Bearer", token)) => token == self.config.token,
            _ => false,
        }
    }

    /// 🧭️ Answers one request through the full coordinator route table.
    pub fn handle(&self, request: &Request) -> Response {
        match request.path.as_str() {
            "/healthz" => self.guard(request, "GET", false, |_| Response::text(200, "ok")),
            "/ticket/open" => self.guard(request, "POST", true, |server| server.ticket_open(request)),
            "/ticket/close" => self.guard(request, "POST", true, |server| server.ticket_close(request)),
            "/ticket/reopen" => self.guard(request, "POST", true, |server| server.ticket_reopen(request)),
            "/tickets" => self.guard(request, "GET", true, |server| server.tickets_query(request)),
            "/diff/ingest" => self.guard(request, "POST", true, |server| server.diff_ingest(request)),
            "/repo/reindex" => self.guard(request, "POST", true, |server| server.reindex(request)),
            "/repo/index-file" => self.guard(request, "POST", true, |server| server.index_file(request)),
            "/warnings" => self.guard(request, "GET", true, |server| server.warnings(request)),
            "/breachs" => self.guard(request, "GET", true, |server| server.breachs(request)),
            "/scopes" => self.guard(request, "GET", true, |server| server.scopes(request)),
            "/events" | "/api/v1/events" => self.guard(request, "POST", true, |server| server.events(request)),
            "/webhooks/github" => self.guard(request, "POST", true, |server| server.github_webhook(request)),
            path if path.starts_with("/ticket/") => self.guard(request, "GET", true, |server| server.ticket_detail(request)),
            _ => Response::error(404, "not found"),
        }
    }

    fn guard(&self, request: &Request, method: &str, authenticated: bool, handler: impl FnOnce(&Self) -> Response) -> Response {
        if request.method != method {
            return Response::empty(405);
        }
        if authenticated && !self.authorized(request) {
            return Response::error(401, "unauthorized");
        }
        handler(self)
    }

    fn ticket_open(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("title").is_empty() {
            return Response::error(400, "ticket_id and title required");
        }
        let ticket = Ticket {
            id: text("ticket_id"),
            status: "open".to_string(),
            title: text("title"),
            prompt: text("prompt"),
            llm: text("llm"),
            client: text("client"),
            author: text("author"),
            github_issue: text("github_issue"),
            created_at: now_rfc3339(),
            ..Ticket::default()
        };
        match self.record_and_publish(&ticket, "TicketOpened") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    fn ticket_close(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("summary").is_empty() {
            return Response::error(400, "ticket_id and summary required");
        }
        let mut ticket = match self.repository.project_ticket(&text("ticket_id")) {
            Err(failure) => return Response::error(404, &failure.to_string()),
            Ok(ticket) => ticket,
        };
        ticket.status = "closed".to_string();
        ticket.summary = text("summary");
        ticket.closed_at = Some(now_rfc3339());
        match self.record_and_publish(&ticket, "TicketClosed") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    fn ticket_reopen(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        if text("ticket_id").is_empty() || text("prompt").is_empty() {
            return Response::error(400, "ticket_id and prompt required");
        }
        let mut ticket = match self.repository.project_ticket(&text("ticket_id")) {
            Err(failure) => return Response::error(404, &failure.to_string()),
            Ok(ticket) => ticket,
        };
        ticket.status = "open".to_string();
        ticket.prompt = text("prompt");
        ticket.llm = text("llm");
        if !text("title").is_empty() {
            ticket.title = text("title");
        }
        ticket.closed_at = None;
        match self.record_and_publish(&ticket, "TicketReopened") {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => Response::json(200, &ticket.to_value()),
        }
    }

    fn record_and_publish(&self, ticket: &Ticket, kind: &str) -> Result<(), StoreError> {
        self.repository.record_ticket(ticket)?;
        self.publish(kind, "repo-cli", &ticket.to_value())
    }

    fn tickets_query(&self, request: &Request) -> Response {
        let status = request.query.get("status").cloned().unwrap_or_default();
        let tickets: Vec<Value> = self.repository.project_tickets(&status).iter().map(Ticket::to_value).collect();
        Response::json(200, &Value::Array(tickets))
    }

    fn ticket_detail(&self, request: &Request) -> Response {
        let rest = request.path.trim_start_matches("/ticket/");
        if rest.is_empty() {
            return Response::error(404, "ticket not found");
        }
        if let Some(ticket_id) = rest.strip_suffix("/claims") {
            let claims: Vec<Value> = self.repository.project_claims_by_ticket(ticket_id).iter().map(Scope::to_value).collect();
            return Response::json(200, &Value::Array(claims));
        }
        match self.repository.project_ticket(rest) {
            Err(failure) => Response::error(404, &failure.to_string()),
            Ok(ticket) => Response::json(200, &ticket.to_value()),
        }
    }

    fn diff_ingest(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let ticket_id = payload.get("ticket_id").and_then(Value::as_str).unwrap_or_default().to_string();
        let patch = payload.get("patch").and_then(Value::as_str).unwrap_or_default().to_string();
        if ticket_id.is_empty() || patch.is_empty() {
            return Response::error(400, "ticket_id and patch required");
        }
        let files = parse_unified_diff(&patch);
        let changed: Vec<String> = files.iter().map(|file| file.path.clone()).collect();
        let mut published = Map::new();
        published.insert("ticket_id".to_string(), Value::String(ticket_id));
        published.insert("files".to_string(), Value::Array(changed.iter().map(|file| Value::String(file.clone())).collect()));
        if let Err(failure) = self.publish("DiffIngested", "repo-cli", &Value::Object(published)) {
            return Response::error(500, &failure.to_string());
        }
        let conflicts = self.repository.project_conflicts();
        let warnings: Vec<Warning> = conflicts
            .iter()
            .map(|(scope, tickets)| Warning {
                id: new_id(),
                kind: "conflict".to_string(),
                severity: "error".to_string(),
                message: format!("conflict on {scope} across tickets {}", tickets.join(", ")),
                scope_id: scope.clone(),
                created_at: now_rfc3339(),
                ..Warning::default()
            })
            .collect();
        if let Err(failure) = self.repository.record_warnings(&warnings) {
            return Response::error(500, &failure.to_string());
        }
        let blockers: Vec<Value> = warnings.iter().filter(|warning| warning.severity == "error").map(|warning| Value::String(warning.message.clone())).collect();
        let mut response = Map::new();
        response.insert("changed_files".to_string(), string_list_or_null(&changed));
        response.insert("claimed_scopes".to_string(), Value::Null);
        response.insert("warnings".to_string(), if warnings.is_empty() { Value::Null } else { Value::Array(warnings.iter().map(Warning::to_value).collect()) });
        response.insert("breachs".to_string(), Value::Array(Vec::new()));
        response.insert("blockers".to_string(), Value::Array(blockers));
        Response::json(200, &Value::Object(response))
    }

    fn reindex(&self, _request: &Request) -> Response {
        let files = walk_repo_files(Path::new(&self.config.repo_root));
        for file in &files {
            let path = Path::new(&self.config.repo_root).join(file);
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            if let Err(failure) = self.index(file, &content) {
                return Response::error(500, &failure.to_string());
            }
        }
        let mut object = Map::new();
        object.insert("files".to_string(), Value::from(files.len()));
        Response::json(200, &Value::Object(object))
    }

    fn index_file(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let file_path = payload.get("file_path").and_then(Value::as_str).unwrap_or_default().to_string();
        if file_path.is_empty() {
            return Response::error(400, "file_path required");
        }
        let content = payload.get("content").and_then(Value::as_str).unwrap_or_default();
        match self.index(&file_path, content) {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => {
                let mut object = Map::new();
                object.insert("status".to_string(), Value::String("ok".to_string()));
                Response::json(200, &Value::Object(object))
            }
        }
    }

    fn index(&self, file_path: &str, content: &str) -> Result<(), StoreError> {
        let scopes = build_scopes_for_file(file_path, content);
        self.repository.record_scopes(file_path, &scopes)?;
        let mut object = Map::new();
        object.insert("file".to_string(), Value::String(file_path.to_string()));
        self.publish("IndexUpdated", "server", &Value::Object(object))
    }

    fn warnings(&self, request: &Request) -> Response {
        let ticket = request.query.get("ticket_id").cloned().unwrap_or_default();
        let warnings: Vec<Value> = self.repository.project_warnings(&ticket).iter().map(Warning::to_value).collect();
        Response::json(200, &Value::Array(warnings))
    }

    fn breachs(&self, request: &Request) -> Response {
        let ticket = request.query.get("ticket_id").cloned().unwrap_or_default();
        Response::json(200, &Value::Array(self.repository.project_breachs(&ticket)))
    }

    fn scopes(&self, request: &Request) -> Response {
        let Some(file) = request.query.get("file").filter(|value| !value.is_empty()) else {
            return Response::error(400, "file query required");
        };
        let scopes: Vec<Value> = self.repository.project_scopes_by_file(file).iter().map(Scope::to_value).collect();
        Response::json(200, &Value::Array(scopes))
    }

    fn events(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let kind = payload.get("kind").and_then(Value::as_str).unwrap_or_default().to_string();
        if kind.is_empty() {
            return Response::error(400, "kind required");
        }
        let source = payload.get("source").and_then(Value::as_str).unwrap_or_default().to_string();
        let body = payload.get("payload").cloned().unwrap_or(Value::Null);
        match self.publish(&kind, &source, &body) {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => {
                let mut object = Map::new();
                object.insert("status".to_string(), Value::String("ok".to_string()));
                Response::json(200, &Value::Object(object))
            }
        }
    }

    /// 📬️ Persists one in-process event and runs the notification handlers that subscribe to it.
    ///
    /// The Go twin routes this through a buffered channel whose `Publish` blocks for the handler
    /// result, so the observable contract is a synchronous dispatch — which is what this is.
    pub fn publish(&self, kind: &str, source: &str, payload: &Value) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(new_id()));
        object.insert("type".to_string(), Value::String(kind.to_string()));
        object.insert("source".to_string(), Value::String(source.to_string()));
        object.insert("payload_json".to_string(), Value::String(to_go_json(payload)));
        object.insert("created_at".to_string(), Value::String(now_rfc3339()));
        let event = Value::Object(object);
        self.repository.record_published_event(&event)?;
        self.notify(kind, payload);
        Ok(())
    }
}

fn string_list_or_null(values: &[String]) -> Value {
    if values.is_empty() {
        Value::Null
    } else {
        Value::Array(values.iter().map(|value| Value::String(value.clone())).collect())
    }
}

/// 📍️ One file entry of a unified diff with its hunks.
#[derive(Debug, Clone, Default)]
pub struct DiffFile {
    pub path: String,
    pub hunks: Vec<(i64, i64)>,
    pub deleted: bool,
}

/// 🧲️ Extracts file paths and new-side hunk ranges from a unified diff patch.
pub fn parse_unified_diff(patch: &str) -> Vec<DiffFile> {
    let mut files: Vec<DiffFile> = Vec::new();
    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let parts: Vec<&str> = rest.split(' ').collect();
            if parts.len() >= 2 {
                files.push(DiffFile { path: parts[1].trim_start_matches("b/").to_string(), ..DiffFile::default() });
            }
            continue;
        }
        let Some(current) = files.last_mut() else { continue };
        if line.starts_with("+++ ") {
            if line.contains("/dev/null") {
                current.deleted = true;
            }
            continue;
        }
        if let Some(range) = line.strip_prefix("@@ ") {
            if let Some(new_side) = range.split('+').nth(1).and_then(|rest| rest.split(' ').next()) {
                let mut halves = new_side.split(',');
                let start: i64 = halves.next().unwrap_or("0").parse().unwrap_or(0);
                let count: i64 = halves.next().map_or(1, |value| value.parse().unwrap_or(1));
                current.hunks.push((start, start + count - 1));
            }
        }
    }
    let mut seen: Vec<String> = Vec::new();
    files.retain(|file| {
        if file.path.is_empty() || seen.contains(&file.path) {
            return false;
        }
        seen.push(file.path.clone());
        true
    });
    files
}

/// 📖️ The scopes of one file.
///
/// Only the file scope is derived here. Section and definition parsing lives in `🗣️languages`, whose
/// Rust crate does not yet expose it; the Go twin reaches it through `repo/events`. Until that
/// symbol lands, `/scopes` answers with the file scope alone and the cases avoid comparing bodies
/// that depend on the parser — recorded in `📓️opus-coordinator.md`.
pub fn build_scopes_for_file(path: &str, _content: &str) -> Vec<Scope> {
    vec![Scope { id: format!("file:{path}"), kind: "file".to_string(), file_path: path.to_string(), updated_at: now_rfc3339(), ..Scope::default() }]
}

fn walk_repo_files(root: &Path) -> Vec<String> {
    fn walk(root: &Path, current: &Path, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(current) else { return };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            if path.is_dir() {
                if !name.starts_with('.') {
                    walk(root, &path, out);
                }
                continue;
            }
            if let Ok(relative) = path.strip_prefix(root) {
                out.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

/// 🛎️ One running coordinator: its listener, its worker pool and the state they share.
pub struct Service {
    pub address: String,
    server: Arc<Server>,
    running: Arc<AtomicBool>,
    workers: Vec<std::thread::JoinHandle<()>>,
    queue: Arc<(Mutex<Vec<TcpStream>>, Condvar)>,
}

impl Service {
    /// 📊️ The state the routes read and write.
    pub fn server(&self) -> &Arc<Server> {
        &self.server
    }

    /// 🛑️ Stops accepting, drains the workers and releases the store.
    pub fn stop(mut self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = TcpStream::connect(&self.address);
        {
            let (queue, condition) = &*self.queue;
            let _unused = queue.lock().expect("queue");
            condition.notify_all();
        }
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
        self.server.repository.close();
    }
}

/// 🚀️ Opens the store, binds the address and serves the coordinator route table.
pub fn start(config: Config) -> Result<Service, StoreError> {
    let repository = Repository::open(&config.database_path)?;
    let listener = TcpListener::bind(&config.address).map_err(|failure| StoreError::Unavailable(format!("bind {}: {failure}", config.address)))?;
    let address = listener.local_addr().map_err(|failure| StoreError::Unavailable(failure.to_string()))?.to_string();
    let server = Arc::new(Server::new(config, repository));
    let running = Arc::new(AtomicBool::new(true));
    let queue: Arc<(Mutex<Vec<TcpStream>>, Condvar)> = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let listener = Arc::new(listener);
    let mut workers = Vec::new();
    for _ in 0..4 {
        let server = Arc::clone(&server);
        let running = Arc::clone(&running);
        let queue = Arc::clone(&queue);
        workers.push(std::thread::spawn(move || worker_loop(&server, &running, &queue)));
    }
    let acceptor = {
        let listener = Arc::clone(&listener);
        let running = Arc::clone(&running);
        let queue = Arc::clone(&queue);
        std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let Ok((stream, _)) = listener.accept() else { break };
                if !running.load(Ordering::SeqCst) {
                    let _ = stream.shutdown(Shutdown::Both);
                    break;
                }
                let (pending, condition) = &*queue;
                pending.lock().expect("queue").push(stream);
                condition.notify_one();
            }
        })
    };
    workers.push(acceptor);
    Ok(Service { address, server, running, workers, queue })
}

fn worker_loop(server: &Arc<Server>, running: &Arc<AtomicBool>, queue: &Arc<(Mutex<Vec<TcpStream>>, Condvar)>) {
    let (pending, condition) = &**queue;
    loop {
        let stream = {
            let mut guard = pending.lock().expect("queue");
            while guard.is_empty() {
                if !running.load(Ordering::SeqCst) {
                    return;
                }
                guard = condition.wait_timeout(guard, Duration::from_millis(50)).expect("queue").0;
            }
            guard.remove(0)
        };
        serve_connection(server, stream);
    }
}

fn serve_connection(server: &Arc<Server>, mut stream: TcpStream) {
    let response = match read_request(&mut stream, server.config.request_body_limit) {
        Err(message) => Response::error(400, &message),
        Ok(request) => server.handle(&request),
    };
    let mut head = format!("HTTP/1.1 {} {}\r\n", response.status, reason(response.status));
    if let Some(content_type) = &response.content_type {
        head.push_str(&format!("Content-Type: {content_type}\r\n"));
    }
    head.push_str(&format!("Content-Length: {}\r\nConnection: close\r\n\r\n", response.body.len()));
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&response.body);
    let _ = stream.flush();
    let _ = stream.shutdown(Shutdown::Both);
}

fn read_request(stream: &mut TcpStream, limit: u64) -> Result<Request, String> {
    let mut reader = BufReader::new(stream.try_clone().map_err(|failure| failure.to_string())?);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|failure| failure.to_string())?;
    let mut parts = line.trim_end().split(' ');
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or_default().to_string();
    let mut headers = HashMap::new();
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).map_err(|failure| failure.to_string())? == 0 {
            break;
        }
        let header = header.trim_end();
        if header.is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }
    let length: u64 = headers.get("content-length").and_then(|value| value.parse().ok()).unwrap_or(0);
    if length > limit {
        return Err("request body too large".to_string());
    }
    let mut body = vec![0u8; length as usize];
    if length > 0 {
        reader.read_exact(&mut body).map_err(|failure| failure.to_string())?;
    }
    let (path, raw_query) = target.split_once('?').unwrap_or((target.as_str(), ""));
    let mut query = HashMap::new();
    for pair in raw_query.split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.insert(percent_decode(key), percent_decode(value));
    }
    Ok(Request { method, path: percent_decode(path), query, headers, body })
}

fn percent_decode(text: &str) -> String {
    let bytes = text.replace('+', " ").into_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[index + 1..index + 3]), 16) {
                out.push(byte);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

//#endregion 🌐️Http

//#region 🪝️Webhooks

impl Server {
    fn github_webhook(&self, request: &Request) -> Response {
        if !self.config.github_secret.is_empty() {
            let signature = request.header("x-hub-signature-256");
            if !verify_github_signature(&request.body, &signature, &self.config.github_secret) {
                return Response::error(401, "invalid signature");
            }
        }
        let event_type = request.header("x-github-event");
        let mut published = Map::new();
        published.insert("type".to_string(), Value::String(event_type.clone()));
        if let Err(failure) = self.publish("GitHubIssueEventReceived", "github", &Value::Object(published)) {
            return Response::error(500, &failure.to_string());
        }
        let payload = request.json().unwrap_or(Value::Null);
        match event_type.as_str() {
            "issue_comment" => self.cache_github_comment(&payload),
            "issues" => {
                if let Err(failure) = self.github_issue_event(&payload) {
                    return Response::error(500, &failure.to_string());
                }
            }
            "push" => {
                if let Err(failure) = self.github_push_event(&payload) {
                    return Response::error(500, &failure.to_string());
                }
            }
            _ => {}
        }
        Response::empty(200)
    }

    fn cache_github_comment(&self, payload: &Value) {
        let (issue, repo, actor) = github_identity(payload);
        let body = payload.get("comment").and_then(|comment| comment.get("body")).and_then(Value::as_str).unwrap_or_default().to_string();
        if issue == 0 || repo.is_empty() || actor.is_empty() || body.is_empty() {
            return;
        }
        self.github_cache.lock().expect("github").insert(format!("{repo}#{issue}#{actor}"), (body, unix_seconds()));
    }

    fn cached_comment(&self, repo: &str, issue: i64, actor: &str) -> String {
        let key = format!("{repo}#{issue}#{actor}");
        let mut cache = self.github_cache.lock().expect("github");
        match cache.get(&key) {
            None => String::new(),
            Some((body, when)) => {
                if unix_seconds().saturating_sub(*when) > 90 {
                    cache.remove(&key);
                    String::new()
                } else {
                    body.clone()
                }
            }
        }
    }

    fn github_issue_event(&self, payload: &Value) -> Result<(), StoreError> {
        let action = payload.get("action").and_then(Value::as_str).unwrap_or_default();
        let (issue, repo, actor) = github_identity(payload);
        if issue == 0 || repo.is_empty() || actor.is_empty() {
            return Ok(());
        }
        let comment = self.cached_comment(&repo, issue, &actor);
        if comment.is_empty() {
            return Ok(());
        }
        let kind = match action {
            "closed" => "TicketClosed",
            "reopened" => "TicketReopened",
            _ => return Ok(()),
        };
        let mut object = Map::new();
        object.insert("issue".to_string(), Value::from(issue));
        object.insert("comment".to_string(), Value::String(comment));
        self.publish(kind, "github", &Value::Object(object))
    }

    fn github_push_event(&self, payload: &Value) -> Result<(), StoreError> {
        let (_, _, mut actor) = github_identity(payload);
        if actor.is_empty() {
            actor = payload.get("pusher").and_then(|pusher| pusher.get("name")).and_then(Value::as_str).unwrap_or_default().to_string();
        }
        let mut files = Vec::new();
        if let Some(commits) = payload.get("commits").and_then(Value::as_array) {
            for commit in commits {
                for key in ["added", "modified"] {
                    if let Some(items) = commit.get(key).and_then(Value::as_array) {
                        files.extend(items.iter().filter_map(Value::as_str).map(str::to_string));
                    }
                }
            }
        }
        if actor.is_empty() || files.is_empty() {
            return Ok(());
        }
        self.repository.record_checkpoint(&actor, &files)
    }

    /// 🔔️ Sends the Discord notification a ticket lifecycle event asks for, when one is configured.
    fn notify(&self, kind: &str, payload: &Value) {
        let title = match kind {
            "TicketOpened" | "TicketReopened" => "# Prompt",
            "TicketClosed" => "# Summary",
            _ => return,
        };
        self.notify_discord(title, &to_go_json(payload));
    }

    fn notify_discord(&self, title: &str, body: &str) {
        if self.config.discord_webhook.is_empty() {
            return;
        }
        let mut object = Map::new();
        object.insert("content".to_string(), Value::String(format!("{title}\n{body}")));
        let encoded = to_go_json(&Value::Object(object));
        let Some(rest) = self.config.discord_webhook.strip_prefix("http://") else { return };
        let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
        let Ok(mut stream) = TcpStream::connect(authority) else { return };
        let request = format!(
            "POST /{path} HTTP/1.1\r\nHost: {authority}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{encoded}",
            encoded.len()
        );
        let _ = stream.write_all(request.as_bytes());
    }
}

fn github_identity(payload: &Value) -> (i64, String, String) {
    let issue = payload.get("issue").and_then(|issue| issue.get("number")).and_then(Value::as_i64).unwrap_or(0);
    let repo = payload.get("repository").and_then(|repo| repo.get("full_name")).and_then(Value::as_str).unwrap_or_default().to_string();
    let actor = payload.get("sender").and_then(|sender| sender.get("login")).and_then(Value::as_str).unwrap_or_default().to_string();
    (issue, repo, actor)
}

/// 🔏️ Validates the HMAC-SHA256 signature of a webhook payload.
pub fn verify_github_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let Some((_, provided)) = signature.split_once('=') else { return false };
    let computed = hmac_sha256(secret.as_bytes(), body);
    computed.len() == provided.len() && computed.bytes().zip(provided.bytes()).fold(0u8, |sum, (left, right)| sum | (left ^ right)) == 0
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> String {
    let mut block = [0u8; 64];
    if key.len() > 64 {
        let folded = hex_to_bytes(&digest(key));
        block[..folded.len()].copy_from_slice(&folded);
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut inner = Sha256::new();
    inner.update(&block.map(|byte| byte ^ 0x36));
    inner.update(message);
    let mut outer = Sha256::new();
    outer.update(&block.map(|byte| byte ^ 0x5c));
    outer.update(&hex_to_bytes(&inner.finish()));
    outer.finish()
}

fn hex_to_bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2).filter_map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).ok()).collect()
}

//#endregion 🪝️Webhooks

//#region 🚀️Main

fn unix_seconds() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_secs()).unwrap_or_default()
}

/// 🆔️ A monotonic-prefixed unique identifier for one in-process event.
pub fn new_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_nanos()).unwrap_or_default();
    format!("{nanos}-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// 🕰️ The current instant as RFC 3339 in UTC, the shape Go's `time.Time` marshals to.
pub fn now_rfc3339() -> String {
    let total = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = total.as_secs() as i64;
    let (days, rest) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:09}Z",
        rest / 3600,
        (rest % 3600) / 60,
        rest % 60,
        total.subsec_nanos()
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// ▶️ Starts the coordinator from the ambient environment and blocks forever.
pub fn main_blocking() -> Result<(), StoreError> {
    let config = Config::from_env();
    let service = start(config)?;
    println!("repo coordinator listening on {}", service.address);
    loop {
        std::thread::sleep(Duration::from_secs(3600));
    }
}

//#endregion 🚀️Main

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("semio-coordinator-{name}-{}", new_id()));
        std::fs::create_dir_all(&dir).expect("dir");
        dir
    }

    #[test]
    fn golden_envelope_matches_the_frozen_fixture() {
        let mut event = EventEnvelope {
            stream: "coordinator".to_string(),
            sequence: 1,
            id: "evt-1".to_string(),
            generation: 7,
            kind: "ticket.recorded".to_string(),
            payload: serde_json::json!({"id": "T-1", "status": "open"}),
            checksum: String::new(),
        };
        event.checksum = event_checksum(&event);
        assert_eq!(event.checksum, "6d6e63ada99d3b4b6ed21114f00421e3c0bb6d480c830351f4b7f323c6fa82ec");
        assert_eq!(
            encode_event(&event),
            "{\"stream\":\"coordinator\",\"sequence\":1,\"id\":\"evt-1\",\"generation\":7,\"type\":\"ticket.recorded\",\"payload\":{\"id\":\"T-1\",\"status\":\"open\"},\"checksum\":\"6d6e63ada99d3b4b6ed21114f00421e3c0bb6d480c830351f4b7f323c6fa82ec\"}\n"
        );
    }

    #[test]
    fn append_then_replay_is_deterministic_and_refuses_a_duplicate() {
        let dir = temp_dir("append");
        let store = EventStore::open(dir.join("coordinator.events"), StoreLimits::default()).expect("open");
        let input = EventInput {
            stream: "coordinator".to_string(),
            id: "evt-1".to_string(),
            generation: 7,
            kind: "ticket.recorded".to_string(),
            payload: serde_json::json!({"id": "T-1", "status": "open"}),
        };
        let first = store.append(0, std::slice::from_ref(&input)).expect("append");
        assert!(first.committed && !first.duplicate);
        let replayed = store.replay().expect("replay");
        assert_eq!(replayed.len(), 1);
        assert_eq!(replayed[0].sequence, 1);
        let again = store.append(1, std::slice::from_ref(&input)).expect("idempotent");
        assert!(again.duplicate);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_injected_fault_preserves_the_committed_prefix() {
        let dir = temp_dir("fault");
        let path = dir.join("coordinator.events");
        let store = EventStore::open(&path, StoreLimits::default()).expect("open");
        let first = EventInput {
            stream: "coordinator".to_string(),
            id: "evt-1".to_string(),
            generation: 1,
            kind: "ticket.recorded".to_string(),
            payload: serde_json::json!({"id": "T-1"}),
        };
        store.append(0, std::slice::from_ref(&first)).expect("first");
        let before = std::fs::read(&path).expect("read");
        store.arm_fault(1);
        let second = EventInput { id: "evt-2".to_string(), ..first };
        assert!(store.append(1, std::slice::from_ref(&second)).is_err());
        store.arm_fault(0);
        assert_eq!(std::fs::read(&path).expect("read"), before);
        let reopened = EventStore::open(&path, StoreLimits::default()).expect("reopen");
        assert_eq!(reopened.replay().expect("replay").len(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_route_table_answers_every_documented_path() {
        let dir = temp_dir("http");
        let config = Config { address: "127.0.0.1:0".to_string(), database_path: dir.join("coordinator.events").display().to_string(), ..Config::default() };
        let service = start(config).expect("start");
        let server = Arc::clone(service.server());
        let health = server.handle(&Request { method: "GET".to_string(), path: "/healthz".to_string(), ..Request::default() });
        assert_eq!((health.status, health.body), (200, b"ok".to_vec()));
        let opened = server.handle(&Request {
            method: "POST".to_string(),
            path: "/ticket/open".to_string(),
            body: br#"{"ticket_id":"T-1","title":"Demo"}"#.to_vec(),
            ..Request::default()
        });
        assert_eq!(opened.status, 200);
        let missing = server.handle(&Request { method: "GET".to_string(), path: "/ticket/absent".to_string(), ..Request::default() });
        assert_eq!(missing.status, 404);
        let wrong_method = server.handle(&Request { method: "GET".to_string(), path: "/events".to_string(), ..Request::default() });
        assert_eq!(wrong_method.status, 405);
        service.stop();
        std::fs::remove_dir_all(&dir).ok();
    }
}

//#endregion 🧪️Tests


