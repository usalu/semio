//! 🏋️ Pseudo-terminal load scenarios (feature: ./🥒️.feature, scenarios tagged @pty): sixteen simultaneous
//! views, the launcher over fifty thousand commands and a flow-controlled ten megabyte burst, driven through
//! `portable-pty` and read through `vt100`.

#[path = "../🧰️steps/🦀️.rs"]
mod steps;
#[path = "../🖥️terminal/🦀️.rs"]
mod terminal;
#[path = "../🏗️workspace/🦀️.rs"]
mod workspace;

use std::time::{Duration, Instant};
use steps::{SLOW, connected, open, tasks, wait_task, command_id};
use terminal::Terminal;
use workspace::Workspace;

/// 🔢️ The match line of the launcher window: `N of M commands` while filtered, `M commands` without a filter,
/// `No command matches` for none. The status bar also says `M commands`, so only lines of the window count.
fn counts(screen: &str) -> Option<(u64, u64)> {
    let last_number = |text: &str| -> Option<u64> { text.rsplit(|c: char| !c.is_ascii_digit()).find(|part| !part.is_empty())?.parse().ok() };
    let first_number = |text: &str| -> Option<u64> { text.split(|c: char| !c.is_ascii_digit()).find(|part| !part.is_empty())?.parse().ok() };
    if screen.contains("No command matches") { return Some((0, 0)); }
    screen.lines().filter(|line| line.starts_with('\u{2503}')).find_map(|line| {
        if let Some((shown, rest)) = line.split_once(" of ") {
            if rest.contains("command") { return Some((last_number(shown)?, first_number(rest)?)); }
        }
        let (head, _) = line.split_once(" commands")?;
        let total = last_number(head)?;
        Some((total, total))
    })
}

#[test]
fn sixteen_views_attach_to_one_workspace_and_see_the_same_output() {
    let ws = Workspace::new("sixteen");
    ws.ok(&["run", "tool:journey-fixture/ticker", "--detach"]);
    wait_task(&ws, "ticker running", |task| command_id(task) == "tool:journey-fixture/ticker" && task["status"] == "running");
    let mut views: Vec<Terminal> = (0..16).map(|_| open(&ws)).collect();
    let mut failures = Vec::new();
    for (index, view) in views.iter_mut().enumerate() {
        view.leader("s");
        std::thread::sleep(Duration::from_millis(300));
        view.send(b"	");
        if view.wait_for("TICK ", Duration::from_secs(20)).is_err() { failures.push(format!("view {index}:\n{}", view.text())); }
    }
    assert!(failures.is_empty(), "{} of 16 views show no output of the running ticker (Ctrl+B s on the freshly attached view):\n{}", failures.len(), failures.join("\n---\n"));
    let listed = tasks(&ws);
    assert!(listed.iter().all(|task| task["status"] != "interrupted"), "with 16 views attached `semio tasks` reports the live ticker as interrupted (daemon status: {:?}; tasks stderr: {:?}; journal tail: {}): {listed:?}", String::from_utf8_lossy(&ws.cli(&["daemon", "status"]).stdout), String::from_utf8_lossy(&ws.cli(&["tasks"]).stderr), journal_tail(&ws));
    assert_eq!(listed.iter().filter(|task| task["status"] == "running").count(), 1, "attaching 16 views changed the set of running tasks: {listed:?}");
    let seventeenth = Terminal::spawn(&workspace::binary(), &[], &ws.root, &ws.env(), steps::ROWS, 240);
    let verdict = seventeenth.wait_until("a refusal or an explanation in the status line", Duration::from_secs(30), |screen| screen.lines().rev().find(|line| !line.trim().is_empty()).is_some_and(|status| ["16", "limit", "too many", "views"].iter().any(|word| status.to_lowercase().contains(word))));
    let mut seventeenth = seventeenth;
    let ended = seventeenth.exit_code(Duration::from_secs(5));
    assert!(verdict.is_ok() || ended.is_some_and(|code| code != 0), "a seventeenth view is neither refused nor explained:\n{}", seventeenth.text());
}


