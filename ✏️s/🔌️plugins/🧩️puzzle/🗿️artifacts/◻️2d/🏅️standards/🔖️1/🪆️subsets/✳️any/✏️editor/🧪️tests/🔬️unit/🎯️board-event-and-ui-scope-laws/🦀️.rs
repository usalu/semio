// 🎯️ Board event and ui scope laws: the camera and window transients, the board event commits and the ui dirty scope each action declares.

//#region 🔖️BoardEvents
/// 🎥️ `setCamera` is session-only view state: a camera drag never creates a VCS edit, so there is
/// nothing to coalesce and nothing for `undo` to revert.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_session_only_and_never_undoable() {
    let mut app = app();
    for x in [1.0, 2.0, 3.0] {
        let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": x, "y": 0.0, "zoom": 1.0 } })), None).expect("camera");
        assert_eq!(committed_edits(&result), 0, "setCamera must never produce a document operation");
    }
    let rendered = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered).0, 3.0, "the camera must update immediately in the rendered scene");
    let undo = dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(committed_edits(&undo), 0, "there is no document edit to undo");
    let rendered_after_undo = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered_after_undo).0, 3.0, "the camera is session state — undo must not revert it");
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn exact_overview_window_cameras_isolate_render_and_reload_through_registered_app() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle2d-overview-a";
    let window_b = "puzzle2d-overview-b";
    let view_a = window_view(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, window_a);
    let view_b = window_view(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, window_b);
    let document_before = fixture_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before window publications");
    let result_a = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 18.0, "y": -4.0, "zoom": 2.0 } })), Some(window_a)).expect("setCamera a");
    let result_b = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": -9.0, "y": 6.0, "zoom": 0.5 } })), Some(window_b)).expect("setCamera b");
    assert_eq!((committed_edits(&result_a), committed_edits(&result_b)), (0, 0));
    assert_eq!(fixture_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    let rendered_a = render_window(&mut app, overview::BODY_KEY, window_a);
    let rendered_b = render_window(&mut app, overview::BODY_KEY, window_b);
    assert_eq!(rendered_camera(&rendered_a), (18.0, -4.0, 2.0));
    assert_eq!(rendered_camera(&rendered_b), (-9.0, 6.0, 0.5));
    assert_eq!(app.window_config_generation(&view_a).await.expect("window a generation"), Some(1));
    assert_eq!(app.window_config_generation(&view_b).await.expect("window b generation"), Some(1));
    let packs = app.window_config_packs().await.expect("two exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact Puzzle 2D window pack");
    }
    assert_eq!(rendered_camera(&render_window(&mut reopened, overview::BODY_KEY, window_a)), (18.0, -4.0, 2.0));
    assert_eq!(rendered_camera(&render_window(&mut reopened, overview::BODY_KEY, window_b)), (-9.0, 6.0, 0.5));
    close_app(&mut reopened);
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn exact_overview_window_transient_isolates_abort_and_resets_on_reload() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle2d-transient-a";
    let window_b = "puzzle2d-transient-b";
    let view_a = window_view(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, window_a);
    let view_b = window_view(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, window_b);
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "fill" })), Some(window_a)).expect("window a engagement input");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some("fill"));
    assert_eq!(transient_b.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementAbort", Some(&json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID })), Some(window_a)).expect("abort window a engagement");
    let aborted = app.window_transient_snapshot(&view_a).expect("aborted transient").expect("aborted owner");
    assert_eq!(aborted.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("window a second engagement input");
    let submitted = dispatch(&mut app, "engagementSubmit", Some(&json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("submit window a engagement");
    assert!(submitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, .. } if window_id == window_a)));
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "draft" })), Some(window_a)).expect("window a third engagement input");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert_eq!(reset.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(5));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    drop((transient_a, transient_b, aborted, reset));
    close_app(&mut reopened);
    close_app(&mut app);
}

