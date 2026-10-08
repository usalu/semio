//! ✉️ The dashboard wire: the frames a client and the workspace daemon exchange, the task identity
//! that travels with a session, and the workspace paths both ends agree on.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use crate::registry::{Launch, LaunchProcess, Stop};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// 🔢 The wire revision both ends state when a connection opens; a mismatch is reported, never tolerated silently.
pub const PROTOCOL: u32 = 2;
pub const KIND_CONTROL: u8 = 1;
pub const KIND_OUTPUT: u8 = 2;
pub const KIND_INPUT: u8 = 3;
pub const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;
/// 📦 The most terminal bytes one output or input frame carries.
pub const MAX_CHUNK_BYTES: usize = 64 * 1024;
const MAX_ENVIRONMENT_PAIRS: usize = 8192;
const MAX_GROUP_MEMBERS: usize = 64;

/// 📂 Canonical workspace paths retain native command-runtime spelling on Windows.
pub fn canonical_path(path: &Path) -> PathBuf {
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    #[cfg(windows)]
    {
        let text = path.to_string_lossy();
        if let Some(rest) = text.strip_prefix(r"\\?\UNC\") { return PathBuf::from(format!(r"\\{rest}")); }
        if let Some(rest) = text.strip_prefix(r"\\?\") { return PathBuf::from(rest); }
    }
    path
}

/// 🎛️ Client → daemon control envelope (JSON).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ClientMsg {
    Hello { client_id: String, protocol: u32, build_id: String, #[serde(default)] env: Vec<(String, String)> },
    Attach { client_id: String },
    Watch {},
    Subscribe { session_id: String },
    Unsubscribe { session_id: String },
    Detach {},
    Spawn { session_id: String, command: Box<SessionCommand> },
    SpawnGroup { group_id: String, stop: GroupStop, members: Vec<GroupMember>, #[serde(default)] requires: Vec<GroupMember> },
    Input { session_id: String, data: Vec<u8> },
    Resize { session_id: String, cols: u16, rows: u16 },
    Kill { session_id: String },
    Stop { session_id: String },
    Restart { session_id: String },
    StopGroup { group_id: String },
    KillGroup { group_id: String },
    Forget { session_id: String },
    List {},
    Shutdown {},
    Ping {},
}

/// 📣 Daemon → client control envelope (JSON).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ServerMsg {
    Attached { daemon_pid: u32, protocol: u32, build_id: String },
    SessionChanged { session: Box<SessionInfo> },
    SessionRemoved { session_id: String },
    Sessions { sessions: Vec<SessionInfo>, #[serde(default)] more: bool },
    ReplayStart { session_id: String, #[serde(default)] truncated: bool },
    ReplayComplete { #[serde(default, skip_serializing_if = "Option::is_none")] session_id: Option<String> },
    Error { message: String, #[serde(default, skip_serializing_if = "Option::is_none")] code: Option<ErrorCode>, #[serde(default, skip_serializing_if = "Option::is_none")] session_id: Option<String> },
    Pong {},
    Shutdown {},
}

/// 🏷️ What a running task is called; the registry computes it at launch and it travels with the session.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TaskLabel {
    pub verb: String,
    pub owner: Vec<String>,
    pub subject: String,
    pub qualifier: String,
    pub parameters: Vec<(String, String)>,
    pub members: u16,
}

/// 🟢 Ready when the output shows `http://(127.0.0.1|localhost|0.0.0.0):<port>`; the ready URL is that match
/// plus `path`, or with `printed` the whole printed address.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🟢️ready/🦀️.rs
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Ready {
    pub port: u16,
    pub path: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub printed: bool,
}

/// 🚨 The kinds of failure an error message names; the schema lists the same words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode { Protocol, Decode, Invalid, HelloRequired, UnknownSession, UnknownGroup, AlreadyRunning, Limit, ViewLimit, Spawn, NotRunning, InputBacklog }

impl ErrorCode {
    pub const ALL: [Self; 12] = [Self::Protocol, Self::Decode, Self::Invalid, Self::HelloRequired, Self::UnknownSession, Self::UnknownGroup, Self::AlreadyRunning, Self::Limit, Self::ViewLimit, Self::Spawn, Self::NotRunning, Self::InputBacklog];

    /// 🔤 The word the wire and the schema use for the kind.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Protocol => "protocol", Self::Decode => "decode", Self::Invalid => "invalid", Self::HelloRequired => "hello_required", Self::UnknownSession => "unknown_session", Self::UnknownGroup => "unknown_group",
            Self::AlreadyRunning => "already_running", Self::Limit => "limit", Self::ViewLimit => "view_limit", Self::Spawn => "spawn", Self::NotRunning => "not_running", Self::InputBacklog => "input_backlog",
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(self.as_str()) }
}

/// 🔡️ The error kinds by their short names.
pub mod code {
    use super::ErrorCode;
    pub const PROTOCOL: ErrorCode = ErrorCode::Protocol;
    pub const DECODE: ErrorCode = ErrorCode::Decode;
    pub const INVALID: ErrorCode = ErrorCode::Invalid;
    pub const HELLO_REQUIRED: ErrorCode = ErrorCode::HelloRequired;
    pub const UNKNOWN_SESSION: ErrorCode = ErrorCode::UnknownSession;
    pub const UNKNOWN_GROUP: ErrorCode = ErrorCode::UnknownGroup;
    pub const ALREADY_RUNNING: ErrorCode = ErrorCode::AlreadyRunning;
    pub const LIMIT: ErrorCode = ErrorCode::Limit;
    pub const VIEW_LIMIT: ErrorCode = ErrorCode::ViewLimit;
    pub const SPAWN: ErrorCode = ErrorCode::Spawn;
    pub const NOT_RUNNING: ErrorCode = ErrorCode::NotRunning;
    pub const INPUT_BACKLOG: ErrorCode = ErrorCode::InputBacklog;
}

/// 🛑 Whether the members of a group stop when one of them stops.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GroupStop { Together, #[default] Independent }

/// 🧩 One process of a group, started in order under the session identifier its requester chose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GroupMember {
    pub session_id: String,
    pub command: SessionCommand,
}

/// 🧬️ A launch as the daemon starts it: the services that must be ready first, then the processes of
/// the launch in order, all under one group identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnGroup {
    pub group_id: String,
    pub stop: GroupStop,
    pub members: Vec<GroupMember>,
    pub requires: Vec<GroupMember>,
}

/// 🆔 A session or group identifier no other start of this machine will choose.
pub fn fresh_id(prefix: &str) -> String {
    static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    format!("{prefix}-{:x}-{nanos:x}-{:x}", std::process::id(), SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

impl SpawnGroup {
    /// 🚀 Turns a resolved launch into the message that starts it: one session per process, services
    /// first, `stop: together` for a compound. `client_env` is extra environment of this launch that
    /// wins over the launch's own; the environment of the requesting client is the base of every
    /// process and travels in the connection's hello.
    pub fn from_launch(launch: &Launch, client_env: &[(String, String)]) -> Self {
        let group_id = fresh_id("group");
        let mut services: Vec<(&LaunchProcess, usize)> = Vec::new();
        fn collect<'a>(launch: &'a Launch, services: &mut Vec<(&'a LaunchProcess, usize)>) {
            for need in &launch.requires { collect(need, services); }
            for process in &launch.processes {
                if !services.iter().any(|(known, _)| known.command_id == process.command_id) { services.push((process, services.len())); }
            }
        }
        for need in &launch.requires { collect(need, &mut services); }
        let grouped = launch.group.is_some().then(|| group_id.clone());
        let requires = services.iter().map(|(process, index)| GroupMember { session_id: format!("{group_id}.r{index}"), command: SessionCommand::from_process(process, client_env, None) }).collect();
        let members = launch.processes.iter().enumerate().map(|(index, process)| GroupMember { session_id: format!("{group_id}.{index}"), command: SessionCommand::from_process(process, client_env, grouped.clone()) }).collect();
        Self { group_id, stop: if launch.stop == Stop::Together { GroupStop::Together } else { GroupStop::Independent }, members, requires }
    }

    /// 🎬️ The control message that asks the daemon to start the group.
    pub fn message(self) -> ClientMsg {
        ClientMsg::SpawnGroup { group_id: self.group_id, stop: self.stop, members: self.members, requires: self.requires }
    }
}

/// ▶️ The exact task invocation retained for restart and restored views.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SessionCommand {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub env: Vec<(String, String)>,
    pub cols: u16,
    pub rows: u16,
    #[serde(default)]
    pub command_id: String,
    #[serde(default)]
    pub label: TaskLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<Ready>,
}

/// 🚦 The lifecycle projected from the daemon's persisted local-only events.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus { #[default] Pending, Running, Stopping, Exited, Failed, Interrupted }

impl SessionStatus {
    /// 🔴 Whether the session still waits for or owns a process.
    pub fn live(self) -> bool { matches!(self, Self::Pending | Self::Running | Self::Stopping) }
}

/// 📋 A process projection shared by every dashboard attached to this workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SessionInfo {
    pub session_id: String,
    pub command: SessionCommand,
    pub status: SessionStatus,
    #[serde(deserialize_with = "nullable")]
    pub pid: Option<u32>,
    #[serde(deserialize_with = "nullable")]
    pub code: Option<i32>,
    #[serde(default)]
    pub started_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

fn nullable<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error> { Option::<T>::deserialize(deserializer) }

fn invalid(message: &str) -> std::io::Error { std::io::Error::new(std::io::ErrorKind::InvalidInput, message.to_string()) }

fn validate_id(id: &str) -> std::io::Result<()> {
    if id.is_empty() || id.chars().count() > 128 { return Err(invalid("invalid session identifier")); }
    Ok(())
}

fn validate_environment(env: &[(String, String)]) -> bool {
    env.iter().all(|(name, value)| !name.is_empty() && !name.contains(['=', '\0']) && !value.contains('\0'))
}

impl SessionCommand {
    /// 🏃️ The invocation of one process of a resolved launch at the default terminal size.
    pub fn from_process(process: &LaunchProcess, extra_env: &[(String, String)], group: Option<String>) -> Self {
        let mut env = process.env.clone();
        for (key, value) in extra_env {
            match env.iter_mut().find(|(name, _)| name == key) { Some(pair) => pair.1 = value.clone(), None => env.push((key.clone(), value.clone())) }
        }
        Self { cmd: process.cmd.clone(), args: process.args.clone(), cwd: process.cwd.display().to_string(), env, cols: 80, rows: 24, command_id: process.command_id.clone(), label: process.label.clone(), group, ready: process.ready.clone() }
    }

    pub fn validate(&self) -> std::io::Result<()> {
        if self.cmd.is_empty() || self.cmd.contains('\0') || self.cols == 0 || self.rows == 0 || self.cols > 32767 || self.rows > 32767
            || self.cwd.contains('\0') || self.args.iter().any(|arg| arg.contains('\0')) || !validate_environment(&self.env) || self.command_id.len() > 512
            || self.ready.as_ref().is_some_and(|ready| ready.port == 0 || ready.path.contains(|character: char| character.is_whitespace() || character == '\0')) {
            return Err(invalid("invalid session command"));
        }
        if let Some(group) = &self.group { validate_id(group)?; }
        Ok(())
    }

    /// 🪞 Whether two invocations are the same process: program, arguments, directory and declared environment.
    pub fn same_task(&self, other: &Self) -> bool {
        let env = |pairs: &[(String, String)]| pairs.iter().map(|(key, value)| (if cfg!(windows) { key.to_uppercase() } else { key.clone() }, value.clone())).collect::<std::collections::BTreeMap<_, _>>();
        self.cmd == other.cmd && self.args == other.args && self.cwd == other.cwd && env(&self.env) == env(&other.env)
    }
}

impl SessionInfo {
    pub fn validate(&self) -> std::io::Result<()> {
        validate_id(&self.session_id)?;
        if self.pid == Some(0) { return Err(invalid("invalid session pid")); }
        if let Some(group) = &self.group { validate_id(group)?; }
        self.command.validate()
    }
}

impl ClientMsg {
    pub fn validate(&self) -> std::io::Result<()> {
        match self {
            Self::Hello { env, .. } => if env.len() > MAX_ENVIRONMENT_PAIRS || !validate_environment(env) { Err(invalid("invalid client environment")) } else { Ok(()) },
            Self::Spawn { session_id, command } => { validate_id(session_id)?; command.validate() }
            Self::SpawnGroup { group_id, members, requires, .. } => {
                validate_id(group_id)?;
                if members.is_empty() || members.len() + requires.len() > MAX_GROUP_MEMBERS { return Err(invalid("invalid group size")); }
                let mut seen = std::collections::BTreeSet::new();
                for member in members.iter().chain(requires) {
                    validate_id(&member.session_id)?;
                    member.command.validate()?;
                    if !seen.insert(&member.session_id) { return Err(invalid("duplicate group member")); }
                }
                Ok(())
            }
            Self::StopGroup { group_id } | Self::KillGroup { group_id } => validate_id(group_id),
            Self::Input { session_id, .. } | Self::Stop { session_id } | Self::Kill { session_id } | Self::Restart { session_id } | Self::Subscribe { session_id } | Self::Unsubscribe { session_id } | Self::Forget { session_id } => validate_id(session_id),
            Self::Resize { session_id, cols, rows } => {
                validate_id(session_id)?;
                if *cols == 0 || *rows == 0 || *cols > 32767 || *rows > 32767 { return Err(invalid("invalid terminal size")); }
                Ok(())
            }
            Self::Attach { .. } | Self::Watch {} | Self::Detach {} | Self::List {} | Self::Shutdown {} | Self::Ping {} => Ok(()),
        }
    }
}

impl ServerMsg {
    pub fn validate(&self) -> std::io::Result<()> {
        match self {
            Self::Attached { daemon_pid: 0, .. } => Err(invalid("invalid daemon pid")),
            Self::SessionChanged { session } => session.validate(),
            Self::SessionRemoved { session_id } | Self::ReplayStart { session_id, .. } => validate_id(session_id),
            Self::ReplayComplete { session_id: Some(session_id) } => validate_id(session_id),
            Self::Sessions { sessions, .. } => sessions.iter().try_for_each(SessionInfo::validate),
            _ => Ok(()),
        }
    }

    /// 🚧️ An error a client can act on: `code` names the kind, `session_id` the session it concerns.
    pub fn error(code: ErrorCode, message: impl Into<String>, session_id: Option<&str>) -> Self {
        Self::Error { message: message.into(), code: Some(code), session_id: session_id.map(str::to_string) }
    }
}

/// 📁 Cache directory for the dashboard daemon endpoint, pid, journal and session logs.
pub fn semio_root_dir(root: &Path) -> PathBuf {
    root.join(".🧬semio")
}

pub fn repo_meta_dir(root: &Path) -> PathBuf {
    semio_root_dir(root).join("🦑️repo")
}

pub fn dashboard_cache_dir(root: &Path) -> PathBuf {
    repo_meta_dir(root).join("⚡️cache").join("🎛️dashboard")
}

/// 🌐 The name of the daemon instance the environment selects, if any. `SEMIO_DASHBOARD_INSTANCE=<name>` gives a
/// workspace a second, isolated daemon with its own endpoint, lock, journal and logs, so a smoke test can run on the
/// real workspace without touching the developer's daemon. Characters other than letters, digits, `.`, `_` and `-`
/// become `_`; the name is cut to 32 characters; an empty value selects the default instance.
pub fn instance() -> Option<String> {
    instance_named(std::env::var("SEMIO_DASHBOARD_INSTANCE").ok().as_deref())
}

/// 🪧 The instance a raw name selects.
pub fn instance_named(raw: Option<&str>) -> Option<String> {
    let name: String = raw?.trim().chars().take(32).map(|character| if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') { character } else { '_' }).collect();
    (!name.is_empty() && name != "." && name != "..").then_some(name)
}

/// 🗂️ The directory of one daemon instance's endpoint record, pid, lock, journal and logs: the dashboard cache
/// itself for the default instance, a folder of its own for a named one.
pub fn daemon_dir_of(root: &Path, instance: Option<&str>) -> PathBuf {
    match instance {
        Some(name) => dashboard_cache_dir(root).join("instances").join(name),
        None => dashboard_cache_dir(root),
    }
}

/// 🪬 The directory of the daemon instance this process selects.
pub fn daemon_dir(root: &Path) -> PathBuf { daemon_dir_of(root, instance().as_deref()) }

pub fn pid_path(root: &Path) -> PathBuf {
    daemon_dir(root).join("daemon.pid")
}

pub fn event_log_path(root: &Path) -> PathBuf {
    daemon_dir(root).join("events.jsonl")
}

pub fn lock_path(root: &Path) -> PathBuf { daemon_dir(root).join("daemon.lock") }

/// 📍 The file in which a running daemon records where it listens.
pub fn endpoint_path(root: &Path) -> PathBuf { daemon_dir(root).join("daemon.endpoint") }

/// 🗃️ The directory of the per-session output logs.
pub fn log_dir(root: &Path) -> PathBuf { daemon_dir(root).join("logs") }

/// #️⃣ The hash every persistent name of the dashboard derives from: XXH3 with the default secret, which is the
/// same value on every platform, toolchain and run, unlike the standard library's randomised hasher.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
pub fn stable_hash(bytes: &[u8]) -> u64 {
    crate::registry::xxh3_64(bytes)
}

/// 🔑 The stable name of a workspace in endpoint names: sixteen hexadecimal digits of its canonical path, and of the
/// instance name when the environment selects one.
pub fn workspace_key(root: &Path) -> String {
    workspace_key_of(root, instance().as_deref())
}

/// 🪪 The stable name of one daemon instance of a workspace.
pub fn workspace_key_of(root: &Path, instance: Option<&str>) -> String {
    let path = canonical_path(root).to_string_lossy().into_owned();
    let named = instance.map_or(path.clone(), |name| format!("{path}\0{name}"));
    format!("{:016x}", stable_hash(named.as_bytes()))
}

/// 🗄️ The per-user directory of daemon sockets. It never depends on `TMPDIR`, which differs between
/// the shells and editors that start views; `SEMIO_DASHBOARD_RUNTIME_DIR` names another directory.
#[cfg(unix)]
pub fn runtime_dir() -> PathBuf {
    match std::env::var_os("SEMIO_DASHBOARD_RUNTIME_DIR").filter(|value| !value.is_empty()) {
        Some(directory) => PathBuf::from(directory),
        None => PathBuf::from("/tmp").join(format!("semio-dashboard-{}", ui_tui::tui::pty::user_id())),
    }
}

/// 🔌 Where the daemon of a workspace listens unless its endpoint record says otherwise.
#[cfg(unix)]
pub fn socket_path(root: &Path) -> PathBuf {
    runtime_dir().join(format!("{}.sock", workspace_key(root)))
}

#[cfg(windows)]
pub fn pipe_name(root: &Path) -> String {
    format!(r"\\.\pipe\semio-dashboard-{}", workspace_key(root))
}

/// 🧭 The endpoint a client connects to: what the running daemon recorded, else the deterministic name.
pub fn endpoint(root: &Path) -> String {
    if let Some(recorded) = std::fs::read_to_string(endpoint_path(root)).ok().map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) { return recorded; }
    #[cfg(unix)]
    { socket_path(root).display().to_string() }
    #[cfg(windows)]
    { pipe_name(root) }
    #[cfg(not(any(unix, windows)))]
    { daemon_dir(root).join("daemon.sock").display().to_string() }
}

/// 🏗️ The identity of this executable's build. An installed dashboard lives in a directory named
/// after the hash of its bytes; any other build is identified by its size and modification instant.
pub fn build_id() -> &'static str {
    static BUILD: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    BUILD.get_or_init(|| {
        let Ok(executable) = std::env::current_exe() else { return "unknown".into() };
        let pinned = executable.parent().and_then(Path::file_name).and_then(std::ffi::OsStr::to_str).filter(|name| name.len() == 64 && name.bytes().all(|byte| byte.is_ascii_hexdigit()));
        if let Some(hash) = pinned { return hash[..16].to_string(); }
        let Ok(metadata) = std::fs::metadata(&executable) else { return "unknown".into() };
        let modified = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |elapsed| elapsed.as_nanos());
        format!("dev-{:016x}", stable_hash(format!("{}:{modified}", metadata.len()).as_bytes()))
    })
}

/// ⏱️ Unix milliseconds of now.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64)
}

