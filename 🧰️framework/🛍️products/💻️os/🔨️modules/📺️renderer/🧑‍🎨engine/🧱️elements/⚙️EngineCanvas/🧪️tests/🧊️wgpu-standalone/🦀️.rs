#[cfg(test)]
#[test]
fn engine_surface_registry_is_fixed_and_generation_keyed() {
    let mut registry = EngineSurfaceRegistry::default();
    let first = registry.reserve("surface-0").expect("first fixed surface slot");
    assert_eq!(registry.token("surface-0"), Some(first));
    assert!(registry.remove("surface-0").is_none());
    let replacement = registry.reserve("surface-0").expect("released fixed surface slot");
    assert_ne!(first.generation, replacement.generation);
    assert_eq!(registry.token("surface-0"), Some(replacement));
    for index in 1..ENGINE_SURFACE_CAPACITY {
        assert!(registry.reserve(&format!("surface-{index}")).is_some());
    }
    assert!(registry.reserve("surface-overflow").is_none());
    assert!(registry.faulted);
}

#[cfg(test)]
#[test]
fn engine_surface_registry_rejects_oversized_identity_before_reservation() {
    let mut registry = EngineSurfaceRegistry::default();
    assert!(registry.reserve(&"s".repeat(ENGINE_SURFACE_ID_BYTE_CAPACITY + 1)).is_none());
    assert!(registry.slots.iter().all(|slot| slot.id.is_none() && slot.value.is_none()));
}

#[cfg(test)]
#[test]
fn engine_surface_registry_accepts_exact_id_capacity_and_refuses_generation_exhaustion() {
    let mut registry = EngineSurfaceRegistry::default();
    let exact = "s".repeat(ENGINE_SURFACE_ID_BYTE_CAPACITY);
    assert!(registry.reserve(&exact).is_some());
    let mut exhausted = EngineSurfaceRegistry::default();
    exhausted.slots[0].generation = u64::MAX;
    assert!(exhausted.reserve("never-alias").is_none());
    assert!(exhausted.slots[0].exhausted);
    assert!(exhausted.slots[0].id.is_none());
}

#[cfg(test)]
#[test]
fn engine_packet_capacity_plus_one_returns_the_exact_snapshot_before_scene_transfer() {
    let Some(id) = EngineSurfaceId::try_from_str("packet-owner").ok() else {
        panic!("bounded packet identity");
    };
    let snapshot = EngineSurfaceSnapshot { identity: EngineSurfaceIdentity { token: EngineSurfaceToken { slot: 7, generation: 11 }, id }, metrics_generation: 13 };
    let mut context = EngineCanvasBuildContext::new(19, 23);
    for index in 0..(ENGINE_CANVAS_FRAME_PACKET_CAPACITY * 2) {
        let Ok(reservation) = context.try_reserve_fresh_packet(snapshot) else {
            panic!("fixed packet and rejection authorities admit their declared capacity");
        };
        assert_eq!(reservation.sequence, (index + 1) as u64);
        if index < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            assert_eq!(reservation.destination, EngineCanvasPacketDestination::Ready(index));
        } else {
            assert_eq!(reservation.destination, EngineCanvasPacketDestination::Rejected(index - ENGINE_CANVAS_FRAME_PACKET_CAPACITY));
        }
    }
    assert_eq!(context.try_reserve_fresh_packet(snapshot), Err(snapshot));
    assert_eq!(context.len, ENGINE_CANVAS_FRAME_PACKET_CAPACITY);
    assert_eq!(context.rejected_len, ENGINE_CANVAS_FRAME_PACKET_CAPACITY);
}

