/// 🪟️ Window option laws: grid, level of detail and vortex options per window instance, the settings steppers, the camera actions and the worker session admission.

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app().await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app).await);
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut reopened = app().await;
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let base_view = app.window_view(main::WINDOW_KIND_ID);
    let second_view = app.window_view(second_window);
    let document_before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before exact window publications");

    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 2.5 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing on base window");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_after = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");
    let base_render = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let second_render = render_window(&mut app, second_window).await;
    assert_eq!(lod_of(&base_render).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&base_render).get("gridFactor").and_then(Value::as_f64), Some(2.5));
    assert_eq!(lod_of(&second_render).get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(projection_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after exact window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    assert_eq!(app.window_config_generation(&base_view).await.expect("base generation"), Some(1));
    assert_eq!(app.window_config_generation(&second_view).await.expect("second generation"), Some(1));
    let packs = app.window_config_packs().await.expect("exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, main::WINDOW_KIND_ID).await, base_render);
    assert_eq!(render_window(&mut reopened, second_window).await, second_render);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
}

/// 📏 Wave B12: grid / spacing / LOD / vortex-show must publish into Puzzle3dWindowConfig and the
/// measures rail must read the new values. A silent `.ok().into_iter()` drop would leave generation
/// unchanged and the rail bound to the previous snapshot.
#[semio_framework_async_macros::async_test]
async fn window_option_rail_round_trips_grid_lod_and_vortex_into_published_config() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 25.0 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing");
    dispatch(&mut app, "setLodAutomatic", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setLodAutomatic");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow");
    assert!(!app.has_pending_typed_operations(), "window-option gestures must quiesce");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 4, "each window-option change must publish a WindowConfig generation, never drop");
    let measures = app.window_measures(&view).await;
    let rail = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible")), Some(false));
    assert_eq!(find_measure_slider(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(25.0));
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto")), Some(false));
    assert_eq!(find_measure_select(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
}

/// 🪟️ The whole per-window option round trip, on the two instances the default layout actually opens
/// (`puzzle3d-main-top` / `puzzle3d-main-perspective`), driven exactly as the measures rail drives it:
/// the rail's own `WindowMeasure::Toggle` args (`{"pressed": …}`) tagged with the instance the rail
/// belongs to. One toggle must (a) advance THAT instance's window-config generation by exactly one,
/// (b) come back through the same instance's measures rail as the new value, (c) reach the world body
/// the instance renders, and (d) leave the sibling instance's rail, body and generation untouched.
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12: the browser symptom was a checkbox that never
/// moved. Every hop below is green, which is what pinned the defect to the two host-side halves — the
/// control rendering the published value with no state of its own while the round trip is in flight,
/// and `Puzzle3dScopeClass::Chrome` making that round trip a full-shell repaint.
#[semio_framework_async_macros::async_test]
async fn window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone() {
    let mut app = app().await;
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let top = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("register the top pane's own config lane");
    let top_generation_before = app.window_config_generation(&top).await.expect("top generation");

    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setGridVisible on the perspective pane");

    let perspective_generation = app.window_config_generation(&perspective).await.expect("perspective generation");
    assert_eq!(perspective_generation, Some(1), "one rail toggle publishes exactly one generation on the instance it was dispatched from");
    assert_eq!(app.window_config_generation(&top).await.expect("top generation"), top_generation_before, "the sibling instance's own config lane must not advance");

    let view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let measures = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective measures"), &toggle_id), Some(false), "the rail re-reads the value its own toggle just published");
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_TOP).expect("top measures"), &toggle_id), Some(true), "the sibling rail keeps the value IT published");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await).get("showLodGrid").and_then(Value::as_bool), Some(false), "the world body the instance renders consumes the published option");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_TOP).await).get("showLodGrid").and_then(Value::as_bool), Some(true), "the sibling's world body keeps its own grid");
}

