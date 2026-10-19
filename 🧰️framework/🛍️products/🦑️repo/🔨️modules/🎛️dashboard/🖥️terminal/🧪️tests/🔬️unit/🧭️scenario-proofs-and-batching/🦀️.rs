/// 🧭️ The scenario proofs of the window, session and pane features, the silent terminal, restoring, glyph capabilities, the armed-prefix notice and batched launcher navigation.

#[test]
fn every_scenario_of_the_window_view_features_is_proved_by_a_test() {
    let sources = [semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🔗️open-ready/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🧲️docking/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🎹️keymap-and-launcher/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🖼️window-focus-and-sessions/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🩺️runtime-audit/🦀️.rs"), semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🧭️scenario-proofs-and-batching/🦀️.rs")];
    dashboard_core::testkit::assert_proved(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🪟️windows/🥒️.feature"), &sources, &[
        ("Tabs and titles come from the task label", &["tabs_and_titles_come_from_the_task_label_with_status_as_a_glyph"]),
        ("Terminals follow the rectangle of their widget", &["terminal_sizes_follow_their_widget_rectangles_through_every_layout_change"]),
        ("Clicking a window moves the keyboard focus there", &["clicking_a_window_moves_the_keyboard_focus_there"]),
        ("Prefix actions split, zoom, cycle and close windows", &["prefix_actions_split_zoom_cycle_and_close_windows"]),
        ("Closing a window focuses its neighbour", &["p1_6_closing_a_window_focuses_its_neighbour_and_typing_q_never_quits"]),
        ("The right button opens a context menu whose rows act on the window", &["the_right_button_opens_a_context_menu_whose_rows_act_on_the_window"]),
        ("Window controls are named in the language of the dashboard", &["window_controls_are_named_in_the_language_of_the_dashboard_and_stay_quiet_when_unchanged"]),
        ("A compound opens one tab per member", &["a_compound_opens_one_tab_per_member_each_with_its_own_task_label"]),
        ("An unchanged screen is neither dirty nor repainted", &["p1_7_an_unchanged_screen_is_neither_dirty_nor_repainted_and_nothing_renders_in_full", "repainting_is_incremental_and_follows_typing"]),
        ("Every docking zone previews the committed geometry", &["docking_preview_matches_the_committed_split_geometry_in_every_zone"]),
        ("Docking preserves identities and collapses empty groups", &["docking_commands_preserve_windows_prune_empty_groups_and_reject_invalid_targets", "nested_splits_preserve_the_weight_of_their_existing_sibling_slot"]),
        ("Pointer docking preserves window bodies and focus", &["real_pointer_drags_preview_every_split_then_join_tabs_without_recreating_their_bodies", "docking_a_live_terminal_preserves_its_session_output_and_reports_the_new_pty_size"]),
        ("Interrupted docking leaves the layout unchanged", &["escape_focus_loss_resize_and_an_outside_release_cancel_docking", "docking_survives_zoom_and_removing_the_held_window_cancels_its_preview"]),
        ("Keyboard users can dock, join and reorder tabs", &["keyboard_docking_and_tab_reordering_have_localized_customizable_actions", "docking_changes_keyboard_focus_order_to_match_the_visible_layout"]),
        ("Tabs dock into every corner with stable identities", &["docking_tabs_reach_all_four_corners_with_preview_and_identity_owned_controls", "docking_corner_contract_covers_cross_group_and_sole_tab_moves", "docking_interleaved_layout_events_keep_the_held_tab_identity"]),
        ("Every tab owns its fullscreen and close controls", &["docking_fullscreen_and_close_on_an_inactive_tab_target_its_identity"]),
        ("Keyboard users can place tabs in every corner", &["keyboard_docking_places_tabs_in_all_corners_and_localizes_every_action"]),
        ("Divider dragging continues across layout frames", &["splitter_drag_keeps_its_capture_across_each_layout_remount"]),
    ]);
    dashboard_core::testkit::assert_proved(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/📡️sessions/🥒️.feature"), &sources, &[
        ("A selection before the connection is an owned, cancellable start", &["selection_before_connection_retains_an_owned_cancellable_start"]),
        ("The limit is checked before a window exists and a failed send is kept", &["p2_7_the_limit_is_checked_before_a_window_exists_and_a_failed_send_is_kept_for_the_reconnect"]),
        ("Reconnecting backs off and starts over after a success", &["p2_7_reconnecting_backs_off_and_starts_over_after_a_success"]),
        ("Restored output and exit status render without stealing the focus", &["restored_process_output_and_exit_status_render_without_stealing_focus"]),
        ("A replay clears only the terminal it replays", &["a_replay_clears_only_the_terminal_it_replays"]),
        ("Daemon errors reach the user in their language and never the terminal", &["daemon_errors_reach_the_user_in_their_language_and_never_the_terminal"]),
        ("Shutdown waits for the confirmation of the daemon", &["shutdown_waits_for_the_daemons_confirmation"]),
    ]);
    dashboard_core::testkit::assert_proved(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/📋️panes/🥒️.feature"), &sources, &[
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
    for (name, source) in [("sessions", semio_framework_test_fixtures::fixture_text!("../../📡️sessions/🦀️.rs")), ("controls", semio_framework_test_fixtures::fixture_text!("../../⌨️controls/🦀️.rs")), ("windows", semio_framework_test_fixtures::fixture_text!("../../🪟️windows/🦀️.rs")), ("panes", semio_framework_test_fixtures::fixture_text!("../../📋️panes/🦀️.rs"))] {
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
        semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::List(list)) => list.items.iter().position(|line| line.contains("[ Start ]")).unwrap(),
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
            semio_framework_ui_tui_frame::scene::NodeContent::Chrome(ChromeState::Footer(footer)) => &footer.status,
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
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
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
        let semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::Input(state)) = &tui.scene.node(input).content else { panic!("editor expected") };
        assert_eq!(state.value, row["expected"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(tui.focus(), Some(input));
    }
}

#[test]
fn launcher_pointer_focus_is_synchronized_before_the_next_character() {
    let vectors: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
    let metadata: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
    let facts: Facts = serde_json::from_value(metadata["workspace"].clone()).unwrap();
    for row in vectors["pointerCases"].as_array().unwrap() {
        let (mut dashboard, mut tui) = dashboard_with(Preferences { language:row["locale"].as_str().unwrap().into(), ..Default::default() });
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
