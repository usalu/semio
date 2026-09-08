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
    assert!(!packet.close_step());
    let mut turns = 1usize;
    while !packet.close_step() {
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
#[test]
fn normal_replacement_drains_displaced_renderer_view_texture_before_next_candidate() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let start = source.find("pub(crate) fn realize_step").unwrap_or(0);
    let end = source[start..].find("pub(crate) fn close_active_candidate_step").map(|offset| start + offset).unwrap_or(source.len());
    let mounted = &source[start..end];
    assert!(mounted.contains("if let Some(retirement) = slot.retirement.as_mut()"));
    assert!(mounted.contains("retirement.close_step() && retirement.terminal_is_empty()"));
    assert!(source.contains("self.retirement = Some(EngineGpuRetirement::new(displaced))"));
}

#[cfg(test)]
#[test]
fn child_and_outer_surface_retirements_require_explicit_field_witnesses() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let start = source.find("struct EngineSurfaceRetirement").unwrap_or(0);
    let end = source[start..].find("//#region 📦️PreparedEngineCanvas").map(|offset| start + offset).unwrap_or(source.len());
    let retirement = &source[start..end];
    assert!(retirement.contains("let EngineSurface {"));
    assert!(retirement.contains("board_pending_events.terminal_is_empty()"));
    assert!(!retirement.contains("ManuallyDrop::drop(&mut self.surface)"));
}

#[cfg(test)]
fn with_engine_close_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
    );
    step(&mut context)
}

#[cfg(test)]
#[test]
fn board_surface_close_freezes_registration_and_reaches_nonopaque_terminal() {
    let mut registry = EngineSurfaceRegistry::default();
    let token = registry.reserve("board-close").expect("fixed surface reservation");
    let mut surface = empty_engine_surface(800, 600);
    surface.board_host = Some(ManuallyDrop::new(puzzle::editor::puzzle2d::engine::BoardHost::default()));
    assert!(registry.publish_reserved(token, surface).is_ok());
    assert!(registry.begin_close(token));
    assert!(registry.get("board-close").is_none());
    assert!(registry.reserve("board-close").is_none());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| registry.close_step(token, context, &mut input)) {
        turns += 1;
        assert!(turns < 8_192, "board surface close reaches a fixed terminal witness");
    }
    assert!(registry.terminal_nonopaque_is_empty(token));
}

#[cfg(test)]
fn drive_engine_surface_close(registry: &mut EngineSurfaceRegistry, token: EngineSurfaceToken, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> usize {
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| registry.close_step(token, context, input)) {
        turns += 1;
        assert!(turns < 262_144, "populated engine surface close reaches a fixed terminal witness");
    }
    turns
}

#[cfg(test)]
#[test]
fn populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time() {
    let mut registry = EngineSurfaceRegistry::default();
    let token = registry.reserve("all-cpu-owners").expect("fixed surface reservation");
    let mut surface = empty_engine_surface(800, 600);
    surface.node_graph = Some(NodeGraphEngine::Dag(GraphHost::default()));
    surface.sync_cache.fixture_json = Some("x".repeat(8_192));
    surface.sync_cache.selection = Some(vec!["selected".repeat(512)]);
    surface.sync_cache.scene_pack = Some(vec![7; 8_192]);
    surface.map_host = Some(MapHost::new());
    surface.map_sync_cache.map_fixture_json = Some("m".repeat(8_192));
    surface.editor = Some(EditorHost::default());
    surface.editor_scene_pack = Some(vec![11; 8_192]);
    assert!(registry.publish_reserved(token, surface).is_ok());
    assert!(registry.begin_close(token));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!with_engine_close_context(0, |context| registry.close_step(token, context, &mut input)));
    let turns = drive_engine_surface_close(&mut registry, token, &mut input);
    assert!(turns > 24_576);
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
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(drive_engine_surface_close(&mut registry, token, &mut input) > 1);
    assert!(registry.terminal_nonopaque_is_empty(token));
    let next = registry.reserve("flow-cpu-owner").expect("terminal slot is reusable");
    assert_ne!(token.generation, next.generation);
}

#[cfg(test)]
fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

