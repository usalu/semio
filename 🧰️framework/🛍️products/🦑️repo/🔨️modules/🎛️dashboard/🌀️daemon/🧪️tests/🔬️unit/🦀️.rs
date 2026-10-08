use super::*;
use std::collections::HashMap;

#[test]
fn unknown_subcommand_returns_usage_without_side_effects() {
    let parsed = ParsedArgs { verb: "daemon".into(), segments: vec!["unknown".into()], flags: HashMap::new() };
    assert_eq!(run(Path::new("."), &parsed), 1);
}

#[test]
fn control_plane_protocol_matches_shared_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌀️control-plane/🔣️.json")).unwrap();
    let accepts = |value: &serde_json::Value| {
        serde_json::from_value::<ipc::ClientMsg>(value.clone()).is_ok_and(|msg| msg.validate().is_ok())
            || serde_json::from_value::<ipc::ServerMsg>(value.clone()).is_ok_and(|msg| msg.validate().is_ok())
    };
    for value in vectors["valid"].as_array().unwrap() {
        assert!(accepts(value), "valid protocol vector: {value}");
    }
    for value in vectors["invalid"].as_array().unwrap() {
        assert!(!accepts(value), "invalid protocol vector: {value}");
    }
}

#[cfg(any(unix, windows))]
mod quick {
use super::*;

#[test]
#[ignore = "Requires the Nx-built CLI executable"]
fn actual_cli_view_auto_starts_daemon_and_detaches_without_stopping_tasks() {
    run_native_view(false);
    run_native_view(true);
}

#[test]
#[ignore = "Requires the installed native CLI executable"]
fn actual_cli_launcher_receives_keys_and_runs_a_launch_configuration() {
    use ui_tui::tui::pty::{Pty, PtySize};
    use std::time::{Duration, Instant};
    let root = control_root("native-launcher");
    let executable = std::env::var("SEMIO_TEST_CLI").expect("installed native CLI executable");
    std::fs::create_dir_all(root.join(".vscode")).unwrap();
    std::fs::write(root.join(".vscode/launch.json"), r#"{ // registered commands
  "configurations": [
    { "name": "📦️build-report", "type": "node-terminal", "request": "launch", "command": "echo launched-$DASHBOARD_TEST_VALUE", "cwd": "${workspaceFolder}", "env": { "DASHBOARD_TEST_VALUE": "build-report" }, "presentation": { "group": "4_build" } },
    { "name": "🚀️publish-report", "type": "node-terminal", "request": "launch", "command": "echo unrelated", "cwd": "${workspaceFolder}" },
  ],
}"#).unwrap();
    let root_text = root.display().to_string();
    let config = root.join("preferences.jsonl").display().to_string();
    let bindings = [("SEMIO_LOCALE", ""), ("SEMIO_APPEARANCE", ""), ("SEMIO_LOCKED_TERMINOLOGY", "")];
    let mut view = Pty::spawn(&executable, &["--root", &root_text, "--config", &config], &bindings, &[], Some(&root), PtySize { cols: 160, rows: 40 }).unwrap();
    let mut screen = ui_tui::tui::vt::VtScreen::new(ui_tui::tui::geometry::Size { width: 160, height: 40 }, 0);
    let mut page = [0u8; 16384];
    let await_text = |view: &mut Pty, screen: &mut ui_tui::tui::vt::VtScreen, expected: &str| {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut page = [0u8; 16384];
        while Instant::now() < deadline && !terminal_text(screen).contains(expected) { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
        assert!(terminal_text(screen).contains(expected), "missing {expected}: {}", terminal_text(screen));
    };
    await_text(&mut view, &mut screen, "New task");
    view.write_all(b"\r").unwrap();
    await_text(&mut view, &mut screen, "publish / launch /");
    view.write_all(b"build report").unwrap();
    await_text(&mut view, &mut screen, "/ build report");
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline && terminal_text(&screen).contains("publish / launch /") { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
    assert!(!terminal_text(&screen).contains("publish / launch /"), "search must narrow the launcher: {}", terminal_text(&screen));
    view.write_all(b"\r").unwrap();
    await_text(&mut view, &mut screen, "launched-build-report");
    await_text(&mut view, &mut screen, "exit 0");
    println!("[DEBUG] actual launcher received keys, searched and ran a launch configuration to exit 0");
    view.write_all(b"\x02Q").unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while view.try_wait().unwrap().is_none() && Instant::now() < deadline { let _ = view.try_read(&mut page); std::thread::sleep(Duration::from_millis(5)); }
    assert_eq!(view.try_wait().unwrap(), Some(0));
    drop(view);
    let deadline = Instant::now() + Duration::from_secs(5);
    while client::Connection::connect(&root).is_ok() && Instant::now() < deadline { std::thread::sleep(Duration::from_millis(20)); }
    assert!(client::Connection::connect(&root).is_err(), "Ctrl+B Q must shut the workspace daemon down");
    remove_control_root(&root);
}

#[test]
#[ignore = "Requires the installed native CLI executable"]
fn actual_cli_first_frame_is_fast_and_settings_survive_reopening() {
    use ui_tui::tui::pty::{Pty, PtySize};
    use std::time::{Duration, Instant};
    let root = control_root("native-startup");
    let executable = std::env::var("SEMIO_TEST_CLI").expect("installed native CLI executable");
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).unwrap();
    let bun = std::env::var("SEMIO_TEST_BUN").unwrap_or_else(|_| "bun".into());
    for index in 0..1000 {
        let directory = root.join(format!("projects/{index}")); std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("project.json"), format!(r#"{{"name":"noise-{index}","targets":{{"build":{{}},"test":{{}}}}}}"#)).unwrap();
    }
    std::fs::create_dir(root.join(".git")).unwrap();
    let root_text = root.display().to_string();
    let config = root.join("preferences.jsonl").display().to_string();
    let mut env = nx_fixture_environment(&root);
    env.extend([("PATH".into(), std::env::var("SEMIO_TEST_PATH").unwrap()), ("SEMIO_LOCALE".into(), "".into()), ("SEMIO_APPEARANCE".into(), "".into()), ("SEMIO_LOCKED_TERMINOLOGY".into(), "".into()), ("SEMIO_DASHBOARD_TRACE".into(), "1".into())]);
    let bindings: Vec<_> = env.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();
    for trial in 0..5 {
        let wrapper = trial >= 3;
        let actual_workspace = trial == 4;
        let workspace_text = workspace.display().to_string();
        let launch_root = if actual_workspace { &workspace_text } else { &root_text };
        if trial == 3 { let status = std::process::Command::new(&executable).args(["daemon", "stop", "--root", &root_text]).status().unwrap(); assert!(status.success()); }
        let started = Instant::now();
        let native_args = ["--root", launch_root, "--config", &config];
        let wrapper_args = ["run", "dashboard", "--", "--root", launch_root, "--config", &config];
        let mut view = Pty::spawn(if wrapper { &bun } else { &executable }, if wrapper { &wrapper_args } else { &native_args }, &bindings, &[], Some(workspace), PtySize { cols: 180, rows: 48 }).unwrap();
        let mut bytes = Vec::new(); let mut page = [0u8; 16384];
        let mut screen = ui_tui::tui::vt::VtScreen::new(ui_tui::tui::geometry::Size { width: 180, height: 48 }, 0);
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline && !String::from_utf8_lossy(&bytes).split("elapsed_us=").nth(1).is_some_and(|tail| tail.contains(['\r', '\n'])) {
            let count = view.try_read(&mut page).unwrap_or(0); bytes.extend_from_slice(&page[..count]); screen.feed(&page[..count]);
            assert_eq!(view.try_wait().unwrap(), None, "startup exited: {}", String::from_utf8_lossy(&bytes));
            std::thread::sleep(Duration::from_millis(2));
        }
        let output = String::from_utf8_lossy(&bytes);
        assert!(output.contains(if trial == 0 { "New task" } else { "Neue Aufgabe" }), "first frame must replay optional language settings: {output}");
        assert!(!output.contains("Language / Sprache"), "startup must not ask questions");
        let rendered = terminal_text(&screen);
        let micros: u128 = rendered.split("elapsed_us=").nth(1).unwrap_or_else(|| panic!("missing rendered trace: {rendered}")).split(|character: char| !character.is_ascii_digit()).next().unwrap().parse().unwrap_or_else(|error| panic!("startup trace: {error}: {rendered}"));
        assert!(output.contains('─') && !output.contains("Ôö"), "native terminal must preserve UTF-8 glyphs");
        assert!(micros < 250_000, "native first frame exceeded 250ms: {micros}us");
        assert!(started.elapsed() < Duration::from_secs(1), "installed front door exceeded 1s: {:?}", started.elapsed());
        println!("[DEBUG] actual native first frame trial={trial} wrapper={wrapper} actual_workspace={actual_workspace} native_us={micros} host_ms={}", started.elapsed().as_millis());
        let deadline = Instant::now() + Duration::from_secs(5);
        let connection_root = if actual_workspace { workspace } else { &root };
        let connection = loop { match client::Connection::connect(connection_root) { Ok(connection) => break connection, Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)), Err(error) => panic!("daemon connection: {error}") } };
        if trial == 0 {
            view.write_all(b"\x02p").unwrap();
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline && !terminal_text(&screen).contains("Language: en") { let count = view.try_read(&mut page).unwrap_or(0); bytes.extend_from_slice(&page[..count]); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
            assert!(terminal_text(&screen).contains("Language: en"), "settings must be optional and accessible: {}", terminal_text(&screen));
            view.write_all(b"\r").unwrap();
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline && crate::preferences::replay(&root.join("preferences.jsonl"), &mut crate::preferences::Preferences::default()).ok() != Some(1) { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(10)); }
            let mut preferences = crate::preferences::Preferences::default();
            assert_eq!(crate::preferences::replay(&root.join("preferences.jsonl"), &mut preferences).unwrap(), 1);
            assert_eq!(preferences.language, "de");
            println!("[DEBUG] actual settings committed German preference event revision 1");
        }
        if actual_workspace {
            view.write_all(b"\x02p").unwrap();
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline && !terminal_text(&screen).contains("Sprache: de") { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
            assert!(terminal_text(&screen).contains("Sprache: de"), "workspace discovery must keep settings responsive: {}", terminal_text(&screen));
            println!("[DEBUG] actual workspace settings stayed responsive during background discovery");
        }
        view.write_all(b"\x02eq").unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while view.try_wait().unwrap().is_none() && Instant::now() < deadline { let _ = view.try_read(&mut page); std::thread::sleep(Duration::from_millis(5)); }
        assert_eq!(view.try_wait().unwrap(), Some(0));
        let daemon_pid = connection.daemon_pid(); drop(view); drop(connection);
        let reattached = client::Connection::connect(connection_root).expect("daemon must survive destroying the native view");
        assert_eq!(reattached.daemon_pid(), daemon_pid);
        println!("[DEBUG] detached startup trial={trial} retained workspace daemon {daemon_pid}");
    }
    let status = std::process::Command::new(&executable).args(["daemon", "stop", "--root", &root_text]).status().unwrap(); assert!(status.success());
    remove_control_root(&root);
}

#[test]
#[ignore = "Requires the Nx-built CLI executable"]
fn actual_cli_daemon_releases_parent_pipes_before_shutdown() {
    let root = control_root("daemon-pipes");
    let executable = std::env::var("SEMIO_TEST_CLI").expect("Nx-built CLI executable");
    let bun = std::env::var("SEMIO_TEST_BUN").unwrap_or_else(|_| "bun".into());
    let source = r#"const args=[process.env.SEMIO_TEST_CLI,'daemon'];let timer;
try{const child=Bun.spawn([...args,'start','--root',process.env.DASHBOARD_TEST_ROOT],{stdout:'pipe',stderr:'pipe'});
const result=await Promise.race([Promise.all([child.exited,new Response(child.stdout).text(),new Response(child.stderr).text()]),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('daemon retained parent pipes')),5000)})]);
if(result[0]!==0)throw Error(result[2]);console.log('[DEBUG] daemon starter released output pipes while daemon remained alive');}
finally{clearTimeout(timer);const child=Bun.spawn([...args,'stop','--root',process.env.DASHBOARD_TEST_ROOT],{stdout:'ignore',stderr:'inherit'});await child.exited;}"#;
    let output = std::process::Command::new(bun).args(["-e", source]).env("SEMIO_TEST_CLI", executable).env("DASHBOARD_TEST_ROOT", &root).output().unwrap();
    assert!(output.status.success(), "pipe lifecycle: {}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    println!("{}", String::from_utf8_lossy(&output.stdout));
    remove_control_root(&root);
}

