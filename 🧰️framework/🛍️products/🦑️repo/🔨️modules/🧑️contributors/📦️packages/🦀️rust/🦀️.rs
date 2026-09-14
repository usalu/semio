//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 🧑️ Contributors of the `🦑️repo` product: who the people and agents are, the sessions they
//! work in, the checkpoints they leave behind and the interactions recorded against artifacts.
//!
//! Every effect is a port. Contributor documents come from [`ContributorStore`], session event
//! directories from [`SessionSource`] and the checkpoint log from [`CheckpointSource`], each with
//! an in-memory implementation so a scenario needs neither a repository on disk nor a `git`
//! binary. Time is [`Clock`], because whether a session counts as running is a question about
//! elapsed minutes.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use semio_framework_repo_identity as identity;

pub use semio_framework_repo_model::{Checkpoint, Contributor, Interaction, InteractionResource};

//#region ✍️AuthorShapes

/// ✍️ A git author line split into its parts.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GitAuthor {
    /// 🔤️ The human name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// 📧️ The email address.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub email: String,
    /// 🐙️ The management handle.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub github: String,
}

impl std::fmt::Display for GitAuthor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.email.is_empty() {
            write!(formatter, "{}", self.name)
        } else {
            write!(formatter, "{} <{}>", self.name, self.email)
        }
    }
}

/// ✂️ Splits a `Name <email>` author line. A line without ` <` is all name.
pub fn parse_git_author(line: &str) -> GitAuthor {
    match line.split_once(" <") {
        Some((name, rest)) => GitAuthor { name: name.trim().to_string(), email: rest.strip_suffix('>').unwrap_or(rest).to_string(), github: String::new() },
        None => GitAuthor { name: line.to_string(), email: String::new(), github: String::new() },
    }
}

/// 🤝️ Splits a `git shortlog`/`git log` identity line into its name and email. The line must
/// carry a four digit run — the year or the commit count the log prefixes it with — before the
/// name, and the email must be angle bracketed. Anything else is not an identity line.
pub fn parse_contributor_identity(line: &str) -> Option<(String, String)> {
    let characters: Vec<char> = line.chars().collect();
    for start in 0..characters.len() {
        if !run_of_four_digits(&characters, start) {
            continue;
        }
        let after_digits = start + 4;
        let mut cursor = after_digits;
        while cursor < characters.len() && characters[cursor].is_whitespace() {
            cursor += 1;
        }
        if cursor == after_digits {
            continue;
        }
        let Some(open) = characters[cursor..].iter().position(|current| *current == '<').map(|offset| cursor + offset) else { continue };
        let Some(close) = characters[open + 1..].iter().position(|current| *current == '>').map(|offset| open + 1 + offset) else { continue };
        let name: String = characters[cursor..open].iter().collect();
        let email: String = characters[open + 1..close].iter().collect();
        if name.trim().is_empty() || email.is_empty() {
            continue;
        }
        return Some((name.trim().to_string(), email.trim().to_string()));
    }
    None
}

fn run_of_four_digits(characters: &[char], start: usize) -> bool {
    if start + 4 > characters.len() {
        return false;
    }
    if start > 0 && characters[start - 1].is_ascii_digit() {
        return false;
    }
    if characters[start..start + 4].iter().any(|current| !current.is_ascii_digit()) {
        return false;
    }
    !characters.get(start + 4).is_some_and(char::is_ascii_digit)
}

//#endregion ✍️AuthorShapes

//#region 🧑️Contributors

/// 🗄️ Where contributor documents live: one `🧑️‍💻️contributor.json` per directory name.
pub trait ContributorStore {
    /// 📋️ Every `(directory name, document)` pair, in directory name order.
    fn documents(&self) -> Vec<(String, String)>;
}

/// 🧠️ The in-memory store the language-agnostic tests run against.
#[derive(Debug, Default)]
pub struct MemoryContributorStore {
    documents: BTreeMap<String, String>,
}

