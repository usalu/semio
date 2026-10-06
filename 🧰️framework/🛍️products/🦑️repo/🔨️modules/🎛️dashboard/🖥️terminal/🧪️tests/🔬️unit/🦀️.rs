use super::*;

#[test]
fn native_leader_bytes_match_the_shared_control_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = ui_tui::tui::ansi::AnsiParser::new(); let mut events = Vec::new(); parser.feed(&bytes, &mut events);
    let projection: Vec<_> = events.into_iter().map(|event| match event { Event::Key(KeyEvent { key: Key::Char(character), mods: modifiers }) => serde_json::json!({"key": character.to_string(), "ctrl": modifiers & mods::CTRL != 0}), _ => panic!("unexpected control event") }).collect();
    assert_eq!(serde_json::Value::Array(projection), fixture["events"]);
}

#[test]
fn filtered_command_selection_survives_inventory_updates() {
    let (mut dashboard, mut tui) = dashboard(Locale::English);
    let leaf = CommandLeaf::Repo(RepoAction::GoalsList);
    dashboard.commands = vec![("build / other".into(), leaf.clone()), ("test / first".into(), leaf.clone()), ("test / second".into(), leaf.clone())];
    dashboard.replace_view(&mut tui, 0, "launcher");
    if let Some(WidgetState::Wizard(state)) = tui.scene.node_mut(dashboard.windows[0].focus).widget() { state.filter = "test".into(); state.selected = 1; }
    dashboard.commands.insert(0, ("test / added".into(), leaf));
    dashboard.refresh_views(&mut tui);
    if let NodeContent::Widget(WidgetState::Wizard(state)) = &tui.scene.node(dashboard.windows[0].focus).content { assert_eq!(state.options[state.visible_indices()[state.selected]], "test / second"); } else { panic!("launcher missing"); }
}

#[test]
fn launcher_search_matches_language_neutral_selection_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔎️launcher/🔣️.json")).unwrap();
    let options: Vec<String> = fixture["options"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
    for case in fixture["cases"].as_array().unwrap() {
        let mut widget = WidgetState::Wizard(WizardState::new(options.clone()));
        for character in case["filter"].as_str().unwrap().chars() { widget.on_key(&KeyEvent { key: Key::Char(character), mods: 0 }); }
        let selected = widget.on_key(&KeyEvent { key: Key::Enter, mods: 0 });
        assert_eq!(selected, case["index"].as_u64().map(|index| WidgetSignal::Activated(index as usize)), "{case}");
    }
}

#[test]
fn mouse_launcher_selection_matches_the_shared_viewport_vectors() {
    use ui_tui::tui::geometry::Pos;
    use ui_tui::tui::event::{MouseEvent, MouseKind};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔎️launcher/🔣️.json")).unwrap();
    for case in fixture["pointer"].as_array().unwrap() {
        let mut tui = Tui::new(Size { width: 80, height: case["height"].as_u64().unwrap() as u16 }, Theme::new(AppearanceName::Dark));
        let mut state = WizardState::new(fixture["options"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect());
        state.filter = case["filter"].as_str().unwrap().into(); state.selected = case["selected"].as_u64().unwrap() as usize;
        let widget = tui.scene.add(tui.scene.root(), Node::new(NodeContent::Widget(WidgetState::Wizard(state))));
        tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        tui.render_full();
        let result = tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(0), pos: Pos { x: 2, y: case["row"].as_u64().unwrap() as u16 }, mods: 0 }));
        assert_eq!(result, case["index"].as_u64().map(|index| vec![(widget, WidgetSignal::Activated(index as usize))]).unwrap_or_default(), "{case}");
    }
}

