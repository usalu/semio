
use super::*;

fn action(name: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: semio_framework::optional_json_to_dsl(args) }
}

#[test]
fn window_action_context_retained_commands_preserve_the_clicked_window() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️tests/🔬️window-action-context/🔣️.json")).unwrap();
    let window_id = fixture["clickedWindowId"].as_str().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let descriptor = action("setValue", (!case["args"].is_null()).then(|| case["args"].clone()));
        apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::App { window_id: window_id.into(), action: descriptor }], &mut input);
        let queued = crate::collect_fixture_actions(&mut input);
        assert_eq!(queued.len(), 1);
        let actual: Option<Value> = queued[0].args.as_ref().map(|args| serde_json::from_str(&dsl::json::from_dsl_value(args).to_string()).unwrap());
        assert_eq!(actual, Some(case["expected"].clone()));
    }
    eprintln!("[DEBUG] retained native actions preserved clicked window identity and replaced conflicting descriptor targets");
}

fn stack_with(id: &str, drop_action: Option<ActionDescriptor>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(ui_wgpu::wgpu::UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children, menu: None })
}

//#region 🔖️DropCommittedTests
#[test]
fn decode_drop_payload_extracts_the_first_semio_mime_entry_as_a_json_object() {
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
    let decoded = decode_drop_payload(&payload).expect("a well-formed semio-mime JSON object payload should decode");
    assert_eq!(decoded.get("id").and_then(Value::as_str), Some("abc"));
}

#[test]
fn decode_drop_payload_ignores_non_semio_mimes_and_rejects_non_object_or_blank_json() {
    let mut no_semio_mime = DragPayload::new();
    no_semio_mime.insert("text/plain".into(), "\"abc\"".into());
    assert!(decode_drop_payload(&no_semio_mime).is_none(), "no application/x-semio-* entry present");

    let mut non_object = DragPayload::new();
    non_object.insert("application/x-semio-catalogue-item".into(), "\"not-an-object\"".into());
    assert!(decode_drop_payload(&non_object).is_none(), "a non-object JSON payload must not decode");

    let mut blank = DragPayload::new();
    blank.insert("application/x-semio-catalogue-item".into(), "   ".into());
    assert!(decode_drop_payload(&blank).is_none(), "a blank payload value must not decode");
}

#[test]
fn merge_action_args_lets_the_patch_win_over_existing_args() {
    let existing = serde_json::json!({"id": "abc", "kept": true});
    let existing_dsl = existing.into();
    let mut patch = serde_json::Map::new();
    patch.insert("id".to_string(), Value::from("overridden"));
    patch.insert("targetId".to_string(), Value::from("t1"));

    let merged: Value = serde_json::from_str(&dsl::json::from_dsl_value(&merge_action_args(Some(&existing_dsl), patch).expect("merged args")).to_string()).expect("json args");

    assert_eq!(merged.get("id").and_then(Value::as_str), Some("overridden"));
    assert_eq!(merged.get("kept").and_then(Value::as_bool), Some(true));
    assert_eq!(merged.get("targetId").and_then(Value::as_str), Some("t1"));
}

#[test]
fn drop_target_action_reads_a_stacks_drop_action_from_the_retained_tree() {
    let window_id = "apply-ui-commands-drop-target-test";
    let expected = action("onDrop", None);
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(expected.clone()), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());

    assert_eq!(drop_target_action(window_id, target), Some(expected));
}

#[test]
fn drop_target_action_is_none_for_a_stack_without_a_drop_action() {
    let window_id = "apply-ui-commands-drop-target-none-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", None, vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());

    assert!(drop_target_action(window_id, target).is_none());
}