/// 🧱 One complete frame as bytes: `u32 le len | u8 kind | payload`.
pub fn frame(kind: u8, payload: &[u8]) -> std::io::Result<Vec<u8>> {
    let len = payload.len().checked_add(1).filter(|len| *len <= MAX_FRAME_BYTES).ok_or_else(|| std::io::Error::other("frame exceeds limit"))? as u32;
    let mut bytes = Vec::with_capacity(5 + payload.len());
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.push(kind);
    bytes.extend_from_slice(payload);
    Ok(bytes)
}

/// 🧾️ One control frame as bytes.
pub fn control_frame(msg: &impl Serialize) -> std::io::Result<Vec<u8>> {
    frame(KIND_CONTROL, &serde_json::to_vec(msg).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?)
}

/// 🧵️ One output or input frame as bytes.
pub fn chunk_frame(kind: u8, session_id: &str, data: &[u8]) -> std::io::Result<Vec<u8>> {
    frame(kind, &encode_output(session_id, data))
}

/// 📤️ Writes one length-prefixed frame: `u32 le len | u8 kind | payload`.
pub fn write_frame(out: &mut impl Write, kind: u8, payload: &[u8]) -> std::io::Result<()> {
    out.write_all(&frame(kind, payload)?)?;
    out.flush()
}

