/// 🎯️ Canvas pointer, wheel and catalogue drag: published canvas capture, the renderer canvas pointer sequence, secondary drag, retargeting, wheel bursts, catalogue drops and double click.

/// ⚖️ LAW: the physical Shell route preserves Canvas2d's full modifier chord and gives every
/// captured gesture exactly one cancelled terminal action on Escape or a hitless release.
#[test]
fn published_canvas_pointer_capture_preserves_modifiers_and_cancels_exactly_once() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture = canvas_input_fixture();
    let surface = fixture["surface"]["id"].as_str().expect("surface id");
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let width = fixture["surface"]["width"].as_f64().expect("surface width") as f32;
    let height = fixture["surface"]["height"].as_f64().expect("surface height") as f32;
    let body = Rect::new(17.0, 31.0, width, height);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let down = &fixture["pointer"]["down"];
    let x = body.x + down["x"].as_f64().expect("pointer x") as f32;
    let y = body.y + down["y"].as_f64().expect("pointer y") as f32;
    input.modifiers = PointerModifiers { shift: true, ctrl: true, alt: true, meta: true };
    let theme = Theme::default();

    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes Canvas2d down");
    PointerCtx::of(&mut shell).handle_keyboard(semio_framework_ui_wgpu_input::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    let down_args = canvas_action_args(&actions[0]);
    assert_json_number_eq(&down_args["x"], &down["x"], "pointer down x");
    assert_json_number_eq(&down_args["y"], &down["y"], "pointer down y");
    assert_json_number_eq(&down_args["worldX"], &down["world"]["x"], "pointer down world x");
    assert_json_number_eq(&down_args["worldY"], &down["world"]["y"], "pointer down world y");
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(down_args[modifier], down["modifiers"][modifier]);
    }
    let cancelled = canvas_action_args(&actions[1]);
    assert_json_number_eq(&cancelled["x"], &fixture["pointer"]["cancel"]["x"], "cancel x");
    assert_json_number_eq(&cancelled["y"], &fixture["pointer"]["cancel"]["y"], "cancel y");
    assert_json_number_eq(&cancelled["worldX"], &fixture["pointer"]["cancel"]["world"]["x"], "cancel world x");
    assert_json_number_eq(&cancelled["worldY"], &fixture["pointer"]["cancel"]["world"]["y"], "cancel world y");
    assert_eq!(cancelled["cancelled"], true);
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(cancelled[modifier], false);
    }

    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a fresh Canvas2d gesture arms");
    let outside = (body.x + body.w + 20.0, body.y + body.h + 20.0);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("captured outside release routes");
    let outside_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(outside_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&outside_actions[1])["cancelled"], true);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("duplicate release stays inert");
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a retired gesture cannot emit a second terminal action");
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("explicit cancellation gesture arms");
    PointerCtx::of(&mut shell).handle_pointer_cancel(&mut input);
    PointerCtx::of(&mut shell).handle_pointer_cancel(&mut input);
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&actions[1])["cancelled"], true);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(!input.pointer_down && !input.drag.active);
    while !document.close_step() {}
}

#[test]
fn renderer_canvas_surface_routes_the_retained_pointer_sequence() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_pointer_sequence(false);
}

#[test]
fn renderer_foreign_pointer_cancel_preserves_the_retained_canvas_sequence() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_pointer_sequence(true);
}