#[test]
fn apply_drop_committed_queues_the_merged_action_into_input() {
    let window_id = "apply-ui-commands-drop-committed-test";
    let drop_action = action("onDrop", Some(serde_json::json!({"kept": true})));
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(drop_action), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\",\"windowId\":\"unrelated-window\"}".into());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &payload, &mut input).expect("fixture drop admission succeeds");

    let queued = crate::collect_fixture_actions(&mut input);
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0].controller_id, "ctrl");
    assert_eq!(queued[0].action, "onDrop");
    let args = queued[0].args.as_ref().expect("merged args");
    assert_eq!(args.get("id").and_then(semio_framework::DslValue::as_str), Some("abc"), "the decoded payload should flow through");
    assert_eq!(args.get("kept").and_then(semio_framework::DslValue::as_bool), Some(true), "the drop_action's own existing args should survive");
    assert_eq!(args.get("windowId").and_then(semio_framework::DslValue::as_str), Some(window_id), "the drop payload cannot replace its host-owned window target");
}

#[test]
fn apply_drop_committed_is_a_no_op_without_a_decodable_semio_payload() {
    let window_id = "apply-ui-commands-drop-committed-no-payload-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("dz", Some(action("onDrop", None)), vec![])));
    let target = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_drop_committed(window_id, target, &DragPayload::new(), &mut input).expect("fixture drop admission succeeds");

    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️DropCommittedTests

//#region 🔖️ClipboardTests
#[test]
fn clipboard_copy_and_cut_commands_write_through_the_mocked_os_clipboard() {
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().clear());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardCopy { window_id: "w".into(), text: "hello".into() }, ui_wgpu::wgpu::UiCommand::ClipboardCut { window_id: "w".into(), text: "world".into() }], &mut input);

    assert_eq!(MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow().clone()), vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn clipboard_paste_requested_reads_the_mocked_clipboard_and_inserts_it_at_the_focused_caret() {
    let window_id = "apply-ui-commands-clipboard-paste-test";
    let input_node = UiNode::Input(ui_wgpu::wgpu::UiInputNode {
        id: "name".into(),
        input_kind: "text".into(),
        value: String::new(),
        placeholder: None,
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: action("onChange", None),
        presence: UiPresence::default(),
        menu: None,
    });
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![input_node])));
    let focus_commands = UI_ENGINE.with(|cell| cell.borrow_mut().dispatch_event(window_id, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }));
    let focused = focus_commands
        .iter()
        .find_map(|cmd| match cmd {
            ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(id), .. } => Some(*id),
            _ => None,
        })
        .expect("Tab should focus the only focusable node (the Input)");
    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = Some("pasted".to_string()));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_clipboard_paste_requested(window_id, &mut input);

    let text = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().node(focused).unwrap().state.edit.clone().unwrap().text);
    assert_eq!(text, "pasted");
}

#[test]
fn clipboard_paste_requested_is_a_no_op_when_the_mocked_clipboard_is_empty() {
    let window_id = "apply-ui-commands-clipboard-paste-empty-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![])));
    MOCK_CLIPBOARD_READ.with(|cell| *cell.borrow_mut() = None);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_clipboard_paste_requested(window_id, &mut input);

    assert!(crate::collect_fixture_actions(&mut input).is_empty());
}
//#endregion 🔖️ClipboardTests

//#region 🔖️NoOpCommandTests
#[test]
fn drop_cancelled_overlay_closed_and_focus_changed_commands_are_explicit_no_ops() {
    let window_id = "apply-ui-commands-noop-test";
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![])));
    let node = UI_ENGINE.with(|cell| cell.borrow().tree(window_id).unwrap().root.unwrap());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[
            ui_wgpu::wgpu::UiCommand::DropCancelled { window_id: window_id.into(), source: node },
            ui_wgpu::wgpu::UiCommand::OverlayClosed { window_id: window_id.into(), root: node, kind: ui_wgpu::wgpu::OverlayKind::Tooltip },
            ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.into(), node: Some(node) },
        ],
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "none of these three commands should ever queue an ActionDescriptor");
}
//#endregion 🔖️NoOpCommandTests

