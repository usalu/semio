/// 🖼️ Windows, focus and sessions: tab titles, mouse operation, restored output, terminal sizes, shutdown, copy, repainting, context menus, compounds, daemon errors and replays.

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
        semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::List(list)) => list.items.iter().position(|line| line.contains("[ Start ]")).unwrap(),
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
    let size = match tui.scene.node_mut(dashboard.windows[1].list).widget() { Some(semio_framework_ui_tui_widgets::widget::WidgetState::Terminal(terminal)) => terminal.screen.size, _ => panic!("terminal expected") };
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
    if let Some(semio_framework_ui_tui_widgets::widget::WidgetState::Terminal(terminal)) = tui.scene.node_mut(dashboard.windows[1].list).widget() { terminal.select_all(); }
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('y')]);
    assert!(dashboard.effects.iter().any(|effect| matches!(effect, Effect::Copy(text) if text.contains("copy me"))), "{:?}", dashboard.effects);
}

#[test]
fn the_usage_text_is_generated_from_the_keymap_in_both_languages() {
    let english = super::help_text(&Preferences { language: Some("en".into()), ..Default::default() });
    assert!(english.contains("split right") && english.contains("Ctrl+B"));
    let german = super::help_text(&Preferences { language: Some("de".into()), ..Default::default() });
    assert!(german.contains("rechts teilen") && german.contains("Nach der Präfixtaste"));
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../⌨️usage/🧪️tests/🔣️.json")).unwrap();
    let expected = fixture["terminalHeader"].as_str().unwrap();
    assert_eq!(english.lines().next(), Some(expected));
    assert_eq!(german.lines().next(), Some(expected));
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
    let (dashboard, mut tui) = dashboard_with(Preferences { language: Some("de".into()), ..Default::default() });
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
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "no such task".into(), code: Some(dashboard_wire::ipc::ErrorCode::UnknownSession), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("that task no longer exists"));
    let (mut dashboard, mut tui) = dashboard_with(Preferences { language: Some("de".into()), ..Default::default() });
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "x".into(), code: Some(dashboard_wire::ipc::ErrorCode::NotRunning), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("Diese Aufgabe läuft nicht"));
}

#[test]
fn a_refused_view_says_so_in_its_language_and_retries_instead_of_showing_an_empty_daemon() {
    let (mut dashboard, mut tui) = view();
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "16 views".into(), code: Some(dashboard_wire::ipc::ErrorCode::ViewLimit), session_id: None }));
    assert_eq!(dashboard.notice.as_deref(), Some("the daemon serves as many views as it allows; close another view and try again"));
    assert!(dashboard.link.connection.is_none() && matches!(dashboard.state, LinkState::Failed(_)), "the refused connection is dropped so that the retry loop runs");
    let (mut dashboard, mut tui) = dashboard_with(Preferences { language: Some("de".into()), ..Default::default() });
    dashboard.apply(&mut tui, Message::Control(ServerMsg::Error { message: "16".into(), code: Some(dashboard_wire::ipc::ErrorCode::ViewLimit), session_id: None }));
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
    let text = |dashboard: &Dashboard, tui: &mut Tui, index: usize| match tui.scene.node_mut(dashboard.windows[index].list).widget() { Some(semio_framework_ui_tui_widgets::widget::WidgetState::Terminal(terminal)) => { terminal.select_all(); terminal.selected_text().unwrap_or_default() } _ => String::new() };
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
    assert_eq!(tui.theme.glyphs, semio_framework_ui_tui_core::theme::GlyphSet::Ascii);
    assert_eq!(Status::Success.glyph(tui.theme.glyphs, 0), "+");
    assert_eq!(tui.width_mode(), semio_framework_ui_tui_core::text::WidthMode::Scalar);
}

#[test]
fn the_search_action_starts_a_search_in_the_focused_terminal() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("a", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.frame(&mut tui);
    press(&mut dashboard, &mut tui, &[ctrl('b'), key('/')]);
    let searching = match tui.scene.node_mut(dashboard.windows[1].list).widget() { Some(semio_framework_ui_tui_widgets::widget::WidgetState::Terminal(terminal)) => terminal.search().is_some(), _ => false };
    assert!(searching);
    assert!(!dashboard.link.outbox.iter().any(|message| matches!(message, ClientMsg::Input { .. })), "the search consumes the keys");
}