fn renderer_canvas_pointer_sequence(foreign_cancel: bool) {
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture = canvas_input_fixture();
    let surface = fixture["surface"]["id"].as_str().unwrap();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut interaction = pointer_interaction(shell, input);
    let x = body.x + fixture["pointer"]["down"]["x"].as_f64().unwrap() as f32;
    let y = body.y + fixture["pointer"]["down"]["y"].as_f64().unwrap() as f32;
    semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(77), x, y, true, 0, PointerModifiers::default()));
    let down = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut interaction.input);
    if foreign_cancel {
        interaction.handle_pointer_cancel(mouse_pointer(78));
    }
    let after_foreign_cancel = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut interaction.input);
    semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(77), body.x + body.w + 20.0, body.y + body.h + 20.0, false, 0, PointerModifiers::default()));
    let up = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut interaction.input);
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut interaction.input);
    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(ui_render::PointerId(77));
    while !document.close_step() {}
    assert_eq!(down.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"], "actual renderer dispatch reaches the retained Canvas2d owner");
    assert!(after_foreign_cancel.is_empty(), "another pointer must not finish the captured Canvas gesture");
    assert_eq!(up.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerUp"]);
    assert_eq!(canvas_action_args(&up[0])["cancelled"], true);
}

#[test]
fn renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let row = fixture["panCases"].as_array().unwrap().iter().find(|row| row["id"] == "right-button-gesture").unwrap();
    let surface = fixture["surface"]["id"].as_str().unwrap();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let initial = &fixture["wheel"]["initialCamera"];
    let layers = serde_json::json!([{ "role": "meta", "utility": row["activeUtility"] }]).to_string();
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(initial["x"].as_f64().unwrap(), initial["y"].as_f64().unwrap(), initial["zoom"].as_f64().unwrap(), layers);
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut interaction = pointer_interaction(shell, input);
    let start = (body.x + row["start"]["x"].as_f64().unwrap() as f32, body.y + row["start"]["y"].as_f64().unwrap() as f32);
    let end = (body.x + row["end"]["x"].as_f64().unwrap() as f32, body.y + row["end"]["y"].as_f64().unwrap() as f32);
    let button = row["button"].as_i64().unwrap() as i16;
    semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(77), start.0, start.1, true, button, PointerModifiers::default()));
    semio_framework_async::block_on(interaction.handle_pointer_move(mouse_pointer(77), end.0, end.1, true, button, PointerModifiers::default()));
    semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(77), end.0, end.1, false, button, PointerModifiers::default()));
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut interaction.input);
    let action_ids = actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>();
    let context_menu_open = interaction.shell.input.context_menu.is_some();
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut interaction.input);
    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(ui_render::PointerId(77));
    while !document.close_step() {}
    assert_eq!(action_ids, row["expectedActions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect::<Vec<_>>(), "the actual App/Shell route preserves the neutral Canvas2d secondary document gesture");
    assert!(context_menu_open, "the document gesture preserves Canvas2d's secondary-click context menu");
}

#[test]
fn a_published_canvas_hit_cannot_retarget_a_same_key_successor() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "canvas-secondary-successor";
    let controller = "controller.canvas-secondary-successor";
    let body = Rect::new(17.0, 31.0, 100.0, 100.0);
    let point = (body.x + body.w * 0.5, body.y + body.h * 0.5);
    let mut shell = host_test_shell();
    let initial_scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut initial = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &initial_scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &initial, body)]);
    let hit = input.hit_at(point.0, point.1).expect("the initial Canvas2d publication owns its body point").clone();
    let old_target = shell.input.retained_scene_hits.get(surface).expect("the published generic Canvas hit carries exact scene provenance").1.clone();
    let old_scene_hits = shell.input.retained_scene_hits.clone();
    let old_hit_windows = shell.input.retained_hit_windows.clone();

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut removed = shell.windows.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _removed_input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &removed, body)]);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, point.0 - body.x, point.1 - body.y).is_none(), "the intermediate publication retires the mounted Canvas owner");

    let successor_scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 2.0, "[]".into());
    let mut successor = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &successor_scene)]).unwrap();
    let _successor_input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &successor, body)]);
    let successor_target = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, point.0 - body.x, point.1 - body.y).expect("the successor Canvas2d scene is live");
    assert_eq!(old_target.surface_id, successor_target.surface_id);
    assert_eq!(old_target.key, successor_target.key, "the regression keeps the exact same retained key");
    assert_ne!(old_target, successor_target, "the successor still has a distinct generation-qualified identity");

    shell.input.retained_scene_hits = old_scene_hits;
    shell.input.retained_hit_windows = old_hit_windows;
    assert!(shell.input.retained_canvas2d_hit_window(&hit, point.0, point.1).is_none(), "an old published hit cannot inherit a same-surface same-key successor");
    while !initial.close_step() {}
    while !removed.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn renderer_canvas_wheel_burst_preserves_each_zoom_in_event() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_wheel_burst("two-zoom-in-events");
}