/// 🐢️ Regression test for a perf-round-2 bug: `parse_fixture_v1` always `clear_scene()`s then
/// rebuilds, so every edge looked "new" and got re-`push_event`'d as `edgeCreate` — which
/// `apply_host_events` then replayed into the snapshot on the *next* action, duplicating every edge
/// once per action forever.
#[semio_framework_async_macros::async_test]
async fn repeated_actions_do_not_duplicate_edges() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let edge_count = |app: &Puzzle2dApp| board_snapshot_edges(&fixture_of(app)).len();
    let before = edge_count(&app);
    assert!(before > 0, "snapshot must have edges for this regression test to be meaningful");
    let node_id = first_node_id(&app);
    for _ in 0..5 {
        dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    }
    assert_eq!(edge_count(&app), before, "selecting repeatedly must not grow the edges array");
    close_app(&mut app);
}

/// 🪞️ Regression test: `applyBoardEvents`'s `select` case only mutated the runtime, never the
/// host, so `apply_host_events`'s `host.selection`-is-truth re-sync silently reverted the
/// selection to whatever the host held before the action (empty, on a fresh sync).
#[semio_framework_async_macros::async_test]
async fn apply_board_events_select_persists_across_the_next_action() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must be visible immediately after the select action");
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
    assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must survive a subsequent unrelated action");
    close_app(&mut app);
}

/// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
/// *pre-action* value) against the runtime and blindly overwrite it, reverting a camera move (the view verb
/// `setCamera`, which a board pan, pinch and wheel all dispatch) before it ever committed.
#[semio_framework_async_macros::async_test]
async fn a_board_camera_move_commits_through_the_view_verb() {
    let mut app = app();
    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 5.0, "y": 6.0, "zoom": 1.2 } })), None).expect("camera move");
    assert_eq!(committed_edits(&result), 0, "a camera move must never produce a document operation");
    let (x, y, zoom) = rendered_camera(&render_body(&mut app, overview::BODY_KEY));
    assert_eq!(x, 5.0);
    assert_eq!(y, 6.0);
    assert_eq!(zoom, 1.2);
    close_app(&mut app);
}

/// 🐢️ A pure selection change is runtime state, not document state — it must not produce any
/// operations (previously it fell back to a whole-document replace once the edge-duplication bug
/// made `before` and `after` genuinely diverge).
#[semio_framework_async_macros::async_test]
async fn select_action_emits_no_operations() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    assert_eq!(committed_edits(&result), 0, "selection must not produce document operations");
    close_app(&mut app);
}
//#endregion 🔖️BoardEvents

//#region 🔖️UiScope
/// 🐢️ Perf round 3: a select event must declare a narrow `Partial` ui_scope (the 3 canvas panes +
/// layers/properties panels + engagements) — never `Full`, or the shell's batched `refresh-ui`
/// call degrades back to fetching everything on every select.
#[semio_framework_async_macros::async_test]
async fn select_action_declares_partial_ui_scope() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![overview::BODY_KEY, detail::BODY_KEY, crate::standards::v1::subsets::any::schema::mutations::selection::BODY_KEY], "window_bodies must be body keys, not pane ids");
            assert!(panel_bodies.contains(&crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
            assert!(panel_bodies.contains(&crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
            assert!(engagements, "select must refresh the engagement bar");
            assert!(!measures, "select must not force a measures refresh");
            assert!(!utilities);
            assert!(!tools);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for select, got {other:?}"),
    }
    close_app(&mut app);
}

/// 🐢️ Perf round 3: a camera-only board event touches only the 3 canvas panes — no panels,
/// engagements, measures, or utilities.
#[semio_framework_async_macros::async_test]
async fn a_camera_move_declares_window_only_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 1.0, "y": 2.0, "zoom": 1.0 } })), None).expect("camera move");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies.len(), 3);
            assert!(panel_bodies.is_empty(), "a config-only camera event does not dirty command history");
            assert!(!engagements && !measures && !utilities && !tools && !labels);
        }
        other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
    }
    close_app(&mut app);
}

/// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
/// history panel body — the empty View action neither logs an edit nor dirties a surface.
#[semio_framework_async_macros::async_test]
async fn empty_board_events_declare_none_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
    assert_eq!(result.ui_scope, UiDirtyScope::None);
    close_app(&mut app);
}

/// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
/// default — no puzzle2d scope helper narrows them.
#[semio_framework_async_macros::async_test]
async fn add_node_action_declares_full_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
    close_app(&mut app);
}
//#endregion 🔖️UiScope