#[cfg(test)]
#[test]
fn engine_packet_close_retains_opaque_scene_until_exact_terminal_release() {
    let id = EngineSurfaceId::try_from_str("packet-retirement").expect("bounded packet identity");
    let snapshot = EngineSurfaceSnapshot { identity: EngineSurfaceIdentity { token: EngineSurfaceToken { slot: 5, generation: 7 }, id }, metrics_generation: 11 };
    let mut context = EngineCanvasBuildContext::new(13, 17);
    let reservation = context.try_reserve_fresh_packet(snapshot).expect("fixed packet retirement authority");
    let mut scene = canvas::Scene::new();
    for _ in 0..128 {
        scene.pop_layer();
    }
    context.publish_reserved(reservation, scene, Color::new([0.0, 0.0, 0.0, 1.0]), 640, 480);
    let mut packet = context.take_packet_step().unwrap_or_else(|_| panic!("ready packet destination")).expect("published packet");
    assert!(!close_engine_canvas_packet_step(&mut packet));
    let mut turns = 1usize;
    while !close_engine_canvas_packet_step(&mut packet) {
        turns += 1;
        assert!(turns < 512, "packet-retained scene reaches exact terminal release");
    }
    assert!(turns > 128);
    assert!(packet.terminal_is_empty());
}

#[cfg(test)]
#[test]
fn gpu_publish_freshness_uses_the_live_cpu_identity_metrics_document_and_scene() {
    let Some(id) = EngineSurfaceId::try_from_str("freshness-owner").ok() else {
        panic!("bounded freshness identity");
    };
    let identity = EngineSurfaceIdentity { token: EngineSurfaceToken { slot: 3, generation: 5 }, id };
    let live = EngineSurfaceLiveFreshness { identity, metrics_generation: 13, document_generation: 7, scene_revision: 11 };
    assert!(engine_gpu_freshness_matches(identity, 7, 11, 13, live));
    assert!(!engine_gpu_freshness_matches(identity, 7, 11, 13, EngineSurfaceLiveFreshness { metrics_generation: 14, ..live }));
    assert!(!engine_gpu_freshness_matches(identity, 7, 11, 13, EngineSurfaceLiveFreshness { document_generation: 8, ..live }));
    assert!(!engine_gpu_freshness_matches(identity, 7, 11, 13, EngineSurfaceLiveFreshness { scene_revision: 12, ..live }));
}

#[cfg(test)]
fn raster_lifecycle_packet(scene_revision: u64, metrics_generation: u64) -> EngineCanvasPacket {
    let id = EngineSurfaceId::try_from_str("raster-lifecycle").expect("bounded raster lifecycle identity");
    EngineCanvasPacket {
        surface: EngineSurfaceIdentity { token: EngineSurfaceToken { slot: 0, generation: 7 }, id },
        document_generation: 5,
        scene_revision,
        metrics_generation,
        scene: ManuallyDrop::new(canvas::Scene::new()),
        scene_retirement: ManuallyDrop::new(canvas::CanvasSceneRetirement::default()),
        clear: Color::new([0.0, 0.0, 0.0, 1.0]),
        width: 64,
        height: 48,
    }
}

#[cfg(test)]
#[test]
fn engine_packet_retirement_is_the_scene_cursor_and_terminal_only_after_the_identity_closes() {
    let mut packet = raster_lifecycle_packet(3, 5);
    assert!(!packet.terminal_is_empty());
    let open = packet.next_close_demands(4096).expect("open identity demands one retirement step");
    assert_eq!(open.depth, 1);
    packet.surface.id.close_step();
    assert!(packet.terminal_is_empty());
    let settled = packet.next_close_demands(4096).expect("terminal packet demands nothing");
    assert_eq!((settled.depth, settled.copy_bytes, settled.release_bytes), (0, 0, 0));
}

