//! 🧭️ Pseudo-terminal journeys of the semio dashboard (feature: ./🥒️.feature, scenarios tagged @pty).
//! The binary is the real `semio`; `portable-pty` is the terminal driver and `vt100` the screen model, so
//! every assertion reads what a person would read on the screen.

use semio_framework_test_fixtures::repo_root;

#[path = "🧰️steps/🦀️.rs"]
mod steps;
#[path = "🖥️terminal/🦀️.rs"]
mod terminal;
#[path = "🏗️workspace/🦀️.rs"]
mod workspace;

use std::time::Duration;
use steps::{SLOW, assert_repaint_consistent, command_id, open, restore, shows_end, start, tasks, wait_task};
use workspace::Workspace;

fn wait_json(path: &std::path::Path, what: &str) -> serde_json::Value {
    let deadline = std::time::Instant::now() + SLOW;
    loop {
        if let Ok(bytes) = std::fs::read(path) { if let Ok(value) = serde_json::from_slice(&bytes) { return value; } }
        assert!(std::time::Instant::now() < deadline, "no {what} at {}", path.display());
        std::thread::sleep(Duration::from_millis(40));
    }
}

fn process_alive(pid: u64) -> bool {
    let pid = pid.to_string();
    if cfg!(windows) {
        std::process::Command::new("tasklist").args(["/FI", &format!("PID eq {pid}"), "/NH"]).output().is_ok_and(|answer| String::from_utf8_lossy(&answer.stdout).split_whitespace().any(|word| word == pid))
    } else { std::process::Command::new("kill").args(["-0", &pid]).output().is_ok_and(|answer| answer.status.success()) }
}

fn tab_chip(text: &str, label: &str, right: bool, bottom: bool) -> Option<(usize, usize, Vec<String>)> {
    let rows: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    for (y, row) in rows.iter().enumerate() {
        let line: String = row.iter().collect();
        for (byte, _) in line.match_indices(label) {
            let first = line[..byte].chars().count();
            if (first > 60) != right || (y > 17) != bottom || y == 0 || y + 1 >= rows.len() { continue; }
            let x = row[..first].iter().rposition(|character| matches!(character, '┃' | '│' | '║' | '|'))?;
            let end = first + label.chars().count();
            let last = end + row[end..].iter().position(|character| matches!(character, '┃' | '│' | '║' | '|'))?;
            if rows[y - 1..=y + 1].iter().any(|row| row.len() <= last) { continue; }
            return Some((x, y - 1, rows[y - 1..=y + 1].iter().map(|row| row[x..=last].iter().collect()).collect()));
        }
    }
    None
}

#[test]
fn a_draft_starts_and_is_discarded_after_visible_confirmation() {
    let fixture = workspace::journey_fixture();
    let flow = &fixture["domainLifecycle"];
    let draft = &flow["draft"];
    let ws = Workspace::new("draft-human-flow");
    let directory = ws.root.join(draft["directory"].as_str().unwrap());
    let mut term = open(&ws);
    domain_form(&mut term, draft["startId"].as_str().unwrap(), "Draft Title");
    term.type_text(draft["title"].as_str().unwrap());
    term.send(b"\x1b[B\x1b[B\x1b[B\r");
    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
    assert!(!directory.exists());
    assert!(tasks(&ws).is_empty());
    term.checkpoint("draft-confirmation");
    term.send(terminal::ESCAPE);
    term.wait_until("returning to the draft form", SLOW, |text| text.contains("Draft Title") && !text.contains(flow["confirmation"].as_str().unwrap())).unwrap();
    assert!(!directory.exists());
    term.send(terminal::ENTER);
    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for(draft["startLine"].as_str().unwrap(), SLOW).unwrap();
    let started = wait_task(&ws, "draft created", |task| command_id(task) == draft["startId"].as_str().unwrap() && task["status"] == "exited");
    assert_eq!(started["code"], 0);
    assert!(directory.is_dir());
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 0);
    term.checkpoint("draft-created");
    domain_form(&mut term, draft["discardId"].as_str().unwrap(), "Draft Identifier");
    term.type_text(draft["slug"].as_str().unwrap());
    term.send(b"\x1b[B\x1b[B\x1b[B\r");
    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
    assert!(directory.is_dir());
    term.send(terminal::ENTER);
    term.wait_for(draft["discardLine"].as_str().unwrap(), SLOW).unwrap();
    let discarded = wait_task(&ws, "draft discarded", |task| command_id(task) == draft["discardId"].as_str().unwrap() && task["status"] == "exited");
    assert_eq!(discarded["code"], 0);
    assert!(!directory.exists());
    assert_eq!(tasks(&ws).len(), 2);
    term.checkpoint("draft-discarded");
    ws.assert_paths();
    println!("[DEBUG] visible draft confirmations created and discarded the exact isolated directory");
}