//#region 🔖️SceneCommandTests
/// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
/// matching `ui_wgpu::wgpu::events::tests::component_scene_ui`'s own fixture shape (that one is private
/// to the sibling `ui_wgpu` crate, so this is a separate copy for this crate's own tests).
fn component_scene_ui(surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

/// 🌱️ `apply_tree`s a single-child `Stack(scene_node)` into `window_id` and returns the scene
/// leaf's own `NodeId` — every scene-command test below needs a real, live tree node since
/// `apply_scene_ui_command` re-fetches it by `(window_id, node)` from `UI_ENGINE`.
fn seed_scene_window_with(window_id: &str, scene_node: UiNode) -> NodeId {
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![scene_node])));
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let tree = engine.tree(window_id).unwrap();
        let child = tree.children(tree.root.unwrap()).next().expect("the ComponentScene child should be in the retained tree");
        child
    })
}

fn seed_scene_window(window_id: &str, surface_id: &str, kind: ui_wgpu::wgpu::SurfaceKind) -> NodeId {
    seed_scene_window_with(window_id, component_scene_ui(surface_id, kind))
}

#[test]
fn scene_command_dispatches_a_canvas2d_pointer_down_action() {
    let window_id = "apply-ui-commands-scene-canvas2d-pointer-down";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary },
        }],
        &mut input,
    );

    let queued = crate::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "canvasPointerDown"), "a real per-event PointerDown over a canvas-2d scene should reach the same handler apply_scene_pointer used to sample, got {queued:?}");
}

#[test]
fn scene_command_dispatches_an_ink_canvas_scroll_action() {
    let window_id = "apply-ui-commands-scene-ink-canvas-scroll";
    // 🎨️ `ink_wheel` reads straight from the scene's own `ink_canvas` payload (unlike TextEditor,
    // it needs no separate lazily-render-created host state) — mirrors `RenderEntry::ink_scene`'s
    // own fixture (`apply_scene_wheel_dispatches_actions_for_a_previously_dead_surface`).
    let mut scene_node = component_scene_ui("s1", ui_wgpu::wgpu::SurfaceKind::InkCanvas);
    if let UiNode::ComponentScene(scene) = &mut scene_node {
        scene.ink_canvas = Some(ui_wgpu::wgpu::InkCanvasScene { document_json: "{}".into(), selection_json: "[]".into(), hovered_id: None, active_utility: String::new(), view_mode: "canvas".into(), interactive: true });
    }
    let node = seed_scene_window_with(window_id, scene_node);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.into(), node, surface_id: "s1".into(), kind: ui_wgpu::wgpu::SurfaceKind::InkCanvas, rect, event: ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: -1.0 } }],
        &mut input,
    );

    let queued = crate::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "setCamera"), "a real per-event Scroll over an ink-canvas scene should reach handle_scene_wheel, got {queued:?}");
}

#[test]
fn stale_scene_revision_retires_without_mutation_or_action_publication() {
    while !close_scene_interaction_step() {}
    let window_id = "apply-ui-commands-stale-scene-revision";
    let node = seed_scene_window(window_id, "original", ui_wgpu::wgpu::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(window_id, node, ui_wgpu::wgpu::SurfaceKind::Canvas2d, rect, &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary }, &mut input);
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree(window_id, &stack_with("root", None, vec![component_scene_ui("replacement", ui_wgpu::wgpu::SurfaceKind::Canvas2d)])));

    assert!(drive_scene_interaction_step(&mut input));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    assert!(scene_interaction_terminal_is_empty());
}

#[test]
fn scene_command_skips_bespoke_surface_kinds_to_avoid_double_dispatch() {
    let window_id = "apply-ui-commands-scene-bespoke-skip";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::NodeGraph);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::NodeGraph,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary },
        }],
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "node-graph already gets real input through its own bespoke dock/engine_canvas host and must not be double-dispatched via UiCommand::Scene");
}

#[test]
fn pointer_button_code_round_trips_through_pointer_button_from_code_for_every_dom_button_code() {
    for code in [0i16, 1, 2] {
        let button = pointer_button_from_code(code);
        assert_eq!(pointer_button_code(button), code, "code {code} should round-trip through PointerButton unchanged (regression guard for the W4 1/2-swap fix)");
    }
}