#[cfg(test)]
#[test]
fn gpu_candidate_executes_reuse_publication_changed_tuple_rejection_and_metrics_invalidation() {
    let expected = ui_gpu::RasterTextureWitness { scene_revision: 5, preview_generation: 7, operation: 11 };
    let packet = raster_lifecycle_packet(9, 13);
    let mut fresh_slot = EngineGpuSlot::new();
    fresh_slot.id = Some(packet.surface.id);
    fresh_slot.generation = packet.surface.token.generation;
    fresh_slot.candidate = Some(EngineGpuCandidate::new(&packet, expected, 17));
    fresh_slot.candidate.as_mut().expect("fresh candidate").phase = EngineGpuBuildPhase::Publish;
    assert_eq!(fresh_slot.publish_candidate(&packet, expected, 17), Ok(false));
    assert!(fresh_slot.candidate.as_ref().is_some_and(EngineGpuCandidate::terminal_is_empty));

    let changed = raster_lifecycle_packet(10, 13);
    let mut changed_slot = EngineGpuSlot::new();
    changed_slot.id = Some(packet.surface.id);
    changed_slot.generation = packet.surface.token.generation;
    changed_slot.candidate = Some(EngineGpuCandidate::new(&packet, expected, 17));
    changed_slot.candidate.as_mut().expect("changed candidate").phase = EngineGpuBuildPhase::Publish;
    assert_eq!(changed_slot.publish_candidate(&changed, expected, 17).expect_err("changed tuple rejects publication"), "engine CPU/GPU publication freshness changed");
    assert_eq!(changed_slot.candidate.as_ref().map(|candidate| candidate.phase), Some(EngineGpuBuildPhase::ClosingAdmission));

    let mut presenter = EngineCanvasPresenter::default();
    {
        let slot = &mut presenter.slots_mut().expect("presenter slots")[0];
        slot.id = Some(packet.surface.id);
        slot.generation = packet.surface.token.generation;
        slot.candidate = Some(EngineGpuCandidate::new(&packet, expected, 17));
        slot.candidate.as_mut().expect("metrics candidate").phase = EngineGpuBuildPhase::Publish;
    }
    assert!(presenter.observe_primary_metrics_generation(18));
    assert!(!presenter.invalidate_primary_metrics_step());
    assert_eq!(presenter.slots().expect("presenter slots")[0].candidate.as_ref().map(|candidate| candidate.phase), Some(EngineGpuBuildPhase::ClosingAdmission));
    let mut completed = None;
    for turn in 0..=ENGINE_SURFACE_CAPACITY {
        if presenter.invalidate_primary_metrics_step() {
            completed = Some(turn);
            break;
        }
    }
    assert_eq!(completed, Some(ENGINE_SURFACE_CAPACITY - 1));
    let slot = &mut presenter.slots_mut().expect("presenter slots")[0];
    let candidate = slot.candidate.take().expect("invalidated candidate");
    assert_eq!(candidate.phase, EngineGpuBuildPhase::ClosingAdmission);
    slot.id = None;
    assert!(presenter.terminal_is_empty());
}

#[cfg(test)]
#[test]
fn rendered_target_transfers_to_the_table_without_an_unused_second_target() {
    let source = semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
    let start = source.find("pub(crate) fn realize_step").unwrap_or(0);
    let end = source[start..].find("pub(crate) fn close_active_candidate_step").map(|offset| start + offset).unwrap_or(source.len());
    let mounted = &source[start..end];
    assert!(mounted.contains("gpu.engine_raster_step(owner,&packet.scene,packet.clear,expected)"));
    assert!(mounted.contains("build.phase=owner.phase();build.gpu=Some(owner);"));
    assert!(mounted.contains("EngineRasterStageFault::Returned{fault,build:owner}"));
    assert!(mounted.contains("EngineRasterStageFault::Retained{fault,build:owner}"));
    assert_eq!(mounted.matches("gpu.begin_engine_raster(").count(), 1);
    assert!(!source.contains("gpu.device()"));
    assert!(!source.contains("gpu.queue()"));
    assert!(!source.contains("replacement_texture"));
    assert!(!source.contains("EngineGpuSurface"));
}

#[cfg(test)]
#[test]
fn child_and_outer_surface_retirements_require_explicit_field_witnesses() {
    let source = semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
    let start = source.find("struct EngineSurfaceRetirement").unwrap_or(0);
    let end = source[start..].find("//#region 📦️PreparedEngineCanvas").map(|offset| start + offset).unwrap_or(source.len());
    let retirement = &source[start..end];
    assert!(retirement.contains("let EngineSurface {"));
    assert!(retirement.contains("board_pending_events.terminal_is_empty()"));
    assert!(!retirement.contains("ManuallyDrop::drop(&mut self.surface)"));
}