#[cfg(test)]
fn ensure_surface(surface_id: &str, pw: u32, ph: u32) -> Option<EngineSurfaceSnapshot> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let needs_create = !map.contains_key(surface_id);
        let needs_resize = map.get(surface_id).is_some_and(|entry| entry.width != pw.max(1) || entry.height != ph.max(1));
        if needs_create {
            let Some(token) = map.reserve(surface_id) else {
                return None;
            };
            let surface = empty_engine_surface(pw, ph);
            if map.publish_reserved(token, surface).is_err() {
                map.faulted = true;
                return None;
            }
        }
        if needs_resize {
            let Some(entry) = map.get_mut(surface_id) else {
                map.faulted = true;
                return None;
            };
            let Some(metrics_generation) = entry.metrics_generation.checked_add(1) else {
                map.faulted = true;
                return None;
            };
            entry.width = pw.max(1);
            entry.height = ph.max(1);
            entry.metrics_generation = metrics_generation;
        }
        let identity = map.identity(surface_id)?;
        let metrics_generation = map.get(surface_id)?.metrics_generation;
        Some(EngineSurfaceSnapshot { identity, metrics_generation })
    })
}

#[cfg(test)]
pub fn node_graph_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_wheel_into(surface_id, controller_id, inner, x, y, delta, ctrl, &mut input);
    crate::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_down(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl: bool, alt: bool, space_pressed: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_down_into(surface_id, controller_id, inner, x, y, button, shift, ctrl, alt, space_pressed, &mut input);
    crate::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_move_into(surface_id, controller_id, inner, x, y, shift, ctrl, alt, &mut input);
    crate::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_up_into(surface_id, controller_id, inner, x, y, shift, ctrl, alt, &mut input);
    crate::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn map_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

#[cfg(test)]
pub fn map_interaction_actions(surface_id: &str, controller_id: &str, host: &MapHost) -> Vec<ActionDescriptor> {
    let selection = json!({
        "positions": host.selected_positions_json(),
        "routes": host.selected_routes_json(),
    });
    let hover = if let (Some(kind), Some(id)) = (host.hovered_kind(), host.hovered_id()) { json!({ "kind": kind, "id": id }) } else { Value::Null };
    vec![
        map_action(controller_id, ui_wgpu::wgpu::tiled_map_actions::SET_CAMERA, json!({ "surfaceId": surface_id, "camera": serde_json::from_str::<Value>(&host.camera_json()).unwrap_or(json!({})) })),
        map_action(controller_id, ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION, json!({ "surfaceId": surface_id, "positions": selection["positions"], "routes": selection["routes"] })),
        map_action(controller_id, ui_wgpu::wgpu::tiled_map_actions::SET_HOVER, json!({ "surfaceId": surface_id, "hover": hover })),
    ]
}

#[cfg(test)]
pub fn tiled_map_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = tiled_map_wheel_into(surface_id, controller_id, inner, x, y, delta, ctrl, &mut input);
    crate::collect_fixture_actions(&mut input)
}

