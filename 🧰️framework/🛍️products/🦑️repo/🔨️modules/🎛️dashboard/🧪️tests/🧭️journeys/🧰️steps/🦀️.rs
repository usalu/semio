//! 🧰️ The steps every pseudo-terminal scenario shares: opening the dashboard, starting a task from the
//! launcher, restoring task views after an attach, reading the daemon's task list and comparing a screen
//! painted step by step with a screen painted from scratch.

use crate::terminal::{self, Terminal};
use crate::workspace::{self, Workspace};
use serde_json::Value;
use std::time::Duration;

pub const SLOW: Duration = Duration::from_secs(25);
pub const ROWS: u16 = 34;
pub const COLS: u16 = 120;

/// 🖥️ The dashboard on the fixture workspace, waiting for its first frame.
pub fn open(ws: &Workspace) -> Terminal {
    let term = Terminal::spawn(&workspace::binary(), &[], &ws.root, &ws.env(), ROWS, COLS);
    term.wait_for("New task", SLOW).unwrap_or_else(|screen| panic!("the dashboard never showed its first frame: {screen}"));
    term
}

/// 🚀️ Ctrl+B n, a search, Enter (and the confirmation of the parameter form when one is offered) until `marker` is on screen.
pub fn start(term: &mut Terminal, search: &str, marker: &str) {
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap_or_else(|screen| panic!("Ctrl+B n opened no launcher: {screen}"));
    term.type_text(search);
    term.wait_until("a match for the search", SLOW, |text| text.contains("1 of")).unwrap_or_else(|screen| panic!("{screen}"));
    term.send(terminal::ENTER);
    std::thread::sleep(Duration::from_millis(600));
    if term.text().contains("[ Start ]") { term.send(terminal::ENTER); }
    term.wait_for(marker, SLOW).unwrap_or_else(|screen| panic!("starting {search:?} showed no {marker:?}: {screen}"));
}

/// 🔁️ Ctrl+B s restores the task views of a newly attached view; the output of the task must be on screen
/// without a further key (the documented `Tab` selects the next pane and is tried only to name the defect).
pub fn restore(term: &mut Terminal, marker: &str) -> String {
    term.leader("s");
    if let Ok(screen) = term.wait_for(marker, Duration::from_secs(8)) { return screen; }
    term.send(b"\t");
    match term.wait_for(marker, Duration::from_secs(8)) {
        Ok(screen) => panic!("the restored task output appears only after a further Tab; Ctrl+B s leaves the Tasks window on top:\n{screen}"),
        Err(screen) => panic!("the attached view replays no output of the task ({marker:?}) even after Ctrl+B s and Tab:\n{screen}"),
    }
}

pub fn tasks(ws: &Workspace) -> Vec<Value> {
    serde_json::from_str::<Value>(&ws.ok(&["tasks", "--json"])).expect("tasks --json").as_array().cloned().unwrap_or_default()
}

pub fn wait_task(ws: &Workspace, what: &str, accept: impl Fn(&Value) -> bool) -> Value {
    let deadline = std::time::Instant::now() + SLOW;
    loop {
        if let Some(found) = tasks(ws).into_iter().find(|task| accept(task)) { return found; }
        assert!(std::time::Instant::now() < deadline, "no task {what} within {SLOW:?}: {:?}", tasks(ws));
        std::thread::sleep(Duration::from_millis(300));
    }
}

pub fn command_id(task: &Value) -> &str { task["commandId"].as_str().unwrap_or_default() }

/// 🏁️ Whether the screen says that a task ended: an exit word or `exit <code>`.
pub fn shows_end(screen: &str) -> bool {
    let lower = screen.to_lowercase();
    ["exited", "stopped", "ended", "terminated"].iter().any(|word| lower.contains(word)) || lower.split("exit ").skip(1).any(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
}

/// 🔌️ Whether the status line reports a live connection to the daemon.
pub fn connected(screen: &str) -> bool {
    let status = screen.lines().rev().find(|line| !line.trim().is_empty()).unwrap_or_default();
    status.contains("\u{b7} connected")
}

/// 🖌️ The screen painted incrementally must equal the screen painted from scratch: a resize forces a full repaint.
pub fn assert_repaint_consistent(term: &mut Terminal, volatile: &[&str]) {
    std::thread::sleep(Duration::from_millis(800));
    let before = term.rows();
    term.resize(ROWS, COLS - 1);
    std::thread::sleep(Duration::from_millis(900));
    term.resize(ROWS, COLS);
    std::thread::sleep(Duration::from_millis(1200));
    let after = term.rows();
    let differing: Vec<String> = before.iter().zip(&after).enumerate()
        .filter(|(_, (left, right))| left.trim_end() != right.trim_end() && !volatile.iter().any(|word| left.contains(word) || right.contains(word)))
        .map(|(row, (left, right))| format!("row {row}:\n  incremental: {}\n  repainted:   {}", left.trim_end(), right.trim_end())).collect();
    assert!(differing.is_empty(), "the incrementally painted screen differs from a full repaint (stale cells):\n{}", differing.join("\n"));
}
