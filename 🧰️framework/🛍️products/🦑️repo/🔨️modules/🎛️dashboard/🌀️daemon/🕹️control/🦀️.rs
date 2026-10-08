//! 🕹️ The daemon without a terminal: what `semio run`, `tasks`, `logs`, `stop`, `restart` and `kill` do.
//! Every function takes a [`Connection`] and speaks the same protocol as the dashboard view, so a task
//! started here is the task the view shows, and the other way round. Waiting functions poll a
//! cancellation flag, report progress through a callback and end within a bound.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧩️groups/🥒️.feature

use super::client::{Connection, Message, Skew};
use super::ipc::{self, ClientMsg, ServerMsg, SessionInfo, SessionStatus, SpawnGroup};
use super::journal;
use super::replay::LogReader;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const POLL: Duration = Duration::from_millis(50);
const QUIET: Duration = Duration::from_millis(250);

/// 🔗 A connection to the workspace daemon, which is started first when none runs.
pub fn connect_or_start(root: &Path, executable: &Path) -> std::io::Result<Connection> {
    if let Ok(connection) = Connection::connect(root) { return Ok(connection); }
    super::supervisor::spawn_daemon(root, executable)?;
    Connection::connect(root)
}

/// 📋 The sessions of the workspace as the daemon lists them.
pub fn tasks(connection: &mut Connection, within: Duration) -> std::io::Result<Vec<SessionInfo>> {
    connection.send(&ClientMsg::List {})?;
    let deadline = Instant::now() + within;
    let mut sessions = Vec::new();
    while Instant::now() < deadline {
        for message in connection.receive(POLL)? {
            match message {
                Message::Control(ServerMsg::Sessions { sessions: page, more }) => { sessions.extend(page); if !more { return Ok(sessions); } }
                Message::Disconnected(reason) => return Err(std::io::Error::other(reason)),
                _ => {}
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "daemon did not list its sessions"))
}

/// 🕰️ The sessions of the workspace while no daemon runs.
pub fn offline_tasks(root: &Path) -> Vec<SessionInfo> { journal::offline(root) }

/// 🔍 Why a task could not be chosen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectError {
    None(String),
    Ambiguous(String, Vec<String>),
}

impl std::fmt::Display for SelectError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None(needle) => write!(formatter, "no task matches {needle:?}"),
            Self::Ambiguous(needle, found) => write!(formatter, "{needle:?} matches several tasks: {}", found.join(", ")),
        }
    }
}

/// 🎯 Chooses one session by what a developer types: its identifier, its 1-based place in the listing,
/// a command identifier (the live or latest run of it), or a unique part of a session or command identifier.
pub fn select<'a>(sessions: &'a [SessionInfo], needle: &str) -> Result<&'a SessionInfo, SelectError> {
    let latest = |found: Vec<&'a SessionInfo>| -> Option<&'a SessionInfo> {
        found.iter().copied().filter(|session| session.status.live()).max_by_key(|session| session.started_ms).or_else(|| found.iter().copied().max_by_key(|session| session.started_ms))
    };
    if let Some(exact) = sessions.iter().find(|session| session.session_id == needle) { return Ok(exact); }
    if let Some(session) = needle.parse::<usize>().ok().and_then(|place| place.checked_sub(1)).and_then(|index| sessions.get(index)) { return Ok(session); }
    let same_command: Vec<&SessionInfo> = sessions.iter().filter(|session| !session.command.command_id.is_empty() && session.command.command_id == needle).collect();
    if let Some(session) = latest(same_command) { return Ok(session); }
    let parts: Vec<&SessionInfo> = sessions.iter().filter(|session| session.session_id.starts_with(needle) || (!needle.is_empty() && session.command.command_id.contains(needle))).collect();
    let mut commands: Vec<String> = parts.iter().map(|session| if session.command.command_id.is_empty() { session.session_id.clone() } else { session.command.command_id.clone() }).collect();
    commands.sort();
    commands.dedup();
    match commands.len() {
        0 => Err(SelectError::None(needle.to_string())),
        1 => latest(parts).ok_or_else(|| SelectError::None(needle.to_string())),
        _ => Err(SelectError::Ambiguous(needle.to_string(), commands)),
    }
}

/// ⏳ What `run` waits for after the daemon accepted the launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wait {
    Accepted,
    Ready,
    Exit,
}

/// 🟢 An address a started process announced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadyUrl {
    pub session_id: String,
    pub command_id: String,
    pub url: String,
}

/// 🚀 A launch the daemon accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Started {
    pub group_id: String,
    pub sessions: Vec<SessionInfo>,
    pub ready: Vec<ReadyUrl>,
}

