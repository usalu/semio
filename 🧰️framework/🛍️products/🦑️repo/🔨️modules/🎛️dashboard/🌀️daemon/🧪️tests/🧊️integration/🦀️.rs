//! 🧊 The daemon with real processes on this host: a pseudo-terminal child, the real transport, real views.
//! A task of these tests is this very test binary, started with `SEMIO_DAEMON_TEST_CHILD` naming what
//! it should do; no other program has to be installed.

use super::*;
use client::{Connection, Message};
use ipc::{ClientMsg, Ready, ServerMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup, TaskLabel};
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const CHILD: &str = "SEMIO_DAEMON_TEST_CHILD";

// #region 🔖️Child
#[cfg(windows)]
mod platform {
    extern "system" {
        fn SetConsoleCtrlHandler(handler: Option<unsafe extern "system" fn(u32) -> i32>, add: i32) -> i32;
        fn ExitProcess(code: u32) -> !;
    }

    unsafe extern "system" fn handler(event: u32) -> i32 {
        if event == 1 {
            println!("got-break");
            unsafe { ExitProcess(9) }
        }
        1
    }

    pub fn ignore_interrupts_and_end_on_termination() { unsafe { SetConsoleCtrlHandler(Some(handler), 1) }; }
}

#[cfg(unix)]
mod platform {
    static TERMINATED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    extern "C" {
        fn signal(signal: i32, handler: usize) -> usize;
    }

    extern "C" fn terminated(_signal: i32) { TERMINATED.store(true, std::sync::atomic::Ordering::SeqCst); }

