//! 🧭️ The non-interactive command line of `semio`: the same registry and the same workspace daemon
//! the dashboard drives, as verbs for developers, scripts and agents — `commands`, `run`, `tasks`,
//! `logs`, `stop`, `restart`, `kill` and `open`. This module parses arguments and renders results;
//! what the daemon does lives in `🌀️daemon/🕹️control`.
//!
//! Agents start servers with `semio run <id> --detach --wait-ready` and attach previews to the
//! printed address; no editor or agent launch file exists.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🕹️control/🦀️.rs

use crate::daemon::client::Connection;
use crate::daemon::control::{self as daemon_control, Action, Followed, RunError, Wait};
use crate::daemon::ipc::{self, ClientMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup};
use std::io::{IsTerminal, Write};
use crate::registry::{ParameterKind, Registry, Request, RunPolicy};
use crate::repo_domain::RepoImplementation;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

// #region 🔖️Arguments
/// ✂️ The arguments of one verb: words, switches, valued flags (repeatable) and everything after `--`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Arguments {
    pub words: Vec<String>,
    pub switches: Vec<String>,
    pub values: Vec<(String, String)>,
    pub rest: Vec<String>,
}

impl Arguments {
    /// 🪚️ Splits `argv` (without the verb): `--name value` and `--name=value` for the `valued` flags,
    /// bare `--name` for every other flag, `--` ends the flags and keeps the rest verbatim.
    pub fn parse(argv: &[String], valued: &[&str]) -> Result<Self, String> {
        let mut parsed = Self::default();
        let mut iter = argv.iter();
        while let Some(word) = iter.next() {
            if word == "--" { parsed.rest.extend(iter.by_ref().cloned()); break; }
            let Some(flag) = word.strip_prefix("--") else { parsed.words.push(word.clone()); continue };
            match flag.split_once('=') {
                Some((name, value)) if valued.contains(&name) => parsed.values.push((name.into(), value.into())),
                Some((name, _)) => return Err(format!("--{name} takes no value")),
                None if valued.contains(&flag) => parsed.values.push((flag.into(), iter.next().ok_or_else(|| format!("--{flag} needs a value"))?.clone())),
                None => parsed.switches.push(flag.into()),
            }
        }
        Ok(parsed)
    }

    pub fn has(&self, name: &str) -> bool { self.switches.iter().any(|known| known == name) }
    pub fn value(&self, name: &str) -> Option<&str> { self.values.iter().rev().find(|(known, _)| known == name).map(|(_, value)| value.as_str()) }

    /// 🔑️ Every `--<name> key=value` as a pair.
    pub fn pairs(&self, name: &str) -> Result<Vec<(String, String)>, String> {
        self.values.iter().filter(|(known, _)| known == name).map(|(_, text)| text.split_once('=').filter(|(key, _)| !key.is_empty()).map(|(key, value)| (key.to_string(), value.to_string())).ok_or_else(|| format!("--{name} {text:?} is not `key=value`"))).collect()
    }

    /// 🧾️ The registry request these arguments state: `--param`, `--env` and everything after `--`.
    /// `--param id` without a value switches a `flag` parameter on; `kind_of` tells what kind of parameter an id is.
    pub fn request(&self, kind_of: &dyn Fn(&str) -> Option<ParameterKind>) -> Result<Request, String> {
        let parameters = self.all("param").map(|text| match text.split_once('=') {
            Some((key, value)) if !key.is_empty() => Ok((key.to_string(), value.to_string())),
            Some(_) => Err(format!("--param {text:?} is not `id` or `id=value`")),
            None => match kind_of(text) {
                Some(ParameterKind::Flag) | None => Ok((text.to_string(), "true".to_string())),
                Some(kind) => Err(format!("--param {text}: a {} parameter needs a value, write `--param {text}=<value>`", match kind { ParameterKind::Choice => "choice", ParameterKind::Text => "text", ParameterKind::Flag => "flag" })),
            },
        }).collect::<Result<_, _>>()?;
        Ok(Request { parameters, args: self.rest.clone(), env: self.pairs("env")? })
    }