/// 🎬️ Drives ONE board surface's retained pointer authority to its terminal and publishes what it
/// produced — the `AppFrameTransactionPhase::BoardAuthority` ladder
/// (`🧊️renderer/🦀️.rs:12233`), reduced to the single surface a law exercises.
///
/// 🩸️ `board2d_pointer_up_into` does NOT finish a gesture: every non-`Idle` plan
/// `requires_retained_commit`, so the handler only ARMS `begin_pointer_commit` and answers whether
/// the plan emits. The gesture ends when this authority reaches `Complete`, which is why a law that
/// read `defers_descriptor_sync_from_js()` straight after the handler saw a still-live gesture.
#[cfg(test)]
fn settle_board_pointer_authority(surface_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    for _ in 0..4_096 {
        match with_engine_close_context(1, |context| drive_board_authority_step(surface_id, context)) {
            infinite_canvas::BoardAuthorityStep::Pending => {}
            infinite_canvas::BoardAuthorityStep::Complete => return publish_board_pointer_step(surface_id, input).is_ok(),
            infinite_canvas::BoardAuthorityStep::Cancelled | infinite_canvas::BoardAuthorityStep::Fault => return false,
        }
    }
    false
}

#[cfg(test)]
#[test]
fn board_surface_close_freezes_registration_and_reaches_nonopaque_terminal() {
    let mut registry = EngineSurfaceRegistry::default();
    let token = registry.reserve("board-close").expect("fixed surface reservation");
    let mut surface = empty_engine_surface(800, 600);
    surface.board_host = Some(ManuallyDrop::new(infinite_canvas::BoardHost::default()));
    assert!(registry.publish_reserved(token, surface).is_ok());
    assert!(registry.begin_close(token));
    assert!(registry.get("board-close").is_none());
    assert!(registry.reserve("board-close").is_none());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| registry.close_step(token, context, &mut input)) {
        turns += 1;
        assert!(turns < 8_192, "board surface close reaches a fixed terminal witness");
    }
    assert!(registry.terminal_nonopaque_is_empty(token));
}

#[cfg(test)]
fn drive_engine_surface_close(registry: &mut EngineSurfaceRegistry, token: EngineSurfaceToken, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> usize {
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| registry.close_step(token, context, input)) {
        turns += 1;
        assert!(turns < 262_144, "populated engine surface close reaches a fixed terminal witness");
    }
    turns
}

#[cfg(test)]
#[test]
/// ♻️ The ladder closes one fuel turn at a time AND page by page: its 8 KiB buffers cost a grant each (not 8 192), so the whole
/// populated surface takes more than one turn per owner and far fewer than one per scalar.
fn populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time() {
    let mut registry = EngineSurfaceRegistry::default();
    let token = registry.reserve("all-cpu-owners").expect("fixed surface reservation");
    let mut surface = empty_engine_surface(800, 600);
    surface.node_graph = Some(NodeGraphEngine::Dag(GraphHost::default()));
    surface.sync_cache.snapshot_json = Some("x".repeat(8_192));
    surface.sync_cache.selection = Some(vec!["selected".repeat(512)]);
    surface.sync_cache.scene_pack = Some(vec![7; 8_192]);
    surface.map_host = Some(MapHost::new());
    surface.map_sync_cache.map_descriptor_json = Some("m".repeat(8_192));
    surface.editor = Some(EditorHost::default());
    surface.editor_scene_pack = Some(vec![11; 8_192]);
    assert!(registry.publish_reserved(token, surface).is_ok());
    assert!(registry.begin_close(token));
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert!(!with_engine_close_context(0, |context| registry.close_step(token, context, &mut input)));
    let turns = drive_engine_surface_close(&mut registry, token, &mut input);
    assert!((16..8_192).contains(&turns), "a populated surface closes owner by owner, a page per grant: {turns}");
    assert!(registry.terminal_nonopaque_is_empty(token));
    assert!(!registry.begin_close(token));
}

