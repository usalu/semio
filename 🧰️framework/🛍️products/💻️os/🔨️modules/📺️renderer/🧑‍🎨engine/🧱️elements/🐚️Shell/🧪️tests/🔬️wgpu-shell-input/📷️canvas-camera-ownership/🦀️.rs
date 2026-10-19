/// 📷️ Canvas camera ownership: closed and reopened windows, sibling components, sequential close, mounted identity, retired deadlines and remount retirement of the pending camera.

/// 🪟️ Closing a Dock window retires its exact Canvas owner before a pending camera can settle.
#[test]
fn renderer_canvas_closed_window_retires_its_pending_camera_owner() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-closed-window";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.dock.sync_active_window(surface);
    shell.windows.active_window_id = Some(surface.into());
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let document = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    shell.docs.window_ui.insert(surface.into(), document);
    let mut interaction = pointer_interaction(shell, input);
    semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
        &mut interaction,
        &mut ui_render::DispatchEvent::Scroll { x: body.x + 200.0, y: body.y + 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() },
    ));

    assert!(SessionCtx::of(&mut interaction.shell).close_dock_window(surface));
    let actions: Vec<_> = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0).into_iter().filter(|action| canvas_action_args(action)["surfaceId"] == surface).collect();
    assert_eq!(actions.len(), fixture["sameKeySceneRemount"]["retiredDeadlineActions"].as_u64().unwrap() as usize, "a Dock close fences the pending camera before retained-layout reconciliation");
    interaction.shell.docs.retire_documents_outside(&[], true, &mut interaction.shell.chrome).expect("the closed Dock body reaches its canonical document retirement lane");
    assert!(PointerCtx::of(&mut interaction.shell).settle_pump_pending(), "the retained close owns a runtime wake until its exact document is retired");
    for _ in 0..262_144 {
        if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn"), "each admitted close opportunity advances one bounded unit");
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending(), "the bounded runtime close reaches terminal");
    let target_after_close = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0);

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut cleanup = interaction.shell.windows.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _ = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &cleanup, body)]);
    semio_framework_os_renderer_wgpu_scene_state::retire_scene_identity(&owner);
    while !cleanup.close_step() {}

    assert!(target_after_close.is_none(), "the complete window close retires the retained component target");
    caller.finish();
}

#[test]
fn renderer_canvas_reopen_waits_for_the_exact_old_window_close() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-reopened-window";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.dock.sync_active_window(surface);
    let old = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &old, body)]);
    let old_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    shell.docs.window_ui.insert(surface.into(), old);
    assert!(SessionCtx::of(&mut shell).close_dock_window(surface));
    shell.docs.retire_documents_outside(&[], true, &mut shell.chrome).unwrap();
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending());
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&old_owner), "a requested close is no longer an interactive live owner");

    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.dock.sync_active_window(surface);
    let mut successor = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let mut cursor = UiDocumentFrameCursor::default();
    let (mut draw, mut atlas, icons, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), InputState::<ActionDescriptor>::default(), Theme::default());
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
    let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
    assert!(!caller.turn(|cx|render_ui_document_step(&mut cursor, &successor, body, &mut ctx, surface, controller, semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle, &mut hosts, cx)));
    assert_eq!(cursor.phase_name(), "ingress", "the successor cannot enter the old retained generation while its close is pending");

    for _ in 0..262_144 {
        if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending());
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &successor, body)]);
    let successor_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    assert_ne!(successor_owner, old_owner, "the reopened same-ID window receives a fresh exact generation");
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&successor_owner));

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut cleanup = shell.windows.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &cleanup, body)]);
    while !successor.close_step() {}
    while !cleanup.close_step() {}
    caller.retire_world(&mut world_resources);
    caller.finish();
}