    fn all<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a str> { self.values.iter().filter(move |(known, _)| known == name).map(|(_, value)| value.as_str()) }

    fn seconds(&self, default: u64) -> Duration { Duration::from_secs(self.value("timeout").and_then(|text| text.parse().ok()).unwrap_or(default)) }
}

fn parse(argv: &[String], valued: &[&str]) -> Result<Arguments, i32> { Arguments::parse(argv, valued).map_err(|error| refuse(&error)) }

fn refuse(message: &str) -> i32 {
    eprintln!("[semio] {message}");
    2
}
// #endregion 🔖️Arguments

// #region 🔖️Commands
fn root_of(root: &Path, arguments: &Arguments) -> PathBuf { arguments.value("root").map_or_else(|| root.to_path_buf(), PathBuf::from) }

fn registry_of(root: &Path, arguments: &Arguments) -> Registry {
    let snapshot = arguments.value("snapshot").map_or_else(|| crate::inventory::cache_path(root), PathBuf::from);
    if arguments.has("refresh") || arguments.has("check") { crate::inventory::discover(root, &AtomicBool::new(false)) } else { crate::inventory::registry_at(root, &snapshot) }
}

/// ⌨️ `semio commands [words…] [--json] [--all] [--check] [--refresh] [--root PATH] [--snapshot PATH]`
/// lists the registry commands whose label or id contains every word; `--check` proves every
/// declaration of the workspace and exits non-zero on any problem.
pub fn commands(root: &Path, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["root", "snapshot"]) { Ok(arguments) => arguments, Err(code) => return code };
    let registry = registry_of(&root_of(root, &arguments), &arguments);
    if arguments.has("check") {
        let problems = registry.check();
        for problem in &problems { eprintln!("{problem}"); }
        println!("{} commands, {} problems", registry.entries().len(), problems.len());
        return i32::from(!problems.is_empty());
    }
    let words: Vec<&str> = arguments.words.iter().map(String::as_str).collect();
    let found = registry.search(&words, arguments.has("all"));
    if arguments.has("json") {
        println!("{}", serde_json::Value::Array(found.iter().map(|position| registry.entry_json(&registry.entries()[*position])).collect()));
        return 0;
    }
    for position in found {
        let entry = &registry.entries()[position];
        let parameters = registry.parameters(entry).iter().map(|parameter| if parameter.values.is_empty() { parameter.id.clone() } else { format!("{}={}", parameter.id, parameter.values.iter().map(|(value, _)| value.as_str()).collect::<Vec<_>>().join("|")) }).collect::<Vec<_>>().join(" ");
        println!("{}\t{}\t{}{}{}", entry.id, entry.label, if entry.long_running { "long-running" } else { "finite" }, if entry.mutating { " mutating" } else { "" }, if parameters.is_empty() { String::new() } else { format!("\t{parameters}") });
    }
    0
}
// #endregion 🔖️Commands

// #region 🔖️Rendering
fn label_text(session: &SessionInfo) -> String {
    let label = &session.command.label;
    if label.subject.is_empty() { return session.command.cmd.clone(); }
    [label.verb.as_str(), label.subject.as_str(), label.qualifier.as_str()].iter().filter(|part| !part.is_empty()).copied().collect::<Vec<_>>().join(" ")
}

fn status_text(session: &SessionInfo) -> String {
    serde_json::to_value(session.status).ok().and_then(|value| value.as_str().map(str::to_string)).unwrap_or_default()
}

fn session_json(session: &SessionInfo) -> serde_json::Value {
    serde_json::json!({
        "session": session.session_id, "commandId": session.command.command_id, "label": label_text(session), "status": session.status,
        "pid": session.pid, "code": session.code, "startedMs": session.started_ms, "endedMs": session.ended_ms,
        "readyUrl": session.ready_url, "group": session.group, "title": session.title,
    })
}