impl MemoryContributorStore {
    /// 🌱️ A store seeded with `(directory name, document)` pairs.
    pub fn seeded<I: IntoIterator<Item = (String, String)>>(entries: I) -> Self {
        Self { documents: entries.into_iter().collect() }
    }
}

impl ContributorStore for MemoryContributorStore {
    fn documents(&self) -> Vec<(String, String)> {
        self.documents.iter().map(|(name, document)| (name.clone(), document.clone())).collect()
    }
}

/// 💽️ The store over a real `.🧬semio/🦑️repo/🧑️‍💻️devs` tree.
#[derive(Debug, Clone)]
pub struct FsContributorStore {
    root: PathBuf,
}

impl FsContributorStore {
    /// 🆕️ The store of the devs directory of a repository root.
    pub fn new(repo_root: &Path) -> Self {
        Self { root: semio_framework_repo_workspace::devs_dir_for_root(repo_root) }
    }
}

impl ContributorStore for FsContributorStore {
    fn documents(&self) -> Vec<(String, String)> {
        let Ok(entries) = std::fs::read_dir(&self.root) else { return Vec::new() };
        let mut found: Vec<(String, String)> = Vec::new();
        for entry in entries.flatten() {
            if !entry.path().is_dir() {
                continue;
            }
            let Ok(document) = std::fs::read_to_string(entry.path().join("🧑️‍💻️contributor.json")) else { continue };
            found.push((entry.file_name().to_string_lossy().to_string(), document));
        }
        found.sort_by(|left, right| left.0.cmp(&right.0));
        found
    }
}

/// 📋️ Every contributor, with the directory name standing in for a blank alias or handle. An
/// empty store yields the single `unknown` contributor, exactly as the reader does.
pub fn list_contributors(store: &dyn ContributorStore) -> Vec<Contributor> {
    let documents = store.documents();
    if documents.is_empty() {
        let Ok(unknown) = serde_json::from_str::<Contributor>(r#"{"alias":"unknown","github":"unknown","name":"Unknown","email":""}"#) else { return Vec::new() };
        return vec![unknown];
    }
    documents
        .into_iter()
        .filter_map(|(directory, document)| {
            let mut contributor: Contributor = serde_json::from_str(&document).ok()?;
            if contributor.alias.is_empty() {
                contributor.alias = directory.clone();
            }
            if contributor.github.is_empty() {
                contributor.github = directory;
            }
            Some(contributor)
        })
        .collect()
}

/// 🔎️ Every contributor whose alias, name or handle contains the term, case-insensitively.
pub fn search_contributors(store: &dyn ContributorStore, term: &str) -> Vec<Contributor> {
    let needle = term.to_lowercase();
    list_contributors(store)
        .into_iter()
        .filter(|contributor| needle.is_empty() || format!("{} {} {}", contributor.alias, contributor.name, contributor.github).to_lowercase().contains(&needle))
        .collect()
}

/// 🤝️ The alias of the contributor an author line belongs to: an email match wins over a name
/// match, both are case-insensitive over the primary value and every recorded alternative, and an
/// author nobody claims falls back to the author line itself.
pub fn resolve_author_to_alias(store: &dyn ContributorStore, name: &str, email: &str) -> String {
    for contributor in list_contributors(store) {
        if !email.is_empty() && std::iter::once(&contributor.email).chain(contributor.emails.iter()).any(|candidate| eq_fold(candidate, email)) {
            return contributor.alias;
        }
        if !name.is_empty() && std::iter::once(&contributor.name).chain(contributor.names.iter()).any(|candidate| eq_fold(candidate, name)) {
            return contributor.alias;
        }
    }
    match (name.is_empty(), email.is_empty()) {
        (false, false) => format!("{name} <{email}>"),
        (false, true) => name.to_string(),
        (true, false) => email.to_string(),
        (true, true) => String::new(),
    }
}

/// 🔎️ The alias an author line resolves to, `unknown` when the line names nobody at all.
pub fn find_contributor(store: &dyn ContributorStore, author_line: &str) -> String {
    let parsed = parse_git_author(author_line);
    if parsed.name.is_empty() && parsed.email.is_empty() {
        return "unknown".to_string();
    }
    resolve_author_to_alias(store, &parsed.name, &parsed.email)
}

fn eq_fold(left: &str, right: &str) -> bool {
    left.to_lowercase() == right.to_lowercase()
}

//#endregion 🧑️Contributors

//#region 🪅️Sessions

/// 🚦️ What became of a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionKind {
    /// 🟡️ Still writing events.
    Running,
    /// 🟢️ Ended with an agent-ended event.
    Completed,
    /// 🔴️ Stopped without ending.
    Interrupted,
}

