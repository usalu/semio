use super::controls::{Action, Flow};
use super::sessions::Link;
use super::windows::{Body, Dashboard, Effect, LinkState};
use crate::daemon::client::Message;
use crate::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
use crate::preferences::{Change, Preferences};
use crate::registry::{Facts, Registry, TaskLabel};
use std::sync::Arc;
use ui_styling::appearance::AppearanceName;
use ui_tui::tui::chrome::ChromeState;
use ui_tui::tui::engine::Tui;
use ui_tui::tui::event::{mods, Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use ui_tui::tui::geometry::{Pos, Size};
use ui_tui::tui::theme::{Status, Theme};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../🧫️fixtures/🔎️launcher/🔣️.json")).unwrap() }

fn registry() -> Arc<Registry> {
    let facts: Facts = serde_json::from_value(fixture()["workspace"].clone()).unwrap();
    Arc::new(Registry::build(std::path::Path::new("workspace"), &facts))
}

fn dashboard_with(preferences: Preferences) -> (Dashboard, Tui) {
    let mut tui = Tui::new(Size { width: 160, height: 48 }, Theme::new(AppearanceName::Dark));
    let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), preferences, std::env::temp_dir().join("unused-dashboard-preferences"), false);
    dashboard.link = Link::offline();
    dashboard.state = LinkState::Connected;
    let loaded = registry();
    dashboard.set_registry(&mut tui, loaded);
    dashboard.frame(&mut tui);
    (dashboard, tui)
}

fn small_view(language: &str) -> (Dashboard, Tui) {
    let mut tui = Tui::new(Size { width: 80, height: 24 }, Theme::new(AppearanceName::Dark));
    let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), Preferences { language: language.into(), ..Default::default() }, std::env::temp_dir().join("unused-dashboard-preferences"), false);
    dashboard.link = Link::offline();
    dashboard.state = LinkState::Connected;
    let loaded = registry();
    dashboard.set_registry(&mut tui, loaded);
    dashboard.frame(&mut tui);
    (dashboard, tui)
}

fn view() -> (Dashboard, Tui) { dashboard_with(Preferences::default()) }

fn key(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: 0 }) }
fn ctrl(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: mods::CTRL }) }
fn named(key: Key) -> Event { Event::Key(KeyEvent { key, mods: 0 }) }

fn press(dashboard: &mut Dashboard, tui: &mut Tui, events: &[Event]) -> Flow {
    let mut flow = Flow::Continue;
    for event in events { flow = dashboard.handle(tui, event); dashboard.frame(tui); }
    flow
}

fn tree_of<R>(dashboard: &Dashboard, tui: &mut Tui, index: usize, read: impl FnOnce(&ui_tui::tui::widget::TreeState) -> R) -> R {
    match &tui.scene.node(dashboard.windows[index].tree.expect("a launcher has a tree")).content { ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::Tree(tree)) => read(tree), _ => panic!("tree expected") }
}

fn visible(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<String> {
    let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
    tree_of(dashboard, tui, index, |tree| launcher.visible_ids(tree))
}

fn query(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> String { tree_of(dashboard, tui, index, |tree| tree.query().to_string()) }

fn type_text(text: &str) -> Vec<Event> { text.chars().map(key).collect() }

fn tabs(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<String> {
    match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.stack_tabs.iter().map(|tab| tab.label.clone()).collect(), _ => Vec::new() }
}

fn statuses(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<Option<Status>> {
    match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.stack_tabs.iter().map(|tab| tab.status).collect(), _ => Vec::new() }
}

fn titles(dashboard: &Dashboard, tui: &mut Tui) -> Vec<String> {
    (0..dashboard.windows.len()).map(|index| match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.title.clone(), _ => String::new() }).collect()
}

fn session(id: &str, label: TaskLabel, status: SessionStatus, code: Option<i32>) -> SessionInfo {
    SessionInfo { session_id: id.into(), command: SessionCommand { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), "x".into()], cwd: "workspace".into(), cols: 80, rows: 24, label, ..Default::default() }, status, pid: Some(42), code, ..Default::default() }
}

fn label(verb: &str, subject: &str, qualifier: &str) -> TaskLabel { TaskLabel { verb: verb.into(), owner: vec!["quiz".into()], subject: subject.into(), qualifier: qualifier.into(), parameters: Vec::new(), members: 0 } }

fn spawned(dashboard: &Dashboard) -> Vec<(String, SessionCommand)> {
    dashboard.link.pending.iter().flat_map(|message| match message {
        ClientMsg::SpawnGroup { members, requires, .. } => requires.iter().chain(members).map(|member| (member.session_id.clone(), member.command.clone())).collect::<Vec<_>>(),
        _ => Vec::new(),
    }).collect()
}

#[test]
fn native_leader_bytes_match_the_shared_control_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = ui_tui::tui::ansi::AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&bytes, &mut events);
    let projection: Vec<_> = events.into_iter().map(|event| match event { Event::Key(KeyEvent { key: Key::Char(character), mods: modifiers }) => serde_json::json!({"key": character.to_string(), "ctrl": modifiers & mods::CTRL != 0}), _ => panic!("unexpected control event") }).collect();
    assert_eq!(serde_json::Value::Array(projection), fixture["events"]);
}

#[test]
fn decoded_terminal_keys_resolve_through_the_keymap_exactly_as_written() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = ui_tui::tui::ansi::AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&bytes, &mut events);
    let keymap = Preferences::default().keymap().0;
    let specs: Vec<String> = events.iter().filter_map(|event| if let Event::Key(key) = event { super::controls::spec_of(key) } else { None }).map(|spec| spec.to_string()).collect();
    assert_eq!(specs, ["ctrl+b", "p", "ctrl+r", "Ü", "ctrl+space"]);
    assert!(keymap.is_prefix(&crate::preferences::keymap::KeySpec::parse(&specs[0]).unwrap()));
    assert_eq!(keymap.resolve(crate::preferences::keymap::Scope::Prefix, &crate::preferences::keymap::KeySpec::parse(&specs[1]).unwrap()), Some("show-settings"));
}

#[test]
fn terminal_bytes_become_the_keymaps_canonical_spellings() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️keymap/🔣️.json")).unwrap();
    for row in fixture["terminal"].as_array().unwrap() {
        let bytes: Vec<u8> = row["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let mut parser = ui_tui::tui::ansi::AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(&bytes, &mut events);
        let specs: Vec<String> = events.iter().filter_map(|event| if let Event::Key(key) = event { super::controls::spec_of(key).map(|spec| spec.to_string()) } else { None }).collect();
        assert_eq!(specs, [row["spec"].as_str().unwrap().to_string()], "{row}");
    }
}