fn print_sessions(sessions: &[SessionInfo], json: bool) {
    if json { println!("{}", serde_json::Value::Array(sessions.iter().map(session_json).collect())); return; }
    for session in sessions { println!("{}\t{}\t{}\t{}", session.session_id, status_text(session), label_text(session), session.ready_url.as_deref().unwrap_or("")); }
}
// #endregion 🔖️Rendering

// #region 🔖️Daemon
const READY_TIMEOUT_SECONDS: u64 = 600;
const QUERY: Duration = Duration::from_secs(5);
static NEVER: AtomicBool = AtomicBool::new(false);

fn connect(root: &Path) -> Result<Connection, String> {
    Connection::connect(&ipc::canonical_path(root)).map_err(|error| format!("no dashboard daemon answers for this workspace ({error}); `semio run` starts one"))
}

fn connect_or_start(root: &Path) -> Result<Connection, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    daemon_control::connect_or_start(&ipc::canonical_path(root), &executable).map_err(|error| format!("cannot reach the dashboard daemon: {error}"))
}

/// 📋️ The sessions of a workspace: the daemon's when one answers, else the journal's.
fn sessions_of(root: &Path) -> Vec<SessionInfo> {
    match connect(root).and_then(|mut connection| daemon_control::tasks(&mut connection, QUERY).map_err(|error| error.to_string())) {
        Ok(sessions) => sessions,
        Err(_) => daemon_control::offline_tasks(&ipc::canonical_path(root)),
    }
}

fn terminal_size() -> (u16, u16) {
    let read = |name: &str, default: u16| std::env::var(name).ok().and_then(|text| text.parse::<u16>().ok()).filter(|value| *value > 0).unwrap_or(default);
    (read("COLUMNS", 120), read("LINES", 32))
}

fn sized(mut group: SpawnGroup) -> SpawnGroup {
    let (cols, rows) = terminal_size();
    for member in group.members.iter_mut().chain(group.requires.iter_mut()) { (member.command.cols, member.command.rows) = (cols, rows); }
    group
}
// #endregion 🔖️Daemon

// #region 🔖️Handles
/// 🎯️ The sessions a developer's handle names. The handle printed by `run`, `tasks` and every other verb is the
/// session id, which never changes; the same verbs also accept a group id (every member of a launch), a command id
/// (its live session, else its latest) and a unique part of a session id or of a command id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picked<'a> {
    pub sessions: Vec<&'a SessionInfo>,
    /// 🧩️ The group id when the handle named a launch as a whole.
    pub group: Option<String>,
}

fn listed(sessions: &[&SessionInfo]) -> String { sessions.iter().map(|session| format!("{} ({})", session.session_id, session.command.command_id)).collect::<Vec<_>>().join(", ") }

