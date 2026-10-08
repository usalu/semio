//! 📜️ The daemon's event journal: every change of a session's lifecycle, one JSON message per line,
//! kept local to the machine. The sessions a restarted daemon or an offline `semio tasks` shows are the
//! projection of this journal; the journal is compacted to that projection whenever it grew far beyond it.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use super::ipc::{self, ServerMsg, SessionInfo, SessionStatus};
use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

/// 🗄️ How many sessions a workspace retains; the one that ended longest ago leaves first.
pub const RETAINED_SESSIONS: usize = 128;
const COMPACT_FACTOR: usize = 4;
const COMPACT_FLOOR: usize = 64;

/// 🧮️ The sessions a journal file describes, with the oldest ended sessions beyond the limit dropped.
pub fn project(path: &Path) -> (Vec<SessionInfo>, usize) {
    let Ok(file) = std::fs::File::open(path) else { return (Vec::new(), 0) };
    let mut sessions: BTreeMap<String, SessionInfo> = BTreeMap::new();
    let mut lines = 0;
    for line in std::io::BufReader::new(file).split(b'\n').map_while(Result::ok) {
        lines += 1;
        match serde_json::from_slice::<ServerMsg>(&line) {
            Ok(ServerMsg::SessionChanged { session }) if session.validate().is_ok() => { sessions.insert(session.session_id.clone(), *session); }
            Ok(ServerMsg::SessionRemoved { session_id }) => { sessions.remove(&session_id); }
            _ => {}
        }
    }
    let mut projected: Vec<SessionInfo> = sessions.into_values().collect();
    projected.sort_by_key(|session| (session.started_ms, session.session_id.clone()));
    while projected.len() > RETAINED_SESSIONS {
        let evicted = projected.iter().enumerate().filter(|(_, session)| !session.status.live()).min_by_key(|(_, session)| session.ended_ms.unwrap_or(session.started_ms)).map_or(0, |(index, _)| index);
        projected.remove(evicted);
    }
    (projected, lines)
}

/// 🕰️ The sessions a workspace's journal describes without asking the daemon. While no daemon runs, whatever was
/// alive when it ended was interrupted. A daemon that runs keeps the journal current with every change, so the
/// journal is taken as it is: a daemon that is merely not answering (all its connections taken) must not turn its
/// running tasks into interrupted ones.
pub fn offline(root: &Path) -> Vec<SessionInfo> {
    let mut sessions = project(&ipc::event_log_path(root)).0;
    if super::supervisor::WorkspaceLease::acquire(root).is_err() { return sessions; }
    for session in sessions.iter_mut().filter(|session| session.status.live()) { interrupt(session, None); }
    sessions
}

/// ⛔ Marks a session whose process the daemon lost track of.
pub fn interrupt(session: &mut SessionInfo, ended_ms: Option<u64>) {
    session.status = SessionStatus::Interrupted;
    session.pid = None;
    session.code = None;
    session.ready_url = None;
    session.ended_ms = ended_ms.or(session.ended_ms);
}

/// 📓 The open journal of one workspace.
pub struct Journal {
    path: PathBuf,
    file: Option<std::fs::File>,
}

impl Journal {
    /// 📂 Opens the journal, compacting it when it outgrew its projection, and answers that projection.
    pub fn open(root: &Path) -> std::io::Result<(Self, Vec<SessionInfo>)> {
        std::fs::create_dir_all(ipc::daemon_dir(root))?;
        let path = ipc::event_log_path(root);
        let (sessions, lines) = project(&path);
        let mut journal = Self { path, file: None };
        if lines > sessions.len() * COMPACT_FACTOR + COMPACT_FLOOR { journal.compact(&sessions)?; }
        Ok((journal, sessions))
    }

    /// ➕ Appends one lifecycle message.
    pub fn append(&mut self, event: &ServerMsg) -> std::io::Result<()> {
        let mut line = serde_json::to_vec(event).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        line.push(b'\n');
        if self.file.is_none() { self.file = Some(std::fs::OpenOptions::new().create(true).append(true).open(&self.path)?); }
        let file = self.file.as_mut().ok_or_else(|| std::io::Error::other("journal closed"))?;
        file.write_all(&line)
    }

    fn compact(&mut self, sessions: &[SessionInfo]) -> std::io::Result<()> {
        self.file = None;
        let temporary = self.path.with_extension(format!("{}.tmp", std::process::id()));
        let mut text = Vec::new();
        for session in sessions {
            serde_json::to_writer(&mut text, &ServerMsg::SessionChanged { session: Box::new(session.clone()) }).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
            text.push(b'\n');
        }
        std::fs::write(&temporary, text)?;
        std::fs::rename(&temporary, &self.path)
    }
}