/// 🪟️ Sibling components retain independent cameras while sharing their document action address.
#[test]
fn sibling_canvas_components_mount_under_one_document_without_sharing_their_camera() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let law = &fixture["siblingWindowLifetime"];
    let surface = "sibling-canvas-document";
    let controller = "sibling-canvas-controller";
    let body = Rect::new(0.0, 0.0, 400.0, 300.0);
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.dock.sync_active_window(surface);
    let scene = |index: usize| {
        let camera = &law["cameras"][index];
        semio_framework_ui_scene_plane::Canvas2dScene::base(camera["x"].as_f64().unwrap(), camera["y"].as_f64().unwrap(), camera["zoom"].as_f64().unwrap(), "[]".into())
    };
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
    let child = |id, key: &str, scene: &semio_framework_ui_scene_plane::Canvas2dScene| {
        let mut record = canvas_pointer_record(id, key, scene);
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        record
    };
    let mut document = shell.windows.publish_surface_records(surface, vec![root(&[2, 3]), child(2, "left-canvas", &scene(0)), child(3, "right-canvas", &scene(1))]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut rects: Vec<_> = input.hits().iter().filter(|hit| hit.kind == semio_framework_ui_wgpu_input::HitKind::ComponentScene).map(|hit| hit.rect).collect();
    assert_eq!(rects.len(), 2);
    rects.sort_by(|a, b| a.x.total_cmp(&b.x));
    assert!(rects.iter().all(|rect| rect.w > 0.0 && rect.h > 0.0));
    let targets: Vec<_> = rects.iter().map(|rect| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, rect.x + rect.w * 0.5, rect.y + rect.h * 0.5).unwrap()).collect();
    assert_ne!(targets[0].host_id, targets[1].host_id);
    assert_eq!(targets[0].surface_id, targets[1].surface_id);
    let mut interaction = pointer_interaction(shell, input);
    for (index, rect) in rects.iter().enumerate() {
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
            &mut interaction,
            &mut ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
        ));
        let actions = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0);
        let cameras: Vec<_> = actions.iter().filter(|action| action.controller_id == controller).map(canvas_action_args).collect();
        assert_eq!(cameras.len(), 1);
        assert_eq!(cameras[0]["surfaceId"], surface);
        assert!((cameras[0]["camera"]["zoom"].as_f64().unwrap() - law["expectedZooms"][index].as_f64().unwrap()).abs() < 0.0001);
    }
    let mut successor = interaction.shell.windows.publish_surface_records(surface, vec![root(&[3]), child(3, "right-canvas", &scene(1))]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &successor, body)]);
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&targets[0]));
    let survivor = interaction.input.hits().iter().find(|hit| hit.kind == semio_framework_ui_wgpu_input::HitKind::ComponentScene).unwrap().rect;
    semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
        &mut interaction,
        &mut ui_render::DispatchEvent::Scroll { x: survivor.x + survivor.w * 0.5, y: survivor.y + survivor.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
    ));
    let actions = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0);
    let camera = actions.iter().find(|action| action.controller_id == controller).map(canvas_action_args).unwrap();
    assert!((camera["camera"]["zoom"].as_f64().unwrap() - law["survivingZoom"].as_f64().unwrap()).abs() < 0.0001);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(surface));
    for _ in 0..262_144 {
        if !caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn") {
            break;
        }
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface));
    while !document.close_step() {}
    while !successor.close_step() {}
    caller.finish();
}