/// 🧲️ Resolves a handle to sessions; an ambiguous handle is an error that lists the candidates.
pub fn pick<'a>(sessions: &'a [SessionInfo], needle: &str) -> Result<Picked<'a>, String> {
    if needle.is_empty() { return Err("a task handle is empty; `semio tasks` lists them".into()); }
    let single = |session: &'a SessionInfo| Ok(Picked { sessions: vec![session], group: None });
    if let Some(exact) = sessions.iter().find(|session| session.session_id == needle) { return single(exact); }
    let members: Vec<&SessionInfo> = sessions.iter().filter(|session| session.group.as_deref() == Some(needle) || session.session_id.strip_prefix(needle).is_some_and(|rest| rest.starts_with('.'))).collect();
    if !members.is_empty() { return Ok(Picked { sessions: members, group: Some(needle.to_string()) }); }
    let of_command = |matching: Vec<&'a SessionInfo>| -> Result<Picked<'a>, String> {
        let live: Vec<&SessionInfo> = matching.iter().copied().filter(|session| session.status.live()).collect();
        match live.len() {
            0 => matching.into_iter().max_by_key(|session| session.started_ms).map_or_else(|| Err(format!("no task matches {needle:?}; `semio tasks` lists them")), |latest| Ok(Picked { sessions: vec![latest], group: None })),
            1 => Ok(Picked { sessions: live, group: None }),
            _ => Err(format!("{needle:?} has {} live tasks: {}; name one session id", live.len(), listed(&live))),
        }
    };
    let by_command: Vec<&SessionInfo> = sessions.iter().filter(|session| session.command.command_id == needle).collect();
    if !by_command.is_empty() { return of_command(by_command); }
    let by_prefix: Vec<&SessionInfo> = sessions.iter().filter(|session| session.session_id.starts_with(needle)).collect();
    match by_prefix.len() {
        0 => {}
        1 => return single(by_prefix[0]),
        _ => return Err(format!("{needle:?} matches {} tasks: {}; name one session id", by_prefix.len(), listed(&by_prefix))),
    }
    let by_part: Vec<&SessionInfo> = sessions.iter().filter(|session| !session.command.command_id.is_empty() && session.command.command_id.contains(needle)).collect();
    let mut commands: Vec<&str> = by_part.iter().map(|session| session.command.command_id.as_str()).collect();
    commands.sort_unstable();
    commands.dedup();
    match commands.len() {
        0 => Err(format!("no task matches {needle:?}; `semio tasks` lists them")),
        1 => of_command(by_part),
        _ => Err(format!("{needle:?} matches several commands: {}; name one", commands.join(", "))),
    }
}

/// 1️⃣ The one session a verb that needs exactly one works on.
fn only<'a>(picked: &Picked<'a>, needle: &str) -> Result<&'a SessionInfo, String> {
    match picked.sessions.as_slice() {
        [session] => Ok(session),
        many => Err(format!("{needle:?} names {} tasks of one launch: {}; name one session id", many.len(), listed(many))),
    }
}
// #endregion 🔖️Handles

// #region 🔖️Output
/// 🧹️ Removes terminal control sequences from a byte stream that is not a terminal, across chunk borders.
#[derive(Debug, Default)]
pub struct Plain { state: Escape }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Escape { #[default] Text, Introducer, Csi, Operating, OperatingEnd, Other }

impl Plain {
    /// 🧽️ The text bytes of `bytes`: CSI, OSC, DCS and two-byte escape sequences are dropped, other control bytes but tab, newline and carriage return too.
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut text = Vec::with_capacity(bytes.len());
        for &byte in bytes {
            self.state = match (self.state, byte) {
                (Escape::Text, 0x1b) => Escape::Introducer,
                (Escape::Text, b'\t' | b'\n' | b'\r') => { text.push(byte); Escape::Text }
                (Escape::Text, 0..=0x1f | 0x7f) => Escape::Text,
                (Escape::Text, _) => { text.push(byte); Escape::Text }
                (Escape::Introducer, b'[') => Escape::Csi,
                (Escape::Introducer, b']' | b'P' | b'X' | b'^' | b'_') => Escape::Operating,
                (Escape::Introducer, 0x20..=0x2f) => Escape::Other,
                (Escape::Introducer, 0x1b) => Escape::Introducer,
                (Escape::Introducer, _) => Escape::Text,
                (Escape::Other, 0x20..=0x2f) => Escape::Other,
                (Escape::Other, _) => Escape::Text,
                (Escape::Csi, 0x40..=0x7e) => Escape::Text,
                (Escape::Csi, _) => Escape::Csi,
                (Escape::Operating, 0x07) => Escape::Text,
                (Escape::Operating, 0x1b) => Escape::OperatingEnd,
                (Escape::Operating, _) => Escape::Operating,
                (Escape::OperatingEnd, b'\\') => Escape::Text,
                (Escape::OperatingEnd, _) => Escape::Operating,
            };
        }
        text
    }
}