#[test]
fn every_default_action_is_implemented_by_the_view() {
    let keymap = Preferences::default().keymap().0;
    for scope in [crate::preferences::keymap::Scope::Prefix, crate::preferences::keymap::Scope::Window] {
        for action in keymap.actions(scope) { assert!(Action::parse(&action).is_some(), "{action} is bound but the view does not implement it"); }
    }
    let (mut dashboard, mut tui) = view();
    assert_eq!(dashboard.run_action(&mut tui, "detach"), Some(Flow::Quit));
    assert!(dashboard.run_action(&mut tui, "teleport").is_none());
}

#[test]
fn the_prefix_key_opens_a_launcher_whose_typing_filters_the_registry() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    assert_eq!(dashboard.windows.len(), 2);
    assert!(matches!(dashboard.windows[dashboard.focused_index(&tui)].body, Body::Launcher(_)));
    press(&mut dashboard, &mut tui, &type_text("quiz test"));
    assert_eq!(visible(&dashboard, &mut tui, 1), ["@fx/quiz:test"]);
    assert!(tui.render_full().0.contains("quiz"));
}

#[test]
fn starting_from_the_launcher_sends_the_resolved_registry_launch_with_its_task_label() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("quiz test"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter), named(Key::Enter)]);
    let started = spawned(&dashboard);
    assert_eq!(started.len(), 1, "{:?}", dashboard.link.pending);
    let (_, command) = &started[0];
    assert_eq!(command.command_id, "@fx/quiz:test");
    assert_eq!((command.label.verb.as_str(), command.label.subject.as_str()), ("test", "quiz"));
    assert_eq!(command.cmd, "bun");
    assert_eq!(&command.args[..3], ["nx", "run", "@fx/quiz:test"]);
    assert!(command.cols > 10 && command.rows > 5, "the terminal starts at the size of its widget: {}x{}", command.cols, command.rows);
    assert!(matches!(dashboard.windows[1].body, Body::Output { .. }));
    assert_eq!(tabs(&dashboard, &mut tui, 1), ["Tasks", "test quiz"]);
    assert_eq!(statuses(&dashboard, &mut tui, 1), [None, Some(Status::Waiting)]);
}

#[test]
fn chosen_parameters_and_extra_arguments_reach_the_launch() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("quiz test"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
    let Body::Launcher(launcher) = &dashboard.windows[1].body else { panic!("launcher expected") };
    assert_eq!(launcher.stage(), super::launcher::Stage::Configure);
    press(&mut dashboard, &mut tui, &[named(Key::PageUp), named(Key::Down), named(Key::Right)]);
    press(&mut dashboard, &mut tui, &[named(Key::Down), key(' ')]);
    press(&mut dashboard, &mut tui, &[named(Key::Down), key('4')]);
    press(&mut dashboard, &mut tui, &[named(Key::Down)]);
    press(&mut dashboard, &mut tui, &type_text("--filter 'a b'"));
    press(&mut dashboard, &mut tui, &[named(Key::Down)]);
    press(&mut dashboard, &mut tui, &type_text("FOO=1"));
    press(&mut dashboard, &mut tui, &[named(Key::Down), named(Key::Enter)]);
    let started = spawned(&dashboard);
    assert_eq!(started.len(), 1);
    let (_, command) = &started[0];
    assert!(command.args.contains(&"--excludeTaskDependencies".to_string()), "{:?}", command.args);
    assert!(command.env.contains(&("SEMIO_TEST_LEVEL".to_string(), "quick".to_string())));
    assert_eq!(&command.args[command.args.len() - 3..], ["--", "--filter", "a b"], "{:?}", command.args);
    assert!(command.env.contains(&("FOO".to_string(), "1".to_string())));
    assert!(command.env.contains(&("CARGO_BUILD_JOBS".to_string(), "4".to_string())));
}

#[test]
fn commands_that_change_the_repository_ask_before_they_start() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("ticket.close"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter), named(Key::Enter)]);
    let Body::Launcher(launcher) = &dashboard.windows[1].body else { panic!("launcher expected") };
    assert_eq!(launcher.stage(), super::launcher::Stage::Confirm);
    assert!(spawned(&dashboard).is_empty());
    press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
    let Body::Launcher(launcher) = &dashboard.windows[1].body else { panic!("launcher expected") };
    assert_eq!(launcher.stage(), super::launcher::Stage::Configure);
    press(&mut dashboard, &mut tui, &[named(Key::Enter), named(Key::Enter)]);
    assert_eq!(spawned(&dashboard).len(), 1);
}

fn start_through_the_launcher(dashboard: &mut Dashboard, tui: &mut Tui, search: &str) {
    press(dashboard, tui, &[ctrl('b'), key('n')]);
    press(dashboard, tui, &type_text(search));
    press(dashboard, tui, &[named(Key::Enter), named(Key::Enter)]);
}

#[test]
fn a_service_a_command_requires_starts_before_it_and_a_running_one_is_reused() {
    let (mut dashboard, mut tui) = view();
    start_through_the_launcher(&mut dashboard, &mut tui, "@fx/quiz:dev");
    let ids: Vec<String> = spawned(&dashboard).into_iter().map(|(_, command)| command.command_id).collect();
    assert_eq!(ids, ["@fx/hub:dev", "@fx/quiz:dev"]);
    let (mut dashboard, mut tui) = view();
    let mut hub = session("hub", label("dev", "hub", ""), SessionStatus::Running, None);
    hub.command.command_id = "@fx/hub:dev".into();
    dashboard.sessions.insert("hub".into(), hub);
    start_through_the_launcher(&mut dashboard, &mut tui, "@fx/quiz:dev");
    let ids: Vec<String> = spawned(&dashboard).into_iter().map(|(_, command)| command.command_id).collect();
    assert_eq!(ids, ["@fx/quiz:dev"], "the running hub is reused, not started twice");
}

#[test]
fn a_terminal_that_owns_the_keyboard_receives_every_key_but_the_prefix() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    assert!(dashboard.terminal_has_keyboard(&tui));
    let windows = dashboard.windows.len();
    let sent = |dashboard: &Dashboard| -> Vec<Vec<u8>> { dashboard.link.outbox.iter().filter_map(|message| if let ClientMsg::Input { data, .. } = message { Some(data.clone()) } else { None }).collect() };
    press(&mut dashboard, &mut tui, &[ctrl('w'), named(Key::Tab), named(Key::Esc), key('q'), named(Key::Enter)]);
    assert_eq!(dashboard.windows.len(), windows, "ctrl+w must not close a window while a terminal owns the keyboard");
    assert_eq!(sent(&dashboard), [vec![0x17], vec![b'\t'], vec![0x1b], vec![b'q'], vec![b'\r']]);
    press(&mut dashboard, &mut tui, &[ctrl('b'), ctrl('b')]);
    assert_eq!(sent(&dashboard).last().unwrap(), &vec![0x02], "the prefix key pressed twice is sent to the program");
    assert!(!dashboard.armed);
}