impl Started {
    /// 🌐️ The address of the last member that announced one: the thing the launch serves.
    pub fn ready_url(&self) -> Option<&str> { self.ready.last().map(|ready| ready.url.as_str()) }

    /// 🏁 The exit code of the last member, once it ended.
    pub fn exit_code(&self) -> Option<i32> { self.sessions.last().and_then(|session| session.code) }
}

/// ❌ Why a launch or a wait did not end well.
#[derive(Debug)]
pub enum RunError {
    Refused { code: String, message: String, session_id: Option<String> },
    Ended(Box<SessionInfo>),
    Cancelled,
    Timeout,
    Skew(Skew),
    Io(std::io::Error),
}

impl std::fmt::Display for RunError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refused { code, message, .. } => write!(formatter, "daemon refused ({code}): {message}"),
            Self::Ended(session) => write!(formatter, "{} {} before it was ready{}", session.command.command_id, format!("{:?}", session.status).to_lowercase(), session.code.map(|code| format!(" with exit code {code}")).unwrap_or_default()),
            Self::Cancelled => write!(formatter, "cancelled"),
            Self::Timeout => write!(formatter, "timed out"),
            Self::Skew(skew) => write!(formatter, "{}", skew.describe()),
            Self::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self { Self::Io(error) }
}

fn satisfied(session: &SessionInfo, wait: Wait) -> Option<bool> {
    match (wait, session.status) {
        (_, SessionStatus::Failed | SessionStatus::Interrupted) => None,
        (Wait::Accepted, _) => Some(true),
        (Wait::Exit, SessionStatus::Exited) => Some(true),
        (Wait::Exit, _) => Some(false),
        (Wait::Ready, SessionStatus::Exited) => (session.code == Some(0) && session.command.ready.is_none()).then_some(true),
        (Wait::Ready, _) => Some(session.command.ready.is_none() && session.status == SessionStatus::Running || session.ready_url.is_some()),
    }
}

/// 🛫️ Starts a launch and waits as asked: until the daemon accepted it, until every member that declares
/// readiness printed its address, or until every member ended. A launch that cannot be served, a member that
/// ends before it is ready, a cancellation (which stops the group) and the timeout end the wait with an error.
pub fn run(connection: &mut Connection, group: SpawnGroup, wait: Wait, timeout: Option<Duration>, cancel: &AtomicBool, progress: &mut dyn FnMut(&SessionInfo)) -> Result<Started, RunError> {
    if let Some(skew) = connection.skew().filter(Skew::incompatible) { return Err(RunError::Skew(skew)); }
    let group_id = group.group_id.clone();
    let ids: Vec<String> = group.members.iter().map(|member| member.session_id.clone()).collect();
    connection.send(&ClientMsg::Watch {})?;
    connection.send(&group.message())?;
    let deadline = timeout.map(|timeout| Instant::now() + timeout);
    let mut seen: Vec<Option<SessionInfo>> = vec![None; ids.len()];
    loop {
        if cancel.load(Ordering::SeqCst) {
            let _ = connection.send(&ClientMsg::StopGroup { group_id });
            return Err(RunError::Cancelled);
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) { return Err(RunError::Timeout); }
        for message in connection.receive(POLL)? {
            match message {
                Message::Control(ServerMsg::SessionChanged { session }) => {
                    if let Some(index) = ids.iter().position(|id| *id == session.session_id) {
                        progress(&session);
                        seen[index] = Some(*session);
                    }
                }
                Message::Control(ServerMsg::Error { message, code, session_id }) => return Err(RunError::Refused { code: code.map_or_else(String::new, |code| code.as_str().to_string()), message, session_id }),
                Message::Disconnected(reason) => return Err(RunError::Io(std::io::Error::other(reason))),
                _ => {}
            }
        }
        if seen.iter().any(Option::is_none) { continue; }
        let sessions: Vec<SessionInfo> = seen.iter().flatten().cloned().collect();
        let mut done = true;
        for session in &sessions {
            match satisfied(session, wait) {
                None => return Err(RunError::Ended(Box::new(session.clone()))),
                Some(false) => done = false,
                Some(true) => {}
            }
        }
        if done {
            let ready = sessions.iter().filter_map(|session| session.ready_url.as_ref().map(|url| ReadyUrl { session_id: session.session_id.clone(), command_id: session.command.command_id.clone(), url: url.clone() })).collect();
            return Ok(Started { group_id, sessions, ready });
        }
    }
}

/// 🏳️ How a followed session ended its stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Followed {
    Ended(SessionInfo),
    Alive(SessionInfo),
    Cancelled,
}

