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
fn the_terminal_driver_preserves_committed_unicode_text() {
    let ws = Workspace::new("unicode-probe");
    let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &["-e", "process.stdin.setRawMode(true);process.stdin.on('data',data=>process.stdout.write(data));console.log('ready');"], &ws.root, &ws.env(), 20, 120);
    term.wait_for("ready", Duration::from_secs(3)).unwrap();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
    let vectors: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(repository.join("🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧫️fixtures/⌨️input-decoding/🔣️.json")).unwrap()).unwrap();
    for vector in vectors["consoleCommits"].as_array().unwrap() {
        let text = vector["text"].as_str().unwrap();
        term.type_text(text);
        term.wait_for(text, Duration::from_secs(3)).unwrap();
        println!("[DEBUG] independent raw terminal driver preserved {text:?}");
    }
}

#[test]
fn launcher_text_editing_follows_the_visible_hardware_cursor() {
    let ws = Workspace::new("text-caret");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text("tool:journey-fixture/text");
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for("required", SLOW).unwrap();
    term.type_text("A界e\u{301}");
    term.wait_for("A界e\u{301}", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(250));
    let (shown, end) = term.cursor();
    assert!(shown, "a text editor must show its hardware cursor: {}", term.text());
    let row = &term.rows()[usize::from(end.0)];
    let (prefix, _) = row.split_once("A界e\u{301}").expect("the visible input row contains the typed text");
    assert_eq!(end.1, prefix.chars().count() as u16 + 4, "the hardware caret follows all four visible text columns");
    term.send(b"\x1b[D");
    term.wait_until("the caret crossing one combining cluster", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, end.1 - 1))).unwrap();
    let (shown, left) = term.cursor();
    assert!(shown);
    assert_eq!(left, (end.0, end.1 - 1), "left crosses one combining cluster");
    term.type_text("x");
    term.wait_for("A界xe\u{301}", SLOW).unwrap();
    term.send(b"\x7f\x1b[3~\x1b[H\x1b[200~Z\nQ\x1b[201~");
    term.wait_for("Z QA界", SLOW).unwrap();
    term.wait_until("the pasted caret", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, end.1 - 1))).unwrap();
    let (shown, pasted) = term.cursor();
    assert!(shown);
    assert_eq!(pasted, (end.0, end.1 - 1), "the three pasted columns precede the remaining A界");
    let (column, row) = (end.1 - 4 + 1, end.0 + 1);
    term.send(format!("\x1b[<0;{column};{row}M\x1b[<0;{column};{row}m").as_bytes());
    std::thread::sleep(Duration::from_millis(250));
    term.type_text("!");
    term.wait_for("!Z QA界", SLOW).unwrap();
    term.send(b"\x1b[6~");
    term.wait_until("a start row without a text caret", Duration::from_millis(1500), |_| !term.cursor().0).unwrap();
    assert!(!term.cursor().0, "the start row owns no text cursor");
    term.send(terminal::ENTER);
    term.wait_for("ENV mark=!Z QA界", SLOW).unwrap();
    let task = wait_task(&ws, "edited text task", |task| command_id(task) == "tool:journey-fixture/text");
    let logs = ws.ok(&["logs", task["session"].as_str().unwrap()]);
    assert!(logs.contains("ENV mark=!Z QA界"), "the process received another value: {logs}");
    println!("[DEBUG] independent VT100 cursor positions: end={end:?} left={left:?} pasted={pasted:?}; task output received !Z QA界");
}

#[test]
fn a_configured_command_starts_with_one_click() {
    let ws = Workspace::new("single-click-start");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text("tool:journey-fixture/text");
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for("required", SLOW).unwrap();
    term.type_text("SINGLE-CLICK");
    term.wait_for("SINGLE-CLICK", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(250));
    let row = term.rows().iter().position(|line| line.contains("[ Start ]")).unwrap() + 1;
    term.send(format!("\x1b[<0;4;{row}M\x1b[<0;4;{row}m").as_bytes());
    term.wait_for("ENV mark=SINGLE-CLICK", SLOW).unwrap();
    let started: Vec<_> = tasks(&ws).into_iter().filter(|task| command_id(task) == "tool:journey-fixture/text").collect();
    assert_eq!(started.len(), 1);
    println!("[DEBUG] one start-action click launched exactly one task with SINGLE-CLICK");
}

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

