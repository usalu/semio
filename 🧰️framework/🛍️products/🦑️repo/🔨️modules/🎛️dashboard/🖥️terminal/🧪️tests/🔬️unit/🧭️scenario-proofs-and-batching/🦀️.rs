/// 🧭️ The scenario proofs of the window, session and pane features, the silent terminal, restoring, glyph capabilities, the armed-prefix notice and batched launcher navigation.


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
        NodeContent::Widget(WidgetState::List(list)) => list.items.iter().position(|line| line.contains("[ Start ]")).unwrap(),
        _ => panic!("configuration list expected"),
    };
    press(&mut dashboard, &mut tui, &[Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x + 2, y: rect.y + row as u16 }, mods: 0, clicks: 1 })]);
    let commands = spawned(&dashboard);
    assert_eq!(commands.len(), 1);
    assert!(commands[0].1.args.iter().any(|arg| arg == "docs"));
}

#[test]
fn glyph_capabilities_preserve_the_framework_width_policy() {
    use semio_framework_ui_tui_core::text::WidthMode;
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
            NodeContent::Chrome(ChromeState::Footer(footer)) => &footer.status,
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

#[test]
fn launcher_batched_navigation_edits_the_new_field_before_any_frame() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let (mut dashboard, mut tui) = view();
        press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
        press(&mut dashboard, &mut tui, &type_text("serve docs"));
        let index = dashboard.focused_index(&tui);
        for event in row["events"].as_array().unwrap() {
            let text = event.as_str().unwrap();
            let event = match text { "enter" => named(Key::Enter), "down" => named(Key::Down), _ => if let Some(value) = text.strip_prefix("char:") { key(value.chars().next().unwrap()) } else { Event::Paste(text.strip_prefix("paste:").unwrap().into()) } };
            dashboard.handle(&mut tui, &event);
        }
        let input = dashboard.windows[index].input.unwrap();
        let NodeContent::Widget(WidgetState::Input(state)) = &tui.scene.node(input).content else { panic!("editor expected") };
        assert_eq!(state.value, row["expected"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(tui.focus(), Some(input));
    }
}

#[test]
fn launcher_pointer_focus_is_synchronized_before_the_next_character() {
    let vectors: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
    let metadata: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
    let facts: Facts = serde_json::from_value(metadata["workspace"].clone()).unwrap();
    for row in vectors["pointerCases"].as_array().unwrap() {
        let (mut dashboard, mut tui) = dashboard_with(Preferences { language:Some(row["locale"].as_str().unwrap().into()), ..Default::default() });
        dashboard.set_registry(&mut tui, Arc::new(Registry::build(std::path::Path::new("workspace"), &facts)));
        press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
        press(&mut dashboard, &mut tui, &type_text("source-open"));
        press(&mut dashboard, &mut tui, &[named(Key::Enter)]);
        press(&mut dashboard, &mut tui, &type_text(row["initialParameter"].as_str().unwrap()));
        press(&mut dashboard, &mut tui, &[named(Key::Down)]);
        press(&mut dashboard, &mut tui, &type_text(row["initialArgs"].as_str().unwrap()));
        tui.render();
        let index = dashboard.focused_index(&tui);
        let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
        let clicked = launcher.screen().items.iter().position(|line| line.contains(row["expectedPlaceholder"].as_str().unwrap())).unwrap();
        let rectangle = tui.scene.rect(dashboard.windows[index].list);
        let click = pointer(MouseKind::Down(MouseButton::Left), Pos { x:rectangle.x+2, y:rectangle.y+clicked as u16 });
        dashboard.handle(&mut tui, &click);
        for event in type_text(row["typed"].as_str().unwrap()) { dashboard.handle(&mut tui, &event); }
        let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
        let screen = launcher.screen();
        assert_eq!(screen.placeholder, row["expectedPlaceholder"].as_str().unwrap());
        assert_eq!(screen.input, row["expectedText"].as_str().unwrap());
    }
}