/// ⚙️ The Settings panel's four steppers are addressable, carry the value they configure, and declare
/// the trigger the control they are built from actually fires.
///
/// 🔢️ `uniform` is the load-bearing half: a `NumberStepper` declared `uniform: false` renders MIXED —
/// a blank box with a placeholder instead of the value, and a +/− bump computed from the widget's own
/// `defaultValue` (0) rather than from the setting. These are one window's own scalars, never a
/// multi-selection aggregate, so the honest declaration is `true` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B12, browser-measured: all four boxes rendered empty).
#[semio_framework_async_macros::async_test]
async fn settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on() {
    fn stepper(node: &Value, id: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(id) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| stepper(child, id)),
            Value::Array(items) => items.iter().find_map(|item| stepper(item, id)),
            _ => None,
        }
    }
    let mut app = app().await;
    let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, settings_panel::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the settings panel renders parseable ui json");
    for (field, action) in [
        ("contact-tolerance", "setBrushPlacementContactTolerance"),
        ("proximity-radius", "setProximityRadius"),
        ("chunk-size", "setChunkSize"),
        ("grid-spacing", "setGridSpacing"),
    ] {
        let row = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}")).unwrap_or_else(|| panic!("the settings panel must declare a {field} field row: {panel}"));
        let control = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} row must own the stepper control it labels: {row}"));
        let props = control.get("component").cloned().unwrap_or_else(|| panic!("{field} control is a number stepper: {control}"));
        assert_eq!(props.get("type").and_then(Value::as_str), Some("numberStepper"), "{field} control is a number stepper: {control}");
        assert_eq!(props.get("uniform").and_then(Value::as_bool), Some(true), "{field} configures ONE window's own scalar, so it can never render as a mixed selection: {props}");
        assert!(props.get("value").and_then(Value::as_f64).is_some(), "{field} must carry the value it configures: {props}");
        let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
        assert_eq!(bindings.len(), 1, "{field} declares exactly the one trigger it handles: {bindings:?}");
        assert_eq!(bindings[0].get("trigger").and_then(Value::as_str), Some("change"), "{field} handles absolute values, so it binds Change and NOT Delta: {bindings:?}");
        assert_eq!(bindings[0].pointer("/action/name").and_then(Value::as_str), Some(action), "{field} dispatches {action}: {bindings:?}");
        assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(main::WINDOW_KIND_ID), "{field} is scoped at the window the panel was rendered for, so the value it publishes reaches THAT window's rail: {bindings:?}");
    }
}

/// 🐢️ Every control the measures rail and the Settings panel bind to a per-window option resolves to
/// [`Puzzle3dScopeClass::WindowOption`], and that class paints exactly the three surfaces those options
/// can move: the world body that re-derives from them, the measures rail that renders them, and the
/// Settings panel that mirrors four of the same fields. Never the outliner, catalogue, inspector,
/// history, labels, utilities or tool measures — and never `Full`, which is what these verbs used to
/// fall through to (one grid toggle re-rendered the whole shell before its own checkbox could move).
#[semio_framework_async_macros::async_test]
async fn window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass};
    for action in [
        "setGridVisible",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setLodManual",
        "setVortexShow",
        "setVortexDirection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setSelectableKind",
        "setTransformGumballFlag",
        "setProximityRadius",
        "setChunkSize",
        "setVoxelDims",
        "setProjection",
        "setProjectionParam",
    ] {
        assert_eq!(puzzle3d_command_scope_class(action), Puzzle3dScopeClass::WindowOption, "{action} is a per-window option and must be classified as one");
    }
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_scope(Puzzle3dScopeClass::WindowOption) else {
        panic!("the window-option class is a narrowed scope, never Full");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert_eq!(panel_bodies, vec![settings_panel::BODY_KEY.to_string()], "the Settings panel mirrors four window options and must never disagree with the rail");
    assert!(measures, "the rail's own controls are bound to the value this publishes");
    assert!(!utilities && !tools && !engagements && !labels, "a window option moves no other chrome");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(!panel_bodies.iter().any(|named| named == body), "a window option cannot move the {body} panel");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_transient_is_exact_instance_local_and_resets_on_reload() {
    let mut reopened = app().await;
    let mut app = app().await;
    let window_a = "puzzle3d-transient-a";
    let window_b = "puzzle3d-transient-b";
    let view_a = app.window_view(window_a);
    let view_b = app.window_view(window_b);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 10.0, "y": 20.0 })), Some(window_a)).await.expect("open suggestions in window a");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle3dWindowTransientOwner>().and_then(|value| value.suggestion_menu.as_ref()).map(|menu| menu.window_id.as_str()), Some(window_a));
    assert!(transient_b.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    dispatch(&mut app, "closeVortexSuggestions", None, Some(window_a)).await.expect("close suggestions in window a");
    let closed = app.window_transient_snapshot(&view_a).expect("closed transient").expect("closed owner");
    assert!(closed.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    let vortex_again = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex_again })), Some(window_a)).await.expect("open suggestions in window a again");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert!(reset.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(3));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    // 🧷️ Every `window_transient_snapshot` above is a live READ of the partition store, and close is
    // blocked — by design — while one is outstanding. The reads have to be surrendered before the
    // close witness, exactly as a host surrenders a rendered body before retiring its app.
    drop((transient_a, transient_b, closed));
    drop(reset);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
}