#[cfg(test)]
#[test]
fn saturated_map_action_queue_preserves_host_revision_and_camera() {
    let surface_id = "map-plan-saturation";
    ensure_surface(surface_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(surface_id).unwrap().map_host = Some(MapHost::new()));
    let before = with_map_host(surface_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    let mut input = ui_wgpu::wgpu::InputState::default();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY - 2 {
        input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    }
    assert_eq!(tiled_map_wheel_into(surface_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, false, &mut input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits));
    let after = with_map_host(surface_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    assert_eq!(after, before);
    ENGINE_SURFACES.with(|cell| {
        cell.borrow_mut().remove(surface_id);
    });
}

#[cfg(test)]
#[test]
fn saturated_graph_and_board_wheel_queues_preserve_cameras() {
    fn saturate(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
        for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY - 1 {
            input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
        }
    }

    let graph_id = "graph-plan-saturation";
    ensure_surface(graph_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(graph_id).unwrap().node_graph = Some(NodeGraphEngine::Dag(GraphHost::default())));
    let graph_before = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        [host.dag.fixture.camera.x, host.dag.fixture.camera.y, host.dag.fixture.camera.zoom]
    });
    let mut graph_input = ui_wgpu::wgpu::InputState::default();
    saturate(&mut graph_input);
    assert_eq!(node_graph_wheel_into(graph_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, false, &mut graph_input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits));
    let graph_after = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        [host.dag.fixture.camera.x, host.dag.fixture.camera.y, host.dag.fixture.camera.zoom]
    });
    assert_eq!(graph_after, graph_before);
    let graph_selection_before = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        host.selected_node_ids_json()
    });
    assert_eq!(node_graph_pointer_down_into(graph_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, 0, false, false, false, false, &mut graph_input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits));
    let graph_selection_after = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Dag(host)) = map.get(graph_id).unwrap().node_graph.as_ref() else { unreachable!() };
        host.selected_node_ids_json()
    });
    assert_eq!(graph_selection_after, graph_selection_before);

    let board_id = "board-plan-saturation";
    ensure_surface(board_id, 800, 600);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(board_id).unwrap().board_host = Some(ManuallyDrop::new(puzzle::editor::puzzle2d::engine::BoardHost::default())));
    let board_before = with_board_host(board_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    let mut board_input = ui_wgpu::wgpu::InputState::default();
    saturate(&mut board_input);
    assert_eq!(puzzle_board_wheel_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 200.0, 200.0, -12.0, &mut board_input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits));
    let board_after = with_board_host(board_id, |host| [host.camera.x, host.camera.y, host.camera.zoom]).unwrap();
    assert_eq!(board_after, board_before);
    puzzle_board_pointer_down(board_id, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 100.0, 100.0, 1, false, false);
    board_input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    assert_eq!(puzzle_board_pointer_up_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 140.0, 130.0, false, false, false, &mut board_input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits));
    assert!(with_board_host(board_id, |host| host.defers_descriptor_sync_from_js()).unwrap());
    let mut retry = ui_wgpu::wgpu::InputState::default();
    assert_eq!(puzzle_board_pointer_up_into(board_id, "controller", Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 }, 140.0, 130.0, false, false, false, &mut retry), Ok(true));
    assert!(!with_board_host(board_id, |host| host.defers_descriptor_sync_from_js()).unwrap());
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        map.remove(graph_id);
        map.remove(board_id);
    });
}

#[cfg(test)]
pub fn coalesce_board2d_events(rows: &[BoardEventRow]) -> CoalescedBoardEvents {
    let has_drag_end = rows.iter().any(|row| row.name == "nodeDragEnd");
    let mut flush_now = false;
    let mut last_camera: Option<BoardEventRow> = None;
    let mut node_move_order: Vec<String> = Vec::new();
    let mut node_move_by_id: HashMap<String, BoardEventRow> = HashMap::new();
    let mut rest: Vec<BoardEventRow> = Vec::new();

    for row in rows {
        if PUZZLE2D_TRANSIENT_EVENT_NAMES.contains(&row.name.as_str()) {
            continue;
        }
        if row.name == "camera" {
            last_camera = Some(row.clone());
            continue;
        }
        if row.name == "nodeMove" {
            if has_drag_end {
                continue;
            }
            if let Some(id) = row.payload.get("id").and_then(Value::as_str) {
                if !node_move_by_id.contains_key(id) {
                    node_move_order.push(id.to_string());
                }
                node_move_by_id.insert(id.to_string(), row.clone());
                continue;
            }
        }
        if PUZZLE2D_FLUSH_NOW_EVENT_NAMES.contains(&row.name.as_str()) {
            flush_now = true;
        }
        rest.push(row.clone());
    }

    let mut coalesced: Vec<BoardEventRow> = Vec::new();
    if let Some(camera) = last_camera {
        coalesced.push(camera);
    }
    for id in &node_move_order {
        if let Some(row) = node_move_by_id.get(id) {
            coalesced.push(row.clone());
        }
    }
    coalesced.extend(rest);
    CoalescedBoardEvents { flush_now, events_json: serde_json::to_string(&coalesced).unwrap_or_else(|_| "[]".into()) }
}

#[cfg(test)]
pub fn board_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

#[cfg(test)]
fn board_take_buffer_coalesced(surface_id: &str) -> Option<String> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        if entry.board_retiring_events.is_some() {
            return None;
        }
        let coalesced = coalesce_owned_board_events(&entry.board_pending_events).ok()?;
        let pending = std::mem::take(&mut entry.board_pending_events);
        entry.board_retiring_events = Some(pending);
        (coalesced.events_json != "[]").then_some(coalesced.events_json)
    })
}

