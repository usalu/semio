/// 🚪️ Window close gestures and generic surfaces: queued scene and capture owners at close, active canvas gestures, bespoke surface skipping, pointer button codes and generic fallback surfaces.

//#region 🔖️SceneCommandTests

#[test]
fn window_close_retires_queued_scene_and_capture_owners_before_slot_release() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let window_id = "closing-window-scene-owners";
    let node = seed_scene_window(window_id, "closing-canvas", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let target = retained_scene_target(window_id, node).unwrap();
    let generation = target.window_generation;
    assert!(claim_scene_pointer_owner(target, ui_render::PointerId(1)));
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    assert!(UI_ENGINE.with(|cell| cell.borrow().surface_token(window_id).is_none()));
    assert!(scene_interaction_terminal_is_empty(), "the sole window close owner also retires its queued scene work");
    assert!(SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().all(|owner| owner.target.window_id != window_id || owner.target.window_generation != generation)));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    caller.finish();
}

#[test]
fn closing_window_silently_retires_an_active_canvas_gesture() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let window_id = "closing-window-active-canvas";
    let node = seed_scene_window(window_id, "active-canvas", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        Rect::new(0.0, 0.0, 200.0, 200.0),
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(drive_scene_interaction_step(&mut input));
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"]);
    assert!(request_ui_document_close(window_id));
    semio_framework_os_renderer_wgpu_scene_render::request_canvas_pointer_gesture_cancel_for_window(window_id);
    drive_scene_interaction_step(&mut input);
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "closing a mounted Canvas cannot publish a synthetic pointer terminal action");
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    assert!(!semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture_for(ui_render::PointerId(1), &mut input));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    caller.finish();
}

#[test]
fn live_canvas_gesture_cancels_when_its_published_generation_is_replaced() {
    while !close_scene_interaction_step() {}
    let mut discarded = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut discarded);
    let window_id = "canvas-generation-cancellation";
    let node = publish_focus_rebase_document(window_id, 1, 454, semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d, &[4]).expect("presented Canvas receiver");
    assert_eq!(retained_scene_surface_id(window_id, node), window_id);
    let canvas_host = retained_scene_host_id(window_id, node);
    let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        rect,
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 20.0, y: 20.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: ui_wgpu::wgpu::EventModifiers::default() },
        Some(ui_render::PointerId(1)),
        &mut input,
    );
    assert!(drive_scene_interaction_step(&mut input));
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"]);

    let successor = publish_focus_rebase_document(window_id, 2, 455, semio_framework_ui_declaration::ui::SurfaceKind::TextEditor, &[4]).expect("presented non-Canvas successor");
    assert_ne!(retained_scene_host_id(window_id, successor), canvas_host, "component-kind replacement receives a fresh private host identity");
    assert!(drive_scene_interaction_step(&mut input), "the terminal lane observes the replaced generation even when the old surface no longer paints");
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerUp"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("cancelled")).and_then(semio_framework::DslValue::as_bool), Some(true));
    retire_presented_document(window_id);
}

#[test]
fn scene_command_skips_bespoke_surface_kinds_to_avoid_double_dispatch() {
    let window_id = "apply-ui-commands-scene-bespoke-skip";
    let node = seed_scene_window(window_id, "s1", semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "node-graph already gets real input through its own bespoke dock/engine_canvas host and must not be double-dispatched via UiCommand::Scene");
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
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
        semio_framework_ui_declaration::ui::SurfaceKind::Paint2d,
        semio_framework_ui_declaration::ui::SurfaceKind::TextEditor,
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas,
        semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline,
        semio_framework_ui_declaration::ui::SurfaceKind::Table,
        semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem,
        semio_framework_ui_declaration::ui::SurfaceKind::IconRender,
        semio_framework_ui_declaration::ui::SurfaceKind::BlockList,
        semio_framework_ui_declaration::ui::SurfaceKind::DiffView,
        semio_framework_ui_declaration::ui::SurfaceKind::EventFeed,
    ];
    for kind in kinds {
        assert!(!semio_framework_os_renderer_wgpu_scene_render::scene_has_bespoke_pointer_dispatch(kind), "{kind:?} must stay in the generic-fallback set for this smoke test to be meaningful");
        let window_id = format!("apply-ui-commands-scene-smoke-{kind:?}");
        let node = seed_scene_window(&window_id, "s1", kind);
        let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let events = [
            ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::PointerMove { x: 12.0, y: 12.0, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::PointerUp { x: 12.0, y: 12.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
            ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: 4.0, modifiers: Default::default() },
        ];
        for event in events {
            apply_ui_commands(&mut vec![ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.clone(), node, surface_id: "s1".into(), kind, rect, event }], None, &mut input);
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
    let node = seed_scene_window(window_id, "s1", semio_framework_ui_declaration::ui::SurfaceKind::TextEditor);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: semio_framework_ui_declaration::ui::SurfaceKind::TextEditor,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Secondary, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "no ENGINE_SURFACES entry exists without a real paint pass, so this should no-op rather than panic or queue a stale action");
}
//#endregion 🔖️SceneCommandTests
