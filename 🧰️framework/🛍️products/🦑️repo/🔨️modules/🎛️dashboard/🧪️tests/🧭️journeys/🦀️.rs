//! 🧭️ Pseudo-terminal journeys of the semio dashboard (feature: ./🥒️.feature, scenarios tagged @pty).
//! The binary is the real `semio`; `portable-pty` is the terminal driver and `vt100` the screen model, so
//! every assertion reads what a person would read on the screen.

#[path = "🧰️steps/🦀️.rs"]
mod steps;
#[path = "🖥️terminal/🦀️.rs"]
mod terminal;
#[path = "🏗️workspace/🦀️.rs"]
mod workspace;

use std::time::Duration;
use steps::{SLOW, assert_repaint_consistent, command_id, open, restore, shows_end, start, tasks, wait_task};
use workspace::Workspace;

#[test]
fn a_task_is_started_from_the_launcher_receives_typed_words_and_shows_its_exit() {
    let ws = Workspace::new("words");
    let mut term = open(&ws);
    start(&mut term, "words", "WORDS-READY");
    let screen = term.text();
    assert!(screen.contains("words"), "the window or tab is not titled after the task:\n{screen}");
    assert!(!screen.contains("start one with New task"), "stale text of the previous view is still painted:\n{screen}");
    term.type_text("hello world");
    term.send(terminal::ENTER);
    term.wait_for("ECHO:hello world", SLOW).unwrap_or_else(|screen| panic!("the typed words never reached the task (terminal input should be active after a start): {screen}"));
    let started = wait_task(&ws, "words running", |task| command_id(task) == "tool:journey-fixture/words" && task["status"] == "running");
    assert!(started["pid"].is_number(), "the daemon reports no process id: {started}");
    term.leader("c");
    term.wait_until("the ended task", SLOW, shows_end).unwrap_or_else(|screen| panic!("Ctrl+B c did not end the task on screen: {screen}"));
    let ended = wait_task(&ws, "words exited", |task| command_id(task) == "tool:journey-fixture/words" && task["status"] == "exited");
    assert!(ended["endedMs"].is_number(), "no end time: {ended}");
}

#[test]
fn a_finished_task_shows_its_exit_code() {
    let ws = Workspace::new("exit");
    let mut term = open(&ws);
    start(&mut term, "fail", "EXITING with 3");
    let ended = wait_task(&ws, "fail exited", |task| command_id(task) == "tool:journey-fixture/fail" && task["status"] == "exited");
    assert_eq!(ended["code"], 3, "the daemon records another exit code: {ended}");
    term.wait_until("the exit code 3 next to an exit word", SLOW, |text| text.lines().any(|line| line.to_lowercase().contains("exit") && line.contains('3') && !line.contains("EXITING with 3"))).unwrap_or_else(|screen| panic!("{screen}"));
}

#[test]
fn resizing_the_terminal_reflows_the_view_and_informs_the_task() {
    let ws = Workspace::new("resize");
    let mut term = open(&ws);
    start(&mut term, "resize", "SIZE ");
    let size_of = |text: &str| -> Option<(u16, u16)> {
        text.lines().rev().find_map(|line| {
            let rest = line.split("SIZE ").nth(1)?;
            let (cols, rows) = rest.trim().split(|c: char| !c.is_ascii_digit() && c != 'x').next()?.split_once('x')?;
            Some((cols.parse().ok()?, rows.parse().ok()?))
        })
    };
    let (cols, rows) = size_of(&term.text()).expect("the task printed its size");
    assert!(cols <= steps::COLS && rows <= steps::ROWS && cols >= 40 && rows >= 8, "the task starts with an implausible size {cols}x{rows} in a {}x{} terminal", steps::COLS, steps::ROWS);
    term.resize(26, 90);
    let screen = term.wait_until("the task seeing a narrower terminal", SLOW, |text| size_of(text).is_some_and(|(width, _)| width < cols)).unwrap_or_else(|screen| panic!("the task was never told about the new size: {screen}"));
    let (narrow, short) = size_of(&screen).unwrap();
    assert!(narrow <= 90 && short <= 26, "the task size {narrow}x{short} exceeds the 90x26 terminal");
    assert_eq!(term.size(), (26, 90));
    let widest = term.rows().iter().map(|row| row.trim_end().chars().count()).max().unwrap_or(0);
    assert!(widest <= 90, "a row of the screen is wider than the terminal: {widest}");
}