#[test]
fn renderer_canvas_first_wheel_uses_the_authored_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_wheel_burst("authored-camera");
}

#[test]
fn renderer_canvas_opposite_wheel_events_preserve_the_react_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_wheel_burst("zoom-in-then-out-events");
}

fn renderer_canvas_wheel_burst(case_id: &str) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let case = if case_id == "authored-camera" {
        serde_json::json!({ "deltas": [fixture["wheel"]["zoomInDeltaY"]], "expectedFactors": [fixture["wheel"]["zoomInFactor"]], "expectedActionCount": 1 })
    } else {
        fixture["wheelBursts"].as_array().unwrap().iter().find(|case| case["id"] == case_id).unwrap().clone()
    };
    let surface = format!("renderer-canvas-{case_id}");
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let initial = &fixture["wheel"]["initialCamera"];
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(initial["x"].as_f64().unwrap(), initial["y"].as_f64().unwrap(), initial["zoom"].as_f64().unwrap(), "[]".into());
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(&surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let mut input = paint_component_pointer_documents(&mut shell, &[(&surface, controller, &document, body)]);
    let x = body.x + fixture["wheel"]["anchor"]["x"].as_f64().unwrap() as f32;
    let y = body.y + fixture["wheel"]["anchor"]["y"].as_f64().unwrap() as f32;
    let mut interaction = pointer_interaction(shell, input);
    for delta in case["deltas"].as_array().unwrap() {
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::Scroll { x, y, delta_x: 0.0, delta_y: delta.as_f64().unwrap() as f32, modifiers: ui_render::EventModifiers::default() }));
    }
    let actions = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0);
    let actions: Vec<_> = actions.iter().filter(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).collect();
    let expected_zoom = case["expectedFactors"].as_array().unwrap().iter().fold(initial["zoom"].as_f64().unwrap() as f32, |zoom, factor| (zoom * factor.as_f64().unwrap() as f32).clamp(0.05, 32.0));
    while !document.close_step() {}
    assert_eq!(actions.len(), case["expectedActionCount"].as_u64().unwrap() as usize, "the burst publishes one settled camera");
    let camera = &canvas_action_args(actions[0])["camera"];
    let zoom = camera["zoom"].as_f64().unwrap() as f32;
    assert_eq!(zoom, expected_zoom, "every admitted wheel event contributes its own React factor");
    let relative_x = x - body.x - body.w * 0.5;
    let relative_y = y - body.y - body.h * 0.5;
    assert!((relative_x / zoom + camera["x"].as_f64().unwrap() as f32 - (relative_x / initial["zoom"].as_f64().unwrap() as f32 + initial["x"].as_f64().unwrap() as f32)).abs() < 0.0001);
    assert!((relative_y / zoom + camera["y"].as_f64().unwrap() as f32 - (relative_y / initial["zoom"].as_f64().unwrap() as f32 + initial["y"].as_f64().unwrap() as f32)).abs() < 0.0001);
}

