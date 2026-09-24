//! 🌀️ The dashboard daemon: the framed client/daemon transport, the pseudo-terminal session
//! supervisor that multiplexes it, and the `semio daemon start|serve|stop|status|attach` verb.

use crate::args::ParsedArgs;
use std::path::{Path, PathBuf};

// #region 🔖️Ipc
/// ✉️ The length-prefixed control/output framing the dashboard client and daemon speak.
pub mod ipc {
    use serde::{Deserialize, Serialize};
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    pub const KIND_CONTROL: u8 = 1;
    pub const KIND_OUTPUT: u8 = 2;
    pub const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

    /// 🎛️ Client → daemon control envelope (JSON).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum ClientMsg {
        Attach { client_id: String },
        Detach,
        Spawn { session_id: String, cmd: String, args: Vec<String>, cwd: Option<String>, cols: u16, rows: u16 },
        Input { session_id: String, data: Vec<u8> },
        Resize { session_id: String, cols: u16, rows: u16 },
        Kill { session_id: String },
        Ping,
    }

    /// 📣 Daemon → client control envelope (JSON).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum ServerMsg {
        Attached { daemon_pid: u32 },
        SessionStarted { session_id: String },
        SessionExited { session_id: String, code: i32 },
        Error { message: String },
        Pong,
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
            dashboard_cache_dir(root).join("daemon.sock")
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

    /// ✉️ Writes one length-prefixed frame: `u32 le len | u8 kind | payload`.
    pub fn write_frame(out: &mut impl Write, kind: u8, payload: &[u8]) -> std::io::Result<()> {
        let len = (1u32).saturating_add(payload.len() as u32);
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
        let dir = dashboard_cache_dir(root);
        std::fs::create_dir_all(&dir)?;
        let path = socket_path(root);
        let _ = std::fs::remove_file(&path);
        std::os::unix::net::UnixListener::bind(&path)
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
        // Named-pipe client open; retries briefly while the daemon starts.
        let name = pipe_name(root);
        for _ in 0..50 {
            match std::fs::OpenOptions::new().read(true).write(true).open(&name) {
                Ok(f) => return Ok(f),
                Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("named pipe not found: {name}")))
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
    use super::ipc::{self, ClientMsg, ServerMsg};
    use std::collections::HashMap;
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

        fn turn(&mut self, messages: &mut Vec<ClientMsg>) -> std::io::Result<bool> {
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
                        messages.push(message);
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

        fn turn(&mut self, messages: &mut Vec<ClientMsg>) -> std::io::Result<bool> {
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
                        messages.push(message);
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
            Ok(())
        }
    }

    /// 🧵 One live pseudo-terminal session. A host with a real pseudo-terminal — `openpty` on unix,
    /// ConPTY on windows — owns the master; every other target keeps the session surface and answers
    /// a spawn with an error instead.
    #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
    struct LiveSession {
        pty: ui_tui::tui::pty::Pty,
    }

    /// 🧵 The session placeholder of a host without a pseudo-terminal.
    #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
    struct LiveSession {
        _marker: (),
    }

    /// 🧠 In-process supervisor: sessions + fan-out to attached client streams. `T` is the daemon's
    /// one duplex transport type per instance — R11: open set (the real `#[cfg(unix)]` accept loop
    /// stores `UnixStream`, the test suite's `DuplexEnd` mock stores an in-memory pipe, and neither
    /// is the other's only impl), so this de-dyns to a GENERIC parameter rather than
    /// `dyn_enum_close!` — a hand-written enum would need a `#[cfg(test)]`-only variant, which the
    /// macro's DSL cannot express (see `📓️terra-dedyn-fw-hub-repo-report.md`). Only `Write + Send`
    /// is required: `Supervisor` writes to attached clients while the daemon's bounded nonblocking
    /// connection cursors own the read halves.
    pub struct Supervisor<T: Write + Send> {
        sessions: HashMap<String, LiveSession>,
        clients: Vec<(u64, T)>,
        next_client_id: u64,
        event_log: EventLog,
        _root: PathBuf,
    }

    impl<T: Write + Send> Supervisor<T> {
        pub fn new(root: &Path) -> std::io::Result<Self> {
            Ok(Self { sessions: HashMap::new(), clients: Vec::new(), next_client_id: 1, event_log: EventLog::open(root)?, _root: root.to_path_buf() })
        }

        fn broadcast_control(&mut self, msg: &ServerMsg) {
            let _ = self.event_log.append(msg);
            let mut dead = Vec::new();
            for (i, (_, client)) in self.clients.iter_mut().enumerate() {
                if ipc::write_control(client, msg).is_err() {
                    dead.push(i);
                }
            }
            for i in dead.into_iter().rev() {
                self.clients.swap_remove(i);
            }
        }

        fn broadcast_output(&mut self, session_id: &str, data: &[u8]) {
            let payload = ipc::encode_output(session_id, data);
            let mut dead = Vec::new();
            for (i, (_, client)) in self.clients.iter_mut().enumerate() {
                if ipc::write_frame(client, ipc::KIND_OUTPUT, &payload).is_err() {
                    dead.push(i);
                }
            }
            for i in dead.into_iter().rev() {
                self.clients.swap_remove(i);
            }
        }

        pub fn attach_client(&mut self, mut stream: T) -> std::io::Result<u64> {
            ipc::write_control(&mut stream, &ServerMsg::Attached { daemon_pid: std::process::id() })?;
            let id = self.next_client_id;
            self.next_client_id = self.next_client_id.checked_add(1).ok_or_else(|| std::io::Error::other("dashboard client id space exhausted"))?;
            self.clients.push((id, stream));
            Ok(id)
        }

        #[cfg(any(unix, windows))]
        fn has_client(&self, id: u64) -> bool {
            self.clients.iter().any(|(candidate, _)| *candidate == id)
        }

        #[cfg(any(unix, windows))]
        fn detach_client(&mut self, id: u64) {
            if let Some(index) = self.clients.iter().position(|(candidate, _)| *candidate == id) {
                self.clients.swap_remove(index);
            }
        }

        pub fn handle_client_msg(&mut self, msg: ClientMsg) -> std::io::Result<()> {
            match msg {
                ClientMsg::Attach { .. } => Ok(()),
                ClientMsg::Detach => Ok(()),
                ClientMsg::Ping => {
                    self.broadcast_control(&ServerMsg::Pong);
                    Ok(())
                }
                ClientMsg::Spawn { session_id, cmd, args, cwd, cols, rows } => self.spawn_session(&session_id, &cmd, &args, cwd.as_deref(), cols, rows),
                ClientMsg::Input { session_id, data } => self.input(&session_id, &data),
                ClientMsg::Resize { session_id, cols, rows } => self.resize(&session_id, cols, rows),
                ClientMsg::Kill { session_id } => {
                    self.kill(&session_id);
                    Ok(())
                }
            }
        }

        fn spawn_session(&mut self, session_id: &str, cmd: &str, args: &[String], cwd: Option<&str>, cols: u16, rows: u16) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                use ui_tui::tui::pty::{Pty, PtySize};
                let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
                let cwd_path = cwd.map(PathBuf::from);
                let pty = Pty::spawn(cmd, &arg_refs, &[], cwd_path.as_deref(), PtySize { cols: cols.max(1), rows: rows.max(1) }).map_err(|e| std::io::Error::other(e.message))?;
                self.sessions.insert(session_id.to_string(), LiveSession { pty });
                self.broadcast_control(&ServerMsg::SessionStarted { session_id: session_id.to_string() });
                Ok(())
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = (session_id, cmd, args, cwd, cols, rows);
                self.broadcast_control(&ServerMsg::Error { message: "PTY supervisor requires a unix or windows tui-terminal host".into() });
                Ok(())
            }
        }

        fn input(&mut self, session_id: &str, data: &[u8]) -> std::io::Result<()> {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                if let Some(session) = self.sessions.get_mut(session_id) {
                    session.pty.write_all(data).map_err(|e| std::io::Error::other(e.message))?;
                }
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
                    session.pty.resize(PtySize { cols: cols.max(1), rows: rows.max(1) }).map_err(|e| std::io::Error::other(e.message))?;
                }
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = (session_id, cols, rows);
            }
            Ok(())
        }

        fn kill(&mut self, session_id: &str) {
            #[cfg(any(all(unix, not(target_arch = "wasm32")), windows))]
            {
                if let Some(mut session) = self.sessions.remove(session_id) {
                    let _ = session.pty.kill();
                    self.broadcast_control(&ServerMsg::SessionExited { session_id: session_id.to_string(), code: -1 });
                }
            }
            #[cfg(not(any(all(unix, not(target_arch = "wasm32")), windows)))]
            {
                let _ = session_id;
            }
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
                    match session.pty.try_read(&mut buf) {
                        Ok(0) => {}
                        Ok(n) => outputs.push((id.clone(), buf[..n].to_vec())),
                        Err(_) => {}
                    }
                    if let Ok(Some(code)) = session.pty.try_wait() {
                        exited.push((id, code));
                    }
                }
                for (id, data) in outputs {
                    self.broadcast_output(&id, &data);
                }
                for (id, code) in exited {
                    self.sessions.remove(&id);
                    self.broadcast_control(&ServerMsg::SessionExited { session_id: id, code });
                }
            }
            Ok(())
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
        match read_pid(root) {
            Some(pid) => format!("daemon pid {pid} socket {}\n", ipc::socket_path(root).display()),
            None => "daemon not running\n".into(),
        }
    }

    pub fn stop(root: &Path) -> i32 {
        if let Some(pid) = read_pid(root) {
            #[cfg(unix)]
            {
                let _ = std::process::Command::new("kill").args(["-TERM", &pid.to_string()]).status();
            }
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill").args(["/PID", &pid.to_string(), "/T", "/F"]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
            }
            let _ = std::fs::remove_file(ipc::pid_path(root));
            let _ = std::fs::remove_file(ipc::socket_path(root));
            println!("stopped daemon {pid}");
            0
        } else {
            eprintln!("daemon not running");
            1
        }
    }

    /// 🌀 Blocking daemon serve loop (unix domain socket).
    #[cfg(unix)]
    pub fn serve(root: &Path, running: Arc<AtomicBool>) -> std::io::Result<()> {
        write_pid(root)?;
        let listener = ipc::listen(root)?;
        listener.set_nonblocking(true)?;
        use std::sync::atomic::Ordering;
        use std::time::Duration;
        let mut supervisor = Supervisor::new(root)?;
        let mut readers = Vec::new();
        while running.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(true)?;
                    let stream_for_client = stream.try_clone()?;
                    let client_id = supervisor.attach_client(stream_for_client)?;
                    readers.push(ClientReader::new(client_id, stream));
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
            for message in messages {
                supervisor.handle_client_msg(message)?;
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
        write_pid(root)?;
        let name = ipc::listen(root)?;
        let (sender, receiver) = mpsc::channel::<std::fs::File>();
        let serving = Arc::clone(&running);
        std::thread::spawn(move || {
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
        while serving.load(Ordering::SeqCst) {
            while let Ok(stream) = receiver.try_recv() {
                let reader = stream.try_clone()?;
                let client_id = supervisor.attach_client(stream)?;
                readers.push(ClientReader::new(client_id, reader));
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
            for message in messages {
                supervisor.handle_client_msg(message)?;
            }
            supervisor.tick()?;
            std::thread::sleep(Duration::from_millis(10));
        }
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
        if read_pid(root).is_some() {
            println!("{}", status(root));
            return 0;
        }
        let mut cmd = std::process::Command::new(exe);
        cmd.args(["daemon", "serve", "--root", &root.display().to_string()]);
        cmd.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
        match cmd.spawn() {
            Ok(child) => {
                println!("started daemon pid {}", child.id());
                0
            }
            Err(e) => {
                eprintln!("failed to start daemon: {e}");
                1
            }
        }
    }

    /// 🧪 Drive a supervisor against an in-memory duplex for unit tests.
    pub fn handle_one_for_test<T: Write + Send>(sup: &mut Supervisor<T>, msg: ClientMsg) -> std::io::Result<()> {
        sup.handle_client_msg(msg)
    }
}
// #endregion 🔖️Supervisor

// #region 🔖️Command
/// 🖥️ Controls the terminal dashboard daemon lifecycle and attachment.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let subcommand = parsed.segments.first().map_or("status", String::as_str);
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
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