/// 🧭️ Smoke-tests every one of the 11 non-bespoke `SurfaceKind`s through the real `UiCommand::Scene`
/// path (PointerDown, PointerMove, PointerUp, Scroll) — proving each one is actually reachable
/// through `apply_scene_ui_command` (no panics, no silently-skipped kind) before the per-frame
/// `apply_scene_wheel`/`apply_scene_pointer` sampling fallback they used to depend on is deleted.
#[test]
fn scene_command_reaches_every_generic_fallback_surface_kind_without_panicking() {
    let kinds = [
        ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        ui_wgpu::wgpu::SurfaceKind::Paint2d,
        ui_wgpu::wgpu::SurfaceKind::TextEditor,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas,
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline,
        ui_wgpu::wgpu::SurfaceKind::Table,
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem,
        ui_wgpu::wgpu::SurfaceKind::IconRender,
        ui_wgpu::wgpu::SurfaceKind::BlockList,
        ui_wgpu::wgpu::SurfaceKind::DiffView,
        ui_wgpu::wgpu::SurfaceKind::EventFeed,
    ];
    for kind in kinds {
        assert!(!crate::scenes::scene_has_bespoke_pointer_dispatch(kind), "{kind:?} must stay in the generic-fallback set for this smoke test to be meaningful");
        let window_id = format!("apply-ui-commands-scene-smoke-{kind:?}");
        let node = seed_scene_window(&window_id, "s1", kind);
        let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let events = [
            ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary },
            ui_wgpu::wgpu::UiEvent::PointerMove { x: 12.0, y: 12.0 },
            ui_wgpu::wgpu::UiEvent::PointerUp { x: 12.0, y: 12.0, button: ui_wgpu::wgpu::PointerButton::Primary },
            ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: 4.0 },
        ];
        for event in events {
            apply_ui_commands(&[ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.clone(), node, surface_id: "s1".into(), kind, rect, event }], &mut input);
        }
    }
}

/// 🖱️➡️ W4 fix regression guard: a right-click (`button == 2`) `PointerDown` on a `✏️TextEditor`
/// scene must route through `apply_scene_ui_command` -> `handle_scene_pointer_button` ->
/// `engine_canvas::text_editor_pointer_down` with the REAL button code — before this ticket's fix,
/// `pointer_button_from_code`'s 1/2 swap would have handed it `1`, not `2`. This test can't observe
/// `EditorHost`'s own caret state from here: `text_editor_pointer_down` only reaches a live
/// `EditorHost` once `engine_canvas::paint_text_editor` has lazily created one in `ENGINE_SURFACES`
/// (a real render pass this unit test intentionally doesn't run), so it correctly no-ops gracefully
/// here. `framework_editor::pointer_down_screen_repositions_caret_for_non_primary_button_but_does_
/// not_start_a_drag_selection` (that crate's own test) is the real assertion on `EditorHost`'s
/// behavior; `pointer_button_code_round_trips_through_pointer_button_from_code_for_every_dom_
/// button_code` (above) is the real assertion on the button-code plumbing. This test's only job is
/// proving the `UiCommand::Scene` route reaches that call site end-to-end without panicking.
#[test]
fn scene_command_right_click_on_text_editor_does_not_panic_and_stays_a_graceful_no_op_without_a_rendered_host() {
    let window_id = "apply-ui-commands-scene-text-editor-right-click";
    let node = seed_scene_window(window_id, "s1", ui_wgpu::wgpu::SurfaceKind::TextEditor);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &[ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: ui_wgpu::wgpu::SurfaceKind::TextEditor,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Secondary },
        }],
        &mut input,
    );

    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "no ENGINE_SURFACES entry exists without a real paint pass, so this should no-op rather than panic or queue a stale action");
}
//#endregion 🔖️SceneCommandTests
