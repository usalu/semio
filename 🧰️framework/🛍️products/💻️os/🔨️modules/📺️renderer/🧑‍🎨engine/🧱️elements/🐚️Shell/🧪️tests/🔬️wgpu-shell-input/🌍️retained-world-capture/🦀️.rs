/// 🌍️ Retained world capture: the topmost retained world owning press and release, capture preservation, rebasing and fixed capture grants.

#[test]
fn the_topmost_retained_world_owns_the_press_and_release_outside_its_bounds() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("captured");
}

#[test]
fn captured_world_movement_and_wheel_never_fan_out_to_an_overlapping_peer() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("motion-wheel");
}

#[test]
fn cancelled_world_capture_cannot_manufacture_a_successful_release() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("cancel");
}

#[test]
fn a_modal_layer_blocks_actual_world_pointer_ingress() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("modal");
}

#[test]
fn a_scene_refresh_preserves_the_exact_captured_owner_until_outside_release() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("refresh");
}

#[test]
fn an_accepted_sibling_insertion_rebases_the_exact_renderer_scene_capture() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let law = &fixture["siblingInsertion"];
    let window = "renderer-capture-arena-rebase";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let pointer = ui_render::PointerId(77);
    let mut shell = host_test_shell();
    let mut documents = Vec::new();
    let mut original = None;
    let mut current = None;
    for (index, ids) in law["records"].as_array().unwrap().iter().enumerate() {
        let ids: Vec<_> = ids.as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect();
        let mut root =
            tree_pointer_record(1, "root", ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &ids, None);
        root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        let mut records = vec![root];
        for id in ids {
            let mut record = canvas_pointer_record(id, &format!("scene-{id}"), &semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into()));
            record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
            records.push(record);
        }
        let document = shell.windows.publish_surface_records(window, records).unwrap();
        let input = paint_component_pointer_documents(&mut shell, &[(window, "capture-controller", &document, body)]);
        if index > 0 {
            let key = semio_framework_ui_retained_tree::tree::NodeKey::Explicit(format!("scene-{}", law["receiver"].as_u64().unwrap()));
            let target = input
                .hits()
                .iter()
                .filter(|hit| hit.kind == HitKind::ComponentScene)
                .find_map(|hit| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(window, hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5).filter(|target| target.key == key))
                .expect("the same protocol record remains mounted");
            if index == 1 {
                assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::claim_scene_pointer_owner(target.clone(), pointer));
                original = Some(target);
            } else {
                current = Some(target);
            }
        }
        documents.push(document);
    }
    let original = original.unwrap();
    let current = current.unwrap();
    let captured = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::captured_scene_pointer(pointer);
    let released = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(pointer);
    let duplicate = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(pointer);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(window));
    for _ in 0..262_144 {
        if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
            break;
        }
        caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn");
    }
    for mut document in documents {
        while !document.close_step() {}
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending());
    assert!(original.same_component_host(&current), "a sibling insertion preserves the mounted receiver");
    assert_ne!(original.node, current.node, "the fixture must exercise different arena histories");
    assert_eq!(captured, Some(current.clone()));
    assert_eq!(usize::from(released == Some(current)), law["terminalReceivers"].as_u64().unwrap() as usize);
    assert_eq!(duplicate, None);
    caller.finish();
}

/// 🕒️ Accepted removal invalidates every old camera before the one-slot retirement lane drains.
#[test]
fn renderer_canvas_multiple_accepted_removals_invalidate_every_checked_out_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let window = "renderer-camera-multiple-retirement";
    let controller = "renderer-camera-multiple-retirement-controller";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let root = |children: &[u64]| {
        let mut record = tree_pointer_record(
            1,
            "root",
            ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
            children,
            None,
        );
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        record
    };
    let child = |id, key: &str| {
        let mut record = canvas_pointer_record(id, key, &scene);
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        record
    };
    let mut shell = host_test_shell();
    let mut presented = shell.windows.publish_surface_records(window, vec![root(&[2, 3]), child(2, "left-camera"), child(3, "right-camera")]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(window, controller, &presented, body)]);
    let mut rects: Vec<_> = input.hits().iter().filter(|hit| hit.kind == HitKind::ComponentScene).map(|hit| hit.rect).collect();
    rects.sort_by(|left, right| left.x.total_cmp(&right.x));
    assert_eq!(rects.len(), 2);
    let mut interaction = pointer_interaction(shell, input);
    for rect in &rects {
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
            &mut interaction,
            &mut ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
        ));
    }
    let mut deadlines = semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchCursor::begin(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0, semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0;
    let mut removed = interaction.shell.windows.publish_surface_records(window, vec![root(&[])]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(window, controller, &removed, body)]);
    let mut actions = Vec::new();
    loop {
        match deadlines.step(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0 {
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Action(action) => actions.push(action),
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Pending => {}
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Complete => break,
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
        }
    }
    assert!(deadlines.terminal_is_empty());
    assert!(actions.is_empty(), "every camera removed by the accepted pixels becomes stale atomically");
    while !presented.close_step() {}
    while !removed.close_step() {}
}