/// 📥️ Reads one length-prefixed frame.
pub fn read_frame(input: &mut impl Read) -> std::io::Result<(u8, Vec<u8>)> {
    let mut len_buf = [0u8; 4];
    input.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len == 0 || len > MAX_FRAME_BYTES {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad frame length"));
    }
    let mut body = vec![0u8; len];
    input.read_exact(&mut body)?;
    let kind = body[0];
    Ok((kind, body[1..].to_vec()))
}

/// 🔓️ Decodes one complete frame from a persistent nonblocking receive buffer.
pub fn try_decode_frame(buffer: &mut Vec<u8>) -> std::io::Result<Option<(u8, Vec<u8>)>> {
    if buffer.len() < 4 {
        return Ok(None);
    }
    let len = u32::from_le_bytes(buffer[..4].try_into().expect("frame prefix is four bytes")) as usize;
    if len == 0 || len > MAX_FRAME_BYTES {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad frame length"));
    }
    let frame_len = 4usize.saturating_add(len);
    if buffer.len() < frame_len {
        return Ok(None);
    }
    let kind = buffer[4];
    let payload = buffer[5..frame_len].to_vec();
    buffer.drain(..frame_len);
    Ok(Some((kind, payload)))
}

/// 🪣 A receive buffer that hands out complete frames in arrival order without moving the bytes behind them.
#[derive(Default)]
pub struct FrameBuffer {
    bytes: Vec<u8>,
    start: usize,
}