/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S. A mounted worker job session is admitted out of a
/// FIXED process-wide array of `semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS` retirement slots, and an
/// app dropped while an operation is still mounted parks its node there instead of freeing it. ONLY a
/// live app's cooperative-maintenance round robin hands that slot back. Without that pump the array
/// filled up for the life of the process: from then on EVERY typed operation had its worker session
/// refused, and a refused session never runs the job, so the operation reached publication with no
/// completion value and the host's continuation loop spun on it forever — which is exactly how one
/// abandoned command turned a whole test binary into `pending typed operations outlived the settle
/// budget` for every later command, including plain view actions.
#[semio_framework_async_macros::async_test]
async fn an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app() {
    let mut abandoned = app().await;
    dispatch_unsettled(&mut abandoned, "setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("setCamera mounts one typed operation");
    drop(abandoned);
    let mut live = app().await;
    for _ in 0..RETIREMENT_RECLAIM_TURNS {
        if !semio_framework_job_worker::worker_job_retirements_are_parked() {
            break;
        }
        live.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("one cooperative maintenance unit");
    }
    assert!(
        !semio_framework_job_worker::worker_job_retirements_are_parked(),
        "a live app's maintenance round robin must reclaim every parked worker-job retirement slot, or the process runs out of session admissions"
    );
    dispatch(&mut live, "setCamera", Some(&json!({ "position": [4.0, 5.0, 6.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("the next app still quiesces on a plain view action");
}

/// 🔁️ Maintenance units the reclaim law grants: every stage of the round robin, many times over, so the
/// law fails on a missing pump rather than on a tight budget.
const RETIREMENT_RECLAIM_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 64;

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = dispatchable_actions(&app_definition).into_iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app().await;
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🎥️ Wave B11 (checklist §22, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14): `f` with
/// nothing selected completed with a bare `Emit::default()` — the camera never moved, no notice was
/// raised, and the key read as dead on every boot before the user had picked anything. A camera verb
/// with no selection has an obvious subject (the whole document), so focus must FRAME it — while still
/// writing only the per-window camera, never a document operation.
#[semio_framework_async_macros::async_test]
async fn focus_selection_frames_the_whole_document_when_nothing_is_selected() {
    let mut app = app().await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [900.0, 900.0, 900.0], "target": [900.0, 900.0, 900.0], "zoom": 1.0 } })), Some(main::WINDOW_KIND_ID)).await.expect("park the camera far away");
    let parked = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    let document_before = projection_of(&app);
    let result = dispatch(&mut app, "focusSelection", None, Some(main::WINDOW_KIND_ID)).await.expect("focusSelection with an empty selection must complete");
    let framed = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_ne!(framed, parked, "focus with nothing selected must frame the document instead of leaving the camera parked");
    assert!(result.mutations.is_empty(), "focus is camera-only — it must emit no artifact history entry: {:?}", result.mutations);
    assert_eq!(projection_of(&app), document_before, "focus must not touch the document");
    let objects = document_before.get("objects").and_then(Value::as_array).expect("the example carries objects");
    let first = objects.first().and_then(|object| object.get("origin")).cloned().expect("an object origin");
    assert_eq!(framed.pointer("/target/0").and_then(Value::as_f64), first.pointer("/0").and_then(Value::as_f64), "a one-object document frames that object");
}

/// 🪟️ ONE per-window setting is ONE window-config publication that QUIESCES. The lane's own bounded
/// preparation used to demand the owner's whole declared publication ceiling
/// (`Puzzle3dWindowConfigOwner::MAXIMUM_PUBLICATION_BYTES`, 65 536) out of the publisher's per-turn
/// byte grant (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 096) and answered `Blocked` on every single
/// turn, so `setCamera` — and every projection/grid/vortex/window option behind the same lane — sat
/// in stage `Publishing` forever and nothing queued behind it could ever run. The law is exactly:
/// one mutation, one generation, no pending operation left over.
#[semio_framework_async_macros::async_test]
async fn one_window_config_mutation_publishes_exactly_one_generation_and_quiesces() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [7.0, 8.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), None).await.expect("setCamera");
    assert!(!app.has_pending_typed_operations(), "one window-config gesture must leave no typed operation pending");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 1, "one window-config mutation publishes exactly one store generation, never zero and never a spin");
    assert_eq!(camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).pointer("/position/0").and_then(Value::as_f64), Some(7.0), "the published window config is what the window renders");
}