impl SessionKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionKind::Running => "running",
            SessionKind::Completed => "completed",
            SessionKind::Interrupted => "interrupted",
        }
    }
}

impl std::fmt::Display for SessionKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// 😀️ The entity emoji of a session kind, from the shared `🪪️identity` vocabulary.
pub fn session_kind_emoji(kind: Option<SessionKind>) -> String {
    match kind {
        Some(SessionKind::Running) => identity::entity("session-running").to_string(),
        Some(SessionKind::Completed) => identity::entity("session-completed").to_string(),
        Some(SessionKind::Interrupted) => identity::entity("session-interrupted").to_string(),
        None => identity::entity("session").to_string(),
    }
}

/// 💿️ One session of one agent on one day.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Session {
    /// 🪪️ The session identifier the client generated.
    pub uuid: String,
    /// 🎆️ The year the session directory sits under.
    pub year: i64,
    /// 🌙️ The month the session directory sits under.
    pub month: i64,
    /// ☀️ The day the session directory sits under.
    pub day: i64,
    /// 💾️ The checkpoint the session recorded, empty when it recorded none.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub checkpoint: String,
    /// 🚦️ What became of the session.
    pub kind: String,
    /// 💻️ The client that ran the session.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    /// 🤖️ The model the session ran.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    /// 🕰️ The earliest second the session recorded.
    #[serde(rename = "startedAt", default, skip_serializing_if = "String::is_empty")]
    pub started_at: String,
    /// 🏁️ The last second the session recorded.
    #[serde(rename = "endedAt", default, skip_serializing_if = "String::is_empty")]
    pub ended_at: String,
}

impl Session {
    /// 🪪️ The artifact identifier: the checkpoint when the session recorded one, otherwise the
    /// year, month and day it sits under, followed by the session's own segment.
    pub fn id(&self) -> String {
        let parent = if self.checkpoint.is_empty() {
            let year = segment("year", &format!("{:02}", self.year), "");
            let month = segment("month", &format!("{:02}", self.month), &year);
            segment("day", &format!("{:02}", self.day), &month)
        } else {
            segment("checkpoint", &self.checkpoint, "")
        };
        segment("session", &identity::flat(&self.uuid), &parent)
    }

    /// 🔗️ The artifact URI, which carries the session segment alone.
    pub fn uri(&self) -> String {
        format!("repo://session/{}", segment("session", &identity::flat(&self.uuid), ""))
    }
}

fn segment(entity: &str, value: &str, parent: &str) -> String {
    format!("{parent}{}", identity::SemanticId { emoji: identity::entity(entity).to_string(), value: value.to_string() })
}

/// 📥️ The `session.json` document of a session directory.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionMeta {
    /// 🪪️ The session identifier.
    #[serde(default)]
    pub id: String,
    /// 💻️ The client that ran the session.
    #[serde(default)]
    pub client: String,
    /// 🕰️ The earliest second the session recorded.
    #[serde(default)]
    pub second: String,
    /// 💾️ The checkpoint the session recorded.
    #[serde(default)]
    pub checkpoint: String,
    /// 🧑️ The contributor the session belongs to.
    #[serde(default)]
    pub contributor: String,
    /// 📜️ The recorded hook events, newest last.
    #[serde(default)]
    pub events: Vec<SessionEventEntry>,
}