#[test]
fn rendered_form_text_remains_inside_its_window() {
    let ws = Workspace::new("form-borders");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text("tool:journey-fixture/text");
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for("required", SLOW).unwrap();
    term.type_text("BORDER-CHECK");
    term.wait_for("BORDER-CHECK", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let rows = term.rows();
    let first = rows.iter().position(|row| row.starts_with('┣')).unwrap() + 1;
    let last = rows.iter().rposition(|row| row.starts_with('┗')).unwrap();
    assert!(last > first);
    for (row, text) in rows.iter().enumerate().take(last).skip(first) {
        assert!(text.starts_with('┃') && text.trim_end().ends_with('┃'), "form row {row} escaped its window: {text:?}\n{}", term.text());
    }
    println!("[DEBUG] every configuration body row remained inside its window borders");
}
#[test]
fn a_control_space_prefix_arms_the_next_key() {
    let ws = Workspace::new("control-space");
    let mut term = terminal::Terminal::spawn(&workspace::binary(), &["--prefix", "ctrl+space"], &ws.root, &ws.env(), steps::ROWS, steps::COLS);
    term.wait_for("New task", SLOW).unwrap();
    term.send(&[0]);
    term.wait_for("waiting for the next key", SLOW).unwrap_or_else(|screen| panic!("control-space did not arm the prefix: {screen}"));
    term.type_text("n");
    term.wait_for("commands", SLOW).unwrap();
    println!("[DEBUG] control-space armed the configured prefix and opened the launcher");
}
#[test]
fn hover_and_selection_are_visible_and_select_the_clicked_command() {
    let ws = Workspace::new("pointer-selection");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text("tool:journey-fixture/");
    term.wait_for("serve-a", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let row = term.rows().iter().position(|row| row.contains("serve-a")).unwrap() as u16;
    let before = term.attributes(row, steps::COLS);
    term.send(format!("\x1b[<35;8;{}M", row + 1).as_bytes());
    term.wait_until("visible hover attributes", Duration::from_millis(1500), |_| term.attributes(row, steps::COLS) != before).unwrap();
    let hovered = term.attributes(row, steps::COLS);
    assert_ne!(hovered, before, "hover has no independently visible attributes:\n{}", term.text());
    term.send(format!("\x1b[<0;8;{}M\x1b[<0;8;{}m", row + 1, row + 1).as_bytes());
    term.wait_until("visible selection attributes", Duration::from_millis(1500), |_| term.attributes(row, steps::COLS) != hovered).unwrap();
    let selected = term.attributes(row, steps::COLS);
    assert_ne!(selected, hovered, "selection has no independently visible attributes:\n{}", term.text());
    assert!(tasks(&ws).is_empty(), "selecting a command must not start it");
    term.send(terminal::ENTER);
    std::thread::sleep(Duration::from_millis(600));
    if term.text().contains("[ Start ]") { term.send(terminal::ENTER); }
    term.wait_for("journey server listening", SLOW).unwrap();
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == "tool:journey-fixture/serve-a").count(), 1);
    println!("[DEBUG] VT100 cell attributes independently showed hover and selection; the clicked command started once");
}
#[test]
fn the_launcher_uses_all_available_window_space() {
    let ws = Workspace::new("launcher-space");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text("tool:journey-fixture/");
    term.wait_for("13 of", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let leaves = term.rows().iter().filter(|row| row.contains("tool:journey-fixture/") && !row.contains("┃/")).count();
    assert_eq!(leaves, 13, "hidden form reserved launcher space:\n{}", term.text());
    println!("[DEBUG] all thirteen command leaves fit inside the available launcher window");
}