fn run_native_view(via_nx: bool) {
    use ui_tui::tui::pty::{Pty, PtySize};
    use std::time::{Duration, Instant};
    let root = control_root(if via_nx { "native-nx-view" } else { "native-view" });
    let executable = std::env::var("SEMIO_TEST_CLI").expect("Nx-built CLI executable");
    let root_text = root.display().to_string();
    nx_fixture(&root, &["view", "build"]);
    std::fs::create_dir(root.join(".git")).unwrap();
    let bun = std::env::var("SEMIO_TEST_BUN").unwrap_or_else(|_| "bun".into());
    let search_path = std::env::var("SEMIO_TEST_PATH").expect("task-launch environment before Cargo");
    let mut env = nx_fixture_environment(&root);
    env.extend([("SEMIO_TEST_CLI".into(), executable.clone()), ("PATH".into(), search_path), ("SEMIO_LOCALE".into(), "".into()), ("NX_DAEMON".into(), "false".into()), ("NX_TUI".into(), "false".into()), ("NX_NATIVE_COMMAND_RUNNER".into(), "true".into())]);
    let env: Vec<_> = env.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();
    let args: &[&str] = if via_nx { &["nx", "run", "dashboard-fixture:view", "--outputStyle=stream"] } else { &["./📜️script.ts", "view"] };
    let mut view = Pty::spawn(&bun, args, &env, &[], Some(&root), PtySize { cols: 180, rows: 48 }).unwrap();
    let mut output = Vec::new();
    let mut page = [0u8; 8192];
    let deadline = Instant::now() + Duration::from_secs(15);
    while Instant::now() < deadline && !String::from_utf8_lossy(&output).contains("New task") {
        let count = view.try_read(&mut page).unwrap_or(0);
        output.extend_from_slice(&page[..count]);
        assert_eq!(view.try_wait().unwrap(), None, "native view exited: {}", String::from_utf8_lossy(&output));
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(String::from_utf8_lossy(&output).contains("New task"), "native UI must render task controls without questions: {}", String::from_utf8_lossy(&output));
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut connection = loop { match client::Connection::connect(&root) { Ok(connection) => break connection, Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)), Err(error) => panic!("native daemon connection: {error}") } };
    let daemon_pid = connection.daemon_pid();
    println!("[DEBUG] native view via_nx={via_nx} attached to daemon {daemon_pid}");
    let mut screen = ui_tui::tui::vt::VtScreen::new(ui_tui::tui::geometry::Size { width: 180, height: 48 }, 0); screen.feed(&output);
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline && !root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/commands.json").is_file() { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
    assert!(root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/commands.json").is_file(), "fixture inventory must finish: {}", terminal_text(&screen));
    view.write_all(b"\x02nbuild workspace").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline && (!terminal_text(&screen).contains("/ build workspace") || terminal_text(&screen).contains("no matches")) { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
    assert!(terminal_text(&screen).contains("/ build workspace") && !terminal_text(&screen).contains("no matches"), "search must select a current command: {}", terminal_text(&screen));
    view.write_all(b"\r").unwrap();
    let deadline = Instant::now() + Duration::from_secs(15); let mut completed = false;
    while Instant::now() < deadline && !completed {
        let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]);
        connection.send(&ipc::ClientMsg::List {}).unwrap();
        completed = connection.receive(Duration::from_millis(20)).unwrap().iter().any(|message| matches!(message, client::Message::Control(ipc::ServerMsg::Sessions { sessions, .. }) if sessions.iter().any(|session| session.command.args.iter().any(|argument| argument == "dashboard-fixture:build") && session.code == Some(0))));
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(completed, "the native searchable launcher must run the selected Nx build: {}", terminal_text(&screen));
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline && !terminal_text(&screen).contains("build completed") { let count = view.try_read(&mut page).unwrap_or(0); screen.feed(&page[..count]); std::thread::sleep(Duration::from_millis(5)); }
    assert!(terminal_text(&screen).contains("build completed"), "actual build output must render");
    println!("[DEBUG] native searchable launcher via_nx={via_nx} completed the selected Nx build and rendered its actual output");
    connection.send(&ipc::ClientMsg::Spawn { session_id: "persistent".into(), command: bun_command(&root, "console.log('native task');setInterval(()=>{},1000)") }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    view.write_all(b"\x02d").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline && view.try_wait().unwrap().is_none() {
        let _ = view.try_read(&mut page);
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(view.try_wait().unwrap(), Some(0));
    println!("[DEBUG] native view process exited");
    drop(view);
    println!("[DEBUG] native view terminal owner closed");
    let mut reattached = client::Connection::connect(&root).unwrap();
    assert_eq!(reattached.daemon_pid(), daemon_pid);
    reattached.send(&ipc::ClientMsg::List {}).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut persistent = false;
    while !persistent && Instant::now() < deadline {
        persistent = reattached.receive(Duration::from_millis(100)).unwrap().iter().any(|message| matches!(message, client::Message::Control(ipc::ServerMsg::Sessions { sessions, .. }) if sessions.iter().any(|session| session.session_id == "persistent" && session.status == ipc::SessionStatus::Running)));
    }
    assert!(persistent, "task must survive destroying the view's process job");
    let stopped = std::process::Command::new(&executable).args(["daemon", "stop", "--root", &root_text]).output().unwrap();
    assert!(stopped.status.success(), "shutdown: {}", String::from_utf8_lossy(&stopped.stderr));
    println!("[DEBUG] actual native CLI rendered task controls, auto-started daemon {daemon_pid}, detached with a live task and shut down cleanly");
    drop(connection);
    drop(reattached);
    remove_control_root(&root);
}

fn terminal_text(screen: &ui_tui::tui::vt::VtScreen) -> String {
    (0..screen.size.height).map(|y| (0..screen.size.width).filter_map(|x| screen.cell_at(x, y).map(|cell| cell.ch)).collect::<String>()).collect::<Vec<_>>().join("\n")
}

fn control_root(name: &str) -> PathBuf {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = base.join(format!("dashboard-{name}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn remove_control_root(root: &Path) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        match std::fs::remove_dir_all(root) {
            Ok(()) => return,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) if std::time::Instant::now() >= deadline => panic!("control-plane fixture still owned after shutdown: {error}"),
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
        }
    }
}

#[cfg(any(unix, windows))]
fn bun_command(root: &Path, script: &str) -> ipc::SessionCommand {
    ipc::SessionCommand { cmd: std::env::var("SEMIO_TEST_BUN").unwrap_or_else(|_| "bun".into()), args: vec!["-e".into(), script.into()], cwd: root.display().to_string(), env: vec![("DASHBOARD_TEST_VALUE".into(), "control-plane".into()), ("PATH".into(), std::env::var("SEMIO_TEST_PATH").expect("task-launch environment before Cargo"))], cols: 80, rows: 24, ..Default::default() }
}

fn nx_fixture(root: &Path, commands: &[&str]) {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).unwrap();
    let modules = repo.join("node_modules");
    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd").args(["/C", "mklink", "/J"]).arg(root.join("node_modules")).arg(&modules).output().unwrap();
        assert!(status.status.success(), "fixture dependency junction: {:?}", status);
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(&modules, root.join("node_modules")).unwrap();
    std::fs::write(root.join("nx.json"), r#"{"useInferencePlugins":false,"analytics":false}"#).unwrap();
    let nx_package: serde_json::Value = serde_json::from_slice(&std::fs::read(modules.join("nx/package.json")).unwrap()).unwrap();
    let nx_entry = nx_package["bin"]["nx"].as_str().unwrap();
    std::fs::write(root.join("package.json"), serde_json::to_vec(&serde_json::json!({"name":"dashboard-fixture","private":true,"scripts":{"nx":format!("node ./node_modules/nx/{nx_entry}")}})).unwrap()).unwrap();
    let targets: serde_json::Map<String, serde_json::Value> = commands.iter().map(|command| ((*command).into(), serde_json::json!({"executor":"nx:run-commands","cache":false,"options":{"command":format!("bun ./📜️script.ts {command}")}}))).collect();
    std::fs::write(root.join("project.json"), serde_json::to_vec(&serde_json::json!({"name":"dashboard-fixture","targets":targets})).unwrap()).unwrap();
    std::fs::write(root.join("📜️script.ts"), include_str!("../../../🧫️fixtures/🌀️control-plane/📜️script.ts")).unwrap();
}

fn nx_fixture_environment(root: &Path) -> Vec<(String, String)> {
    [("NX_WORKSPACE_ROOT_PATH", root.to_path_buf()), ("NX_WORKSPACE_DATA_DIRECTORY", root.join(".nx/workspace-data")), ("NX_CACHE_DIRECTORY", root.join(".nx/cache")), ("NX_NATIVE_FILE_CACHE_DIRECTORY", root.join(".nx/native"))].into_iter().map(|(key, path)| (key.into(), path.display().to_string())).collect()
}

#[test]
fn one_daemon_runs_discovered_nx_servers_builds_and_tests_concurrently() {
    let root = control_root("nx");
    nx_fixture(&root, &["dev", "build", "test"]);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let tree = crate::command_tree::discover(&root);
    fn find(node: &crate::command_tree::CommandNode, target: &str) -> Option<crate::command_tree::CommandSpec> {
        if let Some(crate::command_tree::CommandLeaf::Process(command)) = &node.leaf {
            if command.args.iter().any(|arg| arg == &format!("dashboard-fixture:{target}")) { return Some(command.clone()); }
        }
        node.children.iter().find_map(|node| find(node, target))
    }
    let mut supervisor = supervisor::Supervisor::<Vec<u8>>::new(&root).unwrap();
    for target in ["dev", "build", "test"] {
        let spec = find(&tree, target).expect("discovered Nx task");
        let mut env = spec.env;
        env.extend([("DASHBOARD_TEST_VALUE".into(), "control-plane".into()), ("DASHBOARD_TEST_PORT".into(), port.to_string()), ("PATH".into(), std::env::var("SEMIO_TEST_PATH").expect("task-launch environment before Cargo")), ("NX_DAEMON".into(), "false".into()), ("NX_SKIP_PROJECT_GRAPH_CACHE".into(), "true".into())]);
        env.extend(nx_fixture_environment(&root)); env.push(("NX_ISOLATE_PLUGINS".into(), "false".into()));
        let command = ipc::SessionCommand { cmd: spec.cmd, args: spec.args, cwd: spec.cwd.display().to_string(), env, cols: 512, rows: 40, ..Default::default() };
        supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: target.into(), command }).unwrap();
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while std::time::Instant::now() < deadline {
        supervisor.tick().unwrap();
        let sessions = supervisor.snapshot();
        if sessions.iter().filter(|session| session.code == Some(0)).count() == 2 && std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() { break; }
        if sessions.iter().any(|session| session.code.is_some_and(|code| code != 0)) { break; }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let sessions = supervisor.snapshot();
    for session in &sessions { println!("[DEBUG] Nx {} output: {}", session.session_id, String::from_utf8_lossy(&supervisor.output(&session.session_id).unwrap())); }
    assert_eq!(sessions.iter().filter(|session| session.code == Some(0)).count(), 2, "build and test must complete: {:?}", sessions.iter().map(|session| (&session.session_id, session.status, session.code)).collect::<Vec<_>>());
    assert!(std::net::TcpStream::connect(("127.0.0.1", port)).is_ok(), "dev server must continue while build and test finish");
    let previous_pid = sessions.iter().find(|session| session.session_id == "dev").unwrap().pid;
    supervisor.handle_client_msg(ipc::ClientMsg::Restart { session_id: "dev".into() }).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while std::time::Instant::now() < deadline {
        supervisor.tick().unwrap();
        if String::from_utf8_lossy(&supervisor.output("dev").unwrap()).contains("[DEBUG] dev listening") { break; }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(String::from_utf8_lossy(&supervisor.output("dev").unwrap()).contains("[DEBUG] dev listening"), "restarted server must bind the same port: {}", String::from_utf8_lossy(&supervisor.output("dev").unwrap()));
    assert_ne!(supervisor.snapshot().iter().find(|session| session.session_id == "dev").unwrap().pid, previous_pid);
    assert!(std::net::TcpStream::connect(("127.0.0.1", port)).is_ok());
    supervisor.handle_client_msg(ipc::ClientMsg::Ping {}).unwrap();
    supervisor.handle_client_msg(ipc::ClientMsg::Shutdown {}).unwrap();
    assert!(std::net::TcpListener::bind(("127.0.0.1", port)).is_ok());
    println!("[DEBUG] one daemon ran discovered Nx dev/build/test concurrently, restarted server on the same port and released its port on shutdown");
    drop(supervisor);
    remove_control_root(&root);
}

#[cfg(any(unix, windows))]
#[test]
fn supervisor_keeps_sessions_across_views_and_matches_bun_output() {
    use std::sync::{Arc, Mutex};
    #[derive(Clone)]
    struct Capture(Arc<Mutex<Vec<u8>>>);
    impl std::io::Write for Capture {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> { self.0.lock().unwrap().extend_from_slice(bytes); Ok(bytes.len()) }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }
    let root = control_root("replay");
    let script = "console.log(process.env.DASHBOARD_TEST_VALUE); console.log(process.cwd()); console.log(require('child_process').execFileSync('bun',['--version'],{encoding:'utf8'}).trim());";
    let mut command = bun_command(&root, script);
    command.cwd = ipc::canonical_path(&root).display().to_string();
    command.cols = 512;
    let reference = std::process::Command::new(&command.cmd).args(&command.args).current_dir(&command.cwd).envs(command.env.iter().cloned()).output().unwrap();
    assert!(reference.status.success());
    let mut supervisor = supervisor::Supervisor::<Capture>::new(&root).unwrap();
    supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "oracle".into(), command }).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while supervisor.snapshot()[0].status == ipc::SessionStatus::Running && std::time::Instant::now() < deadline {
        supervisor.tick().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert_eq!(supervisor.snapshot()[0].code, Some(0));
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let id = supervisor.attach_client(Capture(bytes.clone())).unwrap();
    supervisor.handle_for(id, ipc::ClientMsg::Attach { client_id: "new-view".into() }).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let encoded = bytes.lock().unwrap().clone();
    let mut frames = std::io::Cursor::new(encoded);
    let mut output = Vec::new();
    let mut restored = false;
    while frames.position() < frames.get_ref().len() as u64 {
        let (kind, payload) = ipc::read_frame(&mut frames).unwrap();
        if kind == ipc::KIND_OUTPUT {
            let (session, data) = ipc::decode_output(&payload).unwrap();
            assert_eq!(session, "oracle");
            output.extend(data);
        } else if let ipc::ServerMsg::Sessions { sessions, .. } = ipc::decode_control(&payload).unwrap() {
            restored = sessions.len() == 1 && sessions[0].code == Some(0);
        }
    }
    assert!(restored);
    let output = String::from_utf8_lossy(&output);
    for line in String::from_utf8(reference.stdout).unwrap().lines() { assert!(output.contains(line), "PTY output: {output}"); }
    println!("[DEBUG] dashboard replay restored exit=0; PTY output matched Bun reference");
    drop(supervisor);
    remove_control_root(&root);
}

#[cfg(any(unix, windows))]
#[test]
fn duplicate_spawn_restart_failure_and_shutdown_preserve_control() {
    let root = control_root("lifecycle");
    let mut supervisor = supervisor::Supervisor::<Vec<u8>>::new(&root).unwrap();
    let command = bun_command(&root, "setInterval(() => {}, 1000)");
    supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "server".into(), command: command.clone() }).unwrap();
    let first = supervisor.snapshot()[0].pid;
    assert!(supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "server".into(), command: command.clone() }).is_err());
    assert!(supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "duplicate".into(), command }).is_err());
    assert_eq!(supervisor.snapshot().len(), 1);
    assert_eq!(supervisor.snapshot()[0].pid, first);
    supervisor.handle_client_msg(ipc::ClientMsg::Restart { session_id: "server".into() }).unwrap();
    assert_ne!(supervisor.snapshot()[0].pid, first);
    let mut missing = bun_command(&root, "");
    missing.cmd = root.join("missing-executable").display().to_string();
    assert!(supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "failure".into(), command: missing }).is_err());
    assert!(supervisor.snapshot().iter().any(|s| s.session_id == "failure" && s.status == ipc::SessionStatus::Failed));
    supervisor.handle_client_msg(ipc::ClientMsg::Ping {}).unwrap();
    supervisor.handle_client_msg(ipc::ClientMsg::Shutdown {}).unwrap();
    assert!(supervisor.is_shutdown());
    assert!(supervisor.snapshot().iter().all(|s| s.status != ipc::SessionStatus::Running));
    println!("[DEBUG] dashboard rejected duplicate processes, restarted task, survived spawn failure and shut down");
    drop(supervisor);
    remove_control_root(&root);
}