#[test]
fn prefix_actions_split_zoom_cycle_and_close_windows() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('|')]);
    assert_eq!(dashboard.windows.len(), 2);
    assert_eq!(dashboard.focused_index(&tui), 1);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('[')]);
    assert_eq!(dashboard.focused_index(&tui), 0);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('z')]);
    assert!(dashboard.layout.zoomed.is_some());
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('z')]);
    assert!(dashboard.layout.zoomed.is_none());
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('-')]);
    assert_eq!(dashboard.windows.len(), 3);
    press(&mut dashboard, &mut tui, &[ctrl('w')]);
    assert_eq!(dashboard.windows.len(), 2);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('x')]);
    assert_eq!(dashboard.windows.len(), 1);
    assert_eq!(press(&mut dashboard, &mut tui, &[ctrl('b'), key('x')]), Flow::Quit);
}

#[test]
fn an_unbound_key_after_the_prefix_is_reported_and_not_forwarded() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('7')]);
    assert!(!dashboard.link.outbox.iter().any(|message| matches!(message, ClientMsg::Input { .. })));
    assert!(dashboard.notice.as_deref().is_some_and(|notice| notice.contains("7")), "{:?}", dashboard.notice);
    assert!(!dashboard.armed);
}

#[test]
fn q_typed_into_a_search_never_quits_and_detach_is_a_prefix_action() {
    let (mut dashboard, mut tui) = view();
    assert_eq!(press(&mut dashboard, &mut tui, &[key('q')]), Flow::Continue);
    assert!(matches!(dashboard.windows[0].body, Body::Launcher(_)), "typing on the overview opens the launcher");
    assert_eq!(query(&dashboard, &mut tui, 0), "q");
    assert_eq!(press(&mut dashboard, &mut tui, &[ctrl('b'), key('d')]), Flow::Quit);
}

#[test]
fn the_footer_hints_are_generated_from_the_keymap_in_the_chosen_language() {
    let (mut dashboard, tui) = view();
    dashboard.armed = true;
    let hints = dashboard.footer_hints(&tui);
    assert!(hints.iter().any(|hint| hint.key == "n" && hint.label == "new task"), "{:?}", hints.iter().map(|hint| (&hint.key, &hint.label)).collect::<Vec<_>>());
    assert!(hints.iter().any(|hint| hint.key == "| / v" && hint.label == "split right"));
    dashboard.armed = false;
    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "Enter" && hint.label == "select"));
    let german = Preferences { language: "de".into(), ..Default::default() };
    let (mut dashboard, mut tui) = dashboard_with(german);
    dashboard.armed = true;
    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "n" && hint.label == "neue Aufgabe"));
    assert_eq!(titles(&dashboard, &mut tui), ["Aufgaben"]);
}

#[test]
fn a_customized_keymap_changes_behavior_and_what_the_footer_says() {
    let mut preferences = Preferences::default();
    preferences.apply(&Change::parse(r#"{"prefix":"ctrl+a","bindings":{"prefix.split-right":["w"]}}"#).unwrap());
    let (mut dashboard, mut tui) = dashboard_with(preferences);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('w')]);
    assert_eq!(dashboard.windows.len(), 1, "the old prefix is no longer reserved");
    press(&mut dashboard, &mut tui, &[ctrl('a'), key('w')]);
    assert_eq!(dashboard.windows.len(), 2);
    dashboard.armed = true;
    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "w" && hint.label == "split right"));
    dashboard.armed = false;
    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "Ctrl+A" && hint.label == "controls"));
}

#[test]
fn the_keyboard_help_lists_every_binding_of_the_effective_keymap() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('?')]);
    let index = dashboard.focused_index(&tui);
    assert!(matches!(dashboard.windows[index].body, Body::Help));
    let rows: Vec<String> = dashboard.windows[index].pane.rows.iter().map(|(text, _)| text.clone()).collect();
    for needle in ["split right", "| / v", "close window", "Ctrl+W", "new task", "Ctrl+B"] { assert!(rows.iter().any(|row| row.contains(needle)), "{needle} missing from {rows:?}"); }
}

#[test]
fn the_launcher_gives_text_editors_a_visible_cursor() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("serve docs"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
    tui.render();
    let index = dashboard.focused_index(&tui);
    assert_eq!(tui.focus(), dashboard.windows[index].input);
    assert!(tui.cursor().is_some(), "the focused parameter editor has a hardware cursor");
    press(&mut dashboard, &mut tui, &[named(Key::PageDown)]);
    tui.render();
    assert_eq!(tui.focus(), Some(dashboard.windows[index].list));
    assert!(tui.cursor().is_none());
}

#[test]
fn launcher_text_editing_follows_the_shared_input_caret() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("serve docs"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
    press(&mut dashboard, &mut tui, &type_text("A界e\u{301}"));
    assert!(tui.render_full().0.contains("A界e\u{301}"), "the composed dashboard frame retains combining marks");
    press(&mut dashboard, &mut tui, &[named(Key::Left), key('x')]);
    let index = dashboard.focused_index(&tui);
    let input = dashboard.windows[index].input.unwrap();
    let value = |tui: &Tui| match &tui.scene.node(input).content { ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::Input(state)) => state.value.clone(), _ => panic!("editor expected") };
    assert_eq!(value(&tui), "A界xe\u{301}");
    press(&mut dashboard, &mut tui, &[named(Key::Backspace), named(Key::Delete)]);
    assert_eq!(value(&tui), "A界");
    press(&mut dashboard, &mut tui, &[named(Key::Home), Event::Paste("Z\nQ".into())]);
    assert_eq!(value(&tui), "Z QA界");
    tui.render();
    let rect = tui.scene.rect(input);
    press(&mut dashboard, &mut tui, &[Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x, y: rect.y }, mods: 0, clicks: 1 }), key('!')]);
    assert_eq!(value(&tui), "!Z QA界");
    press(&mut dashboard, &mut tui, &[named(Key::PageDown), named(Key::Enter)]);
    assert!(spawned(&dashboard).iter().any(|(_, command)| command.args.iter().any(|arg| arg == "!Z QA界")));
}

