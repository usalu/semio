/// 🩺️ The runtime audit: selection and status per task row, incremental repainting, launcher filtering, hint fit, catalogue-only text, the armed prefix and reconnect backoff.

// #region 🔖️RuntimeAudit
#[test]
fn process_deadline_timeout_state_is_localized_in_task_rows() {
    for (language, expected) in [("en", "time limit exceeded"), ("de", "Zeitlimit überschritten")] {
        let (mut dashboard, mut tui) = small_view(language);
        dashboard.update_session(&mut tui, session("limited", label("run", "external", ""), SessionStatus::TimedOut, Some(124)));
        press(&mut dashboard, &mut tui, &[ctrl('b'), key('h')]);
        let index = dashboard.focused_index(&tui);
        assert!(dashboard.windows[index].pane.rows.iter().any(|(text, _)| text.contains(expected)), "{language}: {:?}", dashboard.windows[index].pane.rows);
    }
}

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
    let statuses = match &tui.scene.node(window.list).content { semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::List(list)) => list.statuses.clone(), _ => Vec::new() };
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
    let source = semio_framework_test_fixtures::fixture_text!("../../🦀️.rs").to_string() + semio_framework_test_fixtures::fixture_text!("../../⌨️controls/🦀️.rs") + semio_framework_test_fixtures::fixture_text!("../../📡️sessions/🦀️.rs") + semio_framework_test_fixtures::fixture_text!("../../🪟️windows/🦀️.rs") + semio_framework_test_fixtures::fixture_text!("../../📋️panes/🦀️.rs");
    assert!(!source.contains("render_full"), "the view never asks for a full repaint");
}

#[test]
fn p1_8_the_launcher_filter_pages_jumps_edits_and_previews() {
    let (mut dashboard, mut tui) = view();
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('n')]);
    let _ = tui.render();
    press(&mut dashboard, &mut tui, &type_text("quiz test"));
    let caption = |dashboard: &Dashboard, tui: &mut Tui| match &tui.scene.node(dashboard.windows[1].caption.unwrap()).content { semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::Label(label)) => label.text.clone(), _ => String::new() };
    let text = caption(&dashboard, &mut tui);
    assert!(text.contains("1 of") && text.contains("bun repo run @fx/quiz:test"), "match count and preview: {text}");
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
    for (name, source) in [("controls", semio_framework_test_fixtures::fixture_text!("../../⌨️controls/🦀️.rs")), ("sessions", semio_framework_test_fixtures::fixture_text!("../../📡️sessions/🦀️.rs")), ("panes", semio_framework_test_fixtures::fixture_text!("../../📋️panes/🦀️.rs")), ("windows", semio_framework_test_fixtures::fixture_text!("../../🪟️windows/🦀️.rs"))] {
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