#[cfg(any(unix, windows))]
#[test]
fn cancellation_is_bounded_and_completed_sessions_replay_from_events() {
    let root = control_root("cancel");
    let mut supervisor = supervisor::Supervisor::<Vec<u8>>::new(&root).unwrap();
    supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "cancel".into(), command: bun_command(&root, "process.on('SIGINT',()=>{});setInterval(()=>{},1000)") }).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    supervisor.handle_client_msg(ipc::ClientMsg::Stop { session_id: "cancel".into() }).unwrap();
    assert_eq!(supervisor.snapshot()[0].status, ipc::SessionStatus::Stopping);
    supervisor.handle_client_msg(ipc::ClientMsg::Ping {}).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while supervisor.snapshot()[0].status == ipc::SessionStatus::Stopping && std::time::Instant::now() < deadline {
        supervisor.tick().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let completed = supervisor.snapshot();
    assert_eq!(completed[0].status, ipc::SessionStatus::Exited);
    drop(supervisor);
    let restored = supervisor::Supervisor::<Vec<u8>>::new(&root).unwrap();
    assert_eq!(restored.snapshot(), completed);
    println!("[DEBUG] cancellation completed within its grace bound and session projection replayed from persisted events");
    drop(restored);
    remove_control_root(&root);
}