/// 📍️ One recorded hook event of a session.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionEventEntry {
    /// 📡️ The event as the client wrote it.
    #[serde(default)]
    pub event: serde_json::Value,
}

/// 📂️ A directory of session event files, addressed by year, month, day and session identifier.
pub trait SessionSource {
    /// 📋️ Every `(year, month, day, uuid)` the source carries, in that order.
    fn sessions(&self) -> Vec<(i64, i64, i64, String)>;
    /// 📄️ The `session.json` document of one session, absent when it wrote none.
    fn meta(&self, key: &SessionKey) -> Option<String>;
    /// 📁️ The event file names of one session, `session.json` included, in directory order.
    fn entries(&self, key: &SessionKey) -> Vec<String>;
    /// 📄️ One event file of one session.
    fn entry(&self, key: &SessionKey, name: &str) -> Option<String>;
    /// 🕰️ How many seconds ago the newest file of one session was written. Absent when the
    /// session has no file at all.
    fn age_seconds(&self, key: &SessionKey) -> Option<i64>;
}

/// 🔑️ Where one session sits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct SessionKey {
    /// 🎆️ The year.
    pub year: i64,
    /// 🌙️ The month.
    pub month: i64,
    /// ☀️ The day.
    pub day: i64,
    /// 🪪️ The session identifier.
    pub uuid: String,
}

/// 🧠️ The in-memory source: a session is a name, a set of files and an age.
#[derive(Debug, Default)]
pub struct MemorySessionSource {
    sessions: BTreeMap<SessionKey, MemorySession>,
}

/// 📦️ The files and age of one in-memory session.
#[derive(Debug, Clone, Default)]
pub struct MemorySession {
    /// 📁️ The event files, by name.
    pub files: BTreeMap<String, String>,
    /// 🕰️ How many seconds ago the newest file was written.
    pub age_seconds: Option<i64>,
}

impl MemorySessionSource {
    /// 🌱️ A source seeded with sessions.
    pub fn seeded<I: IntoIterator<Item = (SessionKey, MemorySession)>>(entries: I) -> Self {
        Self { sessions: entries.into_iter().collect() }
    }
}

impl SessionSource for MemorySessionSource {
    fn sessions(&self) -> Vec<(i64, i64, i64, String)> {
        self.sessions.keys().map(|key| (key.year, key.month, key.day, key.uuid.clone())).collect()
    }

    fn meta(&self, key: &SessionKey) -> Option<String> {
        self.entry(key, "session.json")
    }

    fn entries(&self, key: &SessionKey) -> Vec<String> {
        self.sessions.get(key).map(|session| session.files.keys().cloned().collect()).unwrap_or_default()
    }

    fn entry(&self, key: &SessionKey, name: &str) -> Option<String> {
        self.sessions.get(key).and_then(|session| session.files.get(name).cloned())
    }

    fn age_seconds(&self, key: &SessionKey) -> Option<i64> {
        self.sessions.get(key).and_then(|session| session.age_seconds)
    }
}

/// ⏱️ How long a session may stay silent before it stops counting as running.
pub const RUNNING_WINDOW_SECONDS: i64 = 30 * 60;

/// 🕵️ The event kind whose presence ends a session.
pub const AGENT_ENDED_KIND: &str = "agent.ended";

/// 📡️ What became of a session, decided from its own files: a recorded agent-ended event
/// completes it, a recent write leaves it running, anything else interrupted it. A session with
/// no readable `session.json` falls back to the file names and their age.
pub fn derive_session_kind(source: &dyn SessionSource, key: &SessionKey) -> SessionKind {
    if let Some(document) = source.meta(key) {
        if let Ok(meta) = serde_json::from_str::<SessionMeta>(&document) {
            if meta.events.iter().any(|entry| entry.event.get("kind").and_then(serde_json::Value::as_str) == Some(AGENT_ENDED_KIND)) {
                return SessionKind::Completed;
            }
            if source.age_seconds(key).is_some_and(|age| age < RUNNING_WINDOW_SECONDS) {
                return SessionKind::Running;
            }
            if !meta.events.is_empty() {
                return SessionKind::Interrupted;
            }
        }
    }
    let entries = source.entries(key);
    if entries.iter().any(|name| name.contains("agent-ended")) {
        return SessionKind::Completed;
    }
    if entries.is_empty() {
        return SessionKind::Interrupted;
    }
    if source.age_seconds(key).is_some_and(|age| age < RUNNING_WINDOW_SECONDS) {
        return SessionKind::Running;
    }
    SessionKind::Interrupted
}