/// ⚖️ LAW: a real retained Tree drag reaches a real published Canvas2d leaf through Shell move/drop
/// ingress, preserves the raw catalogue MIME payload, and retires both drag authorities once.
#[test]
fn retained_catalogue_item_drags_and_drops_into_a_published_canvas() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = canvas_input_fixture();
    let catalogue = &fixture["catalogue"];
    let raw_payload = catalogue["rawPayload"].as_str().expect("raw catalogue payload");
    let source_id = "canvas-catalogue-source";
    let destination_id = fixture["surface"]["id"].as_str().expect("destination surface");
    let controller = fixture["surface"]["controllerId"].as_str().expect("destination controller");
    let mut shell = host_test_shell();
    let mut source_document = published_canvas_catalogue_document(&mut shell, source_id, raw_payload);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut destination_document = shell.windows.publish_surface_records(destination_id, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d destination publishes");
    let source_body = Rect::new(11.0, 17.0, 320.0, 200.0);
    let destination_body = Rect::new(411.0, 17.0, 100.0, 100.0);
    let mut input = paint_component_pointer_documents(&mut shell, &[(source_id, "controller.catalogue", &source_document, source_body), (destination_id, controller, &destination_document, destination_body)]);
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("tree.drag.transfer.catalogue")).expect("catalogue source publishes a transfer handle").rect;
    let point = &catalogue["point"];
    let source_point = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let destination_point = (destination_body.x + point["x"].as_f64().expect("drop x") as f32, destination_body.y + point["y"].as_f64().expect("drop y") as f32);
    let theme = Theme::default();
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("catalogue source down routes");
    PointerCtx::of(&mut shell).handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Canvas2d drop routes");
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave", "canvasDrop"]);
    let over = canvas_action_args(&actions[0]);
    assert_json_number_eq(&over["x"], &point["x"], "drag over x");
    assert_json_number_eq(&over["y"], &point["y"], "drag over y");
    assert_json_number_eq(&over["width"], &fixture["surface"]["width"], "drag over width");
    assert_json_number_eq(&over["height"], &fixture["surface"]["height"], "drag over height");
    assert_eq!(over["types"], catalogue["types"]);
    let leave = canvas_action_args(&actions[1]);
    assert_eq!(leave["surfaceId"], destination_id);
    let drop = canvas_action_args(&actions[2]);
    assert_json_number_eq(&drop["x"], &point["x"], "drop x");
    assert_json_number_eq(&drop["y"], &point["y"], "drop y");
    assert_json_number_eq(&drop["width"], &fixture["surface"]["width"], "drop width");
    assert_json_number_eq(&drop["height"], &fixture["surface"]["height"], "drop height");
    assert_eq!(drop["dragData"], raw_payload);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions().is_empty());
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("duplicate drop release stays inert");
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a retired retained drag cannot drop twice");

    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("a second catalogue source down routes");
    PointerCtx::of(&mut shell).handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    PointerCtx::of(&mut shell).handle_pointer_move(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, false, 0, &mut input, &theme)).expect("outside catalogue release cancels cleanly");
    let cancelled = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(cancelled.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave"]);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions().is_empty());
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
}

/// ⚖️ LAW: two complete primary clicks on the published Canvas2d leaf emit one React-shaped
/// `canvasDoubleClick` at the shared fixture point, after both release phases.
#[test]
fn published_canvas_double_click_emits_once_at_surface_coordinates() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = canvas_input_fixture();
    let surface = "canvas-double-click-surface";
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let body = Rect::new(13.0, 29.0, 100.0, 100.0);
    let point = &fixture["doubleClick"];
    let x = body.x + point["x"].as_f64().expect("double click x") as f32;
    let y = body.y + point["y"].as_f64().expect("double click y") as f32;
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let theme = Theme::default();

    assert!(point["intervalMs"].as_f64().expect("double click interval") <= 400.0);
    for _ in 0..2 {
        semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Canvas2d click down routes");
        semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("Canvas2d click up routes");
    }
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp", "canvasPointerDown", "canvasPointerUp", "canvasDoubleClick"]);
    let double_click = canvas_action_args(actions.last().expect("double click action"));
    assert_json_number_eq(&double_click["x"], &point["x"], "double click x");
    assert_json_number_eq(&double_click["y"], &point["y"], "double click y");
    assert_json_number_eq(&double_click["width"], &fixture["surface"]["width"], "double click width");
    assert_json_number_eq(&double_click["height"], &fixture["surface"]["height"], "double click height");
    while !document.close_step() {}
}