#[cfg(any(unix, windows))]
#[test]
fn killing_a_task_releases_its_child_servers_port() {
    let root = control_root("tree");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let child = format!("require('net').createServer().listen({port},'127.0.0.1');setInterval(()=>{{}},1000)");
    let script = format!("require('child_process').spawn(process.execPath,['-e',{}],{{stdio:'inherit',detached:process.platform!=='win32'}});setInterval(()=>{{}},1000)", serde_json::to_string(&child).unwrap());
    let mut supervisor = supervisor::Supervisor::<Vec<u8>>::new(&root).unwrap();
    let mut command = bun_command(&root, &script);
    command.cmd = "node".into();
    supervisor.handle_client_msg(ipc::ClientMsg::Spawn { session_id: "tree".into(), command }).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() && std::time::Instant::now() < deadline {
        supervisor.tick().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(std::net::TcpStream::connect(("127.0.0.1", port)).is_ok(), "child server must actually bind: {:?}; output: {}", supervisor.snapshot(), String::from_utf8_lossy(&supervisor.output("tree").unwrap()));
    supervisor.handle_client_msg(ipc::ClientMsg::Kill { session_id: "tree".into() }).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    while std::net::TcpListener::bind(("127.0.0.1", port)).is_err() && std::time::Instant::now() < deadline { std::thread::sleep(std::time::Duration::from_millis(10)); }
    assert!(std::net::TcpListener::bind(("127.0.0.1", port)).is_ok(), "killing the parent must release its descendant server");
    println!("[DEBUG] dashboard killed the process tree and released child server port {port}");
    drop(supervisor);
    remove_control_root(&root);
}

#[test]
fn one_workspace_instance_survives_views_and_rejects_a_second_daemon() {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    use std::time::Duration;
    let root = control_root("instance");
    let running = Arc::new(AtomicBool::new(true));
    let worker_running = running.clone();
    let worker_root = root.clone();
    let daemon = std::thread::spawn(move || supervisor::serve(&worker_root, worker_running));
    let mut connection = (0..100).find_map(|_| match client::Connection::connect(&root) { Ok(connection) => Some(connection), Err(_) => { std::thread::sleep(Duration::from_millis(10)); None } }).expect("daemon connection");
    assert!(supervisor::serve(&root, Arc::new(AtomicBool::new(false))).is_err(), "a second daemon must not replace the workspace endpoint");
    connection.send(&ipc::ClientMsg::Spawn { session_id: "persistent".into(), command: bun_command(&root, "console.log('attached');setInterval(()=>{},1000)") }).unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    let mut pid = None;
    while pid.is_none() && std::time::Instant::now() < deadline {
        for message in connection.receive(Duration::from_millis(100)).unwrap() {
            if let client::Message::Control(ipc::ServerMsg::SessionChanged { session }) = message { pid = session.pid; }
        }
    }
    assert!(pid.is_some());
    drop(connection);
    let mut reattached = client::Connection::connect(&root).unwrap();
    reattached.send(&ipc::ClientMsg::Attach { client_id: "reattached".into() }).unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    let mut restored = false;
    while !restored && std::time::Instant::now() < deadline {
        for message in reattached.receive(Duration::from_millis(100)).unwrap() {
            if let client::Message::Control(ipc::ServerMsg::Sessions { sessions, .. }) = message { restored = sessions.len() == 1 && sessions[0].pid == pid; }
        }
    }
    assert!(restored, "reattachment must restore the same running process");
    reattached.send(&ipc::ClientMsg::Shutdown {}).unwrap();
    daemon.join().unwrap().unwrap();
    running.store(false, Ordering::SeqCst);
    assert!(!ipc::pid_path(&root).exists());
    println!("[DEBUG] one workspace daemon retained pid {pid:?} across views and rejected a competing instance");
    drop(reattached);
    remove_control_root(&root);
}

}