/// 💻️ The client of a session: the `session.json` member, else the first event that names one,
/// else the first event file that names one.
pub fn extract_session_client(source: &dyn SessionSource, key: &SessionKey) -> String {
    if let Some(document) = source.meta(key) {
        if let Ok(meta) = serde_json::from_str::<SessionMeta>(&document) {
            if !meta.client.is_empty() {
                return meta.client;
            }
            for entry in &meta.events {
                if let Some(client) = entry.event.get("client").and_then(serde_json::Value::as_str) {
                    if !client.is_empty() {
                        return client.to_string();
                    }
                }
            }
        }
    }
    for name in source.entries(key) {
        if name == "session.json" || !name.ends_with(".json") {
            continue;
        }
        let Some(document) = source.entry(key, &name) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&document) else { continue };
        if let Some(client) = value.pointer("/event/client").and_then(serde_json::Value::as_str) {
            if !client.is_empty() {
                return client.to_string();
            }
        }
    }
    String::new()
}

/// 📄️ The earliest second a session recorded, by the same three-step fallback as the client.
pub fn extract_session_second(source: &dyn SessionSource, key: &SessionKey) -> String {
    if let Some(document) = source.meta(key) {
        if let Ok(meta) = serde_json::from_str::<SessionMeta>(&document) {
            if !meta.second.is_empty() {
                return meta.second;
            }
            let mut earliest = String::new();
            for entry in &meta.events {
                if let Some(second) = entry.event.get("second").and_then(serde_json::Value::as_str) {
                    if !second.is_empty() && (earliest.is_empty() || second < earliest.as_str()) {
                        earliest = second.to_string();
                    }
                }
            }
            if !earliest.is_empty() {
                return earliest;
            }
        }
    }
    let mut earliest = String::new();
    for name in source.entries(key) {
        if name == "session.json" || !name.ends_with(".json") {
            continue;
        }
        let Some(document) = source.entry(key, &name) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&document) else { continue };
        if let Some(second) = value.pointer("/event/second").and_then(serde_json::Value::as_str) {
            if !second.is_empty() && (earliest.is_empty() || second < earliest.as_str()) {
                earliest = second.to_string();
            }
        }
    }
    earliest
}

/// 💾️ The checkpoint a session recorded in its `session.json`, empty when it recorded none.
pub fn extract_session_checkpoint(source: &dyn SessionSource, key: &SessionKey) -> String {
    source
        .meta(key)
        .and_then(|document| serde_json::from_str::<SessionMeta>(&document).ok())
        .map(|meta| meta.checkpoint)
        .unwrap_or_default()
}

/// 📂️ Every session the source carries, assembled from its own files.
pub fn list_sessions(source: &dyn SessionSource) -> Vec<Session> {
    source
        .sessions()
        .into_iter()
        .map(|(year, month, day, uuid)| {
            let key = SessionKey { year, month, day, uuid: uuid.clone() };
            Session {
                uuid,
                year,
                month,
                day,
                checkpoint: extract_session_checkpoint(source, &key),
                kind: derive_session_kind(source, &key).as_str().to_string(),
                client: extract_session_client(source, &key),
                llm: String::new(),
                started_at: extract_session_second(source, &key),
                ended_at: String::new(),
            }
        })
        .collect()
}