/// 🕒️ A same-host sibling insertion preserves a checked-out camera across its arena rebase.
#[test]
fn renderer_canvas_same_host_sibling_rebase_preserves_checked_out_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let law = &fixture["siblingInsertion"];
    let receiver = law["receiver"].as_u64().unwrap();
    let window = "renderer-camera-sibling-rebase";
    let controller = "renderer-camera-sibling-rebase-controller";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let mut shell = host_test_shell();
    let mut documents = Vec::new();
    let mut deadline = None;
    let mut presented = None;
    let mut accepted = None;
    for (index, ids) in law["records"].as_array().unwrap().iter().enumerate() {
        let ids: Vec<_> = ids.as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect();
        let mut root =
            tree_pointer_record(1, "root", ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &ids, None);
        root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        let mut records = vec![root];
        for id in ids {
            let mut record = canvas_pointer_record(id, &format!("scene-{id}"), &semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into()));
            record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
            records.push(record);
        }
        let document = shell.windows.publish_surface_records(window, records).unwrap();
        let input = paint_component_pointer_documents(&mut shell, &[(window, controller, &document, body)]);
        let key = semio_framework_ui_retained_tree::tree::NodeKey::Explicit(format!("scene-{receiver}"));
        let target_and_rect = input
            .hits()
            .iter()
            .filter(|hit| hit.kind == HitKind::ComponentScene)
            .find_map(|hit| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(window, hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5).filter(|target| target.key == key).map(|target| (target, hit.rect)));
        if index == 1 {
            let (target, rect) = target_and_rect.expect("the receiver is presented before its sibling insertion");
            presented = Some(target);
            let mut mounted = pointer_interaction(shell, input);
            semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
                &mut mounted,
                &mut ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
            ));
            deadline = Some(semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchCursor::begin(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0, semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0);
            shell = mounted.shell;
        } else {
            if index == 2 {
                accepted = target_and_rect.map(|(target, _)| target);
            }
        }
        documents.push(document);
    }
    let presented = presented.expect("the receiver owns the checked-out camera");
    let accepted = accepted.expect("the accepted sibling candidate retains the receiver");
    assert!(presented.same_component_host(&accepted));
    assert_ne!(presented.node, accepted.node, "the fixture exercises a real arena-node rebase");
    let mut deadline = deadline.expect("the receiver camera is checked out before acknowledgement");
    let mut actions = Vec::new();
    loop {
        match deadline.step(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0 {
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Action(action) => actions.push(action),
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Pending => {}
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Complete => break,
            semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
        }
    }
    assert!(deadline.terminal_is_empty());
    assert_eq!(actions.len(), 1, "the accepted same-host receiver keeps its checked-out camera");
    for mut document in documents {
        while !document.close_step() {}
    }
}

#[test]
fn a_replacement_scene_cannot_inherit_the_previous_keys_capture() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("replacement");
}

#[test]
fn another_pointers_chrome_release_preserves_the_captured_scene_drag() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("foreign-release");
}

#[test]
fn another_pointers_cancellation_preserves_the_captured_scene_drag() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    retained_world_sequence_probe("foreign-cancel");
}

#[test]
fn replaced_scene_pointer_slots_release_the_fixed_capture_grant() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let capacity = fixture["captureCapacity"].as_u64().unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock.dock_tabs = ShellDock::default();
    let scene = semio_framework_ui_scene_world3d::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let window = "scene-capture-capacity";
    let bounds = Rect::new(0.0, 0.0, 300.0, 200.0);
    let mut first = shell.windows.publish_surface_records(window, vec![world3d_pointer_record(1, "original", &scene)]).unwrap();
    let input = paint_tree_pointer_document(&mut shell, window, &first, bounds);
    let target = shell.input.scene_pointer_target_at(100.0, 100.0, &input, &Theme::default()).unwrap();
    let admitted: Vec<_> = (0..capacity).map(|pointer| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::claim_scene_pointer_owner(target.clone(), ui_render::PointerId(pointer + 100))).collect();
    let overflow = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::claim_scene_pointer_owner(target.clone(), ui_render::PointerId(999));
    let mut successor = shell.windows.publish_surface_records(window, vec![world3d_pointer_record(1, "successor", &scene)]).unwrap();
    let input = paint_tree_pointer_document(&mut shell, window, &successor, bounds);
    let target = shell.input.scene_pointer_target_at(100.0, 100.0, &input, &Theme::default()).unwrap();
    let old_live: Vec<_> = (0..capacity).map(|pointer| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::captured_scene_pointer(ui_render::PointerId(pointer + 100)).is_some()).collect();
    let fresh = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::claim_scene_pointer_owner(target, ui_render::PointerId(999));
    for pointer in 0..capacity {
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(ui_render::PointerId(pointer + 100));
    }
    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(ui_render::PointerId(999));
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.surfaces.retired_world3d_states.is_empty() {
            break;
        }
        shell.surfaces.advance_world3d_retirement_step();
    }
    while !first.close_step() {}
    while !successor.close_step() {}
    assert!(admitted.iter().all(|value| *value), "every funded pointer owns a slot");
    assert!(!overflow, "live captures cannot exceed the fixed grant");
    assert!(old_live.iter().all(|value| !value), "a replaced key retires every old receiver");
    assert!(fresh, "stale targets must release their fixed grant before a successor claims it");
}