/// 📤️ Standard output that is plain text unless it is a terminal or `--raw` asks for the bytes as the task wrote them.
struct Output { stdout: std::io::Stdout, plain: Option<Plain> }

impl Output {
    fn new(raw: bool) -> Self {
        let stdout = std::io::stdout();
        Self { plain: (!raw && !stdout.is_terminal()).then(Plain::default), stdout }
    }
}

impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        match &mut self.plain {
            Some(plain) => self.stdout.write_all(&plain.feed(bytes))?,
            None => self.stdout.write_all(bytes)?,
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> { self.stdout.flush() }
}
// #endregion 🔖️Output

// #region 🔖️Run
/// ▶️ `semio run <id> [--param k=v]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--timeout S] [-- args…]`
/// resolves a registry command and runs it as a daemon task. Attached, it streams the output of the
/// launch's last process and exits with that process's exit code (Ctrl-C leaves the task running;
/// stop it with `semio stop`). `--detach` returns once the daemon accepted the launch; `--wait-ready`
/// returns once every process that declares readiness printed its address (the addresses are the
/// last lines of stdout); `--dry-run` prints the resolved launch as JSON and starts nothing.
pub fn run(root: &Path, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["param", "env", "root", "snapshot", "timeout"]) { Ok(arguments) => arguments, Err(code) => return code };
    let [id] = arguments.words.as_slice() else { return refuse("usage: semio run <id> [--param k=v]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [-- args…]") };
    let root = root_of(root, &arguments);
    let registry = registry_of(&root, &arguments);
    let accepted = registry.find(id).map(|entry| registry.parameters(entry)).unwrap_or_default();
    let request = match arguments.request(&|name| accepted.iter().find(|parameter| parameter.id == name).map(|parameter| parameter.kind)) { Ok(request) => request, Err(error) => return refuse(&error) };
    let launch = match registry.resolve(id, &request) { Ok(launch) => launch, Err(error) => return refuse(&error) };
    if arguments.has("dry-run") { println!("{}", registry.launch_plan_json(&launch)); return 0; }
    if let Some(action) = registry.find(id).and_then(|entry| entry.repo_action()).filter(|_| RunPolicy::current().repo == RepoImplementation::Rust) {
        println!("{}", action.execute(&root));
        return 0;
    }
    let result = connect_or_start(&root).and_then(|mut connection| start(&mut connection, SpawnGroup::from_launch(&launch, &[]), &arguments));
    result.unwrap_or_else(|error| refuse(&error))
}

fn start(connection: &mut Connection, group: SpawnGroup, arguments: &Arguments) -> Result<i32, String> {
    let (detach, wait_ready, json) = (arguments.has("detach"), arguments.has("wait-ready"), arguments.has("json"));
    let timeout = arguments.seconds(READY_TIMEOUT_SECONDS);
    let group = sized(group);
    let waiting = if wait_ready { Wait::Ready } else { Wait::Accepted };
    let existing = daemon_control::tasks(connection, QUERY).map_err(|error| error.to_string())?;
    let running: Vec<Option<&SessionInfo>> = group.members.iter().map(|member| existing.iter().filter(|session| session.status.live() && same_launch(&session.command, &member.command)).max_by_key(|session| session.started_ms)).collect();
    let mut last = String::new();
    let mut progress = |session: &SessionInfo| {
        let line = format!("{} {}", label_text(session), status_text(session));
        if line != last { eprintln!("[semio] {line}"); last = line; }
    };
    let (sessions, urls) = if running.iter().all(Option::is_some) {
        let ids: Vec<String> = running.iter().flatten().map(|session| session.session_id.clone()).collect();
        let sessions = wait_for(connection, &ids, waiting, timeout)?;
        let urls = sessions.iter().filter_map(|session| session.ready_url.clone()).collect::<Vec<_>>();
        (sessions, urls)
    } else {
        match daemon_control::run(connection, group, waiting, Some(timeout), &NEVER, &mut progress) {
            Ok(started) => { let urls = started.ready.iter().map(|ready| ready.url.clone()).collect::<Vec<_>>(); (started.sessions, urls) }
            Err(RunError::Refused { code, message, .. }) => return Err(format!("{message} ({code})")),
            Err(error) => return Err(error.to_string()),
        }
    };
    if detach || wait_ready {
        print_sessions(&sessions, json);
        if !json { for url in &urls { println!("{url}"); } }
        if wait_ready && urls.is_empty() { eprintln!("[semio] the command declares no ready address; it is started"); }
        return Ok(0);
    }
    let Some(last) = sessions.last() else { return Ok(0) };
    let mut stdout = Output::new(arguments.has("raw"));
    let followed = daemon_control::follow(connection, &last.session_id, &mut stdout, true, &NEVER).map_err(|error| error.to_string())?;
    Ok(match followed { Followed::Ended(session) => session.code.unwrap_or(1), Followed::Alive(_) | Followed::Cancelled => 0 })
}