/// 📜️ The last lines of the daemon's event journal of a workspace, to name what happened to its tasks.
fn journal_tail(ws: &Workspace) -> String {
    let text = std::fs::read_to_string(ws.root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/events.jsonl")).unwrap_or_default();
    text.lines().rev().take(12).collect::<Vec<_>>().join(" | ")
}

#[test]
fn typing_in_the_launcher_over_fifty_thousand_commands_repaints_within_the_latency_budget() {
    let ws = Workspace::with_bulk_tools("fifty-thousand", 50_000);
    let budget = Duration::from_millis(std::env::var("SEMIO_TYPE_BUDGET_MS").ok().and_then(|text| text.parse().ok()).unwrap_or(1_500));
    let mut term = Terminal::spawn(&workspace::binary(), &[], &ws.root, &ws.env(), steps::ROWS, steps::COLS);
    term.wait_for("New task", SLOW).unwrap_or_else(|screen| panic!("{screen}"));
    term.wait_until("the 50 000 commands discovered", Duration::from_secs(120), |screen| counts(screen).is_some_and(|(_, total)| total >= 50_000) || screen.contains("50,0") || screen.contains("50 0")).ok();
    term.leader("n");
    let opened = term.wait_until("the launcher with every command", Duration::from_secs(120), |screen| counts(screen).is_some_and(|(shown, total)| shown == total && total >= 50_000)).unwrap_or_else(|screen| panic!("the launcher never counted 50 000 commands: {screen}"));
    let (_, total) = counts(&opened).unwrap();
    assert!(total >= 50_000, "the launcher counts {total} commands, expected at least 50 000");
    term.type_text("bulk-");
    term.wait_until("the prefix narrowing to the generated tools", SLOW, |screen| screen.contains("bulk-") && counts(screen).is_some_and(|(shown, _)| shown == 50_000)).unwrap_or_else(|screen| panic!("typing bulk- should match the 50 000 generated tools: {screen}"));
    let mut latencies = Vec::new();
    let mut typed = String::from("bulk-");
    for (key, expected) in [('0', 50_000u64), ('4', 10_000), ('9', 1_000), ('9', 100)] {
        typed.push(key);
        let pressed = Instant::now();
        term.type_text(&key.to_string());
        term.wait_until("the new match count", Duration::from_secs(60), |screen| screen.contains(&typed) && counts(screen).is_some_and(|(shown, _)| shown == expected)).unwrap_or_else(|screen| panic!("after {typed:?} the launcher should show {expected} matches: {screen}"));
        latencies.push((key, pressed.elapsed().saturating_sub(Duration::from_millis(15))));
    }
    let worst = latencies.iter().map(|(_, took)| *took).max().unwrap();
    println!("[load] launcher over {total} commands: key latencies {latencies:?}");
    assert!(worst <= budget, "the slowest key took {worst:?}, budget {budget:?}: {latencies:?}");
}

#[test]
fn a_view_stays_connected_while_a_task_prints_ten_megabytes() {
    let ws = Workspace::new("burst-view");
    let mut term = open(&ws);
    ws.ok(&["run", "tool:journey-fixture/burst", "--param", "bytes=10485760", "--detach"]);
    term.leader("s");
    std::thread::sleep(Duration::from_millis(300));
    term.send(b"	");
    let deadline = Instant::now() + Duration::from_secs(120);
    let mut samples = 0u32;
    let mut disconnected = Vec::new();
    loop {
        let screen = term.text();
        samples += 1;
        if !connected(&screen) && !screen.contains("BURST-DONE") { disconnected.push(screen.lines().last().unwrap_or_default().to_string()); }
        if screen.contains("BURST-DONE 105") { break; }
        assert!(Instant::now() < deadline, "the burst never finished on screen after {samples} samples:\n{screen}");
        std::thread::sleep(Duration::from_millis(120));
    }
    let ended = wait_task(&ws, "burst exited", |task| command_id(task) == "tool:journey-fixture/burst" && task["status"] == "exited");
    assert_eq!(ended["code"], 0, "{ended}");
    assert!(disconnected.len() * 4 <= samples as usize, "the view reported no live connection in {} of {samples} samples, e.g. {:?}", disconnected.len(), disconnected.first());
}

#[test]
#[ignore = "exploration aid: prints the launcher screens"]
fn explore_launcher() {
    let ws = Workspace::with_bulk_tools("explore-launcher", 2_000);
    let mut term = Terminal::spawn(&workspace::binary(), &[], &ws.root, &ws.env(), steps::ROWS, steps::COLS);
    term.wait_for("New task", SLOW).unwrap();
    std::thread::sleep(Duration::from_secs(5));
    term.leader("n");
    std::thread::sleep(Duration::from_secs(2));
    println!("--- opened\n{}", term.text());
    term.type_text("bulk-");
    std::thread::sleep(Duration::from_secs(2));
    println!("--- typed bulk-\n{}", term.text());
    term.type_text("04");
    std::thread::sleep(Duration::from_secs(2));
    println!("--- typed 04\n{}", term.text());
}