fn domain_form(term: &mut terminal::Terminal, command: &str, label: &str) {
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text(command);
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for(label, SLOW).unwrap();
    assert!(term.text().contains(command));
}

#[test]
fn a_closed_ticket_reopens_with_the_required_durable_human_prompt() {
    let fixture = workspace::journey_fixture();
    let flow = &fixture["domainLifecycle"];
    let reopen = &flow["reopen"];
    let ws = Workspace::new("ticket-reopen-human-flow");
    let document = ws.root.join(reopen["documentPath"].as_str().unwrap());
    let important = ws.root.join(reopen["importantPath"].as_str().unwrap());
    std::fs::create_dir_all(document.parent().unwrap()).unwrap();
    let original = serde_json::to_vec_pretty(&reopen["document"]).unwrap();
    std::fs::write(&document, &original).unwrap();
    let mut term = open(&ws);
    domain_form(&mut term, reopen["commandId"].as_str().unwrap(), "Reason to Reopen");
    term.send(b"\x1b[B\x1b[B\x1b[B\r");
    term.wait_for("required", SLOW).unwrap();
    assert!(tasks(&ws).is_empty());
    assert_eq!(std::fs::read(&document).unwrap(), original);
    assert!(!important.exists());
    term.send(b"\x1b[A\x1b[A\x1b[A");
    term.type_text(reopen["prompt"].as_str().unwrap());
    term.send(b"\x1b[B\x1b[B\x1b[B\r");
    term.wait_for(flow["confirmation"].as_str().unwrap(), SLOW).unwrap();
    assert_eq!(std::fs::read(&document).unwrap(), original);
    assert!(!important.exists());
    term.checkpoint("ticket-reopen-confirmation");
    term.send(terminal::ENTER);
    term.wait_for(reopen["expectedLine"].as_str().unwrap(), SLOW).unwrap();
    let task = wait_task(&ws, "ticket reopened", |task| command_id(task) == reopen["commandId"].as_str().unwrap() && task["status"] == "exited");
    assert_eq!(task["code"], 0);
    let actual: serde_json::Value = serde_json::from_slice(&std::fs::read(&document).unwrap()).unwrap();
    assert_eq!(actual["status"], "open");
    let history = actual["interactions"].as_array().expect("durable ticket interaction history");
    let previous = reopen["document"]["interactions"].as_array().unwrap();
    assert_eq!(history.len(), previous.len() + 1);
    for (actual, expected) in history.iter().zip(previous) { for (key, value) in expected.as_object().unwrap() { assert_eq!(&actual[key], value); } }
    assert_eq!(history.last().unwrap()["kind"], "ticket.reopen");
    assert_eq!(history.last().unwrap()["prompt"], reopen["prompt"]);
    assert_eq!(history.last().unwrap()["client"], "claude-code");
    assert!(history.last().unwrap()["date"].as_str().is_some_and(|date| !date.is_empty()));
    assert_eq!(std::fs::read(&important).unwrap(), b"");
    assert_eq!(tasks(&ws).len(), 1);
    term.checkpoint("ticket-reopened");
    ws.assert_paths();
    println!("[DEBUG] visible ticket reopen retained history and persisted the exact required human prompt without management");
}