/// ♻️ Whether a running task is the launch being asked for: same command, same words and same chosen
/// parameters. The daemon normalises working directory and environment, so those are not compared.
fn same_launch(running: &SessionCommand, asked: &SessionCommand) -> bool {
    running.command_id == asked.command_id && running.cmd == asked.cmd && running.args == asked.args && running.label.parameters == asked.label.parameters
}

fn wait_for(connection: &mut Connection, ids: &[String], wait: Wait, timeout: Duration) -> Result<Vec<SessionInfo>, String> {
    let deadline = Instant::now() + timeout;
    loop {
        let sessions = daemon_control::tasks(connection, QUERY).map_err(|error| error.to_string())?;
        let ours: Vec<SessionInfo> = ids.iter().filter_map(|id| sessions.iter().find(|session| &session.session_id == id)).cloned().collect();
        if let Some(ended) = ours.iter().find(|session| !session.status.live() && session.ready_url.is_none() && wait == Wait::Ready) { return Err(RunError::Ended(Box::new(ended.clone())).to_string()); }
        if wait == Wait::Accepted || ours.iter().all(|session| session.command.ready.is_none() || session.ready_url.is_some()) { return Ok(ours); }
        if Instant::now() >= deadline { return Err(RunError::Timeout.to_string()); }
        std::thread::sleep(Duration::from_millis(250));
    }
}
// #endregion 🔖️Run

// #region 🔖️Tasks
/// 🗒️ `semio tasks [--json]` lists the tasks of the workspace: the daemon's, or the journal's while none runs.
pub fn tasks(root: &Path, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };
    print_sessions(&sessions_of(&root_of(root, &arguments)), arguments.has("json"));
    0
}

/// 📜️ `semio logs <task> [--follow]` prints the output the daemon kept for a task; `--follow` keeps
/// streaming until the task ends. A task is chosen by its session id (the handle every verb prints), a group
/// id, a command id or a unique part of a session id or command id.
pub fn logs(root: &Path, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };
    let [needle] = arguments.words.as_slice() else { return refuse("usage: semio logs <task> [--follow]") };
    let root = ipc::canonical_path(&root_of(root, &arguments));
    let sessions = sessions_of(&root);
    let session = match pick(&sessions, needle).and_then(|picked| only(&picked, needle)) { Ok(session) => session, Err(error) => return refuse(&error) };
    let mut stdout = Output::new(arguments.has("raw"));
    let result = match connect(&root) {
        Ok(mut connection) => daemon_control::follow(&mut connection, &session.session_id, &mut stdout, arguments.has("follow"), &NEVER).map(|_| ()),
        Err(_) => daemon_control::logs_offline(&root, &session.session_id, &mut stdout).map(|_| ()),
    };
    match result { Ok(()) => 0, Err(error) => refuse(&error.to_string()) }
}