/// 🪟 A control frame written by a client reaches the server half of a real named pipe unchanged,
/// and the answer written back reaches the client — the transport `serve` is built on.
#[cfg(windows)]
#[test]
fn a_named_pipe_round_trips_one_frame_each_way() {
    use ipc::{ClientMsg, ServerMsg};
    let name = format!(r"\\.\pipe\semio-dashboard-test-{}", std::process::id());
    let listening = name.clone();
    let server = std::thread::spawn(move || {
        let mut stream = ipc::accept(&listening).expect("accept");
        let (kind, payload) = ipc::read_frame(&mut stream).expect("server frame");
        assert_eq!(kind, ipc::KIND_CONTROL);
        let message: ClientMsg = ipc::decode_control(&payload).expect("client message");
        assert_eq!(message, ClientMsg::Attach { client_id: "frame".into() });
        ipc::write_control(&mut stream, &ServerMsg::Attached { daemon_pid: 7, protocol: ipc::PROTOCOL, build_id: String::new() }).expect("server answer");
    });
    let mut client = None;
    for _ in 0..100 {
        match std::fs::OpenOptions::new().read(true).write(true).open(&name) {
            Ok(handle) => {
                client = Some(handle);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
        }
    }
    let mut client = client.expect("client open");
    ipc::write_control(&mut client, &ClientMsg::Attach { client_id: "frame".into() }).expect("client frame");
    let (kind, payload) = ipc::read_frame(&mut client).expect("client answer");
    assert_eq!(kind, ipc::KIND_CONTROL);
    assert_eq!(ipc::decode_control::<ServerMsg>(&payload).expect("server message"), ServerMsg::Attached { daemon_pid: 7, protocol: ipc::PROTOCOL, build_id: String::new() });
    server.join().expect("server thread");
}

/// 🪟 The whole windows serve loop: a client that opens the daemon's named pipe is greeted, and a
/// ping it sends over that pipe is answered — the transport `semio daemon attach` rides on.
#[cfg(windows)]
#[test]
fn the_windows_serve_loop_greets_and_answers_a_client() {
    use ipc::{ClientMsg, ServerMsg};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let root = std::env::temp_dir().join(format!("semio-daemon-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("root");
    let running = Arc::new(AtomicBool::new(true));
    let serving = Arc::clone(&running);
    let served_root = root.clone();
    let daemon = std::thread::spawn(move || supervisor::serve(&served_root, serving));
    let mut client = ipc::connect(&root).expect("connect");
    let (kind, payload) = ipc::read_frame(&mut client).expect("greeting");
    assert_eq!(kind, ipc::KIND_CONTROL);
    assert!(matches!(ipc::decode_control::<ServerMsg>(&payload).expect("greeting message"), ServerMsg::Attached { .. }));
    ipc::write_control(&mut client, &ClientMsg::Ping {}).expect("ping");
    let (_, payload) = ipc::read_frame(&mut client).expect("pong");
    assert_eq!(ipc::decode_control::<ServerMsg>(&payload).expect("pong message"), ServerMsg::Pong {});
    running.store(false, Ordering::SeqCst);
    daemon.join().expect("daemon thread").expect("serve");
    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(windows)]
#[test]
fn connection_retries_a_temporary_gap_between_named_pipe_instances() {
    use std::time::Duration;
    let root = std::env::temp_dir().join(format!("semio-pipe-gap-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let name = ipc::pipe_name(&root);
    let (continue_server, continue_client) = std::sync::mpsc::channel();
    let server = std::thread::spawn(move || {
        let mut first = ipc::accept(&name).unwrap();
        ipc::write_control(&mut first, &ipc::ServerMsg::Attached { daemon_pid: 7, protocol: ipc::PROTOCOL, build_id: String::new() }).unwrap();
        continue_client.recv().unwrap();
        drop(first);
        std::thread::sleep(Duration::from_millis(200));
        let mut second = ipc::accept(&name).unwrap();
        ipc::write_control(&mut second, &ipc::ServerMsg::Attached { daemon_pid: 7, protocol: ipc::PROTOCOL, build_id: String::new() }).unwrap();
        std::thread::sleep(Duration::from_millis(100));
    });
    let mut first = ipc::connect(&root).unwrap();
    ipc::read_frame(&mut first).unwrap();
    continue_server.send(()).unwrap();
    drop(first);
    let connection = client::Connection::connect(&root);
    let fallback = if connection.is_err() { Some(ipc::connect(&root).unwrap()) } else { None };
    server.join().unwrap();
    drop(fallback);
    assert_eq!(connection.expect("a listener gap must not start a competing daemon").daemon_pid(), 7);
    println!("[DEBUG] dashboard connection retained daemon 7 across a named-pipe listener gap");
}