#[cfg(test)]
fn board_flush_events_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    while board_drain_into_buffer(surface_id) {}
    let events_json = board_take_buffer_coalesced(surface_id)?;
    Some(board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json })))
}

#[cfg(test)]
fn board_drain_and_maybe_flush(surface_id: &str, controller_id: &str) -> Vec<ActionDescriptor> {
    while board_drain_into_buffer(surface_id) {}
    let flush_now = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| coalesce_owned_board_events(&entry.board_pending_events).ok()).is_some_and(|events| events.flush_now));
    if !flush_now {
        return Vec::new();
    }
    match board_take_buffer_coalesced(surface_id) {
        Some(events_json) => vec![board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json }))],
        None => Vec::new(),
    }
}

#[cfg(test)]
fn board_camera_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    with_board_host(surface_id, |host| board_action(controller_id, "setCamera", json!({ "camera": { "x": host.camera.x, "y": host.camera.y, "zoom": host.camera.zoom } })))
}

#[cfg(test)]
pub fn puzzle_board_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_set_pointer_inside(surface_id, true);
    board_drain_and_maybe_flush(surface_id, controller_id)
}

#[cfg(test)]
pub fn puzzle_board_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

#[cfg(test)]
pub fn puzzle_board_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    let was_inside = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let was = entry.board_pointer_inside;
        entry.board_pointer_inside = false;
        was
    });
    if !was_inside {
        return Vec::new();
    }
    with_board_host_mut(surface_id, |host| host.pointer_leave_screen(alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

#[cfg(test)]
pub fn puzzle_board_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.wheel_screen(sx, sy, delta as f64));
    let mut actions = Vec::new();
    if let Some(camera_action) = board_camera_action(surface_id, controller_id) {
        actions.push(camera_action);
    }
    if let Some(events_action) = board_flush_events_action(surface_id, controller_id) {
        actions.push(events_action);
    }
    actions
}

#[cfg(test)]
pub fn text_editor_apply_key(scene: &UiComponentSceneNode, key: KeyAction, modifiers: &PointerModifiers) -> Vec<ActionDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        match key {
            KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl) => {
                host.insert_text(&ch.to_string());
            }
            KeyAction::Backspace => host.backspace(),
            KeyAction::Delete => host.delete_forward(),
            KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a") => {
                host.select_all();
            }
            _ => return Vec::new(),
        }
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
pub fn text_editor_wheel(scene: &UiComponentSceneNode, delta: f32) -> Vec<ActionDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.wheel_scroll_screen(delta as f64);
        Vec::new()
    })
}

#[cfg(test)]
pub fn text_editor_pointer_down(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16) -> Vec<ActionDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_down_screen(sx, sy, button as i32);
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
pub fn text_editor_pointer_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_move_screen(sx, sy, 0);
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
pub fn text_editor_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_up_screen(sx, sy, 0);
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
fn text_editor_interaction_actions(scene: &UiComponentSceneNode, host: &EditorHost) -> Vec<ActionDescriptor> {
    vec![
        scene_action(
            scene,
            "textSelect",
            json!({
                "surfaceId": scene.surface_id,
                "selectionJson": json!({ "start": host.anchor(), "end": host.caret() }).to_string(),
            }),
        ),
        scene_action(scene, "textEdit", json!({ "surfaceId": scene.surface_id, "document": host.text() })),
    ]
}

#[cfg(test)]
pub fn text_editor_select_span_at_screen(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.select_span_at_screen(sx, sy);
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
pub fn text_editor_set_selection(scene: &UiComponentSceneNode, anchor: usize, caret: usize) -> Vec<ActionDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.set_selection_range(anchor, caret);
        text_editor_interaction_actions(scene, host)
    })
}

#[cfg(test)]
pub fn text_editor_apply_completion(scene: &UiComponentSceneNode, prefix_start: usize, caret: usize, insert_text: &str) -> Vec<ActionDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.set_selection_range(prefix_start, caret);
        host.replace_selection(insert_text);
        text_editor_interaction_actions(scene, host)
    })
}