/// 🛰️📷️ Wave B50: `setCamera` must move the pose the REFRESH route publishes, not only the one the
/// `<body>:<windowInstanceId>` surface route does. `plugin_refresh_ui` renders a window section as
/// `app.render(body_key, None, view.for_window_instance(key))` (`🔌️plugin/🦀️.rs:33790`) — the
/// instance reaches the guest through the VIEW alone, with no instance-suffixed body key — and the
/// section hash it answers is what `uiRefreshSectionUnchanged` compares. Every pre-existing camera
/// law measured [`render_window`] (the surface route), so a refresh route that publishes the opening
/// pose forever read green while the browser froze: wasm #59's battery reported
/// `camera settle puzzle3d-main-perspective moved=false waitedMs=30184` for orbit, pan AND zoom with
/// `refreshUi sections asked=["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"]
/// changed=[] hashes={…"puzzle3d-main-perspective":"aba169d7:1"}` — the same hash after every
/// `setCamera`, on a `full` scope included. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B50.
#[semio_framework_async_macros::async_test]
async fn set_camera_moves_the_pose_the_window_refresh_route_publishes() {
    let mut app = app().await;
    let window = main::WINDOW_INSTANCE_PERSPECTIVE;
    let before = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window)).await.expect("setCamera on the perspective instance");
    let after = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    assert_ne!(after, before, "the refresh route must publish the gestured pose, not the opening one: before={before} after={after}");
    assert_eq!(after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "the refresh route's published camera is the one `setCamera` wrote: {after}");
    let sibling = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_TOP).await);
    assert_ne!(sibling.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "one instance's gesture must not move its sibling's refresh pose: {sibling}");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app().await;
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b).await);

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a).await);
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b).await), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_always = app.window_measures(&view).await;
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app).await).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app).await);
    assert!(!outwards_vortices.is_empty(), "scene_snapshot must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_inwards = app.window_measures(&view).await;
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app).await).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window).await);
    assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}
//#endregion 🔖️WindowOptions