#[test]
fn tabs_and_titles_come_from_the_task_label_with_status_as_a_glyph() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("a", label("dev", "puzzle3d·react", "#sphere"), SessionStatus::Running, None));
    dashboard.update_session(&mut tui, session("b", label("test", "quiz", ""), SessionStatus::Exited, Some(0)));
    dashboard.update_session(&mut tui, session("c", label("test", "quiz", ""), SessionStatus::Exited, Some(3)));
    let all: Vec<String> = (0..dashboard.windows.len()).flat_map(|index| tabs(&dashboard, &mut tui, index).into_iter().take(1)).collect();
    let stack = tabs(&dashboard, &mut tui, 0);
    assert_eq!(stack.len(), 4, "{all:?}");
    assert_eq!(&stack[2..], ["test quiz", "test quiz ·2"]);
    assert_eq!(statuses(&dashboard, &mut tui, 0), [None, Some(Status::Running), Some(Status::Success), Some(Status::Failure)]);
    assert_eq!(stack[1], "dev puz…·react #sphere");
    assert!(!stack.iter().any(|tab| tab.contains("bun") || tab.contains("nx run")), "{stack:?}");
    dashboard.focus_window(&mut tui, 3);
    let title = dashboard.focus_title(&tui).expect("a focused task has a long title");
    assert!(title.contains("test") && title.contains("quiz") && title.contains("exit 3"), "{title}");
    dashboard.frame(&mut tui);
    assert!(tui.render_full().0.contains("exit 3"), "the long title of the focused task is in the footer");
}

#[test]
fn confirmation_is_operable_with_the_mouse() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("ticket.close"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter), named(Key::PageDown), named(Key::Enter)]);
    let index = dashboard.focused_index(&tui);
    assert!(matches!(&dashboard.windows[index].body, Body::Launcher(launcher) if launcher.stage() == super::launcher::Stage::Confirm));
    assert!(spawned(&dashboard).is_empty());
    tui.render();
    let rect = tui.scene.rect(dashboard.windows[index].list);
    let click = |row| Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x + 2, y: rect.y + row }, mods: 0, clicks: 1 });
    press(&mut dashboard, &mut tui, &[click(0)]);
    assert!(spawned(&dashboard).is_empty());
    let row = match &tui.scene.node(dashboard.windows[index].list).content {
        ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::List(list)) => list.items.iter().position(|line| line.contains("[ Start ]")).unwrap(),
        _ => panic!("confirmation list expected"),
    };
    press(&mut dashboard, &mut tui, &[click(row as u16)]);
    assert_eq!(spawned(&dashboard).len(), 1);
}

#[test]
fn restored_process_output_and_exit_status_render_without_stealing_focus() {
    let (mut dashboard, mut tui) = view();
    let info = session("test", label("test", "quiz", ""), SessionStatus::Exited, Some(0));
    dashboard.update_session(&mut tui, info.clone());
    assert_eq!(dashboard.focused_index(&tui), 0);
    assert!(!dashboard.input_mode);
    let index = dashboard.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == "test")).unwrap();
    dashboard.feed(&mut tui, index, "test output");
    dashboard.focus_window(&mut tui, index);
    dashboard.frame(&mut tui);
    let rendered = tui.render_full().0;
    assert!(rendered.contains("exit 0"), "{rendered:?}");
    assert!(rendered.contains("test output"), "{rendered:?}");
    dashboard.close_window(&mut tui, index);
    dashboard.update_session(&mut tui, info.clone());
    assert_eq!(dashboard.windows.len(), 1);
    dashboard.hidden.clear();
    dashboard.update_session(&mut tui, info);
    assert_eq!(dashboard.windows.len(), 2);
}

#[test]
fn terminal_sizes_follow_their_widget_rectangles_through_every_layout_change() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    let resizes = |dashboard: &Dashboard| -> Vec<(u16, u16)> { dashboard.link.outbox.iter().filter_map(|message| if let ClientMsg::Resize { cols, rows, .. } = message { Some((*cols, *rows)) } else { None }).collect() };
    let first = *resizes(&dashboard).last().expect("the first frame tells the daemon the size");
    let rect = tui.scene.rect(dashboard.windows[1].list);
    assert_eq!((rect.width, rect.height), first);
    let size = match tui.scene.node_mut(dashboard.windows[1].list).widget() { Some(ui_tui::tui::widget::WidgetState::Terminal(terminal)) => terminal.screen.size, _ => panic!("terminal expected") };
    assert_eq!((size.width, size.height), first);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('|')]);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('[')]);
    let after_split = *resizes(&dashboard).last().unwrap();
    assert!(after_split.0 < first.0 || after_split.1 < first.1 || after_split == first, "{after_split:?} vs {first:?}");
    dashboard.handle(&mut tui, &Event::Resize(Size { width: 100, height: 30 }));
    dashboard.frame(&mut tui);
    let shrunk = *resizes(&dashboard).last().unwrap();
    assert!(shrunk.0 < first.0 && shrunk.1 < first.1, "{shrunk:?} vs {first:?}");
    let count = resizes(&dashboard).len();
    dashboard.frame(&mut tui);
    assert_eq!(resizes(&dashboard).len(), count, "an unchanged layout sends nothing");
}

#[test]
fn clicking_a_window_moves_the_keyboard_focus_there() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('|')]);
    assert_eq!(dashboard.focused_index(&tui), 1);
    let rect = tui.scene.rect(dashboard.windows[0].list);
    let click = Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x + 1, y: rect.y }, mods: 0, clicks: 1 });
    press(&mut dashboard, &mut tui, &[click]);
    assert_eq!(dashboard.focused_index(&tui), 0);
}

#[test]
fn shutdown_waits_for_the_daemons_confirmation() {
    let (mut dashboard, mut tui) = view();
    assert_eq!(press(&mut dashboard, &mut tui, &[ctrl('b'), key('Q')]), Flow::Continue);
    assert!(dashboard.shutdown_since.is_some());
    assert!(dashboard.link.outbox.contains(&ClientMsg::Shutdown {}));
    assert!(dashboard.apply(&mut tui, Message::Control(ServerMsg::Shutdown {})), "the daemon's confirmation ends the view");
}

#[test]
fn selection_before_connection_retains_an_owned_cancellable_start() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("quiz build"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter), named(Key::Enter)]);
    let pending = spawned(&dashboard);
    assert_eq!(pending.len(), 1);
    let session_id = pending[0].0.clone();
    assert_eq!(dashboard.sessions[&session_id].status, SessionStatus::Pending);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('c')]);
    assert!(dashboard.link.pending.is_empty());
    assert_eq!(dashboard.sessions[&session_id].code, Some(130));
}

