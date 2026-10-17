/// 🚪️ Engine close and world3d fixtures: the exact staged host close, the painted frame, the scene painter and the world3d ladder, attach and intent helpers.

/// 🪦 A component close owns the exact staged generation as well as its CPU host. Two siblings may
/// publish the same document `surfaceId`; their runtime hosts and raster keys remain independent.
#[test]
fn a_component_engine_close_retires_only_its_exact_staged_host_and_preserves_the_wire_surface() {
    let wire_surface_id = "shared-document-surface";
    let host_a = "scene.41.3.7.1";
    let host_b = "scene.41.4.2.1";
    let snapshot_a = ensure_engine_surface(host_a, wire_surface_id, 32, 32).expect("first sibling reserves its engine host");
    let snapshot_b = ensure_engine_surface(host_b, wire_surface_id, 32, 32).expect("second sibling reserves its engine host");
    assert_ne!(snapshot_a.identity.id, snapshot_b.identity.id, "sibling runtime identities are host-derived");
    assert_eq!(engine_surface_wire_id(host_a).map(|id| id.as_str().to_string()), Some(wire_surface_id.to_string()));
    assert_eq!(engine_surface_wire_id(host_b).map(|id| id.as_str().to_string()), Some(wire_surface_id.to_string()));
    for snapshot in [snapshot_a, snapshot_b] {
        STAGED_ENGINE_SCENES.with(|cell| {
            cell.borrow_mut().upsert(StagedEngineScene { surface: snapshot, document_generation: 1, scene_revision: 1, scene: canvas::Scene::new(), clear: Color::new([0.0, 0.0, 0.0, 1.0]), width: 32, height: 32 });
        });
    }
    let token_a = ENGINE_SURFACES.with(|cell| cell.borrow_mut().token(host_a)).expect("first sibling token remains live");
    assert_eq!(begin_engine_surface_close_token(token_a), Ok(true));
    let mut input = InputState::<ActionDescriptor>::default();
    for turn in 0..262_144 {
        if with_engine_close_context(1, |context| close_engine_surface_step(token_a, context, &mut input)) {
            break;
        }
        assert!(turn + 1 < 262_144, "first sibling CPU owner reaches terminal-empty");
    }
    let staged: Vec<EngineSurfaceId> = STAGED_ENGINE_SCENES.with(|cell| {
        let mut staged = cell.borrow_mut();
        std::iter::from_fn(|| staged.take_one().map(|scene| scene.surface.identity.id)).collect()
    });
    drop_engine_surface(host_b);
    assert_eq!(staged, vec![snapshot_b.identity.id], "closing sibling A removes only A's staged generation and leaves sibling B");
}

struct PaintedFrame {
    draw: DrawList,
    world3d_states: semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap<World3dState>,
}

/// 🎬️ Drives the REAL production paint — `scenes::render_component_scene_step` through a real
/// `FrameworkWidgetContext` and a real `SceneEngineHosts`, not the attach functions directly.
fn paint_scene(scene: &UiComponentSceneNode, bounds: Rect, states: semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap<World3dState>) -> PaintedFrame {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut world3d_states = states;
    let mut world_resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    {
        let mut ctx = semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: "law-window" };
        let mut cursor = ui_wgpu::wgpu::ScenePaintCursor::default();
        for _ in 0..4096 {
            match semio_framework_os_renderer_wgpu_scene_render::render_component_scene_step(scene, bounds, &mut ctx, &mut cursor, &mut hosts, ui_wgpu::wgpu::UiDriverDrag::Handle, 1) {
                ui_wgpu::wgpu::ScenePaintStep::Pending => continue,
                ui_wgpu::wgpu::ScenePaintStep::Complete => break,
                ui_wgpu::wgpu::ScenePaintStep::Fault => panic!("engine surface scene paint faulted"),
            }
        }
    }
    PaintedFrame { draw, world3d_states }
}

/// 🌉️ Drives the frame transaction's own `World3dSnapshot` ladder — bridge, draw rebuild, snapshot
/// apply — exactly as `AppFrameTransactionPhase::World3dSnapshot` does, so the attached state reaches
/// the same point a live frame reaches it.
fn drive_world3d_ladder(state: &mut World3dState) {
    for turn in 0..8_192 {
        let mut faulted = false;
        let pending = with_world_step(64, |context| match step_world3d_scene_bridge(state, context) {
            World3dSceneBridgeStep::Pending => true,
            World3dSceneBridgeStep::Idle | World3dSceneBridgeStep::Complete => false,
            _ => {
                faulted = true;
                false
            }
        });
        assert!(!faulted, "the scene bridge faulted on turn {turn} ({:?})", state.snapshot_fault());
        if !pending {
            break;
        }
        assert!(turn < 8_191, "the scene bridge sealed within its turn ceiling");
    }
    for turn in 0..8_192 {
        let mut faulted = false;
        with_world_step(64, |context| {
            faulted = step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context) == WorldDrawRebuildStep::Fault;
            true
        });
        assert!(!faulted, "the retained draw rebuild faulted on turn {turn}");
        let mut done = false;
        with_world_step(64, |context| {
            done = matches!(step_world3d_snapshot(state, context), World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle);
            true
        });
        if done {
            break;
        }
        assert!(turn < 8_191, "the snapshot apply completed within its turn ceiling");
    }
    for _ in 0..8_192 {
        let mut complete = false;
        with_world_step(64, |context| {
            complete = step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context) == WorldDrawRebuildStep::Complete;
            true
        });
        if complete {
            break;
        }
    }
}

/// 🔁️ Runs the real frame loop — paint, then the frame transaction's `World3dSnapshot` ladder — until
/// the window paints a 3-D pass carrying draws. Production needs more than one frame here by design:
/// the first paint STAGES the mesh-wire bridge, the ladder seals it, and only the next paint's
/// `sync_world3d_state` picks the sealed lease up and opens the snapshot apply.
fn attach_and_settle_world3d(scene: &UiComponentSceneNode, bounds: Rect, surface_id: &str) -> PaintedFrame {
    let mut frame = paint_scene(scene, bounds, semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap::default());
    assert!(frame.world3d_states.contains_key(surface_id), "the first painted frame constructs the World3d host in the shell's own map");
    for turn in 0..16 {
        drive_world3d_ladder(frame.world3d_states.get_mut(surface_id).expect("attached world state"));
        frame = paint_scene(scene, bounds, frame.world3d_states);
        if frame.draw.scene_passes.first().is_some_and(|pass| !pass.draws.is_empty()) {
            return frame;
        }
        assert!(turn < 15, "the preview settled into a drawn frame within its frame ceiling");
    }
    frame
}

/// 🖱️ Publishes one world interaction through the PRODUCTION path the OS event loop uses — enqueue
/// the intent, then drive the bounded authority to its terminal — and collects what it wrote.
fn publish_world3d_intent(state: &mut World3dState, intent: WorldInteractionIntent) -> Vec<ActionDescriptor> {
    let mut input = InputState::<ActionDescriptor>::default();
    enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).expect("the world interaction queue admits the intent");
    for turn in 0..8_192 {
        let Some(generation) = infinite_world::world::world3d_interaction_front_generation(&state.camera, &state.authority) else { break };
        let mut faulted = false;
        with_world_step(64, |context| {
            faulted = matches!(step_world3d_interaction(state, generation, &mut input, context), WorldInteractionAuthorityStep::Fault);
            true
        });
        assert!(!faulted, "the world interaction authority faulted on turn {turn}");
        assert!(turn < 8_191, "the world interaction authority drained its queue within its turn ceiling");
    }
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}