/// 🔎️ Every session whose identifier, kind or client contains the term, case-insensitively.
pub fn search_sessions(source: &dyn SessionSource, term: &str) -> Vec<Session> {
    let needle = term.to_lowercase();
    list_sessions(source)
        .into_iter()
        .filter(|session| needle.is_empty() || format!("{} {} {}", session.uuid, session.kind, session.client).to_lowercase().contains(&needle))
        .collect()
}

//#endregion 🪅️Sessions

//#region 🏁️Checkpoints

/// 📜️ The pretty format the checkpoint log is read in.
pub const CHECKPOINT_LOG_FORMAT: &str = "%H|%aN|%ad|%s";

/// 📖️ Where the checkpoint log comes from.
pub trait CheckpointSource {
    /// 📜️ The `git log --pretty=format:%H|%aN|%ad|%s --date=iso-strict` output, at most `limit`
    /// entries when a limit is given.
    fn log(&self, limit: Option<usize>) -> String;
}

/// 🧠️ The in-memory source, which serves a frozen log.
#[derive(Debug, Clone, Default)]
pub struct MemoryCheckpointSource {
    log: String,
}

impl MemoryCheckpointSource {
    /// 🌱️ A source that serves this log.
    pub fn new(log: &str) -> Self {
        Self { log: log.to_string() }
    }
}

impl CheckpointSource for MemoryCheckpointSource {
    fn log(&self, limit: Option<usize>) -> String {
        match limit {
            Some(limit) => self.log.lines().take(limit).collect::<Vec<_>>().join("\n"),
            None => self.log.clone(),
        }
    }
}

/// 🔀️ The production source: `git log` on the host, in [`CHECKPOINT_LOG_FORMAT`].
#[derive(Debug, Clone)]
pub struct GitCheckpoints {
    root: PathBuf,
}

impl GitCheckpoints {
    /// 🆕️ Reads the checkpoints of one repository root.
    pub fn new(root: impl Into<PathBuf>) -> GitCheckpoints {
        GitCheckpoints { root: root.into() }
    }
}

impl CheckpointSource for GitCheckpoints {
    fn log(&self, limit: Option<usize>) -> String {
        let count = format!("-n{}", limit.unwrap_or(DEFAULT_CHECKPOINT_LIMIT));
        let format = format!("--pretty=format:{CHECKPOINT_LOG_FORMAT}");
        match std::process::Command::new("git").args(["log", &count, &format, "--date=iso-strict"]).current_dir(&self.root).output() {
            Ok(outcome) => String::from_utf8_lossy(&outcome.stdout).to_string(),
            Err(_) => String::new(),
        }
    }
}

/// 🔢️ How many checkpoints the production source reads when the caller states no limit.
pub const DEFAULT_CHECKPOINT_LIMIT: usize = 200;

/// 📥️ Parses a checkpoint log. A line needs four `|` separated parts; the title keeps every `|`
/// it contains, and the author becomes the checkpoint's author identifier.
pub fn parse_checkpoint_log(log: &str) -> Vec<Checkpoint> {
    log.split('\n')
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 4 {
                return None;
            }
            Some(Checkpoint {
                id: parts[0].to_string(),
                sha: parts[0].to_string(),
                title: parts[3..].join("|"),
                author_id: if parts[1].is_empty() { None } else { Some(parts[1].to_string()) },
                date: parts[2].to_string(),
            })
        })
        .collect()
}

/// 📋️ Every checkpoint the source carries, newest first.
pub fn list_checkpoints(source: &dyn CheckpointSource, limit: Option<usize>) -> Vec<Checkpoint> {
    parse_checkpoint_log(&source.log(limit))
}

/// 🔎️ Every checkpoint whose sha or title contains the term, case-insensitively.
pub fn search_checkpoints(source: &dyn CheckpointSource, limit: Option<usize>, term: &str) -> Vec<Checkpoint> {
    let needle = term.to_lowercase();
    list_checkpoints(source, limit)
        .into_iter()
        .filter(|checkpoint| needle.is_empty() || format!("{} {}", checkpoint.sha, checkpoint.title).to_lowercase().contains(&needle))
        .collect()
}