#[test]
fn copying_a_selection_asks_the_terminal_to_copy() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    dashboard.feed(&mut tui, 1, "copy me");
    if let Some(ui_tui::tui::widget::WidgetState::Terminal(terminal)) = tui.scene.node_mut(dashboard.windows[1].list).widget() { terminal.select_all(); }
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('y')]);
    assert!(dashboard.effects.iter().any(|effect| matches!(effect, Effect::Copy(text) if text.contains("copy me"))), "{:?}", dashboard.effects);
}

#[test]
fn the_usage_text_is_generated_from_the_keymap_in_both_languages() {
    let english = super::help_text(&Preferences::default());
    assert!(english.contains("split right") && english.contains("Ctrl+B"));
    let german = super::help_text(&Preferences { language: "de".into(), ..Default::default() });
    assert!(german.contains("rechts teilen") && german.contains("Nach der Präfixtaste"));
}

#[test]
fn repainting_is_incremental_and_follows_typing() {
    let (mut dashboard, mut tui) = view();
    assert!(!tui.render().0.is_empty(), "the first paint draws the screen");
    dashboard.frame(&mut tui);
    assert!(tui.render().0.is_empty(), "an unchanged screen paints nothing");
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    let _ = tui.render();
    press(&mut dashboard, &mut tui, &type_text("zq"));
    let patch = tui.render().0;
    assert!(patch.contains("zq") && patch.len() < 4000, "only the changed cells are emitted: {} bytes", patch.len());
}

#[test]
fn the_right_button_opens_a_context_menu_whose_rows_act_on_the_window() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    let rect = tui.scene.rect(dashboard.windows[1].list);
    let at = Pos { x: rect.x + 2, y: rect.y + 2 };
    press(&mut dashboard, &mut tui, &[Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Right), pos: at, mods: 0, clicks: 1 })]);
    assert_eq!(tui.overlays().len(), 1);
    assert!(dashboard.menu.is_some());
    press(&mut dashboard, &mut tui, &[named(Key::Esc)]);
    assert!(tui.overlays().is_empty() && dashboard.menu.is_none());
    assert!(dashboard.input_mode);
    press(&mut dashboard, &mut tui, &[Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Right), pos: at, mods: 0, clicks: 1 })]);
    press(&mut dashboard, &mut tui, &[named(Key::Down), named(Key::Enter)]);
    assert!(tui.overlays().is_empty());
    assert!(!dashboard.input_mode, "the second row toggles the terminal input");
}

#[test]
fn window_controls_are_named_in_the_language_of_the_dashboard_and_stay_quiet_when_unchanged() {
    let read = |dashboard: &Dashboard, tui: &mut Tui| match tui.scene.node_mut(dashboard.windows[0].chrome).chrome() { Some(ChromeState::Window(window)) => (window.labels.close.clone(), window.new_tab), _ => panic!("window expected") };
    let (dashboard, mut tui) = view();
    assert_eq!(read(&dashboard, &mut tui), ("Close tab".to_string(), true));
    let (dashboard, mut tui) = dashboard_with(Preferences { language: "de".into(), ..Default::default() });
    assert_eq!(read(&dashboard, &mut tui), ("Tab schließen".to_string(), true));
    let _ = tui.render();
    let mut dashboard = dashboard;
    dashboard.frame(&mut tui);
    assert!(tui.render().0.is_empty(), "a frame that changes nothing paints nothing");
}

#[test]
fn a_compound_opens_one_tab_per_member_each_with_its_own_task_label() {
    let (mut dashboard, mut tui) = view();
    start_through_the_launcher(&mut dashboard, &mut tui, "quiz-with-hub");
    let members = spawned(&dashboard);
    assert_eq!(members.iter().map(|(_, command)| command.label.subject.as_str()).collect::<Vec<_>>(), ["hub", "quiz"]);
    for (id, command) in members.iter().skip(1) {
        let mut info = session(id, command.label.clone(), SessionStatus::Running, None);
        info.command = command.clone();
        dashboard.update_session(&mut tui, info);
    }
    let stack = tabs(&dashboard, &mut tui, 0);
    assert_eq!(stack.len(), 3, "{stack:?}");
    assert!(stack.iter().any(|tab| tab.ends_with("dev hub")) && stack.iter().any(|tab| tab.ends_with("dev quiz")), "{stack:?}");
}

#[test]
fn daemon_errors_reach_the_user_in_their_language_and_never_the_terminal() {
    let (mut dashboard, mut tui) = view();
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "no such task".into(), code: Some(crate::ipc::ErrorCode::UnknownSession), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("that task no longer exists"));
    let (mut dashboard, mut tui) = dashboard_with(Preferences { language: "de".into(), ..Default::default() });
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "x".into(), code: Some(crate::ipc::ErrorCode::NotRunning), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("Diese Aufgabe läuft nicht"));
}

#[test]
fn a_refused_view_says_so_in_its_language_and_retries_instead_of_showing_an_empty_daemon() {
    let (mut dashboard, mut tui) = view();
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "16 views".into(), code: Some(crate::ipc::ErrorCode::ViewLimit), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("the daemon serves as many views as it allows; close another view and try again"));
    assert!(dashboard.link.connection.is_none() && matches!(dashboard.state, LinkState::Failed(_)), "the refused connection is dropped so that the retry loop runs");
    let (mut dashboard, mut tui) = dashboard_with(Preferences { language: "de".into(), ..Default::default() });
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "16".into(), code: Some(crate::ipc::ErrorCode::ViewLimit), session_id: None }));
    assert!(dashboard.notice.as_deref().is_some_and(|text| text.contains("Ansichten")));
}

#[test]
fn a_replay_clears_only_the_terminal_it_replays() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("a", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.update_session(&mut tui, session("b", label("test", "hub", ""), SessionStatus::Running, None));
    dashboard.feed(&mut tui, 1, "alpha-output");
    dashboard.feed(&mut tui, 2, "beta-output");
    dashboard.apply(&mut tui, Message::Control(ServerMsg::ReplayStart { session_id: "a".into(), truncated: true }));
    let text = |dashboard: &Dashboard, tui: &mut Tui, index: usize| match tui.scene.node_mut(dashboard.windows[index].list).widget() { Some(ui_tui::tui::widget::WidgetState::Terminal(terminal)) => { terminal.select_all(); terminal.selected_text().unwrap_or_default() } _ => String::new() };
    assert!(!text(&dashboard, &mut tui, 1).contains("alpha-output"));
    assert!(text(&dashboard, &mut tui, 2).contains("beta-output"));
    assert!(dashboard.link.restoring && dashboard.notice.is_some());
    dashboard.apply(&mut tui, Message::Control(ServerMsg::ReplayComplete { session_id: None }));
    assert!(!dashboard.link.restoring);
}