/// 🪟️ Closing each distinct window returns its retained capacity before the next window opens.
#[test]
fn renderer_canvas_sequential_window_close_reuses_retained_surface_capacity() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let law = &fixture["sequentialWindowLifetime"];
    let mount_count = law["mountCount"].as_u64().unwrap() as usize;
    assert!(mount_count > semio_framework_ui_surface_identity::UI_LAYOUT_SURFACE_SLOTS);
    assert!(mount_count > semio_framework_os_renderer_wgpu_contracts::SCENE_SURFACE_CAPACITY);
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(0.0, 0.0, 400.0, 300.0);
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut interaction = pointer_interaction(host_test_shell(), InputState::default());
    for index in 0..mount_count {
        let surface = format!("sequential-canvas-window-{index}");
        interaction.shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(&surface)], active: surface.clone() };
        interaction.shell.dock.dock.sync_active_window(&surface);
        let document = interaction.shell.windows.publish_surface_records(&surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(&surface, controller, &document, body)]);
        assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(&surface, 200.0, 150.0).is_some(), "window {index} mounts after earlier windows have closed");
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::Scroll { x: 200.0, y: 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() }));
        let actions = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0);
        let camera =
            actions.iter().find(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).map(|action| canvas_action_args(action)["camera"].clone()).expect("each admitted mount retains a working camera");
        assert_eq!(camera["x"].as_f64(), Some(12.0));
        assert_eq!(camera["y"].as_f64(), Some(-8.0));
        assert!(camera["zoom"].as_f64().unwrap() > 2.0);
        interaction.shell.docs.window_ui.insert(surface.clone(), document);
        assert_eq!(interaction.shell.docs.window_ui.len(), law["maximumLiveWindows"].as_u64().unwrap() as usize);
        assert!(SessionCtx::of(&mut interaction.shell).close_dock_window(&surface));
        interaction.shell.docs.retire_documents_outside(&[], true, &mut interaction.shell.chrome).unwrap();
        for _ in 0..262_144 {
            if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
                break;
            }
            assert!(caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn"));
        }
        assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending(), "window {index} closes within the bounded opportunity ceiling");
        assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(&surface, 200.0, 150.0).is_none());
        assert_eq!(interaction.shell.docs.window_ui.len(), law["liveWindowsAfterClose"].as_u64().unwrap() as usize);
    }
    caller.finish();
}

/// 🪪️ A retained camera survives an authored refresh and remounts with a replacement key.
#[test]
fn renderer_canvas_camera_obeys_the_mounted_component_identity() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-mount-lifetime";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut interaction = pointer_interaction(host_test_shell(), InputState::default());
    let mut documents = Vec::new();
    let mut actual = Vec::new();
    for step in fixture["mountLifetime"]["steps"].as_array().unwrap() {
        let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, step["authoredZoom"].as_f64().unwrap(), "[]".into());
        let document = interaction.shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, step["key"].as_str().unwrap(), &scene)]).unwrap();
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &document, body)]);
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
            &mut interaction,
            &mut ui_render::DispatchEvent::Scroll { x: body.x + body.w * 0.5, y: body.y + body.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() },
        ));
        let actions = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0);
        let camera = actions.iter().find(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).map(|action| canvas_action_args(action)["camera"].clone());
        actual.push(camera);
        documents.push(document);
    }
    for document in &mut documents {
        while !document.close_step() {}
    }
    for (actual, expected) in actual.iter().zip(fixture["mountLifetime"]["steps"].as_array().unwrap()) {
        let camera = actual.as_ref().expect("each mounted wheel settles its camera");
        assert!((camera["zoom"].as_f64().unwrap() - expected["expectedZoom"].as_f64().unwrap()).abs() < 0.00001, "{actual:?}");
        assert_eq!(camera["x"].as_f64(), Some(12.0));
        assert_eq!(camera["y"].as_f64(), Some(-8.0));
    }
}

/// 🕒️ A checked-out old deadline cannot publish the replacement component's camera.
#[test]
fn renderer_canvas_retired_deadline_cannot_publish_a_successor() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_retirement_probe(true, false, false);
    renderer_canvas_retirement_probe(true, true, false);
}

/// 🧹️ Removing a Canvas2d component retires its pending settled camera action.
#[test]
fn renderer_canvas_removal_retires_its_pending_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_retirement_probe(false, false, false);
    renderer_canvas_retirement_probe(false, true, false);
}

/// 🔁️ A removed and remounted Canvas owns its camera even when its retained key is reused.
#[test]
fn renderer_canvas_same_key_remount_retires_checked_out_camera() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    renderer_canvas_retirement_probe(true, false, true);
    renderer_canvas_retirement_probe(true, true, true);
}