#[test]
fn a_workspace_terminal_streams_shell_input_and_exits_inside_the_dashboard() {
    let fixture = workspace::journey_fixture();
    let flow = &fixture["workspaceTerminal"];
    let input = &flow["input"][if cfg!(windows) { "windows" } else { "unix" }];
    let ws = Workspace::with_terminal_tool("supervised workspace terminal");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text(flow["commandId"].as_str().unwrap());
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for("[ Start ]", SLOW).unwrap();
    term.checkpoint("workspace-terminal-form");
    let form = term.wait_for("Directory: .", SLOW).unwrap();
    assert!(form.contains(&format!("Shell: ‹{}›", flow[if cfg!(windows) { "windowsShell" } else { "unixShell" }].as_str().unwrap())));
    for _ in 0..4 { term.send(b"\x1b[B"); }
    term.send(terminal::ENTER);
    term.wait_for("Running developer command: terminal", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(500));
    term.type_text(input[0].as_str().unwrap());
    term.send(terminal::ENTER);
    term.wait_until("the exact shell output row", SLOW, |screen| screen.lines().any(|line| line.trim_matches(|character| matches!(character, '┃' | '│' | '║' | '|' | ' ')) == flow["expectedLine"].as_str().unwrap())).unwrap();
    term.type_text(input[1].as_str().unwrap());
    term.send(terminal::ENTER);
    let task = wait_task(&ws, "workspace terminal exited", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
    assert_eq!(task["code"], 0);
    term.wait_until("the terminal exit on screen", SLOW, shows_end).unwrap();
    let log = ws.ok(&["logs", task["session"].as_str().unwrap()]);
    let received = wait_json(&ws.terminal_receipt_path(), "the production terminal receiver's exact arguments and root");
    assert_eq!(received["args"], flow["expectedArguments"][if cfg!(windows) { "windows" } else { "unix" }]);
    assert_eq!(std::path::Path::new(received["root"].as_str().unwrap()).canonicalize().unwrap(), ws.root);
    let raw = ws.ok(&["logs", task["session"].as_str().unwrap(), "--raw"]);
    let after = raw.split_once(&format!("\n{}", flow["expectedLine"].as_str().unwrap())).expect("the actual shell emitted the probe at the start of a raw terminal line").1;
    assert!(after.starts_with('\r') || after.starts_with('\n') || after.starts_with("\x1b["), "the shell receipt has unexpected trailing text: {log}");
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == flow["commandId"].as_str().unwrap()).count(), 1);
    term.checkpoint("workspace-terminal");
    ws.assert_paths();
    println!("[DEBUG] production workspace shell received echo and exit through PTY, logged the exact probe line and exited 0");
}

#[test]
fn a_german_form_submits_choices_flags_quoted_arguments_and_environment() {
    let fixture = workspace::journey_fixture();
    let form = &fixture["localizedForm"];
    let labels = &form["labels"]["de"];
    let ws = Workspace::with_form_tool("German configured form");
    let mut term = terminal::Terminal::spawn(&workspace::binary(), &["--language", "de"], &ws.root, &ws.env(), steps::ROWS, steps::COLS);
    term.wait_for(labels["newTask"].as_str().unwrap(), SLOW).unwrap();
    term.leader("n");
    term.wait_for(labels["commands"].as_str().unwrap(), SLOW).unwrap();
    term.type_text(form["commandId"].as_str().unwrap());
    term.wait_for(labels["matched"].as_str().unwrap(), SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for(labels["required"].as_str().unwrap(), SLOW).unwrap();
    term.type_text(form["input"]["message"].as_str().unwrap());
    term.send(b"\x1b[B\x1b[C");
    term.wait_for(&format!("{}: ‹beta›", labels["flavor"].as_str().unwrap()), SLOW).unwrap();
    term.send(b"\x1b[B ");
    term.wait_for(&format!("[x] {}", labels["enabled"].as_str().unwrap()), SLOW).unwrap();
    term.send(b"\x1b[B");
    term.type_text(form["input"]["extraArguments"].as_str().unwrap());
    term.wait_for(form["input"]["extraArguments"].as_str().unwrap(), SLOW).unwrap();
    term.send(b"\x1b[B");
    term.type_text(form["input"]["extraEnvironment"].as_str().unwrap());
    term.wait_for(form["input"]["extraEnvironment"].as_str().unwrap(), SLOW).unwrap();
    term.send(b"\x1b[B");
    let screen = term.wait_for(labels["start"].as_str().unwrap(), SLOW).unwrap();
    assert!(screen.contains(labels["extraArguments"].as_str().unwrap()) && screen.contains(labels["extraEnvironment"].as_str().unwrap()));
    for key in ["title", "description", "message", "flavor", "enabled"] { assert!(screen.contains(labels[key].as_str().unwrap()), "missing localized {key}: {screen}"); }
    assert!(!screen.contains("required") && !screen.contains("Extra arguments"));
    for key in ["title", "description", "message", "flavor", "enabled"] { assert!(!screen.contains(form["labels"]["en"][key].as_str().unwrap()), "English {key} leaked into German form: {screen}"); }
    term.checkpoint("form-before-start");
    term.send(terminal::ENTER);
    term.wait_for("CONFIGURED", SLOW).unwrap();
    let task = wait_task(&ws, "configured form exited", |task| command_id(task) == form["commandId"].as_str().unwrap() && task["status"] == "exited");
    let logs = ws.ok(&["logs", task["session"].as_str().unwrap()]);
    let actual: serde_json::Value = serde_json::from_str(logs.lines().find_map(|line| line.strip_prefix("CONFIGURED ")).expect("configured receipt").trim()).unwrap();
    assert_eq!(actual, form["expected"]);
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == form["commandId"].as_str().unwrap()).count(), 1);
    term.checkpoint("form-received");
    ws.assert_paths();
    println!("[DEBUG] German PTY form received exact independent JSON receipt {actual}");
}