fn dashboard(locale: Locale) -> (Dashboard, Tui) {
    let mut tui = Tui::new(Size { width: 160, height: 40 }, Theme::new(AppearanceName::Dark));
    let layout = create_default_layout(&["w1".into()], "row", None, None);
    let shell = shell(&mut tui.scene, NavbarState { left: Vec::new(), center: Vec::new(), right: Vec::new() }, FooterState { hints: Vec::new(), status: String::new() }, &layout);
    let mut dashboard = Dashboard { root: ".".into(), layout, shell, windows: Vec::new(), next_serial: 2, focused: "w1".into(), leader: LeaderMode::Idle, terminal_input: false, connection: None, pending_starts: Default::default(), connecting: None, connection_status: "disconnected".into(), next_reconnect: std::time::Instant::now(), restoring: false, sessions: Default::default(), hidden: Default::default(), locale, light: false, commands: Vec::new(), inventory: None, inventory_status: String::new(), last_progress: 0, preferences: crate::preferences::Preferences::default(), preference_path: std::env::temp_dir().join("unused-dashboard-preferences"), saving_preferences: None, shared_preferences: false };
    let chrome = dashboard.shell.windows[0].1;
    let window = dashboard.attach_view(&mut tui, chrome, "w1", "overview");
    dashboard.windows.push(window);
    dashboard.remount(&mut tui);
    (dashboard, tui)
}

#[test]
fn selection_before_connection_retains_an_owned_cancellable_start() {
    let (mut dashboard, mut tui) = dashboard(Locale::English);
    dashboard.spawn_output(&mut tui, "w1", CommandSpec { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), "workspace:build".into()], cwd: ".".into(), env: Vec::new() });
    assert_eq!(dashboard.pending_starts.len(), 1);
    let session = dashboard.sessions.values().next().unwrap().clone();
    assert!(session.command.cols > 1 && session.command.rows > 1);
    assert!(dashboard.cancel_pending_start(&mut tui, &session.session_id));
    assert!(dashboard.pending_starts.is_empty());
    assert_eq!(dashboard.sessions[&session.session_id].code, Some(130));
}

#[test]
fn opinionated_defaults_and_controls_are_visible_in_both_languages() {
    let (mut dashboard, mut tui) = dashboard(Locale::English);
    assert!(matches!(dashboard.windows[0].body, WindowBody::Overview { .. }), "ordinary startup must not ask for language");
    assert!(tui.render_full().0.contains("New task"));
    dashboard.handle_view_signal(&mut tui, "w1", WidgetSignal::Activated(0));
    assert!(matches!(dashboard.windows[0].body, WindowBody::Launcher { .. }));
    dashboard.locale = Locale::German;
    dashboard.leader = LeaderMode::Armed;
    assert!(dashboard.footer_hints().iter().any(|hint| hint.key == "r" && hint.label == "neu starten"));
    dashboard.locale = Locale::English;
    assert!(dashboard.footer_hints().iter().any(|hint| hint.key == "Q" && hint.label == "shutdown all"));
}

#[test]
fn restored_process_output_and_exit_status_render_without_stealing_focus() {
    let (mut dashboard, mut tui) = dashboard(Locale::English);
    let command = SessionCommand { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), "workspace:test".into()], cwd: ".".into(), env: Vec::new(), cols: 80, rows: 24 };
    let info = SessionInfo { session_id: "test".into(), command, status: SessionStatus::Exited, pid: Some(42), code: Some(0) };
    dashboard.update_session(&mut tui, info.clone());
    assert_eq!(dashboard.focused, "w1");
    assert!(!dashboard.terminal_input);
    let window = dashboard.windows.iter().find(|window| matches!(&window.body, WindowBody::Output { session: Some(session), .. } if session.session_id == "test")).unwrap();
    let id = window.id.clone();
    let terminal = window.focus;
    Dashboard::feed(&mut tui, terminal, "test output\n");
    activate_stack_tab(&mut dashboard.layout, &id);
    dashboard.remount(&mut tui);
    let rendered = tui.render_full().0;
    assert!(rendered.contains("exit 0"));
    assert!(rendered.contains("test output"), "rendered: {rendered:?}");
    dashboard.close_window(&mut tui, &id);
    dashboard.update_session(&mut tui, info.clone());
    assert_eq!(dashboard.windows.len(), 1);
    assert_eq!(dashboard.sessions.len(), 1);
    dashboard.hidden.clear();
    dashboard.update_session(&mut tui, info);
    assert_eq!(dashboard.windows.len(), 2);
    println!("[DEBUG] dashboard rendered restored process output and exit status");
}
