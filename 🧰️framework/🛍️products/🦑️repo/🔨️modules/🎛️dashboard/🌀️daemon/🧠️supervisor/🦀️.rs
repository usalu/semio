//! 🧠️ The session supervisor of a workspace: one thread that owns every pseudo-terminal, every log and
//! every attached view. A turn waits for something to happen on any of them, applies what the views
//! asked, reads what the processes printed, reaps the ones that ended and writes to the views whatever
//! their connections take. Nothing in a turn waits for a view or for a process.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use super::clients::{Client, Inbound, Outputs};
use super::groups::{Group, Step};
use super::ipc::{self, code, ClientMsg, ErrorCode, GroupMember, GroupStop, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
use super::journal::{self, Journal, RETAINED_SESSIONS};
use super::replay::{self, SessionLog};
use super::transport::{Listener, Poller, Token, Waker};
use crate::terminal::labels::DashboardLabels;
use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use ui_tui::tui::pty::{Pty, PtyRead, PtySize, StopStage};

const MAX_CONNECTIONS: usize = 64;
const MAX_VIEWS: usize = 16;
const INPUT_LIMIT: usize = 4 * 1024 * 1024;
const INPUT_STALL: Duration = Duration::from_secs(10);
const TERMINATE_AFTER: Duration = Duration::from_secs(2);
const KILL_AFTER: Duration = Duration::from_secs(4);
const GIVE_UP_AFTER: Duration = Duration::from_secs(9);
const READY_SETTLE: Duration = Duration::from_millis(100);
const PROBE_FIRST: Duration = Duration::from_millis(50);
const PROBE_LAST: Duration = Duration::from_secs(1);
const PROBE_CONNECT: Duration = Duration::from_millis(100);
const TITLE_INTERVAL: Duration = Duration::from_millis(200);
const FLUSH_INTERVAL: Duration = Duration::from_millis(250);
const SHUTDOWN_GRACE: Duration = Duration::from_secs(3);
const READ_PAGE: usize = 64 * 1024;
const READS_PER_TURN: usize = 32;
const EXIT_QUIET: Duration = Duration::from_millis(250);
const FINISH_WAIT: Duration = if cfg!(windows) { Duration::from_secs(3) } else { Duration::ZERO };
const EXIT_LIMIT: Duration = Duration::from_secs(8);
const SIGKILL_EXIT: i32 = 137;
const DROPPED_ENVIRONMENT: [&str; 1] = ["NX_INVOCATION_ROOT_PID"];

/// 🚨 A request the daemon refuses, with the kind of failure a view can act on.
#[derive(Debug, Clone)]
pub struct Fail {
    pub code: ErrorCode,
    pub message: String,
    pub session_id: Option<String>,
}

type Outcome<T = ()> = Result<T, Fail>;

fn fail<T>(code: ErrorCode, message: impl Into<String>) -> Outcome<T> {
    Err(Fail { code, message: message.into(), session_id: None })
}

fn failed_for<T>(code: ErrorCode, message: impl Into<String>, session_id: &str) -> Outcome<T> {
    Err(Fail { code, message: message.into(), session_id: Some(session_id.to_string()) })
}

/// 🧵 One session: its projection, its output and the process that produces it while there is one.
pub struct Live {
    pub info: SessionInfo,
    pub log: SessionLog,
    pty: Option<Pty>,
    input: VecDeque<u8>,
    input_owner: Option<u64>,
    input_progress: Instant,
    stopping: Option<Instant>,
    stage: Option<StopStage>,
    forced: Option<i32>,
    ended: Option<(i32, Instant)>,
    closed: bool,
    finishing: Option<Instant>,
    last_output: Instant,
    title_due: Option<Instant>,
    candidate: Option<String>,
    probe_due: Instant,
    probe_delay: Duration,
    announced_wait: bool,
    flushed: Instant,
    created: u64,
}

impl Live {
    fn new(info: SessionInfo, log: SessionLog) -> Self {
        let created = info.started_ms.max(1);
        Self { info, log, pty: None, input: VecDeque::new(), input_owner: None, input_progress: Instant::now(), stopping: None, stage: None, forced: None, ended: None, closed: false, finishing: None, last_output: Instant::now(), title_due: None, candidate: None, probe_due: Instant::now(), probe_delay: PROBE_FIRST, announced_wait: false, flushed: Instant::now(), created }
    }
}

impl Live {
    /// 🟢️ An address the output showed is only a candidate: the session is ready once something listens on its port.
    fn offer(&mut self, url: String) {
        self.candidate = Some(url);
        self.probe_due = Instant::now();
        self.probe_delay = PROBE_FIRST;
    }
}

/// 🔌 Whether something accepts connections on the loopback port, on either address family.
fn listening(port: u16) -> bool {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
    port != 0 && [IpAddr::V4(Ipv4Addr::LOCALHOST), IpAddr::V6(Ipv6Addr::LOCALHOST)].into_iter().any(|address| TcpStream::connect_timeout(&SocketAddr::new(address, port), PROBE_CONNECT).is_ok())
}

impl Outputs for BTreeMap<String, Live> {
    fn log(&mut self, session_id: &str) -> Option<&mut SessionLog> { self.get_mut(session_id).map(|live| &mut live.log) }

    fn infos(&self) -> Vec<SessionInfo> {
        let mut lives: Vec<&Live> = self.values().collect();
        lives.sort_by(|left, right| (left.created, &left.info.session_id).cmp(&(right.created, &right.info.session_id)));
        lives.into_iter().map(|live| live.info.clone()).collect()
    }
}

fn set_environment(env: &mut Vec<(String, String)>, key: &str, value: &str) {
    let same = |left: &str| if cfg!(windows) { left.eq_ignore_ascii_case(key) } else { left == key };
    match env.iter_mut().find(|(name, _)| same(name)) {
        Some(pair) => *pair = (key.to_string(), value.to_string()),
        None => env.push((key.to_string(), value.to_string())),
    }
}

/// 🌱 The environment of a task: the requesting client's, the command's own on top, and the terminal
/// the daemon provides unless the command states another.
pub fn child_environment(client: &[(String, String)], declared: &[(String, String)]) -> Vec<(String, String)> {
    let mut env = Vec::new();
    for (key, value) in client.iter().filter(|(key, _)| !DROPPED_ENVIRONMENT.contains(&key.as_str())) { set_environment(&mut env, key, value); }
    for (key, value) in declared { set_environment(&mut env, key, value); }
    for (key, value) in [("TERM", "xterm-256color"), ("COLORTERM", "truecolor")] {
        if !declared.iter().any(|(name, _)| name == key) { set_environment(&mut env, key, value); }
    }
    env
}

/// 🕵️ The supervisor.
pub struct Supervisor {
    root: PathBuf,
    log_dir: PathBuf,
    journal: Journal,
    sessions: BTreeMap<String, Live>,
    clients: BTreeMap<u64, Client>,
    groups: BTreeMap<String, Group>,
    listener: Option<Listener>,
    poller: Poller,
    waker: Waker,
    next_client: u64,
    shutdown_at: Option<Instant>,
    shutdown: bool,
    behind: bool,
}

impl Supervisor {
    /// 🆕 Restores the sessions of the workspace's journal, the ones that were alive as interrupted, and
    /// serves the views that connect to `listener`.
    pub fn new(root: &Path, listener: Option<Listener>) -> std::io::Result<Self> {
        let (mut journal, restored) = Journal::open(root)?;
        let log_dir = ipc::log_dir(root);
        std::fs::create_dir_all(&log_dir)?;
        let mut sessions = BTreeMap::new();
        let now = ipc::now_ms();
        for mut info in restored {
            if info.status.live() {
                journal::interrupt(&mut info, Some(now));
                journal.append(&ServerMsg::SessionChanged { session: Box::new(info.clone()) })?;
            }
            let log = SessionLog::restore(&log_dir, &info.session_id);
            sessions.insert(info.session_id.clone(), Live::new(info, log));
        }
        replay::sweep_logs(&log_dir, &sessions.keys().map(|id| replay::log_stem(id)).collect());
        let poller = Poller::new()?;
        let waker = poller.waker();
        Ok(Self { root: root.to_path_buf(), log_dir, journal, sessions, clients: BTreeMap::new(), groups: BTreeMap::new(), listener, poller, waker, next_client: 1, shutdown_at: None, shutdown: false, behind: false })
    }

    /// 📋 The sessions in the order they were created.
    pub fn snapshot(&self) -> Vec<SessionInfo> { self.sessions.infos() }

    pub fn is_shutdown(&self) -> bool { self.shutdown }

    /// 🛑 Ends every task and then the daemon, as a view's shutdown request does.
    pub fn shut_down(&mut self) { self.begin_shutdown(); }

    /// 📍 Where the views connect.
    pub fn endpoint(&self) -> Option<String> { self.listener.as_ref().map(Listener::endpoint) }

    /// 🔁 Runs one turn: waits for activity for at most `limit`, then serves everything that is ready.
    pub fn turn(&mut self, limit: Duration) -> std::io::Result<()> {
        let timeout = if self.behind { Duration::ZERO } else { self.next_timeout().min(limit) };
        self.poller.begin();
        if let Some(listener) = &self.listener { self.poller.listener(listener); }
        for (id, client) in &self.clients { self.poller.stream(Token::Client(*id), &client.stream, true, client.wants_write()); }
        for (index, live) in self.sessions.values().enumerate() {
            if let Some(pty) = &live.pty { self.poller.pty(Token::Session(index as u64), pty, true, !live.input.is_empty()); }
        }
        self.poller.wait(timeout)?;
        self.accept();
        self.read_clients();
        self.read_sessions();
        self.maintain();
        self.advance_groups();
        self.write_clients();
        Ok(())
    }

    /// ⏳ How long a turn may wait: sooner while a process is alive, because its end is not announced.
    fn next_timeout(&self) -> Duration {
        let mut timeout = Duration::from_secs(1);
        for live in self.sessions.values() {
            if live.pty.is_some() { timeout = timeout.min(Duration::from_millis(100)); }
            if live.candidate.is_some() { timeout = timeout.min(live.probe_due.saturating_duration_since(Instant::now()).max(Duration::from_millis(5))); }
            if !live.input.is_empty() || live.stopping.is_some() || live.ended.is_some() { timeout = timeout.min(Duration::from_millis(20)); }
        }
        if self.shutdown_at.is_some() { timeout = timeout.min(Duration::from_millis(20)); }
        timeout
    }

    /// 🏁 Writes what is left to the views for at most `within`, as a daemon about to end does.
    pub fn drain(&mut self, within: Duration) {
        let deadline = Instant::now() + within;
        while Instant::now() < deadline && self.clients.values().any(|client| !client.flushed()) {
            self.write_clients();
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn accept(&mut self) {
        loop {
            let Some(listener) = self.listener.as_mut() else { return };
            let stream = match listener.accept() { Ok(Some(stream)) => stream, _ => return };
            self.waker.watch_stream(&stream);
            let id = self.next_client;
            self.next_client += 1;
            let mut client = Client::new(id, stream);
            client.tell(&ServerMsg::Attached { daemon_pid: std::process::id(), protocol: ipc::PROTOCOL, build_id: ipc::build_id().to_string() });
            if self.clients.len() >= MAX_CONNECTIONS {
                client.tell(&ServerMsg::error(code::VIEW_LIMIT, "workspace dashboard connection limit reached", None));
                client.closing = true;
            }
            self.clients.insert(id, client);
        }
    }

    fn read_clients(&mut self) {
        let ids: Vec<u64> = self.clients.keys().copied().collect();
        for id in ids {
            let Some(client) = self.clients.get_mut(&id) else { continue };
            if client.closing { continue; }
            let (messages, open) = client.receive();
            for message in messages {
                if !self.clients.contains_key(&id) { break; }
                match message {
                    Inbound::Control(message) => self.handle(id, message),
                    Inbound::Input(session_id, data) => self.deliver(id, ClientMsg::Input { session_id, data }),
                    Inbound::Rejected(kind, message) => self.reply(id, Fail { code: kind, message, session_id: None }),
                }
            }
            if !open { self.detach(id); }
        }
    }

    fn write_clients(&mut self) {
        let mut behind = false;
        let ids: Vec<u64> = self.clients.keys().copied().collect();
        for id in ids {
            let Some(client) = self.clients.get_mut(&id) else { continue };
            match client.pump(&mut self.sessions) {
                Ok(more) => behind |= more,
                Err(_) => { self.detach(id); continue; }
            }
            if client.closing && client.flushed() { self.detach(id); }
        }
        self.behind = behind;
    }

    fn detach(&mut self, id: u64) {
        self.clients.remove(&id);
    }

    fn reply(&mut self, id: u64, failure: Fail) {
        if let Some(client) = self.clients.get_mut(&id) { client.tell(&ServerMsg::error(failure.code, failure.message, failure.session_id.as_deref())); }
    }

    fn broadcast(&mut self, message: &ServerMsg) {
        let Ok(frame) = ipc::control_frame(message) else { return };
        for client in self.clients.values_mut() { client.push(frame.clone()); }
    }

    fn handle(&mut self, id: u64, message: ClientMsg) {
        if let Err(error) = message.validate() { return self.reply(id, Fail { code: code::INVALID, message: error.to_string(), session_id: None }); }
        let outcome = match message {
            ClientMsg::Hello { protocol, env, .. } => self.hello(id, protocol, env),
            ClientMsg::Attach { .. } => { self.attach(id); Ok(()) }
            ClientMsg::Watch {} => { self.watch(id); Ok(()) }
            ClientMsg::Subscribe { session_id } => self.subscribe(id, &session_id),
            ClientMsg::Unsubscribe { session_id } => { if let Some(client) = self.clients.get_mut(&id) { client.unfollow(&session_id); } Ok(()) }
            ClientMsg::Detach {} => { self.detach(id); Ok(()) }
            ClientMsg::List {} => { let infos = self.sessions.infos(); if let Some(client) = self.clients.get_mut(&id) { client.list(infos); } Ok(()) }
            ClientMsg::Ping {} => { if let Some(client) = self.clients.get_mut(&id) { client.tell(&ServerMsg::Pong {}); } Ok(()) }
            ClientMsg::Shutdown {} => { self.begin_shutdown(); Ok(()) }
            other => self.deliver_checked(id, other),
        };
        if let Err(failure) = outcome { self.reply(id, failure); }
    }

    fn deliver(&mut self, id: u64, message: ClientMsg) {
        if let Err(failure) = self.deliver_checked(id, message) { self.reply(id, failure); }
    }

    fn deliver_checked(&mut self, id: u64, message: ClientMsg) -> Outcome {
        match message {
            ClientMsg::Spawn { session_id, command } => { let env = self.environment_of(id)?; self.start(&session_id, *command, &env, Some(id)) }
            ClientMsg::SpawnGroup { group_id, stop, members, requires } => self.spawn_group(id, &group_id, stop, members, requires),
            ClientMsg::Input { session_id, data } => self.input(id, &session_id, &data),
            ClientMsg::Resize { session_id, cols, rows } => self.resize(&session_id, cols, rows),
            ClientMsg::Stop { session_id } => self.stop(&session_id),
            ClientMsg::Kill { session_id } => self.kill(&session_id),
            ClientMsg::Restart { session_id } => { let env = self.environment_of(id)?; self.restart(&session_id, &env, id) }
            ClientMsg::StopGroup { group_id } => self.end_group(&group_id, false),
            ClientMsg::KillGroup { group_id } => self.end_group(&group_id, true),
            ClientMsg::Forget { session_id } => self.forget(&session_id),
            _ => fail(code::INVALID, "message is not a request for this connection"),
        }
    }

    fn environment_of(&self, id: u64) -> Outcome<Vec<(String, String)>> {
        self.clients.get(&id).and_then(|client| client.environment.clone()).map_or_else(|| fail(code::HELLO_REQUIRED, "a task starts in the environment of the client that asks for it; send hello first"), Ok)
    }

    fn hello(&mut self, id: u64, protocol: u32, env: Vec<(String, String)>) -> Outcome {
        let Some(client) = self.clients.get_mut(&id) else { return Ok(()) };
        if protocol != ipc::PROTOCOL {
            client.closing = true;
            return fail(code::PROTOCOL, format!("daemon speaks protocol {}, the client speaks {protocol}", ipc::PROTOCOL));
        }
        client.environment = Some(env);
        Ok(())
    }

    fn watch(&mut self, id: u64) {
        let infos = self.sessions.infos();
        if let Some(client) = self.clients.get_mut(&id) {
            client.watching = true;
            client.list(infos);
        }
    }

    fn attach(&mut self, id: u64) {
        if self.clients.values().filter(|client| client.attached && client.id != id).count() >= MAX_VIEWS {
            return self.reply(id, Fail { code: code::VIEW_LIMIT, message: format!("workspace dashboard view limit of {MAX_VIEWS} attached views reached"), session_id: None });
        }
        let infos = self.sessions.infos();
        let Some(client) = self.clients.get_mut(&id) else { return };
        client.watching = true;
        client.attached = true;
        client.list(infos.clone());
        for info in &infos {
            if let Some(live) = self.sessions.get_mut(&info.session_id) { client.follow(&info.session_id, &mut live.log); }
        }
        client.replay_all();
    }

    fn subscribe(&mut self, id: u64, session_id: &str) -> Outcome {
        let Some(live) = self.sessions.get_mut(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        let info = live.info.clone();
        if let Some(client) = self.clients.get_mut(&id) {
            client.follow(session_id, &mut live.log);
            client.tell(&ServerMsg::SessionChanged { session: Box::new(info) });
        }
        Ok(())
    }

    fn publish(&mut self, session_id: &str) {
        let Some(live) = self.sessions.get_mut(session_id) else { return };
        let message = ServerMsg::SessionChanged { session: Box::new(live.info.clone()) };
        let _ = self.journal.append(&message);
        let Ok(frame) = ipc::control_frame(&message) else { return };
        for client in self.clients.values_mut() {
            if client.attached && !client.follows(session_id) { client.follow(session_id, &mut live.log); }
            if client.watching || client.follows(session_id) { client.push(frame.clone()); }
        }
    }

    fn forget_session(&mut self, session_id: &str) {
        let Some(mut live) = self.sessions.remove(session_id) else { return };
        live.log.discard();
        let message = ServerMsg::SessionRemoved { session_id: session_id.to_string() };
        let _ = self.journal.append(&message);
        for client in self.clients.values_mut() { client.unfollow(session_id); }
        self.broadcast(&message);
    }

    fn forget(&mut self, session_id: &str) -> Outcome {
        let Some(live) = self.sessions.get(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        if live.info.status.live() { return failed_for(code::ALREADY_RUNNING, "a running session is stopped before it is forgotten", session_id); }
        self.forget_session(session_id);
        Ok(())
    }

    fn make_room(&mut self, session_id: &str) -> Outcome {
        if self.sessions.contains_key(session_id) || self.sessions.len() < RETAINED_SESSIONS { return Ok(()); }
        let oldest = self.sessions.iter().filter(|(_, live)| !live.info.status.live()).min_by_key(|(_, live)| live.info.ended_ms.unwrap_or(live.info.started_ms)).map(|(id, _)| id.clone());
        match oldest {
            Some(id) => { self.forget_session(&id); Ok(()) }
            None => fail(code::LIMIT, "workspace session limit reached"),
        }
    }

    fn register(&mut self, session_id: &str, command: SessionCommand, status: SessionStatus) -> Outcome {
        self.make_room(session_id)?;
        let info = SessionInfo { session_id: session_id.to_string(), group: command.group.clone(), command, status, ..Default::default() };
        if let Some(live) = self.sessions.get_mut(session_id) {
            live.info = info;
            return Ok(());
        }
        let log = SessionLog::create(&self.log_dir, session_id, None);
        let mut live = Live::new(info, log);
        live.created = ipc::now_ms();
        self.sessions.insert(session_id.to_string(), live);
        Ok(())
    }

    /// ▶️ Starts the process of a session: a new one, or an ended one that runs again with its output continued.
    fn start(&mut self, session_id: &str, mut command: SessionCommand, base: &[(String, String)], requester: Option<u64>) -> Outcome {
        let cwd = ipc::canonical_path(&self.root.join(&command.cwd));
        command.cwd = cwd.display().to_string();
        let running = |live: &Live| matches!(live.info.status, SessionStatus::Running | SessionStatus::Stopping);
        if self.sessions.values().any(|live| running(live) && (live.info.session_id == session_id || live.info.command.same_task(&command))) {
            return failed_for(code::ALREADY_RUNNING, "task is already running; select its session or restart it", session_id);
        }
        self.register(session_id, command.clone(), SessionStatus::Pending)?;
        let env = child_environment(base, &command.env);
        let args: Vec<&str> = command.args.iter().map(String::as_str).collect();
        let spawned = Pty::spawn_exact(&command.cmd, &args, &env, Some(&cwd), PtySize { cols: command.cols, rows: command.rows });
        let Some(live) = self.sessions.get_mut(session_id) else { return fail(code::SPAWN, "session vanished") };
        let separator = live.log.separator();
        live.log.append(&separator);
        live.log.tracker().await_ready(command.ready.as_ref());
        live.input.clear();
        live.announced_wait = false;
        live.closed = false;
        live.finishing = None;
        live.candidate = None;
        live.input_owner = requester;
        (live.stopping, live.stage, live.forced, live.ended, live.title_due) = (None, None, None, None, None);
        live.last_output = Instant::now();
        let outcome = match spawned {
            Ok(pty) => {
                self.waker.watch_pty(&pty);
                live.info = SessionInfo { session_id: session_id.to_string(), group: command.group.clone(), command, status: SessionStatus::Running, pid: Some(pty.pid()), code: None, started_ms: ipc::now_ms(), ended_ms: None, ready_url: None, title: None };
                live.pty = Some(pty);
                Ok(())
            }
            Err(error) => {
                live.log.append(format!("[semio] could not start {}: {}\r\n", command.cmd, error.message).as_bytes());
                live.info = SessionInfo { session_id: session_id.to_string(), group: command.group.clone(), command, status: SessionStatus::Failed, started_ms: ipc::now_ms(), ended_ms: Some(ipc::now_ms()), ..Default::default() };
                live.pty = None;
                failed_for(code::SPAWN, error.message, session_id)
            }
        };
        self.publish(session_id);
        outcome
    }

    fn halt(&mut self, session_id: &str) {
        if let Some(live) = self.sessions.get_mut(session_id) {
            if let Some(mut pty) = live.pty.take() { let _ = pty.kill(); }
            live.input.clear();
            live.stopping = None;
            live.ended = None;
            if live.info.status.live() { live.info.status = SessionStatus::Exited; }
        }
    }

    fn restart(&mut self, session_id: &str, base: &[(String, String)], requester: u64) -> Outcome {
        let Some(live) = self.sessions.get(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        let command = live.info.command.clone();
        self.halt(session_id);
        self.start(session_id, command, base, Some(requester))
    }

    fn input(&mut self, requester: u64, session_id: &str, data: &[u8]) -> Outcome {
        let Some(live) = self.sessions.get_mut(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        if live.pty.is_none() || live.ended.is_some() { return failed_for(code::NOT_RUNNING, "session is not running", session_id); }
        if live.input.len() + data.len() > INPUT_LIMIT { return failed_for(code::INPUT_BACKLOG, "the task does not take its input", session_id); }
        if live.input.is_empty() { live.input_progress = Instant::now(); }
        live.input.extend(data);
        live.input_owner = Some(requester);
        Self::feed_input(live);
        Ok(())
    }

    fn feed_input(live: &mut Live) {
        let Some(pty) = live.pty.as_mut() else { return };
        while !live.input.is_empty() {
            let written = match pty.try_write(live.input.as_slices().0) { Ok(0) | Err(_) => break, Ok(count) => count };
            live.input.drain(..written);
            live.input_progress = Instant::now();
        }
    }

    fn resize(&mut self, session_id: &str, cols: u16, rows: u16) -> Outcome {
        let Some(live) = self.sessions.get_mut(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        if live.info.command.cols == cols && live.info.command.rows == rows { return Ok(()); }
        live.info.command.cols = cols;
        live.info.command.rows = rows;
        if let Some(pty) = live.pty.as_mut() { let _ = pty.resize(PtySize { cols, rows }); }
        Ok(())
    }

    /// ✋️ Asks a session to end: Ctrl+C, then termination, then the end of the whole tree, each after a bounded wait.
    fn stop(&mut self, session_id: &str) -> Outcome {
        let Some(live) = self.sessions.get_mut(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        match live.info.status {
            SessionStatus::Pending => { self.cancel(session_id, "stopped before it started"); Ok(()) }
            SessionStatus::Running => {
                live.info.status = SessionStatus::Stopping;
                live.stopping = Some(Instant::now());
                live.stage = Some(StopStage::Interrupt);
                if let Some(pty) = live.pty.as_mut() { let _ = pty.signal(StopStage::Interrupt); }
                self.publish(session_id);
                Ok(())
            }
            SessionStatus::Stopping => Ok(()),
            _ => failed_for(code::NOT_RUNNING, "session is not running", session_id),
        }
    }

    fn kill(&mut self, session_id: &str) -> Outcome {
        let Some(live) = self.sessions.get_mut(session_id) else { return failed_for(code::UNKNOWN_SESSION, "unknown session", session_id) };
        match live.info.status {
            SessionStatus::Pending => { self.cancel(session_id, "killed before it started"); Ok(()) }
            SessionStatus::Running | SessionStatus::Stopping => {
                let announce = live.info.status == SessionStatus::Running;
                live.info.status = SessionStatus::Stopping;
                live.stopping.get_or_insert_with(Instant::now);
                live.stage = Some(StopStage::Kill);
                live.forced = Some(SIGKILL_EXIT);
                if let Some(pty) = live.pty.as_mut() { let _ = pty.signal(StopStage::Kill); }
                if announce { self.publish(session_id); }
                Ok(())
            }
            _ => failed_for(code::NOT_RUNNING, "session is not running", session_id),
        }
    }

    fn cancel(&mut self, session_id: &str, reason: &str) {
        let Some(live) = self.sessions.get_mut(session_id) else { return };
        live.log.append(format!("[semio] {reason}\r\n").as_bytes());
        live.info.status = SessionStatus::Exited;
        live.info.ended_ms = Some(ipc::now_ms());
        self.publish(session_id);
        self.session_left(session_id);
    }

    fn read_sessions(&mut self) {
        let mut page = vec![0u8; READ_PAGE];
        let ids: Vec<String> = self.sessions.iter().filter(|(_, live)| live.pty.is_some()).map(|(id, _)| id.clone()).collect();
        for id in ids {
            let Some(live) = self.sessions.get_mut(&id) else { continue };
            Self::feed_input(live);
            let mut read = false;
            for _ in 0..READS_PER_TURN {
                let Some(pty) = live.pty.as_mut() else { break };
                match pty.read(&mut page) {
                    PtyRead::Data(count) => { live.log.append(&page[..count]); read = true; }
                    PtyRead::Empty => break,
                    PtyRead::Closed => { live.closed = true; break; }
                }
            }
            if !read { continue; }
            live.last_output = Instant::now();
            if live.log.tracker().take_title_change() {
                live.info.title = live.log.tracker().title().map(str::to_string);
                live.title_due.get_or_insert_with(|| Instant::now() + TITLE_INTERVAL);
            }
            if let Some(url) = live.log.tracker().take_ready() { live.offer(url); }
        }
    }

    /// 🧹 Everything that happens on a timer: the steps of stopping, input that was not taken, titles,
    /// readiness that waits for its last byte, logs, and the end of processes.
    fn maintain(&mut self) {
        let now = Instant::now();
        let ids: Vec<String> = self.sessions.keys().cloned().collect();
        let mut publish = Vec::new();
        let mut finished = Vec::new();
        let mut stalled = Vec::new();
        for id in ids {
            let Some(live) = self.sessions.get_mut(&id) else { continue };
            if live.pty.is_some() {
                if let Some(started) = live.stopping {
                    let waited = now.duration_since(started);
                    let (terminate_after, kill_after) = if live.pty.as_ref().is_some_and(Pty::batch) { (TERMINATE_AFTER / 2, KILL_AFTER / 2) } else { (TERMINATE_AFTER, KILL_AFTER) };
                    if waited >= kill_after && live.stage != Some(StopStage::Kill) {
                        live.stage = Some(StopStage::Kill);
                        live.forced = Some(SIGKILL_EXIT);
                        if let Some(pty) = live.pty.as_mut() { let _ = pty.signal(StopStage::Kill); }
                    } else if waited >= terminate_after && live.stage == Some(StopStage::Interrupt) {
                        live.stage = Some(StopStage::Terminate);
                        if let Some(pty) = live.pty.as_mut() { let _ = pty.signal(StopStage::Terminate); }
                    }
                    if waited >= GIVE_UP_AFTER && live.ended.is_none() { live.ended = Some((SIGKILL_EXIT, started)); }
                }
                if !live.input.is_empty() && now.duration_since(live.input_progress) >= INPUT_STALL {
                    live.input.clear();
                    if let Some(owner) = live.input_owner { stalled.push((owner, id.clone())); }
                }
                if live.ended.is_none() {
                    if let Ok(Some(status)) = live.pty.as_mut().map_or(Ok(None), Pty::try_wait) {
                        live.ended = Some((status, now));
                    }
                }
                if live.log.tracker().ready_undecided() && now.duration_since(live.last_output) >= READY_SETTLE {
                    live.log.tracker().settle_ready();
                    if let Some(url) = live.log.tracker().take_ready() { live.offer(url); }
                }
                if live.info.ready_url.is_none() && live.candidate.is_some() && now >= live.probe_due {
                    let port = live.info.command.ready.as_ref().map_or(0, |ready| ready.port);
                    if listening(port) {
                        live.info.ready_url = live.candidate.take();
                        publish.push(id.clone());
                    } else {
                        live.probe_delay = (live.probe_delay * 2).min(PROBE_LAST);
                        live.probe_due = now + live.probe_delay;
                    }
                }
            }
            if live.title_due.is_some_and(|due| now >= due) {
                live.title_due = None;
                publish.push(id.clone());
            }
            if live.log.dirty() && now.duration_since(live.flushed) >= FLUSH_INTERVAL {
                live.log.flush();
                live.flushed = now;
            }
            if let Some((_, since)) = live.ended {
                let quiet = now.duration_since(live.last_output) >= EXIT_QUIET && now.duration_since(since) >= EXIT_QUIET;
                if quiet && live.finishing.is_none() {
                    if let Some(pty) = live.pty.as_mut() { pty.finish(); }
                    live.finishing = Some(now);
                }
                let drained = live.finishing.is_some_and(|at| now.duration_since(at) >= FINISH_WAIT);
                if live.closed || drained || now.duration_since(since) >= EXIT_LIMIT { finished.push(id.clone()); }
            }
        }
        for id in publish { self.publish(&id); }
        for (owner, id) in stalled { self.reply(owner, Fail { code: code::INPUT_BACKLOG, message: "the task does not take its input".into(), session_id: Some(id) }); }
        for id in finished { self.finalize(&id); }
        if let Some(deadline) = self.shutdown_at {
            if !self.shutdown && (now >= deadline || !self.sessions.values().any(|live| live.info.status.live())) {
                self.shutdown = true;
                self.broadcast(&ServerMsg::Shutdown {});
            }
        }
    }

    /// 🪦️ Records that the process of a session ended.
    fn finalize(&mut self, session_id: &str) {
        let Some(live) = self.sessions.get_mut(session_id) else { return };
        let Some((status, _)) = live.ended.take() else { return };
        drop(live.pty.take());
        live.input.clear();
        live.stopping = None;
        live.info.status = SessionStatus::Exited;
        live.info.code = Some(live.forced.take().unwrap_or(status));
        live.info.ended_ms = Some(ipc::now_ms());
        live.info.ready_url = None;
        live.candidate = None;
        live.log.cool();
        self.publish(session_id);
        self.session_left(session_id);
    }

    fn spawn_group(&mut self, id: u64, group_id: &str, stop: GroupStop, members: Vec<GroupMember>, requires: Vec<GroupMember>) -> Outcome {
        let environment = self.environment_of(id)?;
        if self.groups.contains_key(group_id) { return fail(code::ALREADY_RUNNING, "a group of this identifier is already running"); }
        let steps: Vec<Step> = requires.into_iter().map(|member| (member, true)).chain(members.into_iter().map(|member| (member, false))).map(|(member, service)| Step { session_id: member.session_id, command: member.command, service }).collect();
        for step in steps.iter().filter(|step| !step.service) {
            if self.sessions.get(&step.session_id).is_some_and(|live| live.info.status.live()) { return failed_for(code::ALREADY_RUNNING, "session is already running", &step.session_id); }
        }
        for step in steps.iter().filter(|step| !step.service) {
            let mut command = step.command.clone();
            command.cwd = ipc::canonical_path(&self.root.join(&command.cwd)).display().to_string();
            self.register(&step.session_id, command, SessionStatus::Pending)?;
            self.publish(&step.session_id);
        }
        self.groups.insert(group_id.to_owned(), Group::new(group_id, stop, id, environment, steps));
        self.advance_group(group_id);
        Ok(())
    }

    fn advance_groups(&mut self) {
        let ids: Vec<String> = self.groups.iter().filter(|(_, group)| group.waiting.is_some() || !group.queue.is_empty()).map(|(id, _)| id.clone()).collect();
        for id in ids { self.advance_group(&id); }
    }

    fn readiness(&self, session_id: &str) -> Option<bool> {
        let live = self.sessions.get(session_id)?;
        match live.info.status {
            SessionStatus::Exited | SessionStatus::Failed | SessionStatus::Interrupted => None,
            _ if live.info.ready_url.is_some() => Some(true),
            SessionStatus::Running if live.info.command.ready.is_none() => Some(true),
            _ => Some(false),
        }
    }

    fn advance_group(&mut self, group_id: &str) {
        loop {
            let Some(group) = self.groups.get_mut(group_id) else { return };
            if group.ending { return; }
            if let Some(waiting) = group.waiting.clone() {
                match self.readiness(&waiting) {
                    Some(true) => { if let Some(group) = self.groups.get_mut(group_id) { group.waiting = None; } }
                    Some(false) => return self.announce_wait(group_id, &waiting),
                    None => return self.fail_group(group_id, Fail { code: code::SPAWN, message: "ended before it was ready".into(), session_id: Some(waiting) }),
                }
                continue;
            }
            let Some(step) = group.queue.pop_front() else { return };
            let environment = group.environment.clone();
            let owner = group.owner;
            if step.service {
                if let Some(running) = self.reusable(&step.command) {
                    if let Some(group) = self.groups.get_mut(group_id) { group.waiting = Some(running); }
                    continue;
                }
            }
            match self.start(&step.session_id, step.command.clone(), &environment, Some(owner)) {
                Ok(()) => { if step.command.ready.is_some() { if let Some(group) = self.groups.get_mut(group_id) { group.waiting = Some(step.session_id.clone()); } } }
                Err(failure) => return self.fail_group(group_id, failure),
            }
        }
    }

    fn announce_wait(&mut self, group_id: &str, cause: &str) {
        let Some(group) = self.groups.get(group_id) else { return };
        let label = self.sessions.get(cause).map_or_else(|| cause.to_string(), |live| if live.info.command.command_id.is_empty() { cause.to_string() } else { live.info.command.command_id.clone() });
        for member in group.members.clone() {
            if let Some(live) = self.sessions.get_mut(&member).filter(|live| live.info.status == SessionStatus::Pending && !live.announced_wait) {
                live.announced_wait = true;
                live.log.append(format!("[semio] waiting for {label} to be ready\r\n").as_bytes());
            }
        }
    }

    fn reusable(&self, command: &SessionCommand) -> Option<String> {
        self.sessions.values().find(|live| live.info.status == SessionStatus::Running && ((!command.command_id.is_empty() && live.info.command.command_id == command.command_id) || live.info.command.same_task(command))).map(|live| live.info.session_id.clone())
    }

    fn fail_group(&mut self, group_id: &str, failure: Fail) {
        let Some(group) = self.groups.get_mut(group_id) else { return };
        group.ending = true;
        group.queue.clear();
        group.waiting = None;
        let (owner, members, together) = (group.owner, group.members.clone(), group.stop == GroupStop::Together);
        let message = format!("{}: {}", failure.session_id.as_deref().unwrap_or(group_id), failure.message);
        self.reply(owner, Fail { message: message.clone(), ..failure });
        for member in members {
            let status = self.sessions.get(&member).map(|live| live.info.status);
            if status == Some(SessionStatus::Pending) {
                if let Some(live) = self.sessions.get_mut(&member) { live.log.append(format!("[semio] not started: {message}\r\n").as_bytes()); live.info.status = SessionStatus::Failed; live.info.ended_ms = Some(ipc::now_ms()); }
                self.publish(&member);
            } else if together && status == Some(SessionStatus::Running) {
                let _ = self.stop(&member);
            }
        }
        self.drop_group_if_done(group_id);
    }

    fn end_group(&mut self, group_id: &str, hard: bool) -> Outcome {
        let Some(group) = self.groups.get_mut(group_id) else { return fail(code::UNKNOWN_GROUP, format!("unknown group {group_id}")) };
        group.ending = true;
        group.queue.clear();
        group.waiting = None;
        let members = group.members.clone();
        for member in members {
            let _ = if hard { self.kill(&member) } else { self.stop(&member) };
        }
        self.drop_group_if_done(group_id);
        Ok(())
    }

    fn drop_group_if_done(&mut self, group_id: &str) {
        let Some(group) = self.groups.get(group_id) else { return };
        let settled = group.queue.is_empty() && group.waiting.is_none();
        if settled && !group.members.iter().any(|member| self.sessions.get(member).is_some_and(|live| live.info.status.live())) { self.groups.remove(group_id); }
    }

    /// 🔚 A session left the live states: a group it belongs to may end with it.
    fn session_left(&mut self, session_id: &str) {
        let Some(group_id) = self.groups.values().find(|group| group.owns(session_id)).map(|group| group.id.clone()) else { return };
        let Some(group) = self.groups.get(&group_id) else { return };
        if !group.ending && group.waiting.as_deref() == Some(session_id) {
            self.fail_group(&group_id, Fail { code: code::SPAWN, message: "ended before it was ready".into(), session_id: Some(session_id.to_string()) });
        } else if !group.ending && group.stop == GroupStop::Together {
            let _ = self.end_group(&group_id, false);
        }
        self.drop_group_if_done(&group_id);
    }

    fn begin_shutdown(&mut self) {
        if self.shutdown_at.is_some() { return; }
        self.shutdown_at = Some(Instant::now() + SHUTDOWN_GRACE);
        self.groups.clear();
        let ids: Vec<String> = self.sessions.iter().filter(|(_, live)| live.info.status.live()).map(|(id, _)| id.clone()).collect();
        for id in ids { let _ = self.kill(&id); }
    }
}

/// 🔒 The operating system releases this workspace lease when the daemon exits.
pub struct WorkspaceLease {
    _file: std::fs::File,
    root: PathBuf,
}

impl WorkspaceLease {
    pub fn acquire(root: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(ipc::daemon_dir(root))?;
        let file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(ipc::lock_path(root))?;
        file.try_lock().map_err(|_| std::io::Error::new(std::io::ErrorKind::AlreadyExists, "workspace dashboard daemon is already running"))?;
        Ok(Self { _file: file, root: root.to_path_buf() })
    }
}

impl Drop for WorkspaceLease {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(ipc::pid_path(&self.root));
    }
}

/// 🏷️ Writes the pid file `semio daemon status` reads.
pub fn write_pid(root: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(ipc::daemon_dir(root))?;
    std::fs::write(ipc::pid_path(root), format!("{}\n", std::process::id()))
}

pub fn read_pid(root: &Path) -> Option<u32> {
    std::fs::read_to_string(ipc::pid_path(root)).ok()?.trim().parse().ok()
}

/// 🌀 Serves the workspace until `running` is cleared or a view asks the daemon to shut down.
pub fn serve(root: &Path, running: &AtomicBool) -> std::io::Result<()> {
    let _lease = WorkspaceLease::acquire(root)?;
    write_pid(root)?;
    let listener = Listener::bind(root)?;
    let mut supervisor = Supervisor::new(root, Some(listener))?;
    let interrupts = ui_tui::tui::pty::interrupts();
    while running.load(Ordering::SeqCst) && !supervisor.is_shutdown() {
        if interrupts.load(Ordering::SeqCst) > 0 { supervisor.shut_down(); }
        supervisor.turn(Duration::from_millis(250))?;
    }
    supervisor.drain(Duration::from_secs(1));
    Ok(())
}

/// 🚀 Starts a detached daemon process (`semio daemon serve --root …`) and waits until it answers; the answer is its pid.
pub fn spawn_daemon(root: &Path, executable: &Path) -> std::io::Result<u32> {
    let spawned = ui_tui::tui::pty::spawn_detached(&executable.to_string_lossy(), &["daemon", "serve", "--root", &root.display().to_string()], &["NX_INVOCATION_ROOT_PID"], root).map_err(|error| std::io::Error::other(error.message))?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(connection) = super::client::Connection::connect(root) { return Ok(connection.daemon_pid()); }
        std::thread::sleep(Duration::from_millis(20));
    }
    Err(std::io::Error::other(format!("daemon {spawned} did not become ready")))
}

/// 🌅️ Starts the workspace daemon unless one runs, and reports it.
pub fn start_detached(root: &Path, executable: &Path, text: &DashboardLabels) -> i32 {
    if super::client::Connection::connect(root).is_ok() {
        print!("{}", status(root, text));
        return 0;
    }
    match spawn_daemon(root, executable) {
        Ok(pid) => { println!("{}", text.cli_daemon_ready.fill(&[("pid", &pid.to_string())]).into_string()); 0 }
        Err(error) => { eprintln!("{}", text.cli_daemon_start_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }
    }
}

/// 📊 One line about the daemon of a workspace, and a second one when its build differs from this one.
pub fn status(root: &Path, text: &DashboardLabels) -> String {
    let Ok(mut connection) = super::client::Connection::connect(root) else { return format!("{}\n", text.cli_daemon_not_running.as_str()) };
    let count = super::control::tasks(&mut connection, Duration::from_secs(2)).map_or(0, |sessions| sessions.iter().filter(|session| session.status.live()).count());
    let mut report = format!("{}\n", text.cli_daemon_status.fill(&[("pid", &connection.daemon_pid().to_string()), ("build", connection.daemon_build()), ("count", &count.to_string()), ("endpoint", &ipc::endpoint(root))]).into_string());
    if let Some(skew) = connection.skew() { report.push_str(&format!("{}\n", text.cli_warning.fill(&[("message", &text.skew(&skew))]).into_string())); }
    report
}

/// 🌇️ Asks the daemon to end its tasks and itself, and waits for it to say so.
pub fn stop(root: &Path, text: &DashboardLabels) -> i32 {
    let outcome = super::client::Connection::connect(root).and_then(|mut connection| {
        connection.send(&ClientMsg::Shutdown {})?;
        let deadline = Instant::now() + Duration::from_secs(10);
        while Instant::now() < deadline {
            let messages = match connection.receive(Duration::from_millis(100)) { Ok(messages) => messages, Err(_) => return Ok(()) };
            if messages.iter().any(|message| matches!(message, super::client::Message::Control(ServerMsg::Shutdown {}) | super::client::Message::Disconnected(_))) { return Ok(()); }
        }
        Err(std::io::Error::other(text.cli_daemon_stop_timed_out.as_str()))
    });
    match outcome {
        Ok(()) => { println!("{}", text.cli_daemon_stopped.as_str()); 0 }
        Err(error) => { eprintln!("{}", text.cli_daemon_stop_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }
    }
}