#[test]
fn settings_changes_persist_when_the_view_is_reopened() {
    let ws = Workspace::new("persisted settings");
    let fixture = workspace::journey_fixture();
    let mut term = open(&ws);
    term.leader("p");
    term.wait_for("Language: en", SLOW).unwrap();
    term.send(terminal::ENTER);
    term.wait_for("Sprache: de", SLOW).unwrap();
    term.wait_for("Einstellungen gespeichert", SLOW).unwrap();
    term.send(b"\x1b[B");
    term.send(terminal::ENTER);
    term.wait_for("Darstellung: light", SLOW).unwrap();
    let journal = std::fs::read_to_string(ws.preference_path()).unwrap();
    let events: Vec<serde_json::Value> = journal.lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    let changes: Vec<serde_json::Value> = events.iter().map(|event| event["change"].clone()).collect();
    assert_eq!(serde_json::Value::Array(changes), fixture["preferences"]["events"]);
    assert!(events.iter().enumerate().all(|(index, event)| event["version"] == 1 && event["revision"] == index + 1));
    term.checkpoint("preferences-saved");
    term.leader("d");
    assert_eq!(term.exit_code(SLOW), Some(0));
    let mut env = ws.env();
    env.iter_mut().find(|(key, _)| key == "SEMIO_LOCALE").unwrap().1.clear();
    let mut reopened = terminal::Terminal::spawn(&workspace::binary(), &[], &ws.root, &env, steps::ROWS, steps::COLS);
    reopened.wait_for("Neue Aufgabe", SLOW).unwrap();
    reopened.leader("p");
    reopened.wait_for("Sprache: de", SLOW).unwrap();
    reopened.wait_for("Darstellung: light", SLOW).unwrap();
    assert_eq!(std::fs::read_to_string(ws.preference_path()).unwrap(), journal);
    reopened.checkpoint("preferences-reopened");
    ws.assert_paths();
    println!("[DEBUG] visible settings wrote consecutive events and reopened German light preferences without environment overrides");
}

#[test]
fn a_ready_server_opens_its_address_restarts_and_stops_from_the_view() {
    let fixture = workspace::journey_fixture();
    let flow = &fixture["readyLifecycle"];
    let ws = Workspace::new("ready lifecycle");
    let mut env = ws.env();
    env.push(("SEMIO_DASHBOARD_URL_OPENER".into(), serde_json::to_string(&["bun".to_string(), workspace::fixture_script().display().to_string(), "open-url".to_string()]).unwrap()));
    env.push(("JOURNEY_OPEN_RECORD".into(), ws.opened_url_path().display().to_string()));
    let mut term = terminal::Terminal::spawn(&workspace::binary(), &[], &ws.root, &env, steps::ROWS, steps::COLS);
    term.wait_for("New task", SLOW).unwrap();
    start(&mut term, "serve-a", "journey server listening");
    let before = wait_task(&ws, "ready server", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["readyUrl"].is_string());
    let url = format!("http://127.0.0.1:{}{}", ws.port_a, flow["path"].as_str().unwrap());
    assert_eq!(before["readyUrl"], url);
    term.leader("h");
    term.wait_for(&url, SLOW).unwrap();
    term.send(b"\x1b[F");
    term.leader(flow["openPrefix"].as_str().unwrap());
    assert_eq!(wait_json(&ws.opened_url_path(), "opened URL"), serde_json::json!({ "url": url }));
    term.wait_for(flow["openComplete"].as_str().unwrap(), SLOW).unwrap();
    term.checkpoint("ready-opened");
    term.send(b"\x1b[A");
    term.send(terminal::ENTER);
    term.wait_for("journey server listening", SLOW).unwrap();
    term.leader("d");
    assert_eq!(term.exit_code(SLOW), Some(0));
    std::fs::remove_file(ws.opened_url_path()).unwrap();
    env.push(("JOURNEY_OPEN_DELAY".into(), "10000".into()));
    let bindings = serde_json::json!({ "prefix.open-ready": [flow["remappedOpenPrefix"].as_str().unwrap()], "prefix.cancel-open-ready": [flow["remappedCancelPrefix"].as_str().unwrap()] }).to_string();
    term = terminal::Terminal::spawn(&workspace::binary(), &["--bindings", &bindings], &ws.root, &env, steps::ROWS, steps::COLS);
    term.wait_for("New task", SLOW).unwrap();
    restore(&mut term, "journey server listening");
    term.leader(flow["remappedOpenPrefix"].as_str().unwrap());
    let opener = wait_json(&ws.opened_url_path(), "slow opener process");
    assert_eq!(opener["url"], url);
    let pid = opener["pid"].as_u64().unwrap();
    assert!(process_alive(pid));
    term.leader(flow["remappedCancelPrefix"].as_str().unwrap());
    term.wait_for(flow["openCancelled"].as_str().unwrap(), SLOW).unwrap();
    let cancel_deadline = std::time::Instant::now() + SLOW;
    while process_alive(pid) {
        assert!(std::time::Instant::now() < cancel_deadline, "cancelled opener {pid} still exists");
        std::thread::sleep(Duration::from_millis(50));
    }
    let retained = wait_task(&ws, "server survived opener cancellation", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "running");
    assert_eq!(retained["pid"], before["pid"]);
    term.checkpoint("ready-cancelled");
    term.leader(flow["restartPrefix"].as_str().unwrap());
    let after = wait_task(&ws, "restarted ready server", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["pid"].is_number() && task["pid"] != before["pid"] && task["readyUrl"] == url);
    assert_eq!(before["session"], after["session"]);
    assert_ne!(before["pid"], after["pid"]);
    assert_eq!(Workspace::get(ws.port_a, flow["path"].as_str().unwrap()).unwrap(), (200, flow["response"].as_str().unwrap().to_string()));
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == flow["commandId"].as_str().unwrap()).count(), 1);
    term.leader(flow["stopPrefix"].as_str().unwrap());
    wait_task(&ws, "server stopped by UI", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
    term.wait_until("the stopped server on screen", SLOW, shows_end).unwrap();
    let deadline = std::time::Instant::now() + SLOW;
    while Workspace::get(ws.port_a, flow["path"].as_str().unwrap()).is_ok() {
        assert!(std::time::Instant::now() < deadline, "stopped server still accepts connections");
        std::thread::sleep(Duration::from_millis(50));
    }
    term.checkpoint("ready-stopped");
    ws.assert_paths();
    println!("[DEBUG] UI ready opener received {url}; restart changed pid {} -> {} and UI stop released its endpoint", before["pid"], after["pid"]);
}

