//! 🌀️ The dashboard daemon: the framed client/daemon transport, the pseudo-terminal session
//! supervisor that multiplexes it, and the `semio daemon start|serve|stop|status|attach` verb.

use crate::args::ParsedArgs;
use std::path::{Path, PathBuf};

#[path = "../📎️connection/🦀️.rs"]
pub mod client;

// #region 🔖️Ipc
/// ✉️ The length-prefixed control/output framing the dashboard client and daemon speak.
pub mod ipc {
    use serde::{Deserialize, Serialize};
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    pub const KIND_CONTROL: u8 = 1;
    pub const KIND_OUTPUT: u8 = 2;
    pub const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

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
        Attach { client_id: String },
        Detach {},
        Spawn { session_id: String, command: SessionCommand },
        Input { session_id: String, data: Vec<u8> },
        Resize { session_id: String, cols: u16, rows: u16 },
        Kill { session_id: String },
        Stop { session_id: String },
        Restart { session_id: String },
        List {},
        Shutdown {},
        Ping {},
    }

    /// 📣 Daemon → client control envelope (JSON).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
    pub enum ServerMsg {
        Attached { daemon_pid: u32 },
        SessionChanged { session: SessionInfo },
        Sessions { sessions: Vec<SessionInfo> },
        ReplayComplete {},
        Error { message: String },
        Pong {},
        Shutdown {},
    }

    /// ▶️ The exact task invocation retained for restart and restored views.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct SessionCommand {
        pub cmd: String,
        pub args: Vec<String>,
        pub cwd: String,
        pub env: Vec<(String, String)>,
        pub cols: u16,
        pub rows: u16,
    }

    /// 🚦 The lifecycle projected from the daemon's persisted local-only events.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum SessionStatus { Running, Stopping, Exited, Failed }

    /// 📋 A process projection shared by every dashboard attached to this workspace.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct SessionInfo {
        pub session_id: String,
        pub command: SessionCommand,
        pub status: SessionStatus,
        #[serde(deserialize_with = "nullable")]
        pub pid: Option<u32>,
        #[serde(deserialize_with = "nullable")]
        pub code: Option<i32>,
    }

    fn nullable<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error> { Option::<T>::deserialize(deserializer) }

    fn validate_id(id: &str) -> std::io::Result<()> {
        if id.is_empty() || id.chars().count() > 128 { return Err(std::io::Error::other("invalid session identifier")); }
        Ok(())
    }

    impl SessionCommand {
        pub fn validate(&self) -> std::io::Result<()> {
            if self.cmd.is_empty() || self.cmd.contains('\0') || self.cols == 0 || self.rows == 0 || self.cols > 32767 || self.rows > 32767
                || self.cwd.contains('\0') || self.args.iter().any(|arg| arg.contains('\0'))
                || self.env.iter().any(|(name, value)| name.is_empty() || name.contains(['=', '\0']) || value.contains('\0')) {
                return Err(std::io::Error::other("invalid session command"));
            }
            Ok(())
        }

        pub fn same_task(&self, other: &Self) -> bool {
            let env = |pairs: &[(String, String)]| pairs.iter().map(|(key, value)| (if cfg!(windows) { key.to_uppercase() } else { key.clone() }, value.clone())).collect::<std::collections::BTreeMap<_, _>>();
            self.cmd == other.cmd && self.args == other.args && self.cwd == other.cwd && env(&self.env) == env(&other.env)
        }
    }

    impl ClientMsg {
        pub fn validate(&self) -> std::io::Result<()> {
            match self {
                Self::Spawn { session_id, command } => { validate_id(session_id)?; command.validate() }
                Self::Input { session_id, .. } | Self::Stop { session_id } | Self::Kill { session_id } | Self::Restart { session_id } => validate_id(session_id),
                Self::Resize { session_id, cols, rows } => {
                    validate_id(session_id)?;
                    if *cols == 0 || *rows == 0 || *cols > 32767 || *rows > 32767 { return Err(std::io::Error::other("invalid terminal size")); }
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }

    impl ServerMsg {
        pub fn validate(&self) -> std::io::Result<()> {
            match self {
                Self::Attached { daemon_pid: 0 } => Err(std::io::Error::other("invalid daemon pid")),
                Self::SessionChanged { session } => { validate_id(&session.session_id)?; if session.pid == Some(0) { return Err(std::io::Error::other("invalid session pid")); } session.command.validate() },
                Self::Sessions { sessions } => {
                    for session in sessions { validate_id(&session.session_id)?; if session.pid == Some(0) { return Err(std::io::Error::other("invalid session pid")); } session.command.validate()?; }
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }

    /// 📁 Cache directory for the dashboard daemon socket / pid / event log.
    pub fn semio_root_dir(root: &Path) -> PathBuf {
        root.join(".🧬semio")
    }

    pub fn repo_meta_dir(root: &Path) -> PathBuf {
        semio_root_dir(root).join("🦑️repo")
    }

    pub fn dashboard_cache_dir(root: &Path) -> PathBuf {
        repo_meta_dir(root).join("⚡️cache").join("🎛️dashboard")
    }

    pub fn socket_path(root: &Path) -> PathBuf {
        #[cfg(unix)]
        {
            use std::hash::{Hash, Hasher};
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            root.hash(&mut hash);
            std::env::temp_dir().join(format!("semio-dashboard-{:x}", hash.finish())).join("daemon.sock")
        }
        #[cfg(windows)]
        {
            dashboard_cache_dir(root).join("daemon.pipe.name")
        }
        #[cfg(not(any(unix, windows)))]
        {
            dashboard_cache_dir(root).join("daemon.sock")
        }
    }

    pub fn pid_path(root: &Path) -> PathBuf {
        dashboard_cache_dir(root).join("daemon.pid")
    }

    pub fn event_log_path(root: &Path) -> PathBuf {
        dashboard_cache_dir(root).join("events.jsonl")
    }

    pub fn lock_path(root: &Path) -> PathBuf { dashboard_cache_dir(root).join("daemon.lock") }

    /// ✉️ Writes one length-prefixed frame: `u32 le len | u8 kind | payload`.
    pub fn write_frame(out: &mut impl Write, kind: u8, payload: &[u8]) -> std::io::Result<()> {
        let len = payload.len().checked_add(1).filter(|len| *len <= MAX_FRAME_BYTES).ok_or_else(|| std::io::Error::other("frame exceeds limit"))? as u32;
        out.write_all(&len.to_le_bytes())?;
        out.write_all(&[kind])?;
        out.write_all(payload)?;
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

    /// 🧩 Decodes one complete frame from a persistent nonblocking receive buffer.
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

    pub fn write_control(out: &mut impl Write, msg: &impl Serialize) -> std::io::Result<()> {
        let payload = serde_json::to_vec(msg).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        write_frame(out, KIND_CONTROL, &payload)
    }

    pub fn decode_control<T: for<'de> Deserialize<'de>>(payload: &[u8]) -> std::io::Result<T> {
        serde_json::from_slice(payload).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// 📤 Session output frame payload: `u16 le id_len | id_bytes | data`.
    pub fn encode_output(session_id: &str, data: &[u8]) -> Vec<u8> {
        let id = session_id.as_bytes();
        let mut out = Vec::with_capacity(2 + id.len() + data.len());
        out.extend_from_slice(&(id.len() as u16).to_le_bytes());
        out.extend_from_slice(id);
        out.extend_from_slice(data);
        out
    }

    pub fn decode_output(payload: &[u8]) -> std::io::Result<(String, Vec<u8>)> {
        if payload.len() < 2 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "short output frame"));
        }
        let id_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
        if payload.len() < 2 + id_len {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "output id truncated"));
        }
        let id = std::str::from_utf8(&payload[2..2 + id_len]).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok((id.to_string(), payload[2 + id_len..].to_vec()))
    }

    #[cfg(unix)]
    pub fn connect(root: &Path) -> std::io::Result<std::os::unix::net::UnixStream> {
        std::os::unix::net::UnixStream::connect(socket_path(root))
    }

    #[cfg(unix)]
    pub fn listen(root: &Path) -> std::io::Result<std::os::unix::net::UnixListener> {
        use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
        let path = socket_path(root);
        let dir = path.parent().ok_or_else(|| std::io::Error::other("dashboard socket needs a parent directory"))?;
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)?;
        std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
        let _ = std::fs::remove_file(&path);
        let listener = std::os::unix::net::UnixListener::bind(&path)?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        Ok(listener)
    }

    #[cfg(windows)]
    pub fn pipe_name(root: &Path) -> String {
        let hash = {
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            root.hash(&mut h);
            h.finish()
        };
        format!(r"\\.\pipe\semio-dashboard-{hash:x}")
    }

    #[cfg(windows)]
    pub fn connect(root: &Path) -> std::io::Result<std::fs::File> {
        let name = pipe_name(root);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
        loop {
            match std::fs::OpenOptions::new().read(true).write(true).open(&name) {
                Ok(file) => return Ok(file),
                Err(error) if matches!(error.raw_os_error(), Some(2 | 231)) && std::time::Instant::now() < deadline => std::thread::sleep(std::time::Duration::from_millis(20)),
                Err(error) => return Err(error),
            }
        }
    }

    // 🪟 The two Win32 entry points a named-pipe SERVER needs. They are operating-system calls, not a
    // third-party crate: the client half already reaches the same object through `std::fs`, and no
    // other API can create a pipe instance or wait for the peer that opens it.
    #[cfg(windows)]
    extern "system" {
        fn CreateNamedPipeW(name: *const u16, open_mode: u32, pipe_mode: u32, max_instances: u32, out_buffer_size: u32, in_buffer_size: u32, default_time_out: u32, security_attributes: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn ConnectNamedPipe(pipe: *mut std::ffi::c_void, overlapped: *mut std::ffi::c_void) -> i32;
        fn PeekNamedPipe(pipe: *mut std::ffi::c_void, buffer: *mut std::ffi::c_void, buffer_size: u32, bytes_read: *mut u32, total_bytes_available: *mut u32, bytes_left_this_message: *mut u32) -> i32;
    }

    /// 👁️ How many bytes the peer has already delivered, or `None` once the pipe is gone.
    ///
    /// A pipe opened without `FILE_FLAG_OVERLAPPED` serialises every operation on the file object, so a
    /// blocking read parked on one handle blocks every WRITE to the same pipe — including a write
    /// through a duplicated handle from another thread. Asking first and reading only what is already
    /// there keeps every read short, which is what lets one thread own both directions.
    #[cfg(windows)]
    pub fn peek(pipe: &std::fs::File) -> Option<usize> {
        use std::os::windows::io::AsRawHandle;
        let mut available: u32 = 0;
        let answered = unsafe { PeekNamedPipe(pipe.as_raw_handle().cast(), std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut available, std::ptr::null_mut()) };
        (answered != 0).then_some(available as usize)
    }

    /// 👂️ Prepares the daemon's listening address and records it where `status` can read it back.
    #[cfg(windows)]
    pub fn listen(root: &Path) -> std::io::Result<String> {
        let dir = dashboard_cache_dir(root);
        std::fs::create_dir_all(&dir)?;
        let name = pipe_name(root);
        std::fs::write(socket_path(root), format!("{name}\n"))?;
        Ok(name)
    }

    /// 🤝️ Creates one pipe instance and blocks until a client opens it, answering the connected stream.
    #[cfg(windows)]
    pub fn accept(name: &str) -> std::io::Result<std::fs::File> {
        use std::os::windows::ffi::OsStrExt;
        use std::os::windows::io::FromRawHandle;
        const PIPE_ACCESS_DUPLEX: u32 = 0x0000_0003;
        const PIPE_TYPE_BYTE_WAIT: u32 = 0x0000_0000;
        const PIPE_UNLIMITED_INSTANCES: u32 = 255;
        const BUFFER_BYTES: u32 = 64 * 1024;
        const DEFAULT_TIMEOUT_MS: u32 = 0;
        const ERROR_PIPE_CONNECTED: i32 = 535;
        let wide: Vec<u16> = std::ffi::OsStr::new(name).encode_wide().chain(std::iter::once(0)).collect();
        let handle = unsafe { CreateNamedPipeW(wide.as_ptr(), PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE_WAIT, PIPE_UNLIMITED_INSTANCES, BUFFER_BYTES, BUFFER_BYTES, DEFAULT_TIMEOUT_MS, std::ptr::null_mut()) };
        if handle.is_null() || handle as isize == -1 {
            return Err(std::io::Error::last_os_error());
        }
        let stream = unsafe { std::fs::File::from_raw_handle(handle.cast()) };
        if unsafe { ConnectNamedPipe(handle, std::ptr::null_mut()) } == 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() != Some(ERROR_PIPE_CONNECTED) {
                return Err(error);
            }
        }
        Ok(stream)
    }
}
// #endregion 🔖️Ipc

// #region 🔖️Supervisor
/// 🧠 The session supervisor: pseudo-terminal lifecycle plus fan-out to attached clients.
pub mod supervisor {
    use super::ipc::{self, ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
    use std::collections::{BTreeMap, VecDeque};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    #[cfg(any(unix, windows))]
    const CLIENT_READ_BYTES_PER_TICK: usize = 64 * 1024;
    #[cfg(any(unix, windows))]
    const CLIENT_FRAMES_PER_TICK: usize = 32;

    #[cfg(unix)]
    struct ClientReader {
        id: u64,
        stream: std::os::unix::net::UnixStream,
        buffer: Vec<u8>,
    }

    #[cfg(unix)]
    impl ClientReader {
        fn new(id: u64, stream: std::os::unix::net::UnixStream) -> Self {
            Self { id, stream, buffer: Vec::new() }
        }

        fn turn(&mut self, messages: &mut Vec<(u64, ClientMsg)>) -> std::io::Result<bool> {
            use std::io::Read;
            let mut chunk = [0u8; 16 * 1024];
            let mut read_bytes = 0usize;
            while read_bytes < CLIENT_READ_BYTES_PER_TICK {
                let allowance = (CLIENT_READ_BYTES_PER_TICK - read_bytes).min(chunk.len());
                match self.stream.read(&mut chunk[..allowance]) {
                    Ok(0) => return Ok(false),
                    Ok(count) => {
                        self.buffer.extend_from_slice(&chunk[..count]);
                        read_bytes = read_bytes.saturating_add(count);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => return Ok(false),
                }
            }
            for _ in 0..CLIENT_FRAMES_PER_TICK {
                let Some((kind, payload)) = ipc::try_decode_frame(&mut self.buffer)? else { break };
                if kind == ipc::KIND_CONTROL {
                    if let Ok(message) = ipc::decode_control::<ClientMsg>(&payload) {
                        messages.push((self.id, message));
                    }
                }
            }
            Ok(true)
        }
    }

    /// 🪟 The same bounded cursor over a windows named pipe. A pipe handle has no non-blocking mode
    /// worth having, so readiness is asked for with `PeekNamedPipe` and only the bytes already
    /// delivered are read — every read returns at once, which is what keeps the daemon's writes on the
    /// same pipe from being serialised behind a parked read.
    #[cfg(windows)]
    struct ClientReader {
        id: u64,
        stream: std::fs::File,
        buffer: Vec<u8>,
    }

    #[cfg(windows)]
    impl ClientReader {
        fn new(id: u64, stream: std::fs::File) -> Self {
            Self { id, stream, buffer: Vec::new() }
        }

        fn turn(&mut self, messages: &mut Vec<(u64, ClientMsg)>) -> std::io::Result<bool> {
            use std::io::Read;
            let mut chunk = [0u8; 16 * 1024];
            let mut read_bytes = 0usize;
            while read_bytes < CLIENT_READ_BYTES_PER_TICK {
                let Some(available) = ipc::peek(&self.stream) else { return Ok(false) };
                if available == 0 {
                    break;
                }
                let allowance = available.min(chunk.len()).min(CLIENT_READ_BYTES_PER_TICK - read_bytes);
                match self.stream.read(&mut chunk[..allowance]) {
                    Ok(0) => return Ok(false),
                    Ok(count) => {
                        self.buffer.extend_from_slice(&chunk[..count]);
                        read_bytes = read_bytes.saturating_add(count);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => return Ok(false),
                }
            }
            for _ in 0..CLIENT_FRAMES_PER_TICK {
                let Some((kind, payload)) = ipc::try_decode_frame(&mut self.buffer)? else { break };
                if kind == ipc::KIND_CONTROL {
                    if let Ok(message) = ipc::decode_control::<ClientMsg>(&payload) {
                        messages.push((self.id, message));
                    }
                }
            }
            Ok(true)
        }
    }

    /// 📜 Append-only JSONL event log for session lifecycle (not PTY bytes).
    pub struct EventLog {
        path: PathBuf,
    }

    impl EventLog {
        pub fn open(root: &Path) -> std::io::Result<Self> {
            let dir = ipc::dashboard_cache_dir(root);
            std::fs::create_dir_all(&dir)?;
            Ok(Self { path: ipc::event_log_path(root) })
        }

        pub fn append(&self, event: &ServerMsg) -> std::io::Result<()> {
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.path)?;
            let line = serde_json::to_string(event).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            f.write_all(line.as_bytes())?;
            f.write_all(b"\n")?;
            f.sync_data()?;
            Ok(())
        }

        pub fn replay(&self) -> std::io::Result<Vec<SessionInfo>> {
            use std::io::BufRead;
            let file = match std::fs::File::open(&self.path) { Ok(file) => file, Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()), Err(error) => return Err(error) };
            let mut sessions = BTreeMap::new();
            for line in std::io::BufReader::new(file).lines() {
                if let Ok(ServerMsg::SessionChanged { session }) = serde_json::from_str::<ServerMsg>(&line?) {
                    sessions.insert(session.session_id.clone(), session);
                    while sessions.len() > 128 {
                        let key = sessions.keys().next().expect("session").clone();
                        sessions.remove(&key);
                    }
                }
            }
            Ok(sessions.into_values().collect())
        }
    }

    /// 🧵 One live pseudo-terminal session. A host with a real pseudo-terminal — `openpty` on unix,
    /// ConPTY on windows — owns the master; every other target keeps the session surface and answers
    /// a spawn with an error instead.
    struct LiveSession {
        info: SessionInfo,
        output: VecDeque<u8>,
        stop_deadline: Option<std::time::Instant>,
        exit_deadline: Option<std::time::Instant>,
        #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
        pty: Option<ui_tui::tui::pty::Pty>,
    }

    /// 🧠 In-process supervisor: sessions + fan-out to attached client streams. `T` is the daemon's
    /// one duplex transport type per instance — R11: open set (the real `#[cfg(unix)]` accept loop
    /// stores `UnixStream`, the test suite's `DuplexEnd` mock stores an in-memory pipe, and neither
    /// is the other's only impl), so this de-dyns to a GENERIC parameter rather than
    /// `dyn_enum_close!` — a hand-written enum would need a `#[cfg(test)]`-only variant, which the
    /// macro's DSL cannot express (see `📓️terra-dedyn-fw-hub-repo-report.md`). Only `Write + Send`
    /// is required: `Supervisor` writes to attached clients while the daemon's bounded nonblocking
    /// connection cursors own the read halves.
    pub struct Supervisor<T: Write + Send + 'static> {
        sessions: BTreeMap<String, LiveSession>,
        clients: Vec<(u64, std::sync::mpsc::SyncSender<Vec<u8>>)>,
        next_client_id: u64,
        event_log: EventLog,
        root: PathBuf,
        shutdown: bool,
        _transport: std::marker::PhantomData<T>,
    }

    impl<T: Write + Send + 'static> Supervisor<T> {
        pub fn new(root: &Path) -> std::io::Result<Self> {
            let event_log = EventLog::open(root)?;
            let mut sessions = BTreeMap::new();
            for mut info in event_log.replay()? {
                if matches!(info.status, SessionStatus::Running | SessionStatus::Stopping) {
                    info.status = SessionStatus::Exited;
                    info.code = Some(-1);
                    event_log.append(&ServerMsg::SessionChanged { session: info.clone() })?;
                }
                sessions.insert(info.session_id.clone(), LiveSession { info, output: VecDeque::new(), stop_deadline: None, exit_deadline: None, #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))] pty: None });
            }
            Ok(Self { sessions, clients: Vec::new(), next_client_id: 1, event_log, root: root.to_path_buf(), shutdown: false, _transport: std::marker::PhantomData })
        }

        fn broadcast_control(&mut self, msg: &ServerMsg) -> std::io::Result<()> {
            if matches!(msg, ServerMsg::SessionChanged { .. } | ServerMsg::Shutdown {}) { self.event_log.append(msg)?; }
            let mut bytes = Vec::new();
            if ipc::write_control(&mut bytes, msg).is_ok() { self.broadcast(bytes); }
            Ok(())
        }

        fn broadcast_output(&mut self, session_id: &str, data: &[u8]) {
            if let Some(session) = self.sessions.get_mut(session_id) {
                session.output.extend(data);
                let excess = session.output.len().saturating_sub(64 * 1024);
                session.output.drain(..excess);
            }
            let payload = ipc::encode_output(session_id, data);
            let mut bytes = Vec::new();
            if ipc::write_frame(&mut bytes, ipc::KIND_OUTPUT, &payload).is_ok() { self.broadcast(bytes); }
        }

        fn broadcast(&mut self, bytes: Vec<u8>) {
            self.clients.retain(|(_, sender)| sender.try_send(bytes.clone()).is_ok());
        }

        fn send_control(&mut self, id: u64, msg: &ServerMsg) -> std::io::Result<()> {
            let mut bytes = Vec::new();
            ipc::write_control(&mut bytes, msg)?;
            self.send(id, bytes)
        }

        fn send(&mut self, id: u64, bytes: Vec<u8>) -> std::io::Result<()> {
            let sender = self.clients.iter().find(|(candidate, _)| *candidate == id).map(|(_, sender)| sender);
            sender.ok_or_else(|| std::io::Error::other("dashboard client disconnected"))?.try_send(bytes).map_err(|_| std::io::Error::other("dashboard client output backlog exceeded"))
        }

        pub fn attach_client(&mut self, mut stream: T) -> std::io::Result<u64> {
            if self.clients.len() >= 16 { return Err(std::io::Error::other("workspace dashboard view limit reached")); }
            ipc::write_control(&mut stream, &ServerMsg::Attached { daemon_pid: std::process::id() })?;
            let id = self.next_client_id;
            self.next_client_id = self.next_client_id.checked_add(1).ok_or_else(|| std::io::Error::other("dashboard client id space exhausted"))?;
            let (sender, receiver) = std::sync::mpsc::sync_channel::<Vec<u8>>(4096);
            std::thread::spawn(move || {
                while let Ok(bytes) = receiver.recv() {
                    if stream.write_all(&bytes).and_then(|_| stream.flush()).is_err() { break; }
                }
            });
            self.clients.push((id, sender));
            Ok(id)
        }

        pub fn snapshot(&self) -> Vec<SessionInfo> { self.sessions.values().map(|session| session.info.clone()).collect() }

        pub fn output(&self, session_id: &str) -> Option<Vec<u8>> { self.sessions.get(session_id).map(|session| session.output.iter().copied().collect()) }

        pub fn is_shutdown(&self) -> bool { self.shutdown }

        pub fn handle_for(&mut self, id: u64, msg: ClientMsg) -> std::io::Result<()> {
            match msg {
                ClientMsg::Attach { .. } => {
                    self.send_control(id, &ServerMsg::Sessions { sessions: self.snapshot() })?;
                    let replay: Vec<_> = self.sessions.iter().map(|(key, session)| (key.clone(), session.output.iter().copied().collect::<Vec<_>>())).collect();
                    for (key, output) in replay {
                        for data in output.chunks(4096) {
                            let mut bytes = Vec::new();
                            ipc::write_frame(&mut bytes, ipc::KIND_OUTPUT, &ipc::encode_output(&key, data))?;
                            self.send(id, bytes)?;
                        }
                    }
                    self.send_control(id, &ServerMsg::ReplayComplete {})
                }
                ClientMsg::List {} => self.send_control(id, &ServerMsg::Sessions { sessions: self.snapshot() }),
                ClientMsg::Detach {} => { self.detach_client(id); Ok(()) }
                ClientMsg::Ping {} => self.send_control(id, &ServerMsg::Pong {}),
                message => {
                    if let Err(error) = self.handle_client_msg(message) {
                        self.send_control(id, &ServerMsg::Error { message: error.to_string() })?;
                    }
                    Ok(())
                }
            }
        }

        #[cfg(any(unix, windows))]
        fn has_client(&self, id: u64) -> bool {
            self.clients.iter().any(|(candidate, _)| *candidate == id)
        }

        fn detach_client(&mut self, id: u64) {
            if let Some(index) = self.clients.iter().position(|(candidate, _)| *candidate == id) {
                self.clients.swap_remove(index);
            }
        }

        pub fn handle_client_msg(&mut self, msg: ClientMsg) -> std::io::Result<()> {
            msg.validate()?;
            match msg {
                ClientMsg::Attach { .. } => Ok(()),
                ClientMsg::Detach {} => Ok(()),
                ClientMsg::Ping {} => {
                    self.broadcast_control(&ServerMsg::Pong {})?;
                    Ok(())
                }
                ClientMsg::Spawn { session_id, command } => self.spawn_session(&session_id, command),
                ClientMsg::Input { session_id, data } => self.input(&session_id, &data),
                ClientMsg::Resize { session_id, cols, rows } => self.resize(&session_id, cols, rows),
                ClientMsg::Kill { session_id } => {
                    self.kill(&session_id)
                }
                ClientMsg::Stop { session_id } => self.stop_session(&session_id),
                ClientMsg::Restart { session_id } => {
                    let command = self.sessions.get(&session_id).ok_or_else(|| std::io::Error::other("unknown session"))?.info.command.clone();
                    self.kill(&session_id)?;
                    self.spawn_session(&session_id, command)
                }
                ClientMsg::List {} => { self.broadcast_control(&ServerMsg::Sessions { sessions: self.snapshot() })?; Ok(()) }
                ClientMsg::Shutdown {} => {
                    let ids: Vec<_> = self.sessions.keys().cloned().collect();
                    for id in ids { self.kill(&id)?; }
                    self.shutdown = true;
                    self.broadcast_control(&ServerMsg::Shutdown {})?;
                    Ok(())
                }
            }
        }

        fn spawn_session(&mut self, session_id: &str, mut command: SessionCommand) -> std::io::Result<()> {
            let cwd = self.root.join(&command.cwd);
            command.cwd = ipc::canonical_path(&cwd).display().to_string();
            if self.sessions.values().any(|session| matches!(session.info.status, SessionStatus::Running | SessionStatus::Stopping)
                && (session.info.session_id == session_id || session.info.command.same_task(&command))) {
                return Err(std::io::Error::other("task is already running; select its session or restart it"));
            }
            if !self.sessions.contains_key(session_id) && self.sessions.len() >= 128 {
                let evict = self.sessions.iter().find(|(_, session)| matches!(session.info.status, SessionStatus::Exited | SessionStatus::Failed)).map(|(id, _)| id.clone());
                if let Some(id) = evict { self.sessions.remove(&id); } else { return Err(std::io::Error::other("workspace session limit reached")); }
            }
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                use ui_tui::tui::pty::{Pty, PtySize};
                let args: Vec<_> = command.args.iter().map(String::as_str).collect();
                let env: Vec<_> = command.env.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();
                let cwd = if command.cwd.is_empty() { self.root.clone() } else { PathBuf::from(&command.cwd) };
                let spawned = Pty::spawn(&command.cmd, &args, &env, &["NX_INVOCATION_ROOT_PID"], Some(&cwd), PtySize { cols: command.cols, rows: command.rows });
                let (pty, error) = match spawned { Ok(pty) => (Some(pty), None), Err(error) => (None, Some(std::io::Error::other(error.message))) };
                let info = SessionInfo { session_id: session_id.into(), command, status: if error.is_some() { SessionStatus::Failed } else { SessionStatus::Running }, pid: pty.as_ref().map(Pty::pid), code: error.as_ref().map(|_| -1) };
                self.sessions.insert(session_id.to_string(), LiveSession { info: info.clone(), output: VecDeque::new(), stop_deadline: None, exit_deadline: None, pty });
                self.broadcast_control(&ServerMsg::SessionChanged { session: info })?;
                if let Some(error) = error { Err(error) } else { Ok(()) }
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = (session_id, command);
                Err(std::io::Error::other("PTY supervisor requires a unix or windows tui-terminal host"))
            }
        }

        fn input(&mut self, session_id: &str, data: &[u8]) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                let pty = self.sessions.get_mut(session_id).and_then(|session| session.pty.as_mut()).ok_or_else(|| std::io::Error::other("session is not running"))?;
                pty.write_all(data).map_err(|e| std::io::Error::other(e.message))?;
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = (session_id, data);
            }
            Ok(())
        }

        fn resize(&mut self, session_id: &str, cols: u16, rows: u16) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                use ui_tui::tui::pty::PtySize;
                if let Some(session) = self.sessions.get_mut(session_id) {
                    session.info.command.cols = cols;
                    session.info.command.rows = rows;
                    if let Some(pty) = session.pty.as_mut() { pty.resize(PtySize { cols, rows }).map_err(|e| std::io::Error::other(e.message))?; }
                }
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = (session_id, cols, rows);
            }
            Ok(())
        }

        fn stop_session(&mut self, session_id: &str) -> std::io::Result<()> {
            self.input(session_id, &[3])?;
            let session = self.sessions.get_mut(session_id).ok_or_else(|| std::io::Error::other("unknown session"))?;
            session.info.status = SessionStatus::Stopping;
            session.stop_deadline = Some(std::time::Instant::now() + std::time::Duration::from_secs(2));
            let info = session.info.clone();
            self.broadcast_control(&ServerMsg::SessionChanged { session: info })?;
            Ok(())
        }

        fn kill(&mut self, session_id: &str) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                let session = self.sessions.get_mut(session_id).ok_or_else(|| std::io::Error::other("unknown session"))?;
                let Some(pty) = session.pty.as_mut() else { return Ok(()) };
                pty.terminate().map_err(|e| std::io::Error::other(e.message))?;
                session.pty.take();
                session.stop_deadline = None;
                session.info.status = SessionStatus::Exited;
                session.info.code = Some(-1);
                let info = session.info.clone();
                self.broadcast_control(&ServerMsg::SessionChanged { session: info })?;
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = session_id;
            }
            Ok(())
        }

        /// ⏱️ Polls PTY masters and reaps exited children.
        pub fn tick(&mut self) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                let mut buf = [0u8; 4096];
                let ids: Vec<String> = self.sessions.keys().cloned().collect();
                let mut outputs: Vec<(String, Vec<u8>)> = Vec::new();
                let mut exited = Vec::new();
                for id in ids {
                    let Some(session) = self.sessions.get_mut(&id) else { continue };
                    let Some(pty) = session.pty.as_mut() else { continue };
                    let code = pty.try_wait().ok().flatten();
                    for _ in 0..16 {
                        match pty.try_read(&mut buf) { Ok(0) | Err(_) => break, Ok(n) => outputs.push((id.clone(), buf[..n].to_vec())) }
                    }
                    if let Some(code) = code {
                        let deadline = session.exit_deadline.get_or_insert_with(|| std::time::Instant::now() + std::time::Duration::from_millis(50));
                        if std::time::Instant::now() >= *deadline { exited.push((id, code)); }
                    } else if session.stop_deadline.is_some_and(|deadline| std::time::Instant::now() >= deadline) {
                        exited.push((id, -1));
                    }
                }
                for (id, data) in outputs {
                    self.broadcast_output(&id, &data);
                }
                for (id, code) in exited {
                    let session = self.sessions.get_mut(&id).expect("session");
                    if let Some(mut pty) = session.pty.take() { let _ = pty.terminate(); }
                    session.info.status = SessionStatus::Exited;
                    session.info.code = Some(code);
                    session.stop_deadline = None;
                    let info = session.info.clone();
                    self.broadcast_control(&ServerMsg::SessionChanged { session: info })?;
                }
            }
            Ok(())
        }
    }

    /// 🔒 The operating system releases this workspace lease when the daemon exits.
    struct WorkspaceLease { _file: std::fs::File, root: PathBuf }

    impl WorkspaceLease {
        fn acquire(root: &Path) -> std::io::Result<Self> {
            std::fs::create_dir_all(ipc::dashboard_cache_dir(root))?;
            let file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(ipc::lock_path(root))?;
            file.try_lock().map_err(|_| std::io::Error::new(std::io::ErrorKind::AlreadyExists, "workspace dashboard daemon is already running"))?;
            Ok(Self { _file: file, root: root.to_path_buf() })
        }
    }

    impl Drop for WorkspaceLease {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(ipc::pid_path(&self.root));
            let _ = std::fs::remove_file(ipc::socket_path(&self.root));
        }
    }

    /// 🏷️ Writes pid file for `semio daemon status`.
    pub fn write_pid(root: &Path) -> std::io::Result<()> {
        let dir = ipc::dashboard_cache_dir(root);
        std::fs::create_dir_all(&dir)?;
        std::fs::write(ipc::pid_path(root), format!("{}\n", std::process::id()))
    }

    pub fn read_pid(root: &Path) -> Option<u32> {
        let text = std::fs::read_to_string(ipc::pid_path(root)).ok()?;
        text.trim().parse().ok()
    }

    pub fn status(root: &Path) -> String {
        match super::client::Connection::connect(root) {
            Ok(mut connection) => {
                let _ = connection.send(&ClientMsg::List {});
                let messages = connection.receive(std::time::Duration::from_secs(2)).unwrap_or_default();
                let sessions = messages.iter().find_map(|message| match message { super::client::Message::Control(ServerMsg::Sessions { sessions }) => Some(sessions), _ => None });
                let count = sessions.map_or(0, |sessions| sessions.iter().filter(|session| matches!(session.status, SessionStatus::Running | SessionStatus::Stopping)).count());
                format!("daemon pid {} · {count} active tasks · {}\n", connection.daemon_pid(), ipc::socket_path(root).display())
            }
            Err(_) => "daemon not running\n".into(),
        }
    }

    pub fn stop(root: &Path) -> i32 {
        match super::client::Connection::connect(root).and_then(|mut connection| {
            connection.send(&ClientMsg::Shutdown {})?;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            while std::time::Instant::now() < deadline {
                if connection.receive(std::time::Duration::from_millis(100))?.iter().any(|message| matches!(message, super::client::Message::Control(ServerMsg::Shutdown {}))) { return Ok(()); }
            }
            Err(std::io::Error::other("daemon shutdown timed out"))
        }) {
            Ok(()) => { println!("stopped workspace dashboard daemon"); 0 }
            Err(error) => { eprintln!("daemon shutdown failed: {error}"); 1 }
        }
    }

    /// 🌀 Blocking daemon serve loop (unix domain socket).
    #[cfg(unix)]
    pub fn serve(root: &Path, running: Arc<AtomicBool>) -> std::io::Result<()> {
        let _lease = WorkspaceLease::acquire(root)?;
        write_pid(root)?;
        let listener = ipc::listen(root)?;
        listener.set_nonblocking(true)?;
        use std::sync::atomic::Ordering;
        use std::time::Duration;
        let mut supervisor = Supervisor::new(root)?;
        let mut readers = Vec::new();
        while running.load(Ordering::SeqCst) && !supervisor.is_shutdown() {
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(true)?;
                    let stream_for_client = stream.try_clone()?;
                    if let Ok(client_id) = supervisor.attach_client(stream_for_client) {
                        readers.push(ClientReader::new(client_id, stream));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
            let mut messages = Vec::new();
            let mut index = 0usize;
            while index < readers.len() {
                let client_id = readers[index].id;
                let keep = supervisor.has_client(client_id) && matches!(readers[index].turn(&mut messages), Ok(true));
                if !keep {
                    readers.swap_remove(index);
                    supervisor.detach_client(client_id);
                } else {
                    index += 1;
                }
            }
            for (client_id, message) in messages {
                if supervisor.handle_for(client_id, message).is_err() { supervisor.detach_client(client_id); }
            }
            supervisor.tick()?;
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = std::fs::remove_file(ipc::pid_path(root));
        let _ = std::fs::remove_file(ipc::socket_path(root));
        Ok(())
    }

    /// 🌀 Blocking daemon serve loop (windows named pipe).
    ///
    /// A named pipe has no non-blocking accept worth having — `PIPE_NOWAIT` is a compatibility relic —
    /// so only the wait for the next peer lives on its own thread, and each accepted instance is handed
    /// to the same single-threaded turn the unix loop takes: poll every client, apply what they said,
    /// tick the sessions.
    #[cfg(windows)]
    pub fn serve(root: &Path, running: Arc<AtomicBool>) -> std::io::Result<()> {
        use std::sync::atomic::Ordering;
        use std::sync::mpsc;
        use std::time::Duration;
        let _lease = WorkspaceLease::acquire(root)?;
        write_pid(root)?;
        let name = ipc::listen(root)?;
        let (sender, receiver) = mpsc::channel::<std::fs::File>();
        let serving = Arc::clone(&running);
        let acceptor = std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match ipc::accept(&name) {
                    Ok(stream) => {
                        if sender.send(stream).is_err() {
                            return;
                        }
                    }
                    Err(_) => std::thread::sleep(Duration::from_millis(50)),
                }
            }
        });
        let mut supervisor = Supervisor::new(root)?;
        let mut readers = Vec::new();
        while serving.load(Ordering::SeqCst) && !supervisor.is_shutdown() {
            while let Ok(stream) = receiver.try_recv() {
                let reader = stream.try_clone()?;
                if let Ok(client_id) = supervisor.attach_client(stream) {
                    readers.push(ClientReader::new(client_id, reader));
                }
            }
            let mut messages = Vec::new();
            let mut index = 0usize;
            while index < readers.len() {
                let client_id = readers[index].id;
                let keep = supervisor.has_client(client_id) && matches!(readers[index].turn(&mut messages), Ok(true));
                if !keep {
                    readers.swap_remove(index);
                    supervisor.detach_client(client_id);
                } else {
                    index += 1;
                }
            }
            for (client_id, message) in messages {
                if supervisor.handle_for(client_id, message).is_err() { supervisor.detach_client(client_id); }
            }
            supervisor.tick()?;
            std::thread::sleep(Duration::from_millis(10));
        }
        serving.store(false, Ordering::SeqCst);
        let _ = ipc::connect(root);
        let _ = acceptor.join();
        let _ = std::fs::remove_file(ipc::pid_path(root));
        let _ = std::fs::remove_file(ipc::socket_path(root));
        Ok(())
    }

    #[cfg(not(any(unix, windows)))]
    pub fn serve(root: &Path, _running: Arc<AtomicBool>) -> std::io::Result<()> {
        let _ = root;
        Err(std::io::Error::other("dashboard daemon listen needs a unix socket or a windows named pipe"))
    }

    /// 🚀 Starts a detached daemon process (`semio daemon serve --root …`).
    pub fn start_detached(root: &Path, exe: &Path) -> i32 {
        if super::client::Connection::connect(root).is_ok() {
            println!("{}", status(root));
            return 0;
        }
        let spawned = ui_tui::tui::pty::spawn_detached(&exe.to_string_lossy(), &["daemon", "serve", "--root", &root.display().to_string()], &["NX_INVOCATION_ROOT_PID"], root);
        match spawned {
            Ok(pid) => {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                while std::time::Instant::now() < deadline {
                    if let Ok(connection) = super::client::Connection::connect(root) {
                        println!("workspace dashboard daemon ready at pid {}", connection.daemon_pid());
                        return 0;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
                eprintln!("daemon {pid} did not become ready");
                1
            }
            Err(e) => {
                eprintln!("failed to start daemon: {e}");
                1
            }
        }
    }

    /// 🧪 Drive a supervisor against an in-memory duplex for unit tests.
    pub fn handle_one_for_test<T: Write + Send + 'static>(sup: &mut Supervisor<T>, msg: ClientMsg) -> std::io::Result<()> {
        sup.handle_client_msg(msg)
    }
}
// #endregion 🔖️Supervisor

// #region 🔖️Command
/// 🖥️ Controls the terminal dashboard daemon lifecycle and attachment.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let subcommand = parsed.segments.first().map_or("status", String::as_str);
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let root = ipc::canonical_path(&root);
    match subcommand {
        "start" => {
            let executable = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("semio"));
            supervisor::start_detached(&root, &executable)
        }
        "serve" => {
            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            match supervisor::serve(&root, running) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("daemon serve failed: {error}");
                    1
                }
            }
        }
        "stop" => supervisor::stop(&root),
        "status" => {
            print!("{}", supervisor::status(&root));
            0
        }
        "attach" => crate::terminal::run(&root),
        _ => {
            eprintln!("usage: semio daemon start|stop|status|attach|serve");
            1
        }
    }
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