    pub fn ignore_interrupts_and_end_on_termination() {
        unsafe {
            signal(2, 1);
            signal(15, terminated as extern "C" fn(i32) as usize);
        }
        std::thread::spawn(|| loop {
            if TERMINATED.load(std::sync::atomic::Ordering::SeqCst) {
                println!("got-term");
                std::process::exit(9);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
    }
}

fn idle() -> ! {
    loop { std::thread::sleep(Duration::from_millis(50)); }
}

#[test]
fn child() {
    let Ok(mode) = std::env::var(CHILD) else { return };
    let parts: Vec<&str> = mode.split(':').collect();
    let number = |index: usize| parts.get(index).and_then(|part| part.parse::<u64>().ok()).unwrap_or(0);
    match parts[0] {
        "print" => { for index in 0..number(1) { println!("child-output-line-{index}"); } std::process::exit(0) }
        "exit" => { println!("exiting with {}", number(1)); std::process::exit(number(1) as i32) }
        "serve" => {
            std::thread::sleep(Duration::from_millis(number(2)));
            let _listener = std::net::TcpListener::bind(("127.0.0.1", number(1) as u16));
            println!("  Local:   http://localhost:{}/", number(1));
            idle()
        }
        "sleep" => { println!("sleeping"); idle() }
        "late" => {
            println!("waiting for the site http://127.0.0.1:{}", number(1));
            while !std::path::Path::new(parts.get(2).copied().unwrap_or("gate")).exists() { std::thread::sleep(Duration::from_millis(20)); }
            let _listener = std::net::TcpListener::bind(("127.0.0.1", number(1) as u16));
            println!("Local: http://localhost:{}/", number(1));
            idle()
        }
        "env" => {
            let mut variables: Vec<(String, String)> = std::env::vars().collect();
            variables.sort();
            for (key, value) in variables { println!("ENV:{key}={value}"); }
            std::process::exit(0)
        }
        "echo" => {
            let (mut lines, mut input) = (0, String::new());
            println!("echo-ready");
            loop {
                input.clear();
                if std::io::stdin().read_line(&mut input).unwrap_or(0) == 0 { std::process::exit(4) }
                if input.trim() == "quit" { println!("lines:{lines}"); std::process::exit(3) }
                lines += 1;
            }
        }
        "stubborn" => { platform::ignore_interrupts_and_end_on_termination(); println!("stubborn"); idle() }
        "title" => { print!("\x1b]0;daemon test title\x07titled\r\n"); std::io::stdout().flush().ok(); idle() }
        "alt" => { print!("before\r\n\x1b[?1049h\x1b[2J\x1b[H\x1b[5;10Hfull screen marker"); std::io::stdout().flush().ok(); idle() }
        "burst" => {
            let block = format!("{}
", "0123456789abcdef".repeat(8)).repeat(256);
            let mut out = std::io::stdout().lock();
            for _ in 0..number(1) * 1024 * 1024 / block.len() as u64 { out.write_all(block.as_bytes()).ok(); }
            out.write_all(b"burst-done
").ok();
            out.flush().ok();
            std::process::exit(0)
        }
        other => panic!("unknown child mode {other}"),
    }
}
// #endregion 🔖️Child

// #region 🔖️Harness
fn control_root(name: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("semio-dashboard-{name}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    ipc::canonical_path(&root)
}

fn remove_root(root: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while std::fs::remove_dir_all(root).is_err() && root.exists() && Instant::now() < deadline { std::thread::sleep(Duration::from_millis(20)); }
}

fn environment(extra: &[(&str, &str)]) -> Vec<(String, String)> {
    const KEPT: [&str; 12] = ["PATH", "Path", "SystemRoot", "SYSTEMROOT", "windir", "TEMP", "TMP", "HOME", "USERPROFILE", "LANG", "ComSpec", "PATHEXT"];
    let mut env: Vec<(String, String)> = std::env::vars().filter(|(key, _)| KEPT.contains(&key.as_str())).collect();
    env.extend(extra.iter().map(|(key, value)| (key.to_string(), value.to_string())));
    env
}

fn child_command(mode: &str, root: &Path) -> SessionCommand {
    let path = module_path!().split_once("::").unwrap().1;
    SessionCommand { cmd: std::env::current_exe().unwrap().display().to_string(), args: vec!["--exact".into(), format!("{path}::child"), "--nocapture".into(), "--test-threads=1".into()], cwd: root.display().to_string(), env: vec![(CHILD.into(), mode.into())], cols: 100, rows: 30, ..Default::default() }
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port()
}

struct Daemon {
    root: PathBuf,
    running: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<std::io::Result<()>>>,
}

impl Daemon {
    fn start(name: &str) -> Self { Self::start_in(control_root(name)) }

    fn start_in(root: PathBuf) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (served, flag) = (root.clone(), running.clone());
        let thread = Some(std::thread::spawn(move || supervisor::serve(&served, &flag)));
        let deadline = Instant::now() + Duration::from_secs(90);
        while Connection::connect(&root).is_err() { assert!(Instant::now() < deadline, "the daemon must start serving"); std::thread::sleep(Duration::from_millis(20)); }
        Self { root, running, thread }
    }

    fn view(&self) -> View { self.view_with(&[]) }

    fn view_with(&self, extra: &[(&str, &str)]) -> View { View::new(Connection::connect_with(&self.root, "test", environment(extra)).unwrap()) }

    fn stop(&mut self) {
        if let Ok(mut connection) = Connection::connect(&self.root) { let _ = connection.send(&ClientMsg::Shutdown {}); let _ = connection.receive(Duration::from_secs(8)); }
        self.running.store(false, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() { let _ = thread.join(); }
    }

    fn restart(mut self) -> Self {
        self.stop();
        let root = self.root.clone();
        std::mem::forget(self);
        Self::start_in(root)
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        self.stop();
        remove_root(&self.root);
    }
}

struct View {
    connection: Connection,
    sessions: BTreeMap<String, SessionInfo>,
    output: BTreeMap<String, Vec<u8>>,
    control: Vec<ServerMsg>,
    closed: Option<String>,
}

impl View {
    fn new(connection: Connection) -> Self {
        Self { connection, sessions: BTreeMap::new(), output: BTreeMap::new(), control: Vec::new(), closed: None }
    }

    fn send(&mut self, message: &ClientMsg) { self.connection.send(message).unwrap(); }

    fn pump(&mut self, wait: Duration) {
        for message in self.connection.receive(wait).unwrap_or_default() {
            match message {
                Message::Control(ServerMsg::SessionChanged { session }) => { self.sessions.insert(session.session_id.clone(), (*session).clone()); self.control.push(ServerMsg::SessionChanged { session }); }
                Message::Control(ServerMsg::Sessions { sessions, more }) => { for session in &sessions { self.sessions.insert(session.session_id.clone(), session.clone()); } self.control.push(ServerMsg::Sessions { sessions, more }); }
                Message::Control(ServerMsg::SessionRemoved { session_id }) => { self.sessions.remove(&session_id); }
                Message::Control(ServerMsg::ReplayStart { session_id, truncated }) => { self.output.insert(session_id.clone(), Vec::new()); self.control.push(ServerMsg::ReplayStart { session_id, truncated }); }
                Message::Control(other) => self.control.push(other),
                Message::Output { session_id, data } => self.output.entry(session_id).or_default().extend(data),
                Message::Disconnected(reason) => self.closed = Some(reason),
            }
        }
    }

    fn until(&mut self, timeout: Duration, ready: impl Fn(&View) -> bool) -> bool {
        let deadline = Instant::now() + timeout * 6;
        while Instant::now() < deadline {
            if ready(self) { return true; }
            self.pump(Duration::from_millis(20));
        }
        ready(self)
    }

    fn until_progress(&mut self, stall: Duration, measure: impl Fn(&View) -> usize, ready: impl Fn(&View) -> bool) -> bool {
        let (mut best, mut moved) = (0usize, Instant::now());
        while !ready(self) {
            self.pump(Duration::from_millis(20));
            let now = measure(self);
            if now > best { (best, moved) = (now, Instant::now()); }
            if moved.elapsed() > stall { return false; }
        }
        true
    }

    fn text(&self, session_id: &str) -> String { String::from_utf8_lossy(self.output.get(session_id).map_or(&[][..], Vec::as_slice)).into_owned() }

    fn tail(&self, session_id: &str, bytes: usize) -> String {
        let output = self.output.get(session_id).map_or(&[][..], Vec::as_slice);
        String::from_utf8_lossy(&output[output.len().saturating_sub(bytes)..]).into_owned()
    }

    fn session(&self, session_id: &str) -> SessionInfo { self.sessions.get(session_id).cloned().unwrap_or_else(|| panic!("no session {session_id}: {:?}", self.sessions.keys().collect::<Vec<_>>())) }

    fn has_ended(&self, session_id: &str) -> bool { self.sessions.get(session_id).is_some_and(|session| matches!(session.status, SessionStatus::Exited | SessionStatus::Failed)) }

    fn errors(&self) -> Vec<(String, String)> { self.control.iter().filter_map(|message| match message { ServerMsg::Error { code, message, .. } => Some((code.map_or("", ipc::ErrorCode::as_str).to_string(), message.clone())), _ => None }).collect() }
}

struct Raw {
    stream: transport::Stream,
    buffer: ipc::FrameBuffer,
    ended: bool,
}

impl Raw {
    fn connect(root: &Path) -> Self { Self { stream: transport::Stream::connect(root).unwrap(), buffer: ipc::FrameBuffer::default(), ended: false } }

    fn send(&mut self, message: &ClientMsg) {
        let frame = ipc::control_frame(message).unwrap();
        let mut sent = 0;
        let deadline = Instant::now() + Duration::from_secs(5);
        while sent < frame.len() && Instant::now() < deadline { let count = self.stream.try_write(&frame[sent..]).unwrap(); sent += count; if count == 0 { std::thread::sleep(Duration::from_millis(2)); } }
        assert_eq!(sent, frame.len());
    }

    fn hello(&mut self, extra: &[(&str, &str)]) {
        self.send(&ClientMsg::Hello { client_id: "raw".into(), protocol: ipc::PROTOCOL, build_id: ipc::build_id().into(), env: environment(extra) });
    }

    fn read(&mut self, within: Duration) -> Vec<(u8, Vec<u8>)> {
        let deadline = Instant::now() + within * 6;
        let mut frames = Vec::new();
        let mut page = [0u8; 64 * 1024];
        let mut last = Instant::now();
        loop {
            match self.stream.try_read(&mut page).unwrap_or(Some(0)) {
                Some(0) => { self.ended = true; break; }
                Some(count) => { self.buffer.extend(&page[..count]); last = Instant::now(); }
                None if !frames.is_empty() && last.elapsed() > Duration::from_millis(150) => break,
                None => if Instant::now() >= deadline { break } else { std::thread::sleep(Duration::from_millis(2)) },
            }
            while let Some((kind, payload)) = self.buffer.next_frame().unwrap() { frames.push((kind, payload.to_vec())); }
        }
        frames
    }

    fn controls(&mut self, within: Duration) -> Vec<ServerMsg> {
        self.read(within).into_iter().filter(|(kind, _)| *kind == ipc::KIND_CONTROL).map(|(_, payload)| ipc::decode_control(&payload).unwrap()).collect()
    }
}

fn launch_process(command_id: &str, mode: &str, root: &Path, ready: Option<Ready>) -> crate::registry::LaunchProcess {
    let command = child_command(mode, root);
    crate::registry::LaunchProcess { command_id: command_id.into(), cmd: command.cmd, args: command.args, cwd: PathBuf::from(&command.cwd), env: command.env, label: TaskLabel { verb: "dev".into(), subject: command_id.into(), ..Default::default() }, ready, long_running: true }
}

fn launch(command_id: &str, processes: Vec<crate::registry::LaunchProcess>, requires: Vec<crate::registry::Launch>, group: Option<&str>, together: bool) -> crate::registry::Launch {
    crate::registry::Launch { command_id: command_id.into(), label: processes[0].label.clone(), processes, requires, group: group.map(str::to_string), stop: if together { crate::registry::Stop::Together } else { crate::registry::Stop::Independent } }
}

fn spawn(view: &mut View, session_id: &str, mode: &str, root: &Path) {
    view.send(&ClientMsg::Spawn { session_id: session_id.into(), command: Box::new(child_command(mode, root)) });
}
// #endregion 🔖️Harness

// #region 🔖️Output
#[test]
fn a_task_runs_to_its_exit_code_and_later_views_and_a_restarted_daemon_replay_its_output() {
    let daemon = Daemon::start("replay");
    let mut first = daemon.view();
    first.send(&ClientMsg::Attach { client_id: "first".into() });
    spawn(&mut first, "t1", "exit:7", &daemon.root);
    assert!(first.until(Duration::from_secs(20), |view| view.sessions.get("t1").is_some_and(|session| session.status == SessionStatus::Exited)), "exited: {:?} {:?}", first.sessions, first.errors());
    let info = first.session("t1");
    assert_eq!(info.code, Some(7));
    assert!(info.pid.is_some() && info.started_ms > 0 && info.ended_ms.is_some_and(|ended| ended >= info.started_ms));
    assert!(first.until(Duration::from_secs(5), |view| view.text("t1").contains("exiting with 7")), "live output: {:?}", first.text("t1"));

    let mut second = daemon.view();
    second.send(&ClientMsg::Attach { client_id: "second".into() });
    assert!(second.until(Duration::from_secs(10), |view| view.control.iter().any(|message| matches!(message, ServerMsg::ReplayComplete { session_id: None }))));
    assert!(second.text("t1").contains("exiting with 7"), "replay: {:?}", second.text("t1"));
    let position = |wanted: &dyn Fn(&ServerMsg) -> bool| second.control.iter().position(wanted).unwrap();
    let (start, complete, all) = (position(&|message| matches!(message, ServerMsg::ReplayStart { session_id, .. } if session_id == "t1")), position(&|message| matches!(message, ServerMsg::ReplayComplete { session_id: Some(id) } if id == "t1")), position(&|message| matches!(message, ServerMsg::ReplayComplete { session_id: None })));
    assert!(start < complete && complete < all, "replay messages come in order: {start} {complete} {all}");

    let mut logged = Vec::new();
    assert!(control::logs_offline(&daemon.root, "t1", &mut logged).unwrap() > 0);
    assert!(String::from_utf8_lossy(&logged).contains("exiting with 7"));

    let daemon = daemon.restart();
    let mut third = daemon.view();
    third.send(&ClientMsg::Attach { client_id: "third".into() });
    assert!(third.until(Duration::from_secs(10), |view| view.control.iter().any(|message| matches!(message, ServerMsg::ReplayComplete { session_id: None }))));
    assert_eq!(third.session("t1").code, Some(7));
    assert!(third.text("t1").contains("exiting with 7"), "a restarted daemon replays from the log files: {:?}", third.text("t1"));
}

#[test]
fn a_restart_runs_the_task_again_into_the_same_log() {
    let daemon = Daemon::start("restart");
    let mut view = daemon.view();
    view.send(&ClientMsg::Attach { client_id: "v".into() });
    spawn(&mut view, "r", "print:3", &daemon.root);
    assert!(view.until(Duration::from_secs(20), |view| view.has_ended("r")));
    let first = view.session("r");
    view.send(&ClientMsg::Restart { session_id: "r".into() });
    assert!(view.until(Duration::from_secs(20), |view| view.sessions.get("r").is_some_and(|session| session.started_ms > first.started_ms && session.status == SessionStatus::Exited)), "{:?}", view.errors());
    assert_ne!(view.session("r").pid, first.pid);
    assert!(view.until(Duration::from_secs(5), |view| view.text("r").matches("child-output-line-2").count() == 2), "both runs in one log: {:?}", view.text("r"));
}

#[test]
fn a_child_title_and_a_full_screen_program_reach_every_view() {
    let daemon = Daemon::start("screen");
    let mut view = daemon.view();
    view.send(&ClientMsg::Attach { client_id: "v".into() });
    spawn(&mut view, "title", "title", &daemon.root);
    spawn(&mut view, "alt", "alt", &daemon.root);
    assert!(view.until(Duration::from_secs(20), |view| view.sessions.get("title").is_some_and(|session| session.title.as_deref() == Some("daemon test title"))), "title: {:?}", view.sessions.get("title"));
    assert!(view.until(Duration::from_secs(10), |view| view.text("alt").contains("full screen marker")));
    let mut late = daemon.view();
    late.send(&ClientMsg::Attach { client_id: "late".into() });
    assert!(late.until(Duration::from_secs(10), |view| view.control.iter().any(|message| matches!(message, ServerMsg::ReplayComplete { session_id: None }))));
    let replayed = late.text("alt");
    assert!(replayed.contains("full screen marker"), "the screen is rebuilt for a late view: {replayed:?}");
    #[cfg(unix)]
    assert!(replayed.contains("\x1b[?1049h"), "a program that took the alternate screen is replayed from the moment it did: {replayed:?}");
    assert_eq!(late.session("title").title.as_deref(), Some("daemon test title"));
}
// #endregion 🔖️Output

// #region 🔖️Environment
#[test]
fn a_task_starts_in_the_environment_of_the_view_that_asked_for_it() {
    let daemon = Daemon::start("environment");
    let mut quiet = daemon.view_with(&[("SEMIO_CLIENT_MARK", "first")]);
    let mut asking = daemon.view_with(&[("SEMIO_CLIENT_MARK", "second")]);
    asking.send(&ClientMsg::Attach { client_id: "asking".into() });
    spawn(&mut asking, "env", "env", &daemon.root);
    assert!(asking.until(Duration::from_secs(20), |view| view.has_ended("env")));
    assert!(asking.until(Duration::from_secs(5), |view| view.text("env").contains("ENV:SEMIO_DAEMON_TEST_CHILD=env")));
    let text = asking.text("env");
    assert!(text.contains("ENV:SEMIO_CLIENT_MARK=second"), "{text}");
    assert!(text.contains("ENV:TERM=xterm-256color") && text.contains("ENV:COLORTERM=truecolor"), "{text}");
    assert!(!text.contains("CARGO_MANIFEST_DIR") && !text.contains("=first"), "nothing of the daemon's own environment reaches the task: {text}");
    quiet.pump(Duration::from_millis(10));
}

#[test]
fn a_start_without_hello_and_a_foreign_protocol_are_refused_loudly() {
    let daemon = Daemon::start("hello");
    let mut raw = Raw::connect(&daemon.root);
    let greeting = raw.controls(Duration::from_secs(5));
    let ServerMsg::Attached { daemon_pid, protocol, build_id } = &greeting[0] else { panic!("greeting: {greeting:?}") };
    assert!(*daemon_pid > 0 && *protocol == ipc::PROTOCOL && build_id == ipc::build_id());
    raw.send(&ClientMsg::Spawn { session_id: "x".into(), command: Box::new(child_command("sleep", &daemon.root)) });
    let answered = raw.controls(Duration::from_secs(5));
    assert!(answered.iter().any(|message| matches!(message, ServerMsg::Error { code: Some(ipc::ErrorCode::HelloRequired), session_id: None, .. })), "{answered:?}");

    raw.send(&ClientMsg::Hello { client_id: "other build".into(), protocol: ipc::PROTOCOL, build_id: "another-build".into(), env: environment(&[]) });
    raw.send(&ClientMsg::Ping {});
    assert!(raw.controls(Duration::from_secs(5)).contains(&ServerMsg::Pong {}), "a different build is served");

    let mut foreign = Raw::connect(&daemon.root);
    foreign.controls(Duration::from_secs(5));
    foreign.send(&ClientMsg::Hello { client_id: "future".into(), protocol: ipc::PROTOCOL + 1, build_id: "x".into(), env: vec![] });
    let refused = foreign.controls(Duration::from_secs(5));
    assert!(refused.iter().any(|message| matches!(message, ServerMsg::Error { code: Some(ipc::ErrorCode::Protocol), .. })), "{refused:?}");
    foreign.read(Duration::from_secs(2));
    assert!(foreign.ended, "a client of another protocol revision is disconnected after the error");

    let mut garbage = Raw::connect(&daemon.root);
    garbage.controls(Duration::from_secs(5));
    let frame = ipc::frame(ipc::KIND_CONTROL, b"{\"type\":\"nonsense\"}").unwrap();
    let mut sent = 0;
    while sent < frame.len() { sent += garbage.stream.try_write(&frame[sent..]).unwrap(); }
    let answered = garbage.controls(Duration::from_secs(5));
    assert!(answered.iter().any(|message| matches!(message, ServerMsg::Error { code: Some(ipc::ErrorCode::Decode), .. })), "an unreadable message is answered, not dropped: {answered:?}");
}

#[test]
fn a_second_daemon_cannot_take_over_a_running_workspace() {
    let daemon = Daemon::start("instance");
    assert!(supervisor::serve(&daemon.root, &AtomicBool::new(true)).is_err());
    assert!(transport::Listener::bind(&daemon.root).is_err(), "the endpoint belongs to the running daemon");
    #[cfg(windows)]
    assert_eq!(ipc::pipe_name(&daemon.root), format!(r"\\.\pipe\semio-dashboard-{}", ipc::workspace_key(&daemon.root)));
    let connection = Connection::connect(&daemon.root).unwrap();
    assert!(connection.skew().is_none() && connection.daemon_build() == ipc::build_id());
}
// #endregion 🔖️Environment

// #region 🔖️Lifecycle
#[test]
fn stopping_escalates_and_killing_reports_the_signal_death() {
    let daemon = Daemon::start("stop");
    let mut view = daemon.view();
    view.send(&ClientMsg::Attach { client_id: "v".into() });
    spawn(&mut view, "polite", "sleep:polite", &daemon.root);
    spawn(&mut view, "stubborn", "stubborn", &daemon.root);
    spawn(&mut view, "victim", "sleep:victim", &daemon.root);
    assert!(view.until(Duration::from_secs(20), |view| ["polite", "stubborn", "victim"].iter().all(|id| view.text(id).contains("sleeping") || view.text(id).contains("stubborn"))));
    view.send(&ClientMsg::Stop { session_id: "polite".into() });
    view.send(&ClientMsg::Kill { session_id: "victim".into() });
    assert!(view.until(Duration::from_secs(10), |view| view.has_ended("polite") && view.has_ended("victim")), "{:?}", view.sessions.values().map(|session| (&session.session_id, session.status, session.code)).collect::<Vec<_>>());
    assert_eq!(view.session("polite").code, Some(130), "an interrupted task reports 128 + SIGINT");
    assert_eq!(view.session("victim").code, Some(137), "a killed task reports 128 + SIGKILL");
    view.send(&ClientMsg::Stop { session_id: "stubborn".into() });
    assert!(view.until(Duration::from_secs(12), |view| view.has_ended("stubborn")), "the task that ignores interrupts still ends");
    assert_eq!(view.session("stubborn").code, Some(9), "ended by the termination request it handles, after it ignored the interrupt, and not by the kill; output {:?}", view.text("stubborn"));
}

#[test]
fn terminal_input_is_queued_and_delivered_in_full() {
    let daemon = Daemon::start("input");
    let mut view = daemon.view();
    view.send(&ClientMsg::Attach { client_id: "v".into() });
    spawn(&mut view, "echo", "echo", &daemon.root);
    assert!(view.until(Duration::from_secs(20), |view| view.text("echo").contains("echo-ready")), "the task reads its input: {:?}", view.text("echo"));
    let paste: Vec<u8> = (0..500).flat_map(|index| format!("{index:04} {}\r", "x".repeat(40)).into_bytes()).chain(b"quit\r".iter().copied()).collect();
    view.connection.input("echo", &paste).unwrap();
    assert!(view.until_progress(Duration::from_secs(120), |view| view.output.get("echo").map_or(0, Vec::len), |view| view.has_ended("echo")), "{:?} {:?}", view.errors(), view.text("echo"));
    assert_eq!(view.session("echo").code, Some(3));
    assert!(view.until(Duration::from_secs(5), |view| view.text("echo").contains("lines:500")), "every pasted line reached the task: {:?}", &view.text("echo")[view.text("echo").len().saturating_sub(300)..]);
    assert!(view.errors().is_empty(), "{:?}", view.errors());
    view.connection.send(&ClientMsg::Input { session_id: "echo".into(), data: vec![3] }).unwrap();
    assert!(view.until(Duration::from_secs(5), |view| view.errors().iter().any(|(code, _)| code == "not_running")), "input for an ended task is answered");
}

#[test]
fn sessions_that_were_alive_when_the_daemon_vanished_come_back_interrupted() {
    let root = control_root("interrupted");
    {
        let (mut journal, _) = journal::Journal::open(&root).unwrap();
        for (id, status) in [("running", SessionStatus::Running), ("waiting", SessionStatus::Pending), ("done", SessionStatus::Exited)] {
            journal.append(&ServerMsg::SessionChanged { session: Box::new(SessionInfo { session_id: id.into(), command: SessionCommand { cmd: "x".into(), cols: 80, rows: 24, ..Default::default() }, status, pid: (id == "running").then_some(4242), code: (id == "done").then_some(0), started_ms: 1, ..Default::default() }) }).unwrap();
        }
    }
    let daemon = Daemon::start_in(root);
    let mut view = daemon.view();
    view.send(&ClientMsg::List {});
    assert!(view.until(Duration::from_secs(5), |view| view.sessions.len() == 3));
    assert_eq!(view.session("running").status, SessionStatus::Interrupted);
    assert_eq!(view.session("waiting").status, SessionStatus::Interrupted);
    assert_eq!(view.session("done").status, SessionStatus::Exited);
    assert!(view.session("running").ended_ms.is_some() && view.session("running").pid.is_none() && view.session("running").code.is_none());
}
// #endregion 🔖️Lifecycle

// #region 🔖️Readiness
#[test]
fn text_alone_never_declares_readiness_a_listening_port_does() {
    let daemon = Daemon::start("late");
    let port = free_port();
    let mut view = daemon.view();
    view.send(&ClientMsg::Attach { client_id: "v".into() });
    let mut command = child_command(&format!("late:{port}:gate"), &daemon.root);
    command.ready = Some(Ready { port, path: "/x".into(), printed: false });
    view.send(&ClientMsg::Spawn { session_id: "late".into(), command: Box::new(command) });
    assert!(view.until(Duration::from_secs(20), |view| view.text("late").contains("waiting for the site http://127.0.0.1:")), "{:?}", view.text("late"));
    for _ in 0..75 { view.pump(Duration::from_millis(20)); }
    assert_eq!(view.session("late").ready_url, None, "an address in the output is a candidate; nothing listens yet");
    std::fs::write(daemon.root.join("gate"), "open").unwrap();
    assert!(view.until(Duration::from_secs(30), |view| view.sessions.get("late").is_some_and(|session| session.ready_url.is_some())), "{:?}", view.sessions.get("late"));
    assert_eq!(view.session("late").ready_url, Some(format!("http://127.0.0.1:{port}/x")));
    assert_eq!(view.session("late").status, SessionStatus::Running);
}
// #endregion 🔖️Readiness

// #region 🔖️Views
#[test]
fn sixteen_attached_views_neither_hide_a_running_task_nor_make_it_interrupted() {
    let daemon = Daemon::start("views");
    let mut views: Vec<View> = (0..16).map(|index| { let mut view = daemon.view(); view.send(&ClientMsg::Attach { client_id: format!("view-{index}") }); view }).collect();
    spawn(&mut views[0], "alive", "sleep:alive", &daemon.root);
    assert!(views[0].until(Duration::from_secs(20), |view| view.sessions.get("alive").is_some_and(|session| session.status == SessionStatus::Running)));
    for view in &mut views { view.pump(Duration::from_millis(20)); assert!(view.errors().is_empty(), "{:?}", view.errors()); }

    let mut control = Connection::connect_with(&daemon.root, "cli", environment(&[])).unwrap();
    let listed = control::tasks(&mut control, Duration::from_secs(5)).unwrap();
    assert_eq!(listed.iter().find(|session| session.session_id == "alive").map(|session| session.status), Some(SessionStatus::Running), "a command line beside sixteen views still reaches the daemon");

    let offline = control::offline_tasks(&daemon.root);
    assert_eq!(offline.iter().find(|session| session.session_id == "alive").map(|session| session.status), Some(SessionStatus::Running), "the journal of a daemon that runs is not read as the journal of one that is gone");

    let mut extra = daemon.view();
    extra.send(&ClientMsg::Attach { client_id: "seventeenth".into() });
    assert!(extra.until(Duration::from_secs(5), |view| view.errors().iter().any(|(code, _)| code == "view_limit")), "the seventeenth attached view is refused with the view-limit code: {:?}", extra.errors());
    extra.send(&ClientMsg::List {});
    assert!(extra.until(Duration::from_secs(5), |view| view.sessions.contains_key("alive")), "a refused view can still list");

    drop(views);
    drop(control);
    let mut stopper = daemon.view();
    stopper.send(&ClientMsg::Kill { session_id: "alive".into() });
    assert!(stopper.until(Duration::from_secs(10), |view| view.sessions.get("alive").is_some_and(|session| !session.status.live())) || stopper.sessions.is_empty());
}
// #endregion 🔖️Views

#[cfg(windows)]
fn batch_script(root: &Path) -> (PathBuf, SessionCommand) {
    let task = child_command("sleep", root);
    let linked = root.join("child.exe");
    let executable = std::env::current_exe().unwrap();
    std::fs::hard_link(&executable, &linked).or_else(|_| std::fs::copy(&executable, &linked).map(|_| ())).unwrap();
    let script = root.join("loop.cmd");
    std::fs::write(&script, format!("@echo off\r\necho batch-started\r\n\"{}\" {}\r\n", linked.display(), task.args.join(" "))).unwrap();
    (script, task)
}

#[cfg(windows)]
mod windows {
    use super::*;
    use ui_tui::tui::pty::{resolve_program, Pty, PtySize};

    fn plain(bytes: &[u8]) -> String {
        let text = String::from_utf8_lossy(bytes);
        let mut out = String::new();
        let mut chars = text.chars().peekable();
        while let Some(character) = chars.next() {
            if character != '\u{1b}' { out.push(character); continue; }
            match chars.next() {
                Some('[') => { for next in chars.by_ref() { if ('@'..='~').contains(&next) { break; } } }
                Some(']') => { for next in chars.by_ref() { if next == '\u{7}' { break; } } }
                _ => {}
            }
        }
        out
    }

    fn cmd_output(root: &Path, line: &str, extra: &[(&str, &str)]) -> String {
        let env = environment(extra);
        let mut pty = Pty::spawn_exact("cmd", &["/c", line], &env, Some(root), PtySize { cols: 200, rows: 30 }).unwrap();
        let (mut bytes, mut page) = (Vec::new(), vec![0u8; 16 * 1024]);
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut ended: Option<Instant> = None;
        while Instant::now() < deadline {
            let count = pty.try_read(&mut page).unwrap();
            bytes.extend_from_slice(&page[..count]);
            if count > 0 { continue; }
            if pty.try_wait().unwrap().is_some() && ended.get_or_insert_with(Instant::now).elapsed() > Duration::from_millis(300) { break; }
            std::thread::sleep(Duration::from_millis(10));
        }
        plain(&bytes)
    }

    #[test]
    fn cmd_receives_its_string_exactly_as_written() {
        let root = control_root("cmdline");
        let spaced = root.join("📂 dir with space");
        std::fs::create_dir_all(&spaced).unwrap();
        std::fs::write(spaced.join("f.txt"), "x").unwrap();
        std::fs::write(spaced.join("e.cmd"), "@echo off\r\necho [%~1] [%~2]\r\n").unwrap();
        let out = cmd_output(&root, "echo   two  spaces", &[]);
        assert!(out.contains("two  spaces"), "spaces are kept: {out:?}");
        let out = cmd_output(&root, "echo \"a & b\" & echo second-part", &[]);
        assert!(out.contains("\"a & b\"") && out.contains("second-part"), "an ampersand inside quotes stays, one outside separates: {out:?}");
        let out = cmd_output(&root, "echo x^&y", &[]);
        assert!(out.contains("x&y"), "a caret escapes: {out:?}");
        let out = cmd_output(&root, "echo %SEMIO_A2_VAR%", &[("SEMIO_A2_VAR", "value one")]);
        assert!(out.contains("value one"), "the shell expands variables of the task's environment: {out:?}");
        let out = cmd_output(&root, &format!("if exist \"{}\\f.txt\" (echo found-it) else (echo missing-it)", spaced.display()), &[]);
        assert!(out.contains("found-it"), "a path with spaces and an emoji: {out:?}");
        let out = cmd_output(&root, &format!("\"{}\\e.cmd\" one \"two words\"", spaced.display()), &[]);
        assert!(out.contains("[one] [two words]"), "a quoted program path followed by quoted arguments: {out:?}");
        remove_root(&root);
    }

    #[test]
    fn a_program_resolves_to_its_executable_before_its_script_shim() {
        let root = control_root("resolve");
        let (shims, real) = (root.join("shims"), root.join("real"));
        std::fs::create_dir_all(&shims).unwrap();
        std::fs::create_dir_all(&real).unwrap();
        std::fs::write(shims.join("tool.cmd"), "@echo shim\r\n").unwrap();
        std::fs::write(shims.join("only.cmd"), "@echo only\r\n").unwrap();
        std::fs::copy(std::env::current_exe().unwrap(), real.join("tool.exe")).unwrap();
        let path = std::env::join_paths([&shims, &real]).unwrap().to_string_lossy().into_owned();
        let env = vec![("PATH".to_string(), path)];
        assert_eq!(resolve_program("tool", &env, None).map(|path| path.to_string_lossy().to_lowercase()), Some(real.join("tool.exe").to_string_lossy().to_lowercase()), "the real executable wins although the shim directory comes first");
        assert_eq!(resolve_program("only", &env, None).map(|path| path.to_string_lossy().to_lowercase()), Some(shims.join("only.cmd").to_string_lossy().to_lowercase()), "a script is still found when nothing else exists");
        assert_eq!(resolve_program("missing", &env, None), None);
        remove_root(&root);
    }

    #[test]
    fn interrupting_a_batch_file_ends_it_and_leaves_no_prompt() {
        let daemon = Daemon::start("batch");
        let (script, task) = super::batch_script(&daemon.root);
        let mut view = daemon.view();
        view.send(&ClientMsg::Attach { client_id: "v".into() });
        let mut command = task;
        command.cmd = script.display().to_string();
        command.args = vec![];
        view.send(&ClientMsg::Spawn { session_id: "batch".into(), command: Box::new(command) });
        assert!(view.until(Duration::from_secs(20), |view| view.text("batch").contains("sleeping")), "the program of the batch file runs: {:?} {:?}", view.text("batch"), view.errors());
        view.send(&ClientMsg::Stop { session_id: "batch".into() });
        assert!(view.until(Duration::from_secs(20), |view| view.has_ended("batch")), "a batch job leaves no prompt behind: it ends after the stop: {:?}", view.text("batch"));
    }
}

// #region 🔖️Groups
fn serving(command_id: &str, port: u16, delay: u64, root: &Path, path: &str) -> crate::registry::LaunchProcess {
    launch_process(command_id, &format!("serve:{port}:{delay}"), root, Some(Ready { port, path: path.into(), printed: false }))
}

#[test]
fn a_launch_starts_services_first_members_in_order_and_stops_together() {
    let daemon = Daemon::start("group");
    let (service_port, server_port) = (free_port(), free_port());
    let service = launch("svc:serve", vec![serving("svc:serve", service_port, 400, &daemon.root, "")], vec![], None, false);
    let compound = launch("compound:t/pair", vec![serving("srv:dev", server_port, 400, &daemon.root, "/admin"), launch_process("cli:dev", "sleep:client", &daemon.root, None)], vec![service.clone()], Some("compound:t/pair"), true);
    let group = SpawnGroup::from_launch(&compound, &[]);
    let ids: Vec<String> = group.members.iter().chain(&group.requires).map(|member| member.session_id.clone()).collect();
    let (server, client_id, service_id) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
    let mut view = daemon.view();
    let mut seen: Vec<(String, SessionStatus)> = Vec::new();
    let started = control::run(&mut view.connection, group, control::Wait::Ready, Some(Duration::from_secs(40)), &AtomicBool::new(false), &mut |session| seen.push((session.session_id.clone(), session.status))).unwrap_or_else(|error| panic!("{error}; seen {seen:?}"));
    assert_eq!(started.ready_url(), Some(format!("http://localhost:{server_port}/admin").as_str()));
    let first_of = |id: &str, status: SessionStatus| seen.iter().position(|(seen_id, seen_status)| seen_id == id && *seen_status == status);
    assert!(first_of(&server, SessionStatus::Pending).unwrap() < first_of(&server, SessionStatus::Running).unwrap(), "the server waited for the service: {seen:?}");
    assert!(first_of(&client_id, SessionStatus::Pending).is_some() && first_of(&client_id, SessionStatus::Pending) < first_of(&client_id, SessionStatus::Running), "the client waited for the server: {seen:?}");
    let tasks = control::tasks(&mut view.connection, Duration::from_secs(5)).unwrap();
    let find = |id: &str| tasks.iter().find(|session| session.session_id == id).unwrap_or_else(|| panic!("{id} in {tasks:?}"));
    assert!(find(&service_id).started_ms <= find(&server).started_ms && find(&server).started_ms <= find(&client_id).started_ms, "services first, members in order");
    assert_eq!(find(&service_id).group, None);
    assert_eq!(find(&server).group, find(&client_id).group);
    assert!(find(&server).group.is_some() && find(&server).ready_url.is_some());

    let again = launch("compound:t/other", vec![launch_process("other:dev", "sleep:other", &daemon.root, None)], vec![service], None, false);
    let reused = SpawnGroup::from_launch(&again, &[]);
    let other = reused.members[0].session_id.clone();
    control::run(&mut view.connection, reused, control::Wait::Ready, Some(Duration::from_secs(30)), &AtomicBool::new(false), &mut |_| {}).unwrap();
    let tasks = control::tasks(&mut view.connection, Duration::from_secs(5)).unwrap();
    assert_eq!(tasks.iter().filter(|session| session.command.command_id == "svc:serve").count(), 1, "a running service is reused, not started twice");
    assert!(tasks.iter().any(|session| session.session_id == other && session.status == SessionStatus::Running));

    control::act(&mut view.connection, &server, control::Action::Stop, Some(Duration::from_secs(15)), &AtomicBool::new(false)).unwrap();
    let mut watcher = daemon.view();
    watcher.send(&ClientMsg::Watch {});
    assert!(watcher.until(Duration::from_secs(15), |view| view.sessions.get(&client_id).is_some_and(|session| !session.status.live())), "the other member ends with the first: {:?}", watcher.sessions.get(&client_id));
    assert_eq!(watcher.session(&service_id).status, SessionStatus::Running, "a service outlives the group that needed it");
}

#[test]
fn the_waiting_line_is_written_only_for_members_that_actually_wait() {
    let daemon = Daemon::start("waiting");
    let port = free_port();
    let compound = launch("compound:t/wait", vec![serving("first:dev", port, 700, &daemon.root, ""), launch_process("second:dev", "sleep:second", &daemon.root, None)], vec![], Some("compound:t/wait"), false);
    let group = SpawnGroup::from_launch(&compound, &[]);
    let (first, second) = (group.members[0].session_id.clone(), group.members[1].session_id.clone());
    let mut watcher = daemon.view();
    watcher.send(&ClientMsg::Attach { client_id: "watcher".into() });
    let mut view = daemon.view();
    control::run(&mut view.connection, group, control::Wait::Ready, Some(Duration::from_secs(30)), &AtomicBool::new(false), &mut |_| {}).unwrap();
    assert!(watcher.until(Duration::from_secs(10), |view| view.text(&second).contains("[semio] waiting for first:dev to be ready") && view.text(&first).contains("http://localhost")), "{:?}", watcher.text(&second));
    assert!(!watcher.text(&first).contains("[semio] waiting"), "a member that starts at once has no waiting line: {:?}", watcher.text(&first));
    assert_eq!(watcher.text(&second).matches("[semio] waiting").count(), 1, "the line is written once");
}

#[test]
fn a_connection_calls_its_notifier_when_a_message_arrives_and_when_its_sends_drain() {
    let daemon = Daemon::start("notifier");
    let mut view = daemon.view();
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let counted = calls.clone();
    view.connection.set_notifier(Arc::new(move || { counted.fetch_add(1, Ordering::SeqCst); }));
    view.send(&ClientMsg::Ping {});
    assert!(view.until(Duration::from_secs(5), |view| view.control.contains(&ServerMsg::Pong {}) && calls.load(Ordering::SeqCst) >= 2), "a drained send and an arrived message each call the notifier: {}", calls.load(Ordering::SeqCst));
}

#[test]
fn a_member_that_ends_before_it_is_ready_fails_the_launch_and_the_members_after_it() {
    let daemon = Daemon::start("failure");
    let compound = launch("compound:t/broken", vec![launch_process("bad:dev", "exit:5", &daemon.root, Some(Ready { port: free_port(), path: String::new(), printed: false })), launch_process("after:dev", "sleep", &daemon.root, None)], vec![], Some("compound:t/broken"), true);
    let group = SpawnGroup::from_launch(&compound, &[]);
    let after = group.members[1].session_id.clone();
    let mut view = daemon.view();
    let error = control::run(&mut view.connection, group, control::Wait::Ready, Some(Duration::from_secs(30)), &AtomicBool::new(false), &mut |_| {}).unwrap_err();
    assert!(matches!(&error, control::RunError::Ended(session) if session.code == Some(5)) || matches!(&error, control::RunError::Refused { code, .. } if code == "spawn"), "{error}");
    let mut watcher = daemon.view();
    watcher.send(&ClientMsg::Attach { client_id: "w".into() });
    assert!(watcher.until(Duration::from_secs(10), |view| view.sessions.get(&after).is_some_and(|session| !session.status.live())), "{:?}", watcher.sessions.get(&after));
    assert!(watcher.until(Duration::from_secs(5), |view| view.text(&after).contains("not started")), "the reason is in the output of the member that never started: {:?}", watcher.text(&after));
    assert_eq!(watcher.session(&after).pid, None);
}

#[test]
fn a_cancelled_launch_never_starts_the_members_still_waiting() {
    let daemon = Daemon::start("cancel");
    let port = free_port();
    let compound = launch("compound:t/slow", vec![serving("slow:dev", port, 60_000, &daemon.root, ""), launch_process("later:dev", "sleep", &daemon.root, None)], vec![], Some("compound:t/slow"), true);
    let group = SpawnGroup::from_launch(&compound, &[]);
    let later = group.members[1].session_id.clone();
    let mut view = daemon.view();
    let cancel = Arc::new(AtomicBool::new(false));
    let flag = cancel.clone();
    let canceller = std::thread::spawn(move || { std::thread::sleep(Duration::from_millis(1500)); flag.store(true, Ordering::SeqCst); });
    let error = control::run(&mut view.connection, group, control::Wait::Ready, Some(Duration::from_secs(30)), &cancel, &mut |_| {}).unwrap_err();
    canceller.join().unwrap();
    assert!(matches!(error, control::RunError::Cancelled), "{error}");
    let mut watcher = daemon.view();
    watcher.send(&ClientMsg::Watch {});
    assert!(watcher.until(Duration::from_secs(15), |view| view.sessions.get(&later).is_some_and(|session| !session.status.live())));
    assert_eq!(watcher.session(&later).pid, None, "the member that waited never got a process");
}
// #endregion 🔖️Groups

// #region 🔖️FlowControl
#[test]
fn a_view_that_does_not_read_is_neither_disconnected_nor_allowed_to_slow_the_others() {
    let daemon = Daemon::start("stalled");
    let mut stalled = Raw::connect(&daemon.root);
    stalled.controls(Duration::from_secs(5));
    stalled.hello(&[]);
    stalled.send(&ClientMsg::Attach { client_id: "stalled".into() });
    let mut reader = daemon.view();
    reader.send(&ClientMsg::Attach { client_id: "reader".into() });
    spawn(&mut reader, "burst", "burst:12", &daemon.root);
    assert!(reader.until_progress(Duration::from_secs(120), |view| view.output.get("burst").map_or(0, Vec::len), |view| view.tail("burst", 4096).contains("burst-done")), "the reader received {} bytes", reader.output.get("burst").map_or(0, Vec::len));
    let received = reader.output.get("burst").map_or(0, Vec::len);
    assert!(received >= 12 * 1024 * 1024, "{received}");

    let (mut total, mut completed, mut moved) = (0usize, false, Instant::now());
    while !(total >= 8 * 1024 * 1024 && completed) && moved.elapsed() < Duration::from_secs(120) {
        let frames = stalled.read(Duration::from_secs(1));
        if !frames.is_empty() { moved = Instant::now(); }
        total += frames.iter().filter(|(kind, _)| *kind == ipc::KIND_OUTPUT).map(|(_, payload)| payload.len()).sum::<usize>();
        completed |= frames.iter().any(|(kind, payload)| *kind == ipc::KIND_CONTROL && String::from_utf8_lossy(payload).contains("replay_complete"));
        assert!(!stalled.ended, "a stalled view stays connected");
    }
    assert!(total >= 8 * 1024 * 1024 && completed, "the stalled view still gets the output it can read when it reads again: {total} bytes");
}

#[test]
fn a_hundred_retained_sessions_are_listed_to_a_fresh_view_beside_a_stalled_one() {
    let root = control_root("many");
    {
        let (mut journal, _) = journal::Journal::open(&root).unwrap();
        for index in 0..200u64 {
            let session = SessionInfo { session_id: format!("old-{index:03}"), command: SessionCommand { cmd: "x".into(), args: vec!["a".repeat(200)], cols: 80, rows: 24, ..Default::default() }, status: SessionStatus::Exited, code: Some(0), started_ms: 1000 + index, ended_ms: Some(2000 + index), ..Default::default() };
            journal.append(&ServerMsg::SessionChanged { session: Box::new(session) }).unwrap();
        }
    }
    let daemon = Daemon::start_in(root);
    let mut stalled = Raw::connect(&daemon.root);
    stalled.controls(Duration::from_secs(5));
    stalled.hello(&[]);
    stalled.send(&ClientMsg::Attach { client_id: "stalled".into() });
    let mut fresh = daemon.view();
    let listed = control::tasks(&mut fresh.connection, Duration::from_secs(10)).unwrap();
    assert_eq!(listed.len(), journal::RETAINED_SESSIONS);
    assert!(listed.windows(2).all(|pair| pair[0].started_ms <= pair[1].started_ms), "sessions are listed in the order they started");
    fresh.send(&ClientMsg::Attach { client_id: "fresh".into() });
    assert!(fresh.until(Duration::from_secs(10), |view| view.control.iter().any(|message| matches!(message, ServerMsg::ReplayComplete { session_id: None }))));
    let pages = fresh.control.iter().filter(|message| matches!(message, ServerMsg::Sessions { .. })).count();
    assert!(pages >= journal::RETAINED_SESSIONS / clients::PAGE_SESSIONS && fresh.sessions.len() == journal::RETAINED_SESSIONS, "{pages} pages, {} sessions", fresh.sessions.len());
    assert!(!stalled.ended);
}
// #endregion 🔖️FlowControl