#[test]
fn a_terminal_without_unicode_gets_ascii_status_glyphs() {
    let (mut dashboard, mut tui) = view();
    dashboard.apply_capabilities(&mut tui, false);
    dashboard.update_session(&mut tui, session("a", label("test", "quiz", ""), SessionStatus::Exited, Some(0)));
    assert_eq!(statuses(&dashboard, &mut tui, 0)[1], Some(Status::Success));
    assert_eq!(tui.theme.glyphs, ui_tui::tui::theme::GlyphSet::Ascii);
    assert_eq!(Status::Success.glyph(tui.theme.glyphs, 0), "+");
    assert_eq!(tui.width_mode(), ui_tui::tui::text::WidthMode::Scalar);
}

#[test]
fn the_search_action_starts_a_search_in_the_focused_terminal() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("a", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('/')]);
    let searching = match tui.scene.node_mut(dashboard.windows[1].list).widget() { Some(ui_tui::tui::widget::WidgetState::Terminal(terminal)) => terminal.search().is_some(), _ => false };
    assert!(searching);
    assert!(!dashboard.link.outbox.iter().any(|message| matches!(message, ClientMsg::Input { .. })), "the search consumes the keys");
}

// #region 🔖️RuntimeAudit
#[test]
fn p1_6_the_tasks_list_keeps_its_selection_on_the_same_task_and_shows_status_per_row() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("b", label("test", "hub", ""), SessionStatus::Running, None));
    dashboard.update_session(&mut tui, session("d", label("test", "quiz", ""), SessionStatus::Running, None));
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('h')]);
    let overview = dashboard.focused_index(&tui);
    assert!(matches!(dashboard.windows[overview].body, Body::Overview));
    let rows: Vec<String> = dashboard.windows[overview].pane.rows.iter().map(|(text, _)| text.clone()).collect();
    assert!(rows.iter().any(|row| row.contains("test hub")) && rows.iter().any(|row| row.contains("test quiz")), "the overview created later still lists the tasks: {rows:?}");
    let last = dashboard.windows[overview].pane.rows.len() - 1;
    dashboard.windows[overview].pane.selected = last;
    dashboard.update_session(&mut tui, session("a", label("test", "aaa", ""), SessionStatus::Running, None));
    dashboard.update_session(&mut tui, session("d", label("test", "quiz", ""), SessionStatus::Exited, Some(0)));
    dashboard.frame(&mut tui);
    let window = &dashboard.windows[overview];
    assert!(matches!(window.pane.current(), Some(super::panes::RowAction::Session(id)) if id == "d"), "the selection stays on task d although a row appeared above it and its status changed");
    let statuses = match &tui.scene.node(window.list).content { ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::List(list)) => list.statuses.clone(), _ => Vec::new() };
    assert!(statuses.contains(&Some(Status::Success)) && statuses.contains(&Some(Status::Running)), "{statuses:?}");
}

#[test]
fn p1_6_closing_a_window_focuses_its_neighbour_and_typing_q_never_quits() {
    let (mut dashboard, mut tui) = view();
    for _ in 0..2 { press(&mut dashboard, &mut tui, &[ctrl('b'), key('|')]); }
    assert_eq!(dashboard.windows.len(), 3);
    dashboard.focus_window(&mut tui, 1);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('x')]);
    assert_eq!(dashboard.focused_index(&tui), 0, "the previous window takes the focus");
    assert_eq!(dashboard.windows.len(), 2);
    assert_eq!(press(&mut dashboard, &mut tui, &type_text("quit")), Flow::Continue);
}

#[test]
fn p1_7_an_unchanged_screen_is_neither_dirty_nor_repainted_and_nothing_renders_in_full() {
    let (mut dashboard, mut tui) = view();
    let _ = tui.render();
    for _ in 0..3 { dashboard.frame(&mut tui); }
    assert!(!tui.dirty(), "frames that change nothing leave the engine clean");
    assert!(tui.render_due(10_000).is_none());
    let source = include_str!("../../🦀️.rs").to_string() + include_str!("../../⌨️controls/🦀️.rs") + include_str!("../../📡️sessions/🦀️.rs") + include_str!("../../🪟️windows/🦀️.rs") + include_str!("../../📋️panes/🦀️.rs");
    assert!(!source.contains("render_full"), "the view never asks for a full repaint");
}

#[test]
fn p1_8_the_launcher_filter_pages_jumps_edits_and_previews() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    let _ = tui.render();
    press(&mut dashboard, &mut tui, &type_text("quiz test"));
    let caption = |dashboard: &Dashboard, tui: &mut Tui| match &tui.scene.node(dashboard.windows[1].caption.unwrap()).content { ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::Label(label)) => label.text.clone(), _ => String::new() };
    let text = caption(&dashboard, &mut tui);
    assert!(text.contains("1 of") && text.contains("bun nx run @fx/quiz:test"), "match count and preview: {text}");
    press(&mut dashboard, &mut tui, &[Event::Key(KeyEvent { key: Key::Backspace, mods: mods::ALT })]);
    assert_eq!(query(&dashboard, &mut tui, 1), "quiz");
    press(&mut dashboard, &mut tui, &[ctrl('u')]);
    assert_eq!(query(&dashboard, &mut tui, 1), "");
    let selected = |dashboard: &Dashboard, tui: &mut Tui| tree_of(dashboard, tui, 1, |tree| tree.selected_item());
    let first = selected(&dashboard, &mut tui);
    press(&mut dashboard, &mut tui, &[named(Key::End)]);
    let last = selected(&dashboard, &mut tui);
    assert_ne!(first, last);
    press(&mut dashboard, &mut tui, &[named(Key::Home)]);
    assert_eq!(selected(&dashboard, &mut tui), Some(0));
    let _ = tui.render();
    press(&mut dashboard, &mut tui, &[named(Key::PageDown)]);
    assert_ne!(selected(&dashboard, &mut tui), Some(0), "a page key moves the selection");
    press(&mut dashboard, &mut tui, &[Event::Paste("quiz dev".into())]);
    assert_eq!(query(&dashboard, &mut tui, 1), "quiz dev");
}