/// 📜 Writes the output of a session to `out`: what the daemon replays, then with `follow` whatever the
/// session prints until it ends. Without `follow` it stops where the replay reaches live output.
pub fn follow(connection: &mut Connection, session_id: &str, out: &mut dyn Write, follow: bool, cancel: &AtomicBool) -> std::io::Result<Followed> {
    connection.send(&ClientMsg::Subscribe { session_id: session_id.to_string() })?;
    let mut info: Option<SessionInfo> = None;
    let mut replayed = false;
    let mut quiet_since: Option<Instant> = None;
    loop {
        if cancel.load(Ordering::SeqCst) { return Ok(Followed::Cancelled); }
        let messages = connection.receive(POLL)?;
        let idle = messages.is_empty();
        for message in messages {
            match message {
                Message::Output { session_id: id, data } if id == session_id => { out.write_all(&data)?; out.flush()?; quiet_since = None; }
                Message::Control(ServerMsg::SessionChanged { session }) if session.session_id == session_id => info = Some(*session),
                Message::Control(ServerMsg::ReplayComplete { session_id: Some(id) }) if id == session_id => replayed = true,
                Message::Control(ServerMsg::Error { message, .. }) => return Err(std::io::Error::other(message)),
                Message::Disconnected(reason) => return Err(std::io::Error::other(reason)),
                _ => {}
            }
        }
        let Some(current) = info.clone() else { continue };
        if replayed && !follow { return Ok(if current.status.live() { Followed::Alive(current) } else { Followed::Ended(current) }); }
        if replayed && !current.status.live() {
            if idle { let since = *quiet_since.get_or_insert_with(Instant::now); if since.elapsed() >= QUIET { return Ok(Followed::Ended(current)); } } else { quiet_since = None; }
        }
    }
}

/// 🛑 What to do to a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Stop,
    Kill,
    Restart,
}

/// 🎛️ Asks the daemon to stop, kill or restart a session and, with `wait`, waits until the session has left the
/// state it was in: ended after a stop or kill, running again after a restart.
pub fn act(connection: &mut Connection, session_id: &str, action: Action, wait: Option<Duration>, cancel: &AtomicBool) -> Result<SessionInfo, RunError> {
    connection.send(&ClientMsg::Subscribe { session_id: session_id.to_string() })?;
    let before = loop {
        match connection.receive(POLL)?.into_iter().find_map(|message| match message {
            Message::Control(ServerMsg::SessionChanged { session }) if session.session_id == session_id => Some(Ok(*session)),
            Message::Control(ServerMsg::Error { message, code, session_id }) => Some(Err(RunError::Refused { code: code.map_or_else(String::new, |code| code.as_str().to_string()), message, session_id })),
            _ => None,
        }) { Some(found) => break found?, None => if cancel.load(Ordering::SeqCst) { return Err(RunError::Cancelled); } }
    };
    connection.send(&match action {
        Action::Stop => ClientMsg::Stop { session_id: session_id.to_string() },
        Action::Kill => ClientMsg::Kill { session_id: session_id.to_string() },
        Action::Restart => ClientMsg::Restart { session_id: session_id.to_string() },
    })?;
    let deadline = wait.map(|wait| Instant::now() + wait);
    let mut last = before.clone();
    loop {
        for message in connection.receive(POLL)? {
            match message {
                Message::Control(ServerMsg::SessionChanged { session }) if session.session_id == session_id => last = *session,
                Message::Control(ServerMsg::Error { message, code, session_id }) => return Err(RunError::Refused { code: code.map_or_else(String::new, |code| code.as_str().to_string()), message, session_id }),
                Message::Disconnected(reason) => return Err(RunError::Io(std::io::Error::other(reason))),
                _ => {}
            }
        }
        let reached = match action {
            Action::Stop | Action::Kill => !last.status.live(),
            Action::Restart => last.status == SessionStatus::Running && last.pid != before.pid,
        };
        if reached || deadline.is_none() { return Ok(last); }
        if cancel.load(Ordering::SeqCst) { return Err(RunError::Cancelled); }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) { return Err(RunError::Timeout); }
    }
}

/// 🗒️ Writes the log files of a session to `out` without a daemon; the files are what `follow` replays from.
pub fn logs_offline(root: &Path, session_id: &str, out: &mut dyn Write) -> std::io::Result<u64> {
    let mut reader = LogReader::open(&ipc::log_dir(root), session_id);
    let mut written = 0u64;
    loop {
        let mut chunk = Vec::new();
        if reader.read(&mut chunk)? == 0 { return Ok(written); }
        out.write_all(&chunk)?;
        written += chunk.len() as u64;
    }
}