impl FrameBuffer {
    pub fn extend(&mut self, bytes: &[u8]) {
        if self.start > 0 && self.start == self.bytes.len() { self.bytes.clear(); self.start = 0; }
        if self.start > 256 * 1024 { self.bytes.drain(..self.start); self.start = 0; }
        self.bytes.extend_from_slice(bytes);
    }

    /// 📏 How many received bytes still wait for the rest of their frame or for their turn.
    pub fn pending(&self) -> usize { self.bytes.len() - self.start }

    pub fn next_frame(&mut self) -> std::io::Result<Option<(u8, &[u8])>> {
        let rest = &self.bytes[self.start..];
        if rest.len() < 4 { return Ok(None); }
        let len = u32::from_le_bytes(rest[..4].try_into().expect("frame prefix is four bytes")) as usize;
        if len == 0 || len > MAX_FRAME_BYTES { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad frame length")); }
        if rest.len() < 4 + len { return Ok(None); }
        let at = self.start;
        self.start += 4 + len;
        Ok(Some((self.bytes[at + 4], &self.bytes[at + 5..at + 4 + len])))
    }
}

pub fn write_control(out: &mut impl Write, msg: &impl Serialize) -> std::io::Result<()> {
    out.write_all(&control_frame(msg)?)?;
    out.flush()
}

pub fn decode_control<T: for<'de> Deserialize<'de>>(payload: &[u8]) -> std::io::Result<T> {
    serde_json::from_slice(payload).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// 🎁️ Session output or input frame payload: `u16 le id_len | id_bytes | data`.
pub fn encode_output(session_id: &str, data: &[u8]) -> Vec<u8> {
    let id = session_id.as_bytes();
    let mut out = Vec::with_capacity(2 + id.len() + data.len());
    out.extend_from_slice(&(id.len() as u16).to_le_bytes());
    out.extend_from_slice(id);
    out.extend_from_slice(data);
    out
}

pub fn decode_chunk(payload: &[u8]) -> std::io::Result<(&str, &[u8])> {
    if payload.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "short output frame"));
    }
    let id_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    if payload.len() < 2 + id_len {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "output id truncated"));
    }
    let id = std::str::from_utf8(&payload[2..2 + id_len]).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    validate_id(id).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid session identifier"))?;
    Ok((id, &payload[2 + id_len..]))
}

pub fn decode_output(payload: &[u8]) -> std::io::Result<(String, Vec<u8>)> {
    decode_chunk(payload).map(|(id, data)| (id.to_string(), data.to_vec()))
}
