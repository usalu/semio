/// 🗺️ Tiled map and board attach laws: the production seam attach, the board trace lane, the heap first slot tables, the visible tile reservation and the cancel separation.

#[test]
fn tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam() {
    let fixture = fixture();
    for surface_id in ["tiled-map-attach", "board2d-attach"] {
        drop_engine_surface(surface_id);
    }
    let map_scene = tiled_map_scene("tiled-map-attach", &fixture);
    let board_scene = board2d_scene("board2d-attach", &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };

    let map_frame = paint_retained_surface_scene(&map_scene, bounds);
    let map_host_id = map_frame.owner.host_id.clone();
    let map_draw = map_frame.draw;
    let overlay = map_overlay_fixture();
    let map_expect = &overlay["expect"];
    let granularity = map_expect["selectedGranularity"].as_str().expect("granularity");
    let (pins, paths, selected, hovered) = with_map_host(&map_host_id, |host| (host.overlays.features("pins").count(), host.overlays.features("paths").count(), host.selected_ids(granularity).map(str::to_owned).collect::<Vec<_>>(), host.hovered_id().map(str::to_owned)))
        .expect("the first painted frame constructs the map engine");
    assert_eq!(pins, map_expect["pins"].as_u64().expect("pins") as usize, "the descriptor reaches the host whole");
    assert_eq!(paths, map_expect["paths"].as_u64().expect("paths") as usize);
    assert_eq!(selected, map_expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_owned()).collect::<Vec<_>>(), "React's resolveMapInteractionSync granularity rule, reproduced");
    assert_eq!(hovered.as_deref(), map_expect["hoveredId"].as_str());
    let map_key = engine_raster_key(&map_host_id).expect("bounded engine raster key");
    assert!(map_draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).any(|(key, _)| key == &map_key), "the painted map is composited into the window draw list under {map_key}");

    let board_frame = paint_retained_surface_scene(&board_scene, bounds);
    let board_host_id = board_frame.owner.host_id.clone();
    let board_draw = board_frame.draw;
    let board_expect = &fixture["board2d"]["expect"];
    let (nodes, edges, board_selection) = with_board_host(&board_host_id, |host| (host.nodes.len(), host.edges.len(), host.selection.iter().cloned().collect::<Vec<String>>())).expect("the first painted frame constructs the board engine");
    assert_eq!(nodes, board_expect["nodes"].as_u64().expect("nodes") as usize, "the fixture reaches the board host whole");
    assert_eq!(edges, board_expect["edges"].as_u64().expect("edges") as usize);
    assert_eq!(
        board_selection,
        board_expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_owned()).collect::<Vec<_>>(),
        "load_board_snapshot_json resets selection, so the sync re-applies it silently right after — React's applySnapshotToSession rule"
    );
    let board_key = engine_raster_key(&board_host_id).expect("bounded engine raster key");
    assert!(board_draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).any(|(key, _)| key == &board_key), "the painted board is composited into the window draw list under {board_key}");

    let registrations = take_engine_surface_registrations();
    let kinds: Vec<&EngineSurfaceKindDetail> = registrations.iter().map(|entry| &entry.detail).collect();
    assert!(kinds.iter().any(|detail| matches!(detail, EngineSurfaceKindDetail::TiledMap { .. })), "the map surface is recorded on the shared registration list the shell mirrors");
    assert!(kinds.iter().any(|detail| matches!(detail, EngineSurfaceKindDetail::Board2d { .. })), "the board surface is recorded on the shared registration list the shell mirrors");

    for window_id in ["tiled-map-attach", "board2d-attach"] {
        close_retained_surface_fixture(window_id);
    }
}

/// 🎲️ The board-2d lane contract whose `traceShapes.placementLane` every board host paints.
const BOARD2D_SCENE_LANES: &str = include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json");

/// ⚖️ LAW: a board window's `toolRunTrace` lane reaches its engine surface on the production paint seam — the resident
/// layer holds the lane's records, every placement draws against a footprint of its kind catalogs — and the surface
/// echoes the cursor for the window it paints into.
#[test]
fn a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor() {
    let fixture = fixture();
    let contract: Value = serde_json::from_str(BOARD2D_SCENE_LANES).expect("board-2d lane contract parses");
    let shapes = &contract["traceShapes"];
    let expected = &shapes["placementLaneRecords"];
    let surface_id = "board2d-trace";
    drop_engine_surface(surface_id);
    let mut scene = board2d_scene(surface_id, &fixture);
    let board = scene.board2d.as_mut().expect("board scene");
    board.glyph_catalogs_json = shapes["glyphCatalogsJson"].as_str().expect("catalogs").to_string();
    board.tool_run_trace = Some(shapes["placementLane"].as_str().expect("placement lane").to_string());
    let frame = paint_retained_surface_scene(&scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    let host_id = frame.owner.host_id.clone();
    let (records, footprints, draws) = board2d_tool_run_trace_state(&host_id).expect("the painted board holds its trace layer");
    println!("[STATS] board2d trace records={records} footprints={footprints} draws={draws}");
    assert_eq!(footprints, shapes["shapes"].as_array().expect("shapes").len(), "the footprints follow the scene's kind catalogs");
    assert_eq!(draws, expected["placements"].as_u64().expect("placements") as usize, "every placement record paints");
    let cursor = board2d_tool_run_trace_cursors().get(surface_id).copied().expect("the board echoes its cursor for the window it paints into");
    assert_eq!((cursor.run, u64::from(cursor.generation), u64::from(cursor.page)), (expected["run"].as_u64().expect("run"), expected["generation"].as_u64().expect("generation"), expected["page"].as_u64().expect("page")));
    let _ = take_engine_surface_registrations();
    close_retained_surface_fixture(surface_id);
}

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "renderer::engine_canvas")
        .map(|table| {
            semio_framework_async::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::EngineSurfaceRegistry", ENGINE_SURFACE_CAPACITY, size_of::<EngineSurfaceSlot>(), size_of::<EngineSurfaceRegistry>()),
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::StagedEngineScenes", ENGINE_CANVAS_FRAME_PACKET_CAPACITY, size_of::<Option<StagedEngineScene>>(), size_of::<StagedEngineScenes>()),
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::EngineCanvasBuildContext", ENGINE_CANVAS_FRAME_PACKET_CAPACITY, size_of::<Option<EngineCanvasPacket>>(), size_of::<EngineCanvasBuildContext>()),
    ];
    semio_framework_async::assert_fixed_slot_tables(
        "renderer::engine_canvas",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(EngineSurfaceRegistry::default());
            drop(StagedEngineScenes::default());
            drop(EngineCanvasBuildContext::default());
        },
    );
}