#[test]
fn a_read_only_repository_action_runs_from_the_launcher() {
    let fixture = workspace::journey_fixture();
    let flow = &fixture["repository"];
    let ws = Workspace::new("read only repository action");
    let path = ws.root.join(".🧬semio/🦑️repo/🎯️goals").join(flow["goalId"].as_str().unwrap()).join("🎯️goal.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let bytes = serde_json::to_vec_pretty(&flow["document"]).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    term.type_text(flow["commandId"].as_str().unwrap());
    term.wait_for("1 of", SLOW).unwrap();
    term.send(terminal::ENTER);
    let form = term.wait_for("[ Start ]", SLOW).unwrap();
    term.checkpoint("repository-form");
    assert!(form.contains("repo:goals.list"));
    term.send(terminal::ENTER);
    term.wait_for(flow["expectedLine"].as_str().unwrap(), SLOW).unwrap();
    let task = wait_task(&ws, "repository action completed", |task| command_id(task) == flow["commandId"].as_str().unwrap() && task["status"] == "exited");
    assert_eq!(task["code"], 0);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let log = ws.ok(&["logs", task["session"].as_str().unwrap()]);
    assert!(log.lines().any(|line| line.trim_end() == flow["expectedLine"].as_str().unwrap()));
    term.checkpoint("repository-action");
    ws.assert_paths();
    println!("[DEBUG] read-only repo action showed {:?}, exited 0 and preserved the goal bytes", flow["expectedLine"]);
}

#[test]
fn tabs_occupy_every_corner_and_own_fullscreen_and_close_controls() {
    let fixture = workspace::journey_fixture();
    let preview = &fixture["tabPreview"];
    let ws = Workspace::new("corner-tab-controls");
    let mut term = open(&ws);
    let command_body = |text: &str| text.lines().any(|line| {
        let mut words = line.trim_start_matches(|c| matches!(c, '┃' | '│' | '║' | '|' | ' ')).split_whitespace();
        words.next().is_some_and(|word| word.parse::<usize>().is_ok()) && words.next() == Some("commands")
    });
    term.leader("n");
    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
    let locate = |term: &terminal::Terminal, label: &str, glyph: Option<char>| -> (usize, usize) {
        let rows = term.rows();
        let row = rows.iter().position(|line| line.contains(label)).unwrap();
        let chars: Vec<char> = rows[row].chars().collect();
        let title = rows[row].split_once(label).unwrap().0.chars().count();
        let column = glyph.map_or(title + 1, |glyph| title + chars[title..].iter().position(|c| *c == glyph).unwrap());
        (column + 1, row + 1)
    };
    for drop in preview["corners"].as_array().unwrap() {
        let (x, y, right, bottom, label) = (drop["x"].as_u64().unwrap(), drop["y"].as_u64().unwrap(), drop["right"].as_bool().unwrap(), drop["bottom"].as_bool().unwrap(), drop["label"].as_str().unwrap());
        let (column, row) = locate(&term, "Commands", None);
        term.send(format!("\x1b[<0;{column};{row}M\x1b[<32;{x};{y}M").as_bytes());
        let proposed = term.wait_until("the exact proposed destination tab", SLOW, |text| text.contains(&format!("tab at {label}")) && tab_chip(text, preview["label"].as_str().unwrap(), right, bottom).is_some()).unwrap();
        let chip = tab_chip(&proposed, preview["label"].as_str().unwrap(), right, bottom).unwrap();
        assert_eq!(chip.2.len(), preview["rows"].as_u64().unwrap() as usize);
        for control in preview["controls"].as_array().unwrap() { assert!(chip.2[1].contains(control.as_str().unwrap()), "missing preview control: {proposed}"); }
        term.send(format!("\x1b[<0;{x};{y}m").as_bytes());
        term.wait_until("the committed corner tab agrees with its preview", SLOW, |text| !text.contains("Esc cancels") && tab_chip(text, preview["label"].as_str().unwrap(), right, bottom).as_ref() == Some(&chip)).unwrap();
        println!("[DEBUG] independent VT100 committed {label} tab after its visible slot preview");
    }
    let (x, y) = locate(&term, "Tasks", None);
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    let (x, y) = locate(&term, "Commands", Some('⤢'));
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    term.wait_for("⤡", SLOW).unwrap();
    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
    let (x, y) = locate(&term, "Tasks", None);
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    term.wait_until("overview active inside the fullscreen group", SLOW, |text| text.contains("⤡") && !command_body(text)).unwrap();
    let (x, y) = locate(&term, "Commands", None);
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    term.wait_until("the command launcher body", SLOW, command_body).unwrap();
    assert!(term.text().contains("⤡"));
    let (x, y) = locate(&term, "Commands", Some('⤡'));
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    term.wait_for("⤢", SLOW).unwrap();
    let (x, y) = locate(&term, "Tasks", None);
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    let (x, y) = locate(&term, "Commands", Some('✕'));
    term.send(format!("\x1b[<0;{x};{y}M\x1b[<0;{x};{y}m").as_bytes());
    term.wait_until("only the neighbouring overview remains", SLOW, |text| text.contains("Tasks") && !text.contains("Commands") && !command_body(text)).unwrap();
    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
    println!("[DEBUG] per-tab fullscreen focused the inactive command body; restore retained it and close removed only its identity");
}

#[test]
fn tabs_dock_with_a_preview_and_escape_cancels_the_drag() {
    let fixture = workspace::journey_fixture();
    let preview = &fixture["tabPreview"];
    let ws = Workspace::new("window-docking");
    let mut term = open(&ws);
    term.leader("n");
    term.wait_for("commands", SLOW).unwrap();
    std::thread::sleep(Duration::from_millis(250));
    let rows = term.rows();
    let row = rows.iter().position(|line| line.contains("Tasks") && line.contains("Commands")).expect("both tab chips are visible");
    let column = rows[row].split_once("Commands").unwrap().0.chars().count() + 2;
    let (drop_x, drop_y) = (preview["split"]["x"].as_u64().unwrap(), preview["split"]["y"].as_u64().unwrap());
    let held = format!("\x1b[<0;{column};{}M\x1b[<32;{drop_x};{drop_y}M", row + 1);
    term.send(held.as_bytes());
    let proposed = term.wait_until("the proposed right split tab", SLOW, |text| text.contains(preview["split"]["label"].as_str().unwrap()) && tab_chip(text, preview["label"].as_str().unwrap(), true, false).is_some()).unwrap();
    let chip = tab_chip(&proposed, preview["label"].as_str().unwrap(), true, false).unwrap();
    for control in preview["controls"].as_array().unwrap() { assert!(chip.2[1].contains(control.as_str().unwrap()), "missing split preview control: {proposed}"); }
    term.send(b"\x1b");
    term.send(format!("\x1b[<0;{drop_x};{drop_y}m").as_bytes());
    std::thread::sleep(Duration::from_millis(250));
    assert!(term.rows().iter().any(|line| line.contains("Tasks") && line.contains("Commands")), "Escape committed a split: {}", term.text());
    assert!(tab_chip(&term.text(), preview["label"].as_str().unwrap(), true, false).is_none(), "Escape retained the proposed tab: {}", term.text());
    term.send(held.as_bytes());
    term.wait_until("the right split tab preview", SLOW, |text| tab_chip(text, preview["label"].as_str().unwrap(), true, false).as_ref() == Some(&chip)).unwrap();
    term.send(format!("\x1b[<0;{drop_x};{drop_y}m").as_bytes());
    let borders = |text: &str| -> Vec<usize> { text.lines().nth(16).unwrap_or_default().chars().enumerate().filter_map(|(x, c)| matches!(c, '┃' | '│' | '║' | '|').then_some(x)).collect() };
    let split = term.wait_until("two independently tiled windows", SLOW, |text| borders(text).len() == 4).unwrap();
    assert_eq!(tab_chip(&split, preview["label"].as_str().unwrap(), true, false), Some(chip));
    let divider = borders(&split)[2];
    term.send(format!("\x1b[<0;{divider};17M").as_bytes());
    for delta in [3, 7, 5] {
        term.send(format!("\x1b[<32;{};17M", divider + delta).as_bytes());
        term.wait_until("the divider following its held pointer", SLOW, |text| borders(text).get(2) == Some(&(divider + delta))).unwrap();
    }
    term.send(format!("\x1b[<0;{};17m", divider + 5).as_bytes());
    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
    println!("[DEBUG] independent VT100 screen confirmed preview, cancellation, committed side-by-side windows and continuous divider resizing");
}

#[test]
fn a_live_server_moves_between_split_axes_and_tabs_without_restarting() {
    let ws = Workspace::new("live-window-docking");
    let mut term = open(&ws);
    start(&mut term, "serve-a", "journey server listening");
    let before = wait_task(&ws, "the live server", |task| command_id(task) == "tool:journey-fixture/serve-a");
    assert!(before["pid"].is_number());
    let hold = |term: &mut terminal::Terminal, x: usize, y: usize| {
        let rows = term.rows();
        let row = rows.iter().position(|line| line.contains("serve-a")).unwrap();
        let column = rows[row].split_once("serve-a").unwrap().0.chars().count() + 2;
        term.send(format!("\x1b[<0;{column};{}M\x1b[<32;{x};{y}M", row + 1).as_bytes());
    };
    hold(&mut term, 118, 17);
    term.wait_for("split right", SLOW).unwrap();
    term.send(b"\x1b[<0;118;17m");
    term.wait_until("the server beside the task overview", SLOW, |text| text.lines().any(|line| line.contains("Tasks") && line.contains("serve-a")) && text.lines().nth(16).unwrap_or_default().chars().filter(|c| matches!(*c, '┃' | '│' | '║' | '|')).count() == 4).unwrap();
    hold(&mut term, 30, 5);
    term.wait_for("split above", SLOW).unwrap();
    term.send(b"\x1b[<0;30;5m");
    term.wait_until("the server above the task overview", SLOW, |text| {
        let lines: Vec<_> = text.lines().collect();
        match (lines.iter().position(|line| line.contains("serve-a")), lines.iter().position(|line| line.contains("Tasks"))) { (Some(server), Some(overview)) => server < overview, _ => false }
    }).unwrap();
    let rows = term.rows();
    let target_row = rows.iter().position(|line| line.contains("Tasks")).unwrap();
    let target_x = rows[target_row].split_once("Tasks").unwrap().0.chars().count() + 2;
    let target_y = target_row + 1;
    hold(&mut term, target_x, target_y);
    term.wait_for("tab at top left", SLOW).unwrap();
    term.send(format!("\x1b[<0;{target_x};{target_y}m").as_bytes());
    term.wait_until("the server rejoined as a tab", SLOW, |text| text.lines().any(|line| line.contains("Tasks") && line.contains("serve-a"))).unwrap();
    let after = wait_task(&ws, "the same live server", |task| command_id(task) == "tool:journey-fixture/serve-a");
    assert_eq!(before["session"], after["session"]);
    assert_eq!(before["pid"], after["pid"]);
    assert_eq!(tasks(&ws).iter().filter(|task| command_id(task) == "tool:journey-fixture/serve-a").count(), 1);
    assert!(ws.ok(&["logs", after["session"].as_str().unwrap()]).contains("journey server listening"));
    assert_repaint_consistent(&mut term, &["connect", "running", "◐", "◑", "◒", "◓"]);
    println!("[DEBUG] independent VT100, daemon task list and logs confirmed a live process survived right/top/tab docking");
}

#[test]
#[cfg(windows)]
fn the_terminal_driver_reports_physical_emoji_widths() {
    let ws = Workspace::new("physical-width-probe");
    let receipt = ws.root.join("width.json");
    let mut environment = ws.env();
    environment.push(("JOURNEY_WIDTH_RECORD".into(), receipt.display().to_string()));
    let script = workspace::fixture_script();
    let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &[script.to_str().unwrap(), "width-probe"], &ws.root, &environment, 20, 240);
    let fixture = workspace::journey_fixture();
    let mut measurements = Vec::new();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
    let widths: serde_json::Value = serde_json::from_slice(&std::fs::read(repository.join(fixture["widthProbe"]["fixture"].as_str().unwrap())).unwrap()).unwrap();
    for sample in widths["cases"].as_array().unwrap() {
        for model in ["scalar", "cluster"] {
            let text = sample[if model == "scalar" { "scalarText" } else { "text" }].as_str().unwrap();
            let deadline = std::time::Instant::now() + SLOW;
            let native = loop {
                if let Ok(bytes) = std::fs::read(&receipt) {
                    if let Ok(records) = serde_json::from_slice::<Vec<serde_json::Value>>(&bytes) {
                        if records.len() == measurements.len() + 1 { break records.last().unwrap().clone(); }
                    }
                }
                assert!(std::time::Instant::now() < deadline, "physical cursor receipt missing: {}", String::from_utf8_lossy(&term.raw_bytes()));
                std::thread::sleep(Duration::from_millis(10));
            };
            term.wait_for(text, SLOW).unwrap();
            let (_, (row, column)) = term.cursor();
            assert_eq!(native["id"], sample["id"]);
            assert_eq!(native["model"], model);
            assert_eq!(native["columns"], 240);
            assert_eq!(native["y"], 0);
            assert_eq!(row, 0);
            assert_eq!(native["x"], sample[if model == "scalar" { "windowsScalarCells" } else { "windowsClusterCells" }]);
            assert_eq!(u64::from(column), native["x"].as_u64().unwrap());
            let mut measured = native;
            measured["vt100X"] = column.into();
            println!("[DEBUG] actual ConPTY cursor {measured}");
            measurements.push(measured);
            term.send(terminal::ENTER);
        }
    }
    term.wait_for("WIDTH-PROBE-DONE", SLOW).unwrap();
    if let Some(directory) = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR") { std::fs::write(std::path::Path::new(&directory).join("physical-cursor-measurements.json"), serde_json::to_vec_pretty(&measurements).unwrap()).unwrap(); }
    term.checkpoint("physical-cursor");
    ws.assert_paths();
}