/// 🪪️ The artifact identifier of a checkpoint: the contributor segment, when the checkpoint names
/// an author, followed by the checkpoint segment carrying the full sha.
pub fn checkpoint_id(checkpoint: &Checkpoint) -> String {
    let contributor = checkpoint
        .author_id
        .as_deref()
        .filter(|author| !author.is_empty())
        .map(|author| segment("contributor", &identity::flat(author), ""))
        .unwrap_or_default();
    segment("checkpoint", &checkpoint.sha, &contributor)
}

//#endregion 🏁️Checkpoints

//#region 🎁️Interactions

/// 🎁️ Where an interaction was recorded.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InteractionOrigin {
    /// 🏷️ `ticket` or `goal`.
    #[serde(rename = "sourceKind")]
    pub source_kind: String,
    /// 🪪️ The identifier of the artifact.
    #[serde(rename = "sourceId")]
    pub source_id: String,
    /// 🎯️ The goal the artifact belongs to, empty when it belongs to none.
    #[serde(rename = "goalId", default, skip_serializing_if = "String::is_empty")]
    pub goal_id: String,
    /// 🎫️ The ticket the artifact is, empty when the artifact is not a ticket.
    #[serde(rename = "ticketId", default, skip_serializing_if = "String::is_empty")]
    pub ticket_id: String,
}

/// ✍️ The author of a sequence of interactions: the first one that names an author. An artifact
/// whose interactions name nobody has no author.
pub fn interactions_author(interactions: &[Interaction]) -> String {
    interactions.iter().map(|interaction| interaction.author.clone()).find(|author| !author.is_empty()).unwrap_or_default()
}

/// 💾️ The checkpoint of a sequence of interactions: the first one that names a checkpoint.
pub fn interactions_checkpoint(interactions: &[Interaction]) -> String {
    interactions.iter().map(|interaction| interaction.checkpoint.clone()).find(|checkpoint| !checkpoint.is_empty()).unwrap_or_default()
}

/// 🧑️ The interactions of one origin, resolved to contributor aliases through the store.
pub fn attribute_interactions(store: &dyn ContributorStore, origin: &InteractionOrigin, interactions: &[Interaction]) -> Vec<(InteractionOrigin, String)> {
    interactions
        .iter()
        .map(|interaction| {
            let parsed = parse_git_author(&interaction.author);
            (origin.clone(), resolve_author_to_alias(store, &parsed.name, &parsed.email))
        })
        .collect()
}

//#endregion 🎁️Interactions

//#region 🕰️Clock

/// 🕰️ The instant a decision about elapsed time is made against.
pub trait Clock {
    /// ⏱️ Seconds since the Unix epoch.
    fn now_seconds(&self) -> i64;
}

/// 📌️ A clock frozen at one instant.
#[derive(Debug, Clone, Copy)]
pub struct FixedClock {
    /// ⏱️ The frozen instant, in seconds since the Unix epoch.
    pub instant: i64,
}

impl Clock for FixedClock {
    fn now_seconds(&self) -> i64 {
        self.instant
    }
}

/// 🕰️ The clock of the machine this runs on.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_seconds(&self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64).unwrap_or_default()
    }
}

//#endregion 🕰️Clock

//#region 🧾️Recording

/// 🧠️ An emitter that keeps `kind\tsource\tpayload` lines instead of reaching a coordinator.
#[derive(Debug, Default)]
pub struct MemoryEmitter {
    envelopes: RefCell<Vec<String>>,
}

impl MemoryEmitter {
    /// 🆕️ An emitter with an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📜️ Every envelope emitted so far, in order.
    pub fn envelopes(&self) -> Vec<String> {
        self.envelopes.borrow().clone()
    }
}

impl semio_framework_repo_events::Emitter for MemoryEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
        self.envelopes.borrow_mut().push(format!("{kind}\t{source}\t{payload}"));
    }
}

//#endregion 🧾️Recording