/// 🛑️ `semio stop|restart|kill <task> [--group] [--timeout S]` asks the daemon and waits until the
/// task has left the state it was in; `--group` stops or kills every member of the task's compound.
pub fn act(root: &Path, verb: &str, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["root", "timeout"]) { Ok(arguments) => arguments, Err(code) => return code };
    let [needle] = arguments.words.as_slice() else { return refuse(&format!("usage: semio {verb} <task> [--group]")) };
    let root = root_of(root, &arguments);
    let result = (|| -> Result<i32, String> {
        let mut connection = connect(&root)?;
        let sessions = daemon_control::tasks(&mut connection, QUERY).map_err(|error| error.to_string())?;
        let picked = pick(&sessions, needle)?;
        let wait = arguments.seconds(30);
        let whole = if arguments.has("group") { Some(picked.sessions.first().and_then(|session| session.group.clone()).ok_or_else(|| format!("{needle:?} belongs to no compound"))?) } else { None };
        if let Some(group) = whole {
            connection.send(&match verb { "kill" => ClientMsg::KillGroup { group_id: group.clone() }, "stop" => ClientMsg::StopGroup { group_id: group.clone() }, _ => return Err("--group stops or kills; it does not restart".into()) }).map_err(|error| error.to_string())?;
            let deadline = Instant::now() + wait;
            loop {
                let now = daemon_control::tasks(&mut connection, QUERY).map_err(|error| error.to_string())?;
                let members: Vec<&SessionInfo> = now.iter().filter(|known| known.group.as_deref() == Some(group.as_str())).collect();
                if members.iter().all(|known| !known.status.live()) { for known in members { println!("{}\t{}", known.session_id, status_text(known)); } return Ok(0); }
                if Instant::now() >= deadline { return Err(RunError::Timeout.to_string()); }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        let action = match verb { "stop" => Action::Stop, "kill" => Action::Kill, _ => Action::Restart };
        let targets: Vec<&SessionInfo> = if action == Action::Restart { picked.sessions.clone() } else { picked.sessions.iter().copied().filter(|session| session.status.live()).collect() };
        if targets.is_empty() { return Err(format!("{needle:?} has no live task to {verb}; {}", listed(&picked.sessions))); }
        for session in targets {
            let after = daemon_control::act(&mut connection, &session.session_id, action, Some(wait), &NEVER).map_err(|error| error.to_string())?;
            println!("{}\t{}", after.session_id, status_text(&after));
        }
        Ok(0)
    })();
    result.unwrap_or_else(|error| refuse(&error))
}

/// 🌐️ `semio open <task> [--print]` opens the ready address of a task in the system browser.
pub fn open(root: &Path, argv: &[String]) -> i32 {
    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };
    let [needle] = arguments.words.as_slice() else { return refuse("usage: semio open <task> [--print]") };
    let sessions = sessions_of(&root_of(root, &arguments));
    let session = match pick(&sessions, needle).and_then(|picked| { let ready: Vec<&SessionInfo> = picked.sessions.iter().copied().filter(|session| session.ready_url.is_some()).collect(); only(&Picked { sessions: if ready.len() == 1 { ready } else { picked.sessions }, group: None }, needle) }) { Ok(session) => session, Err(error) => return refuse(&error) };
    let Some(url) = session.ready_url.as_deref().filter(|_| session.status != SessionStatus::Interrupted) else { return refuse(&format!("{} has no ready address", label_text(session))) };
    println!("{url}");
    if arguments.has("print") { return 0; }
    match browser(url) { Ok(()) => 0, Err(error) => refuse(&format!("cannot open a browser: {error}")) }
}

fn browser(url: &str) -> std::io::Result<()> {
    let mut command = if cfg!(windows) { let mut command = std::process::Command::new("cmd"); command.args(["/c", "start", "", url]); command } else if cfg!(target_os = "macos") { let mut command = std::process::Command::new("open"); command.arg(url); command } else { let mut command = std::process::Command::new("xdg-open"); command.arg(url); command };
    command.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().map(|_| ())
}
// #endregion 🔖️Tasks

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