/// ⚖️ Law: painting a tiled-map surface ADMITS its visible tiles into the bounded renderer-asset
/// pipeline. `apply_map_tile_bytes` existed with zero production callers and nothing reserved a
/// `WorldAssetRequestKind::MapTile` anywhere in the repo, so raster and vector tiles were dead on
/// both browser and native — the map painted its vector fixture over an empty basemap forever.
/// React's `MapRenderer.uploadTileRow` (`🧭️TiledMapHost/🟦️.tsx:608-631`) is the reference: fetch
/// every visible tile the session does not already hold, skip the ones it does, never re-request a
/// miss.
///
/// 🔁️ A second paint must not double-reserve: the pending set de-duplicates by `(vector, z, x, y)`
/// exactly like React's in-flight promise map.
///
/// 🧹️ Drain the shared registration list this paint appended to: `take_engine_surface_registrations`
/// is process-wide, and a sibling test asserting on its OWN entries must not find these.
#[test]
fn tiled_map_paint_reserves_the_visible_tiles_react_fetches() {
    let fixture = fixture();
    drop_engine_surface("tiled-map-tiles");
    let map_scene = tiled_map_scene("tiled-map-tiles", &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let frame = paint_retained_surface_scene(&map_scene, bounds);
    let host_id = frame.owner.host_id.clone();

    let pending = ENGINE_SURFACES.with(|cell| cell.borrow().get(&host_id).map(|entry| entry.map_sync_cache.tile_pending.iter().copied().collect::<Vec<_>>())).expect("the painted frame constructed the map engine");
    assert!(!pending.is_empty(), "a painted map must offer its visible tiles to the asset lane; nothing was reserved");
    assert!(pending.iter().any(|(vector, ..)| !*vector), "the raster lane must be offered in the default `combined` render mode");
    let held = with_map_host(&host_id, |host| pending.iter().any(|(vector, z, x, y)| if *vector { host.has_vector_tile(&map_tiles::tile_key(*z, *x, *y)) } else { host.has_tile(&map_tiles::tile_key(*z, *x, *y)) })).expect("map host");
    assert!(!held, "React skips the fetch for a tile the session already holds, and so does this — no pending tile may already be resident");

    let before = pending.len();
    let frame = paint_retained_surface_scene(&map_scene, bounds);
    assert_eq!(frame.owner.host_id, host_id, "same retained component generation preserves its exact engine host");
    let after = ENGINE_SURFACES.with(|cell| cell.borrow().get(&host_id).map(|entry| entry.map_sync_cache.tile_pending.len())).expect("map engine");
    assert_eq!(after, before, "the per-frame re-offer must be free");

    let _ = take_engine_surface_registrations();
    close_retained_surface_fixture("tiled-map-tiles");
}

#[test]
fn tiled_map_cancel_separates_pan_camera_settle_from_marquee_selection() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("scene cancellation contract");
    let fixture = fixture();
    let surface_id = contract["tiledMap"]["owner"]["surfaceId"].as_str().expect("map surface id");
    let window_id = contract["tiledMap"]["owner"]["windowId"].as_str().expect("map window id");
    let controller_id = "map.cancel.fixture";
    let mut scene = tiled_map_scene(surface_id, &fixture);
    scene.controller_id = controller_id.into();
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    drop_engine_surface(surface_id);
    assert!(sync_tiled_map_scene(&scene, window_id, bounds, &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();

    let owner = map_fixture_owner(window_id, surface_id);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&owner, controller_id, bounds, 360.0, 260.0, 1, false, false, "rectangle", &mut input), Ok(true));
    let _ = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_move_into(&owner, controller_id, bounds, 410.0, 290.0, true, &mut input), Ok(true));
    let _ = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(true));
    let pan = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(pan.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_drag_active(surface_id));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(false));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_up_into(surface_id, surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(false));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());

    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&owner, controller_id, bounds, 120.0, 120.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_move_into(&owner, controller_id, bounds, 220.0, 220.0, true, &mut input), Ok(true));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 220.0, 220.0, &mut input), Ok(true));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "marquee cancellation cannot publish interactionSelect");
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_drag_active(surface_id));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_up_into(surface_id, surface_id, controller_id, bounds, 220.0, 220.0, &mut input), Ok(false));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the old outside-up is inert");
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&owner, controller_id, bounds, 140.0, 140.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 140.0, 140.0, &mut input), Ok(true));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 140.0, 140.0, &mut input), Ok(false));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    drop_engine_surface(surface_id);
}