//#region 💽️ContributorWrites
// 💽️ The write half of the contributor registry: the four filesystem operations `contributor add`
// and `contributor remove` need, plus the git identity the emitted events are attributed to.

/// 📁️ The directory one contributor's document lives in.
pub fn contributor_dir(repo_root: &Path, alias: &str) -> PathBuf {
    semio_framework_repo_workspace::devs_dir_for_root(repo_root).join(alias)
}

/// 📄️ The contributor document path of one alias.
pub fn contributor_document(repo_root: &Path, alias: &str) -> PathBuf {
    contributor_dir(repo_root, alias).join("🧑️‍💻️contributor.json")
}

/// 🔖️ Reads one contributor, defaulting a blank alias to the directory name.
///
/// Twin of `LoadContributor` in `📦️packages/🐹️go/🐹️.go`, refusal wording included.
pub fn load_contributor(repo_root: &Path, alias: &str) -> Result<Contributor, String> {
    let path = contributor_document(repo_root, alias);
    if !path.is_file() {
        return Err(format!("contributor not found: {alias}"));
    }
    let document = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let mut contributor: Contributor = serde_json::from_str(&document).map_err(|error| error.to_string())?;
    if contributor.alias.is_empty() {
        contributor.alias = alias.to_string();
    }
    Ok(contributor)
}

/// 🆕️ Creates an empty contributor document for an alias that has none yet.
///
/// Twin of `CreateContributor`: an existing directory is the refusal `contributor already exists`.
pub fn create_contributor(repo_root: &Path, alias: &str) -> Result<Contributor, String> {
    let dir = contributor_dir(repo_root, alias);
    if dir.exists() {
        return Err(format!("contributor already exists: {alias}"));
    }
    std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let contributor: Contributor = serde_json::from_value(serde_json::json!({ "alias": alias })).map_err(|error| error.to_string())?;
    write_contributor_document(&dir, &contributor)?;
    Ok(contributor)
}

/// 🏪️ Persists one contributor, defaulting a blank alias to its handle.
///
/// Twin of `SaveContributor`, including the two-space `json.MarshalIndent` encoding.
pub fn save_contributor(repo_root: &Path, contributor: &Contributor) -> Result<(), String> {
    let mut contributor = contributor.clone();
    if contributor.alias.is_empty() {
        contributor.alias = contributor.github.clone();
    }
    let dir = contributor_dir(repo_root, &contributor.alias);
    std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    write_contributor_document(&dir, &contributor)
}

/// 🚚️ Deletes one contributor's whole directory.
///
/// Twin of `RemoveContributor`, refusal wording included.
pub fn remove_contributor(repo_root: &Path, alias: &str) -> Result<(), String> {
    let dir = contributor_dir(repo_root, alias);
    if !dir.exists() {
        return Err(format!("contributor not found: {alias}"));
    }
    std::fs::remove_dir_all(&dir).map_err(|error| error.to_string())
}

/// ✍️ Writes the document with Go's `json.MarshalIndent(c, "", "  ")` encoding.
fn write_contributor_document(dir: &Path, contributor: &Contributor) -> Result<(), String> {
    let document = serde_json::to_string_pretty(contributor).map_err(|error| error.to_string())?;
    std::fs::write(dir.join("🧑️‍💻️contributor.json"), document).map_err(|error| error.to_string())
}

/// 👤️ The alias the invoking git identity resolves to, or the raw author line when nobody claims it.
///
/// Twin of `GetGitAuthorAlias`: `user.name <user.email>` looked up in the registry.
pub fn git_author_alias(repo_root: &Path) -> String {
    let read = |key: &str| -> String {
        std::process::Command::new("git")
            .args(["config", "--get", key])
            .current_dir(repo_root)
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_default()
    };
    let name = read("user.name");
    let email = read("user.email");
    let fallback = if email.is_empty() { name } else { format!("{name} <{email}>") };
    find_contributor(&FsContributorStore::new(repo_root), &fallback)
}
//#endregion 💽️ContributorWrites