#[test]
fn detaching_leaves_the_task_running_and_attaching_restores_its_output() {
    let ws = Workspace::new("detach");
    let mut term = open(&ws);
    start(&mut term, "ticker", "TICKER-START");
    let before = wait_task(&ws, "ticker running", |task| command_id(task) == "tool:journey-fixture/ticker" && task["status"] == "running");
    std::thread::sleep(Duration::from_millis(1500));
    term.leader("d");
    assert_eq!(term.exit_code(SLOW), Some(0), "Ctrl+B d did not end the view cleanly:\n{}", term.text());
    drop(term);
    let during = wait_task(&ws, "ticker still running", |task| command_id(task) == "tool:journey-fixture/ticker" && task["status"] == "running");
    assert_eq!(during["pid"], before["pid"], "the task was restarted by the detach");
    let mut again = open(&ws);
    let screen = restore(&mut again, "TICK ");
    assert!(screen.contains("TICK "), "the replay shows no later output:\n{screen}");
}

#[test]
fn two_views_of_one_workspace_show_the_same_tasks() {
    let ws = Workspace::new("two-views");
    let mut first = open(&ws);
    start(&mut first, "words", "WORDS-READY");
    first.type_text("from the first view");
    first.send(terminal::ENTER);
    first.wait_for("ECHO:from the first view", SLOW).unwrap_or_else(|screen| panic!("{screen}"));
    let mut second = open(&ws);
    let screen = restore(&mut second, "WORDS-READY");
    assert!(screen.contains("ECHO:from the first view"), "the second view lacks the output typed in the first:\n{screen}");
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == "tool:journey-fixture/words").count(), 1, "a second view started a second process");
    second.type_text("from the second view");
    second.send(terminal::ENTER);
    first.wait_for("ECHO:from the second view", SLOW).unwrap_or_else(|screen| panic!("input typed in the second view never reached the shared task: {screen}"));
}

#[test]
fn a_task_started_from_the_dashboard_is_listed_by_the_command_line() {
    let ws = Workspace::new("listed");
    let mut term = open(&ws);
    start(&mut term, "ticker", "TICKER-START");
    let listed = wait_task(&ws, "ticker running", |task| command_id(task) == "tool:journey-fixture/ticker" && task["status"] == "running");
    assert!(listed["pid"].is_number() && listed["startedMs"].is_number(), "{listed}");
    let text = ws.ok(&["tasks"]);
    assert!(text.contains("running") && text.contains("ticker"), "{text}");
    ws.ok(&["stop", listed["session"].as_str().expect("session id")]);
    term.wait_until("the stop shown in the view", SLOW, shows_end).unwrap_or_else(|screen| panic!("a stop from the command line is not shown in the view: {screen}"));
}

#[test]
fn the_incremental_screen_equals_a_full_repaint() {
    let ws = Workspace::new("repaint");
    let mut term = open(&ws);
    start(&mut term, "words", "WORDS-READY");
    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
}

#[test]
fn fixture_resize_task_reports_a_plain_pseudo_terminal_resize() {
    let ws = Workspace::new("fixture-resize");
    let bun = workspace::which("bun");
    let mut term = terminal::Terminal::spawn(&bun, &["📜️script.ts", "resize"], &ws.root, &ws.env(), 20, 70);
    term.wait_for("SIZE 70x20", SLOW).unwrap_or_else(|screen| panic!("{screen}"));
    term.resize(25, 90);
    term.wait_for("SIZE 90x25", SLOW).unwrap_or_else(|screen| panic!("the fixture task does not notice a resize of a plain pseudo-terminal: {screen}"));
}

#[test]
fn a_reattached_view_replays_a_gapless_run_of_the_task_output() {
    let ws = Workspace::new("replay-gapless");
    ws.ok(&["run", "tool:journey-fixture/ticker", "--detach"]);
    wait_task(&ws, "ticker running", |task| command_id(task) == "tool:journey-fixture/ticker" && task["status"] == "running");
    std::thread::sleep(Duration::from_secs(12));
    let mut view = open(&ws);
    view.leader("s");
    std::thread::sleep(Duration::from_millis(500));
    view.send(b"\t");
    let screen = view.wait_for("TICK ", SLOW).unwrap_or_else(|screen| panic!("no replay after Ctrl+B s and Tab: {screen}"));
    std::thread::sleep(Duration::from_millis(600));
    let screen = view.text().max(screen);
    let numbers: Vec<u32> = screen.lines().filter_map(|line| line.trim_matches(|c: char| c == '\u{2503}' || c == ' ').strip_prefix("TICK ")?.split(' ').next()?.parse().ok()).collect();
    assert!(numbers.len() >= 5 && numbers.windows(2).all(|pair| pair[1] == pair[0] + 1), "the replay is not a gapless run of ticks: {numbers:?}\n{screen}");
}