#[test]
fn p1_11_hints_fit_whole_at_eighty_columns_in_both_languages() {
    for language in ["en", "de"] {
        let (mut dashboard, mut tui) = small_view(language);
        for armed in [false, true] {
            dashboard.armed = armed;
            dashboard.frame(&mut tui);
            let hints = dashboard.footer_hints(&tui);
            assert!(hints.len() >= 3, "{language}: {:?}", hints.iter().map(|hint| &hint.label).collect::<Vec<_>>());
            let rendered = tui.render_full().0;
            for hint in &hints { assert!(rendered.contains(&hint.label), "{language} armed={armed}: {} missing from the footer", hint.label); }
        }
    }
}

#[test]
fn p1_11_no_user_visible_text_is_hard_coded_outside_the_catalogue() {
    for (name, source) in [("controls", include_str!("../../⌨️controls/🦀️.rs")), ("sessions", include_str!("../../📡️sessions/🦀️.rs")), ("panes", include_str!("../../📋️panes/🦀️.rs")), ("windows", include_str!("../../🪟️windows/🦀️.rs"))] {
        for forbidden in ["\"connecting\"", "\"reconnecting\"", "\"daemon stopped\"", "\"Commands\"", "\"Tasks\"", "\"Settings\"", "commands are still being discovered", "daemon shutdown timed out"] { assert!(!source.contains(forbidden), "{name} spells {forbidden} itself"); }
    }
}

#[test]
fn p2_6_escape_then_prefix_is_two_presses_and_the_armed_prefix_gives_up() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("a", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    let chord = Event::Key(KeyEvent { key: Key::Char('b'), mods: mods::CTRL | mods::ALT });
    press(&mut dashboard, &mut tui, &[chord]);
    let inputs = |dashboard: &Dashboard| -> Vec<Vec<u8>> { dashboard.link.outbox.iter().filter_map(|message| if let ClientMsg::Input { data, .. } = message { Some(data.clone()) } else { None }).collect() };
    assert_eq!(inputs(&dashboard), [vec![0x1b]], "the escape reaches the program");
    assert!(dashboard.armed, "and the prefix is armed");
    let now = std::time::Instant::now();
    assert!(!dashboard.expire_prefix(now));
    assert!(dashboard.prefix_wait(now).is_some());
    assert!(dashboard.expire_prefix(now + std::time::Duration::from_secs(4)));
    assert!(!dashboard.armed && dashboard.prefix_wait(now).is_none());
    press(&mut dashboard, &mut tui, &[ctrl('b'), ctrl('b')]);
    assert_eq!(inputs(&dashboard).last(), Some(&vec![0x02]), "the prefix twice is sent literally");
}

#[test]
fn p2_7_the_limit_is_checked_before_a_window_exists_and_a_failed_send_is_kept_for_the_reconnect() {
    let (mut dashboard, mut tui) = view();
    for _ in 0..128 { dashboard.link.pending.push_back(ClientMsg::Ping {}); }
    start_through_the_launcher(&mut dashboard, &mut tui, "quiz build");
    assert!(spawned(&dashboard).is_empty());
    assert_eq!(dashboard.sessions.len(), 0, "no orphan session");
    assert!(matches!(dashboard.windows[1].body, Body::Launcher(_)), "the launcher window stays a launcher");
    let (mut dashboard, mut tui) = view();
    dashboard.link.online_for_test = true;
    dashboard.link.fail_sends = true;
    start_through_the_launcher(&mut dashboard, &mut tui, "quiz build");
    assert_eq!(spawned(&dashboard).len(), 1, "the start is queued again instead of lost");
    assert!(dashboard.notice.as_deref().is_some_and(|notice| notice.contains("sent again")), "{:?}", dashboard.notice);
}

#[test]
fn p2_7_reconnecting_backs_off_and_starts_over_after_a_success() {
    let mut link = Link::default();
    let now = std::time::Instant::now();
    let mut waits = Vec::new();
    for _ in 0..6 { link.schedule_reconnect(true, now); waits.push(link.next_attempt() - now); }
    assert!(waits.windows(2).all(|pair| pair[1] >= pair[0]) && waits[5] == std::time::Duration::from_secs(8), "{waits:?}");
    link.schedule_reconnect(false, now);
    assert_eq!(link.backoff(), std::time::Duration::from_millis(500));
}
// #endregion 🔖️RuntimeAudit

#[test]
fn every_scenario_of_the_window_view_features_is_proved_by_a_test() {
    let source = include_str!("🦀️.rs");
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🪟️windows/🥒️.feature"), &[source], &[
        ("Tabs and titles come from the task label", &["tabs_and_titles_come_from_the_task_label_with_status_as_a_glyph"]),
        ("Terminals follow the rectangle of their widget", &["terminal_sizes_follow_their_widget_rectangles_through_every_layout_change"]),
        ("Clicking a window moves the keyboard focus there", &["clicking_a_window_moves_the_keyboard_focus_there"]),
        ("Prefix actions split, zoom, cycle and close windows", &["prefix_actions_split_zoom_cycle_and_close_windows"]),
        ("Closing a window focuses its neighbour", &["p1_6_closing_a_window_focuses_its_neighbour_and_typing_q_never_quits"]),
        ("The right button opens a context menu whose rows act on the window", &["the_right_button_opens_a_context_menu_whose_rows_act_on_the_window"]),
        ("Window controls are named in the language of the dashboard", &["window_controls_are_named_in_the_language_of_the_dashboard_and_stay_quiet_when_unchanged"]),
        ("A compound opens one tab per member", &["a_compound_opens_one_tab_per_member_each_with_its_own_task_label"]),
        ("An unchanged screen is neither dirty nor repainted", &["p1_7_an_unchanged_screen_is_neither_dirty_nor_repainted_and_nothing_renders_in_full", "repainting_is_incremental_and_follows_typing"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/📡️sessions/🥒️.feature"), &[source], &[
        ("A selection before the connection is an owned, cancellable start", &["selection_before_connection_retains_an_owned_cancellable_start"]),
        ("The limit is checked before a window exists and a failed send is kept", &["p2_7_the_limit_is_checked_before_a_window_exists_and_a_failed_send_is_kept_for_the_reconnect"]),
        ("Reconnecting backs off and starts over after a success", &["p2_7_reconnecting_backs_off_and_starts_over_after_a_success"]),
        ("Restored output and exit status render without stealing the focus", &["restored_process_output_and_exit_status_render_without_stealing_focus"]),
        ("A replay clears only the terminal it replays", &["a_replay_clears_only_the_terminal_it_replays"]),
        ("Daemon errors reach the user in their language and never the terminal", &["daemon_errors_reach_the_user_in_their_language_and_never_the_terminal"]),
        ("Shutdown waits for the confirmation of the daemon", &["shutdown_waits_for_the_daemons_confirmation"]),
    ]);
    crate::tests::assert_proved(include_str!("../../../🧪️tests/📋️panes/🥒️.feature"), &[source], &[
        ("The tasks list keeps its selection on the same task and shows status per row", &["p1_6_the_tasks_list_keeps_its_selection_on_the_same_task_and_shows_status_per_row"]),
        ("The keyboard help lists every binding of the effective keymap", &["the_keyboard_help_lists_every_binding_of_the_effective_keymap"]),
        ("The usage text is generated from the keymap in both languages", &["the_usage_text_is_generated_from_the_keymap_in_both_languages"]),
        ("Hints fit whole at eighty columns in both languages", &["p1_11_hints_fit_whole_at_eighty_columns_in_both_languages"]),
        ("The launcher gives text editors a visible cursor", &["the_launcher_gives_text_editors_a_visible_cursor"]),
        ("Launcher text editing follows the shared input caret", &["launcher_text_editing_follows_the_shared_input_caret"]),
        ("Confirmation is operable with the mouse", &["confirmation_is_operable_with_the_mouse"]),
        ("The configured start action needs one click", &["the_configured_start_action_needs_one_click"]),
        ("Glyph capabilities preserve the framework width policy", &["glyph_capabilities_preserve_the_framework_width_policy"]),
        ("A pending action is visible before routine footer status", &["an_armed_prefix_notice_precedes_routine_footer_status"]),
        ("Restore requested before the first snapshot raises the replayed task", &["restoring_before_the_first_snapshot_focuses_the_replayed_task"]),
    ]);
}

