/// 🎹️ The keymap and the launcher: leader bytes and canonical spellings, prefix actions, starting, parameters and confirmation, the footer hints and the keyboard help.

#[test]
fn native_leader_bytes_match_the_shared_control_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = semio_framework_ui_tui_terminal::ansi::AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&bytes, &mut events);
    let projection: Vec<_> = events.into_iter().map(|event| match event { Event::Key(KeyEvent { key: Key::Char(character), mods: modifiers }) => serde_json::json!({"key": character.to_string(), "ctrl": modifiers & mods::CTRL != 0}), _ => panic!("unexpected control event") }).collect();
    assert_eq!(serde_json::Value::Array(projection), fixture["events"]);
}

#[test]
fn decoded_terminal_keys_resolve_through_the_keymap_exactly_as_written() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = semio_framework_ui_tui_terminal::ansi::AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&bytes, &mut events);
    let keymap = Preferences::default().keymap().0;
    let specs: Vec<String> = events.iter().filter_map(|event| if let Event::Key(key) = event { super::controls::spec_of(key) } else { None }).map(|spec| spec.to_string()).collect();
    assert_eq!(specs, ["ctrl+b", "p", "ctrl+r", "Ü", "ctrl+space"]);
    assert!(keymap.is_prefix(&dashboard_preferences::keymap::KeySpec::parse(&specs[0]).unwrap()));
    assert_eq!(keymap.resolve(dashboard_preferences::keymap::Scope::Prefix, &dashboard_preferences::keymap::KeySpec::parse(&specs[1]).unwrap()), Some("show-settings"));
}

#[test]
fn terminal_bytes_become_the_keymaps_canonical_spellings() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/⌨️keymap/🔣️.json")).unwrap();
    for row in fixture["terminal"].as_array().unwrap() {
        let bytes: Vec<u8> = row["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let mut parser = semio_framework_ui_tui_terminal::ansi::AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(&bytes, &mut events);
        let specs: Vec<String> = events.iter().filter_map(|event| if let Event::Key(key) = event { super::controls::spec_of(key).map(|spec| spec.to_string()) } else { None }).collect();
        assert_eq!(specs, [row["spec"].as_str().unwrap().to_string()], "{row}");
    }
}

#[test]
fn every_default_action_is_implemented_by_the_view() {
    let keymap = Preferences::default().keymap().0;
    for scope in [dashboard_preferences::keymap::Scope::Prefix, dashboard_preferences::keymap::Scope::Window] {
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
    assert_eq!(&command.args[..3], ["repo", "run", "@fx/quiz:test"]);
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
    assert!(command.args.contains(&"--exclude-dependencies".to_string()), "{:?}", command.args);
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
    let german = Preferences { language: Some("de".into()), ..Default::default() };
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
    let value = |tui: &Tui| match &tui.scene.node(input).content { semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::Input(state)) => state.value.clone(), _ => panic!("editor expected") };
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