#[cfg(test)]
#[test]
fn populated_flow_surface_closes_history_and_cache_before_slot_reuse() {
    let mut registry = EngineSurfaceRegistry::default();
    let token = registry.reserve("flow-cpu-owner").expect("fixed surface reservation");
    let mut surface = empty_engine_surface(640, 480);
    surface.node_graph = Some(NodeGraphEngine::Flow(FlowHost::default()));
    assert!(registry.publish_reserved(token, surface).is_ok());
    assert!(registry.begin_close(token));
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert!(drive_engine_surface_close(&mut registry, token, &mut input) > 1);
    assert!(registry.terminal_nonopaque_is_empty(token));
    let next = registry.reserve("flow-cpu-owner").expect("terminal slot is reusable");
    assert_ne!(token.generation, next.generation);
}

include!("../🧵️wgpu-standalone-wrappers/🦀️.rs");

#[cfg(test)]
#[test]
fn saturated_map_action_queue_preserves_host_revision_and_camera() {
    let surface_id = "map-plan-saturation";
    ensure_engine_surface(surface_id, surface_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(surface_id).unwrap().map_host = Some(MapHost::new()));
    let before = with_map_host(surface_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    let mut input = semio_framework_ui_wgpu_input::InputState::default();
    for _ in 0..semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY {
        input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    }
    assert_eq!(tiled_map_wheel_into(surface_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, false, &mut input), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits));
    let after = with_map_host(surface_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    assert_eq!(after, before);
    ENGINE_SURFACES.with(|cell| {
        cell.borrow_mut().remove(surface_id);
    });
}

#[cfg(test)]
#[test]
fn saturated_graph_and_board_wheel_queues_preserve_cameras() {
    fn saturate(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, remaining_items: usize) {
        for _ in 0..semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY.saturating_sub(remaining_items) {
            input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
        }
    }

    let graph_id = "graph-plan-saturation";
    ensure_engine_surface(graph_id, graph_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(graph_id).unwrap().node_graph = Some(NodeGraphEngine::Dag(GraphHost::default())));
    let graph_before = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom]
    });
    let mut graph_input = semio_framework_ui_wgpu_input::InputState::default();
    saturate(&mut graph_input, 0);
    assert_eq!(node_graph_wheel_into(graph_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, false, &mut graph_input), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits));
    let graph_after = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom]
    });
    assert_eq!(graph_after, graph_before);
    let graph_selection_before = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        host.selected_node_ids_json()
    });
    assert_eq!(node_graph_pointer_down_into(graph_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, 0, false, false, false, false, &mut graph_input), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits));
    let graph_selection_after = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        host.selected_node_ids_json()
    });
    assert_eq!(graph_selection_after, graph_selection_before);

    let board_id = "board-plan-saturation";
    ensure_engine_surface(board_id, board_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(board_id).unwrap().board_host = Some(ManuallyDrop::new(infinite_canvas::BoardHost::default())));
    let board_before = with_board_host(board_id, |host| [host.scene.camera.x, host.scene.camera.y, host.scene.camera.zoom]).unwrap();
    let mut board_input = semio_framework_ui_wgpu_input::InputState::default();
    saturate(&mut board_input, 1);
    assert_eq!(board2d_wheel_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, &mut board_input), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits));
    let board_after = with_board_host(board_id, |host| [host.scene.camera.x, host.scene.camera.y, host.scene.camera.zoom]).unwrap();
    assert_eq!(board_after, board_before);
    board2d_pointer_down_into(board_id, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 100.0, 100.0, 1, false, false, &mut board_input).expect("an idle board takes the press");
    board_input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    assert_eq!(board2d_pointer_up_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 140.0, 130.0, false, false, false, &mut board_input), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits));
    assert!(with_board_host(board_id, |host| defers_descriptor_sync_from_js(&host.focus, &host.gestures)).unwrap());
    let mut retry = semio_framework_ui_wgpu_input::InputState::default();
    assert_eq!(board2d_pointer_up_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 140.0, 130.0, false, false, false, &mut retry), Ok(true));
    assert!(settle_board_pointer_authority(board_id, &mut retry), "the retried pointer-up's retained commit reaches its terminal");
    assert!(!with_board_host(board_id, |host| defers_descriptor_sync_from_js(&host.focus, &host.gestures)).unwrap());
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        map.remove(graph_id);
        map.remove(board_id);
    });
}