#[test]
fn restoring_all_task_views_raises_and_focuses_the_most_recent_one() {
    let (mut dashboard, mut tui) = view();
    for (id, started) in [("a", 10), ("b", 30), ("c", 20)] {
        let mut info = session(id, label("test", id, ""), SessionStatus::Running, None);
        info.started_ms = started;
        dashboard.update_session(&mut tui, info);
    }
    for index in (1..dashboard.windows.len()).rev() { dashboard.close_window(&mut tui, index); }
    dashboard.hidden.extend(["a", "b", "c"].map(String::from));
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('s')]);
    let focused = dashboard.focused_index(&tui);
    assert_eq!(dashboard.windows[focused].session().map(|session| session.session_id.as_str()), Some("b"), "the newest task is in front");
    assert!(dashboard.terminal_has_keyboard(&tui), "and takes the keys without another Tab");
    for _ in 0..3 { tui.render(); dashboard.frame(&mut tui); }
    assert_eq!(dashboard.windows[dashboard.focused_index(&tui)].session().map(|session| session.session_id.as_str()), Some("b"), "layout and repaint retain the restored task focus");
}

#[test]
fn nothing_the_view_does_prints_into_the_terminal_it_draws_on() {
    for (name, source) in [("sessions", include_str!("../../📡️sessions/🦀️.rs")), ("controls", include_str!("../../⌨️controls/🦀️.rs")), ("windows", include_str!("../../🪟️windows/🦀️.rs")), ("panes", include_str!("../../📋️panes/🦀️.rs"))] {
        for forbidden in ["println!", "eprintln!", "print!(", "start_detached"] { assert!(!source.contains(forbidden), "{name} uses {forbidden}, which writes under the screen"); }
    }
}

#[test]
fn the_configured_start_action_needs_one_click() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    press(&mut dashboard, &mut tui, &type_text("serve docs"));
    press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
    press(&mut dashboard, &mut tui, &type_text("docs"));
    let index = dashboard.focused_index(&tui);
    tui.render();
    let rect = tui.scene.rect(dashboard.windows[index].list);
    let row = match &tui.scene.node(dashboard.windows[index].list).content {
        ui_tui::tui::scene::NodeContent::Widget(ui_tui::tui::widget::WidgetState::List(list)) => list.items.iter().position(|line| line.contains("[ Start ]")).unwrap(),
        _ => panic!("configuration list expected"),
    };
    press(&mut dashboard, &mut tui, &[Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x + 2, y: rect.y + row as u16 }, mods: 0, clicks: 1 })]);
    let commands = spawned(&dashboard);
    assert_eq!(commands.len(), 1);
    assert!(commands[0].1.args.iter().any(|arg| arg == "docs"));
}

#[test]
fn glyph_capabilities_preserve_the_framework_width_policy() {
    use ui_tui::tui::text::WidthMode;
    let (mut dashboard, mut tui) = view();
    dashboard.apply_capabilities(&mut tui, true);
    assert_eq!(tui.width_mode(), WidthMode::Scalar);
    tui.set_width_mode(WidthMode::Cluster);
    dashboard.apply_capabilities(&mut tui, true);
    assert_eq!(tui.width_mode(), WidthMode::Cluster);
    dashboard.apply_capabilities(&mut tui, false);
    assert_eq!(tui.width_mode(), WidthMode::Cluster);
}

#[test]
fn an_armed_prefix_notice_precedes_routine_footer_status() {
    for language in ["en", "de"] {
        let (mut dashboard, mut tui) = small_view(language);
        press(&mut dashboard, &mut tui, &[ctrl('b')]);
        let notice = dashboard.text().key_armed.as_str();
        let status = match &tui.scene.node(dashboard.shell.footer).content {
            ui_tui::tui::scene::NodeContent::Chrome(ChromeState::Footer(footer)) => &footer.status,
            _ => panic!("footer expected"),
        };
        assert!(status.starts_with(notice), "the pending action must fit before routine status: {status}");
        assert!(tui.render_full().0.contains(notice), "the pending prefix is actually visible in {language}");
    }
}
#[test]
fn restoring_before_the_first_snapshot_focuses_the_replayed_task() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('s')]);
    let mut info = session("late", label("test", "late", ""), SessionStatus::Running, None);
    info.started_ms = 30;
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Sessions { sessions: vec![info], more: true }));
    assert!(dashboard.windows[dashboard.focused_index(&tui)].session().is_none(), "restore waits for the final snapshot page");
    let mut older = session("older", label("test", "older", ""), SessionStatus::Running, None);
    older.started_ms = 10;
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Sessions { sessions: vec![older], more: false }));
    for _ in 0..3 { tui.render(); dashboard.frame(&mut tui); }
    assert_eq!(dashboard.windows[dashboard.focused_index(&tui)].session().map(|session| session.session_id.as_str()), Some("late"));
    assert!(dashboard.terminal_has_keyboard(&tui));
}
