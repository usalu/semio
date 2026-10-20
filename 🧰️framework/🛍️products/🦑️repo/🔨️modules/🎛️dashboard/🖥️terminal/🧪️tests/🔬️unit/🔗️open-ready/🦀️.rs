/// 🔗️ Opening a ready task's URL from the keyboard, the overview and the context menu with shared vectors, and its localized, deduplicated and cancellable customization.

#[test]
fn open_ready_routes_keyboard_overview_and_context_menu_using_shared_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧫️fixtures/🌐️open/🔣️.json")).unwrap();
    for row in vectors["tasks"].as_array().unwrap() {
        let (mut dashboard, mut tui) = view();
        let status = serde_json::from_value(row["status"].clone()).unwrap();
        let mut task = session("ready-task", label("dev", "quiz", ""), status, None);
        task.ready_url = row["url"].as_str().map(str::to_string);
        dashboard.update_session(&mut tui, task);
        dashboard.focus_window(&mut tui, 1);
        press(&mut dashboard, &mut tui, &[ctrl('b'), key('u')]);
        let expected = if row["accepted"].as_bool().unwrap() { vec![Effect::OpenReady(row["url"].as_str().unwrap().into())] } else { vec![] };
        assert_eq!(dashboard.effects, expected, "keyboard: {row}");
        dashboard.effects.clear();
        dashboard.focus_window(&mut tui, 0);
        dashboard.refresh_panes(&mut tui);
        let ready_row = dashboard.windows[0].pane.rows.iter().position(|(_, action)| matches!(action, super::panes::RowAction::OpenReady(_)));
        assert_eq!(ready_row.is_some(), row["accepted"].as_bool().unwrap());
        if let Some(selected) = ready_row { dashboard.windows[0].pane.selected = selected; dashboard.activate_row(&mut tui, 0); assert_eq!(dashboard.effects, expected, "overview: {row}"); }
        dashboard.effects.clear();
        dashboard.focus_window(&mut tui, 1);
        dashboard.frame(&mut tui);
        let rect = tui.scene.rect(dashboard.windows[1].list);
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Right), Pos { x: rect.x + 2, y: rect.y + 2 })]);
        assert_eq!(dashboard.menu.as_ref().unwrap().actions.contains(&Some(Action::OpenReady)), row["accepted"].as_bool().unwrap());
        if !expected.is_empty() { press(&mut dashboard, &mut tui, &[named(Key::Down), named(Key::Down), named(Key::Enter)]); assert_eq!(dashboard.effects, expected, "context menu: {row}"); }
    }
}

#[test]
fn open_ready_customization_is_localized_deduplicated_and_cancellable() {
    let mut preferences = Preferences { language: Some("de".into()), ..Default::default() };
    preferences.apply(&Change::parse(r#"{"prefix":"ctrl+a","bindings":{"prefix.open-ready":["g"],"prefix.cancel-open-ready":["G"]}}"#).unwrap());
    let (mut dashboard, mut tui) = dashboard_with(preferences);
    let mut task = session("ready-task", label("dev", "quiz", ""), SessionStatus::Running, None);
    task.ready_url = Some("http://127.0.0.1:6061/".into());
    dashboard.update_session(&mut tui, task);
    dashboard.focus_window(&mut tui, 1);
    assert_eq!(dashboard.text().action("open-ready"), "Bereite Adresse öffnen");
    assert!(dashboard.footer_hints(&tui).iter().any(|hint| hint.key == "Ctrl+A g" && hint.label == "Bereite Adresse öffnen"));
    press(&mut dashboard, &mut tui, &[ctrl('a'), key('g'), ctrl('a'), key('g')]);
    assert_eq!(dashboard.effects, [Effect::OpenReady("http://127.0.0.1:6061/".into())]);
    assert!(dashboard.notice.as_ref().unwrap().contains("Ctrl+A G"));
    press(&mut dashboard, &mut tui, &[ctrl('a'), key('G')]);
    assert!(dashboard.effects.is_empty());
    assert_eq!(dashboard.sessions["ready-task"].status, SessionStatus::Running);
    assert_eq!(dashboard.notice.as_deref(), Some("Öffnen abgebrochen; Aufgabe läuft weiter"));
}