fn renderer_canvas_retirement_probe(replace: bool, close: bool, remount: bool) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let lifetime = &fixture["sameKeySceneRemount"];
    let key = if remount { lifetime["key"].as_str().unwrap() } else { "original" };
    let surface_id = format!("renderer-canvas-retirement-{replace}-{close}-{remount}");
    let surface = surface_id.as_str();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut shell = host_test_shell();
    let scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut original = shell.windows.publish_surface_records(surface, vec![canvas_pointer_record(1, key, &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &original, body)]);
    let original_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    let mut interaction = pointer_interaction(shell, input);
    let scroll = || ui_render::DispatchEvent::Scroll { x: body.x + 200.0, y: body.y + 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() };
    semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut scroll()));
    let mut deadline = semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchCursor::begin(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0, semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0;
    let container =
        |key| tree_pointer_record(1, key, ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &[], None);
    let mut removed = if remount { Some(interaction.shell.windows.publish_surface_records(surface, vec![container(key)]).unwrap()) } else { None };
    if let Some(removed) = removed.as_ref() {
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, removed, body)]);
        assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface, 200.0, 150.0).is_none());
    }
    let next_scene = semio_framework_ui_scene_plane::Canvas2dScene::base(12.0, -8.0, if remount { lifetime["remountAuthoredZoom"].as_f64().unwrap() } else { 3.0 }, "[]".into());
    let record = if replace { canvas_pointer_record(1, if remount { key } else { "successor" }, &next_scene) } else { container("removed") };
    let mut successor = interaction.shell.windows.publish_surface_records(surface, vec![record]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &successor, body)]);
    if replace {
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut scroll()));
    }
    let mut actions = Vec::new();
    if close {
        while !matches!(deadline.close_step(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {}
    } else {
        loop {
            match deadline.step(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant()).unwrap().0 {
                semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Action(action) => {
                    if canvas_action_args(&action)["surfaceId"] == surface {
                        actions.push(action);
                    }
                }
                semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Pending => {}
                semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Complete => break,
                semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
            }
        }
    }
    assert!(deadline.terminal_is_empty());
    semio_framework_os_renderer_wgpu_scene_state::retire_scene_identity(&original_owner);
    let settled: Vec<_> = semio_framework_os_renderer_wgpu_scene_state::sweep_expired_scene_camera_dispatches(semio_framework_os_renderer_wgpu_host_base::app_now_ms() + 400.0).into_iter().filter(|action| canvas_action_args(action)["surfaceId"] == surface).collect();
    while !original.close_step() {}
    if let Some(removed) = removed.as_mut() {
        while !removed.close_step() {}
    }
    while !successor.close_step() {}
    let expected = if remount { lifetime["retiredDeadlineActions"].as_u64().unwrap() as usize } else { fixture["mountLifetime"][if replace { "retiredDeadlineActions" } else { "removedCameraActions" }].as_u64().unwrap() as usize };
    assert_eq!(actions.len(), expected, "a retired Canvas camera cannot publish after replacement/removal");
    assert_eq!(settled.len(), if remount { lifetime["remountedCameraActions"].as_u64().unwrap() as usize } else { usize::from(replace) }, "only the live successor retains its camera deadline");
    if replace {
        let camera = canvas_action_args(&settled[0])["camera"].clone();
        let expected_zoom = if remount { lifetime["expectedRemountZoom"].as_f64().unwrap() } else { fixture["mountLifetime"]["steps"][2]["expectedZoom"].as_f64().unwrap() };
        assert!((camera["zoom"].as_f64().unwrap() - expected_zoom).abs() < 0.00001);
    }
    println!("Canvas retirement replace={replace} close={close} remount={remount} emitted {} stale actions and {} successor actions", actions.len(), settled.len());
}