#[test]
fn the_terminal_driver_preserves_committed_unicode_text() {
    let ws = Workspace::new("unicode-probe");
    let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &["-e", "process.stdin.setRawMode(true);process.stdin.on('data',data=>process.stdout.write(data));console.log('ready');"], &ws.root, &ws.env(), 20, 120);
    term.wait_for("ready", SLOW).unwrap();
    let repository = repo_root!();
    let vectors: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(repository.join("🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/⌨️input-decoding/🔣️.json")).unwrap()).unwrap();
    for vector in vectors["consoleCommits"].as_array().unwrap() {
        let text = vector["text"].as_str().unwrap();
        term.type_text(text);
        term.wait_for(text, Duration::from_secs(3)).unwrap();
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
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).unwrap();
    let fixture = workspace::journey_fixture();
    let widths: serde_json::Value = serde_json::from_slice(&std::fs::read(repository.join(fixture["widthProbe"]["fixture"].as_str().unwrap())).unwrap()).unwrap();
    let combining = widths["cases"].as_array().unwrap().iter().find(|sample| sample["id"] == "combining").unwrap();
    let combining_cells = combining[if cfg!(windows) { "windowsScalarCells" } else { "scalarCells" }].as_u64().unwrap() as u16;
    let start_column = prefix.chars().count() as u16;
    assert_eq!(end.1, start_column + 3 + combining_cells, "the hardware caret follows the independently measured text columns");
    term.send(b"\x1b[D");
    term.wait_until("the caret crossing one combining cluster", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, start_column + 3))).unwrap();
    let (shown, left) = term.cursor();
    assert!(shown);
    assert_eq!(left, (end.0, start_column + 3), "left crosses one complete combining cluster");
    term.type_text("x");
    term.wait_for("A界xe\u{301}", SLOW).unwrap();
    term.send(b"\x7f\x1b[3~\x1b[H\x1b[200~Z\nQ\x1b[201~");
    term.wait_for("Z QA界", SLOW).unwrap();
    term.wait_until("the pasted caret", Duration::from_millis(1500), |_| term.cursor() == (true, (end.0, start_column + 3))).unwrap();
    let (shown, pasted) = term.cursor();
    assert!(shown);
    assert_eq!(pasted, (end.0, start_column + 3), "the three pasted columns precede the remaining A界");
    let (column, row) = (start_column + 1, end.0 + 1);
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
    let script = workspace::fixture_script().display().to_string();
    let mut term = terminal::Terminal::spawn(&bun, &[&script, "resize"], &ws.root, &ws.env(), 20, 70);
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
}
