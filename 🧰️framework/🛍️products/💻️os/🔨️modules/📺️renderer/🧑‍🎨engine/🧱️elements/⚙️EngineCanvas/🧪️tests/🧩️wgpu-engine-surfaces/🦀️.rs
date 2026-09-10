//! 🧩️ Wgpu engine-surface attach — the ONE production seam that constructs the engine host behind
//! every `SurfaceKind` the wgpu shell can paint, generalised from the node-graph-only seam of
//! `📓️wgpu-node-graph-2026-09-10.md` §2.
//!
//! Three of the four kinds composite through a vello texture and attach in `ENGINE_SURFACES`
//! (`sync_engine_scene`); `World3d` attaches against the shell's own `world3d_states` and paints a
//! real 3-D pass into the window's retained draw list. All four are recorded through the same drained
//! `EngineSurfaceRegistration` list, which is what the shell mirrors into its pointer-dispatch maps.
//!
//! The `World3d` payload is generation3d's own `hexagonal-mushroom-column` preview scene, committed as
//! data in `🔣️.json` with its provenance; the action payloads are pinned against React's `World3dHost`
//! (`🧱️elements/🌍️World3dHost/🟦️.tsx`), the other implementation of this same host.

use super::*;
use infinite_world::world::{begin_world3d_dynamic_retirement, enqueue_world3d_event, step_world3d_dynamic_retirement, world3d_dynamic_retirement_terminal_is_empty, step_world3d_draw_rebuild, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, World3dBuildContext, World3dSceneBridgeStep, World3dSnapshotApplyStep, World3dState, WorldCursorWakeAuthority, WorldDrawRebuildStep, WorldInteractionAuthorityStep, WorldInteractionIntent};
use ui_wgpu::wgpu::{Board2dScene, DrawList, FontAtlas, IconAtlas, InputState, SurfaceKind, TiledMapScene, UiPresence, World3dScene};

const ENGINE_SURFACES_FIXTURE: &str = include_str!("🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(ENGINE_SURFACES_FIXTURE).expect("committed engine-surface fixture parses")
}

fn json_text(value: &Value) -> String {
    serde_json::to_string(value).expect("fixture payload re-encodes")
}

fn scene_shell(surface_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "generation3d".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        menu: None,
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
    }
}

/// 🌍️ The exact `World3dScene` generation3d's preview window publishes for the bundled example.
fn world3d_preview_scene(surface_id: &str, fixture: &Value) -> UiComponentSceneNode {
    world3d_preview_scene_with_selection(surface_id, fixture, json_text(&fixture["world3d"]["selectionJson"]))
}

/// 🌍️ The same window with its selection document replaced — an unselected preview is what React's
/// `World3dHost` is in before the user's first click, and it is the state a plain pick must be tested
/// from: clicking an ALREADY-selected instance grabs the gumball and commits a `translateSelection`.
fn world3d_preview_scene_with_selection(surface_id: &str, fixture: &Value, selection_json: String) -> UiComponentSceneNode {
    let world = &fixture["world3d"];
    let mut scene = scene_shell(surface_id, SurfaceKind::World3d);
    scene.world_3d = Some(World3dScene {
        environment_json: Some(json_text(&world["environmentJson"])),
        domain_id: world["domainId"].as_str().map(str::to_owned),
        domain_granularity_id: world["domainGranularityId"].as_str().map(str::to_owned),
        ..World3dScene::base(json_text(&world["cameraJson"]), json_text(&world["meshesJson"]), json_text(&world["instancesJson"]), selection_json)
    });
    scene
}

fn tiled_map_scene(surface_id: &str, fixture: &Value) -> UiComponentSceneNode {
    let map = &fixture["tiledMap"];
    let mut scene = scene_shell(surface_id, SurfaceKind::TiledMap);
    scene.tiled_map = Some(TiledMapScene {
        selection_json: json_text(&map["selectionJson"]),
        hover_json: json_text(&map["hoverJson"]),
        ..TiledMapScene::base(json_text(&map["mapFixtureJson"]), json_text(&map["cameraJson"]))
    });
    scene
}

fn board2d_scene(surface_id: &str, fixture: &Value) -> UiComponentSceneNode {
    let board = &fixture["board2d"];
    let mut scene = scene_shell(surface_id, SurfaceKind::Board2d);
    scene.board2d = Some(Board2dScene { selection_json: json_text(&board["selectionJson"]), ..Board2dScene::base(json_text(&board["fixtureJson"]), json_text(&board["cameraJson"]), true) });
    scene
}

/// 🧹️ Drains one attached surface through the registry's OWN retirement ladder — a plain `remove`
/// panics inside the registry mutex on a host whose ordered maps refuse to drop unretired, and
/// poisons it for every later lane in the binary.
fn drop_engine_surface(surface_id: &str) {
    let _ = take_engine_surface_registrations();
    while STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().take_one()).is_some() {}
    let Some(token) = ENGINE_SURFACES.with(|cell| cell.borrow_mut().token(surface_id)) else {
        return;
    };
    assert_eq!(begin_engine_surface_close_token(token), Ok(true), "the attached surface begins closing");
    let mut input = InputState::<ActionDescriptor>::default();
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| close_engine_surface_step(token, context, &mut input)) {
        turns += 1;
        assert!(turns < 262_144, "the attached surface reaches a fixed terminal witness");
    }
}

/// 🧹️ Drains every attached `World3dState` through its OWN dynamic retirement ladder. The state's
/// `WorldDynamicRegistry`s refuse to drop unretired, so letting the map fall out of scope aborts the
/// whole test binary in a destructor — the ladder is the only correct teardown, exactly as
/// `close_engine_surface_step` is for the vello-composited kinds.
fn drop_world3d_states(mut states: crate::scenes::AdmittedSurfaceMap<World3dState>) {
    while let Some(owner) = states.close_step() {
        let mut state = owner.value;
        begin_world3d_dynamic_retirement(&mut state);
        let mut turns = 0usize;
        while !world3d_dynamic_retirement_terminal_is_empty(&state) {
            with_world_step(64, |context| {
                step_world3d_dynamic_retirement(&mut state, context);
                true
            });
            turns += 1;
            assert!(turns < 262_144, "the attached world state reaches a fixed terminal witness");
        }
    }
}

struct PaintedFrame {
    draw: DrawList,
    world3d_states: crate::scenes::AdmittedSurfaceMap<World3dState>,
}

/// 🎬️ Drives the REAL production paint — `scenes::render_component_scene_step` through a real
/// `FrameworkWidgetContext` and a real `SceneEngineHosts`, not the attach functions directly.
fn paint_scene(scene: &UiComponentSceneNode, bounds: Rect, states: crate::scenes::AdmittedSurfaceMap<World3dState>) -> PaintedFrame {
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
        let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources };
        let mut cursor = ui_wgpu::wgpu::ScenePaintCursor::default();
        for _ in 0..4096 {
            match crate::scenes::render_component_scene_step(scene, bounds, &mut ctx, &mut cursor, &mut hosts) {
                ui_wgpu::wgpu::ScenePaintStep::Pending => continue,
                ui_wgpu::wgpu::ScenePaintStep::Complete => break,
                ui_wgpu::wgpu::ScenePaintStep::Fault => panic!("engine surface scene paint faulted"),
            }
        }
    }
    PaintedFrame { draw, world3d_states }
}

fn with_world_step(fuel: u64, apply: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> bool) -> bool {
    with_engine_close_context(fuel, apply)
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
            faulted = step_world3d_draw_rebuild(state, context) == WorldDrawRebuildStep::Fault;
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
            complete = step_world3d_draw_rebuild(state, context) == WorldDrawRebuildStep::Complete;
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
    let mut frame = paint_scene(scene, bounds, crate::scenes::AdmittedSurfaceMap::default());
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
    enqueue_world3d_event(state, intent).expect("the world interaction queue admits the intent");
    for turn in 0..8_192 {
        let Some(generation) = infinite_world::world::world3d_interaction_front_generation(state) else { break };
        let mut faulted = false;
        with_world_step(64, |context| {
            faulted = matches!(step_world3d_interaction(state, generation, &mut input, context), WorldInteractionAuthorityStep::Fault);
            true
        });
        assert!(!faulted, "the world interaction authority faulted on turn {turn}");
        assert!(turn < 8_191, "the world interaction authority drained its queue within its turn ceiling");
    }
    crate::collect_fixture_actions(&mut input)
}

#[test]
fn world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid() {
    let fixture = fixture();
    let scene = world3d_preview_scene("world3d-attach-draw", &fixture);
    let bounds = Rect { x: 24.0, y: 36.0, w: 800.0, h: 600.0 };

    let painted = attach_and_settle_world3d(&scene, bounds, "world3d-attach-draw");

    let expect = &fixture["world3d"]["expect"];
    let state = painted.world3d_states.get("world3d-attach-draw").expect("attached world state");
    assert_eq!(state.snapshot_fault(), None, "the production attach reaches a published snapshot without faulting");
    let pass = painted.draw.scene_passes.first().expect("the World3d host paints a real 3-D pass into the window draw list");
    assert_eq!(pass.viewport, [bounds.x, bounds.y, bounds.w, bounds.h], "the pass is sized to the window body");
    assert_eq!(pass.draws.len(), expect["draws"].as_u64().expect("draws") as usize, "one draw per previewed mesh that actually carries triangles");

    let draw = pass.draws.first().expect("published draw");
    assert_eq!(draw.mesh_key, expect["drawMeshKey"].as_str().expect("mesh key"));
    let instance_ids: Vec<&str> = draw.instances.iter().map(|instance| instance.id.as_str()).collect();
    let expected_ids: Vec<&str> = expect["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("id")).collect();
    assert_eq!(instance_ids, expected_ids, "instance ids stay channel-qualified through the production paint");
    let mesh = state.mesh_lease(&draw.mesh_key).expect("published mesh geometry");
    let schema = mesh.schema().expect("mesh schema");
    assert_eq!(schema.indices / 3, expect["triangles"].as_u64().expect("triangles") as u32, "the drawn mesh is the tessellated solid, not the 12-triangle placeholder box");
    assert_eq!(schema.vertices, expect["vertices"].as_u64().expect("vertices") as u32);
    assert!(draw.instances.first().expect("instance").selected, "the scene's own selection document reaches the painted instance");
    drop_world3d_states(painted.world3d_states);
}

#[test]
fn world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches() {
    let fixture = fixture();
    let scene = world3d_preview_scene_with_selection("world3d-attach-pick", &fixture, json!({ "method": "pick", "mode": "replace", "ids": [] }).to_string());
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, "world3d-attach-pick").world3d_states;
    let state = states.get_mut("world3d-attach-pick").expect("attached world state");

    // 🎯️ Aim through the fixture camera's own target — the centre of the prism's base face, the one
    // point guaranteed both inside the solid and inside the 45° frustum.
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(bounds.w / bounds.h), ui_wgpu::wgpu::Vec3::ZERO, bounds.w, bounds.h).expect("the camera target projects");
    // 🖱️ A left press opens the marquee gesture and the RELEASE is what picks — the same
    // press-then-click contract React's World3dHost binds, so the lane drives both halves.
    let mut actions = publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], true, 0, &PointerModifiers::default()));
    actions.extend(publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], false, 0, &PointerModifiers::default())));

    let expected_id = fixture["world3d"]["expect"]["instanceIds"][0].as_str().expect("instance id");
    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("a pointer-down over an instance publishes interactionSelect");
    assert_eq!(select.controller_id, "generation3d");
    let args = select.args.clone().expect("select args");
    assert_eq!(args["domainId"].as_str(), Some("graph"), "the window's own bound domain, never the OS `world` fallback");
    assert_eq!(args["merge"].as_str(), Some("replace"));
    assert_eq!(args["method"].as_str(), Some("pick"));
    assert_eq!(args["targets"][0]["granularity"].as_str(), Some("handle"));
    assert_eq!(args["targets"][0]["id"].as_str(), Some(expected_id), "the target is the bare channel-qualified id React's World3dHost dispatches — never `surfaceId/id`");
    drop_world3d_states(states);
}

#[test]
fn world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload() {
    let fixture = fixture();
    let scene = world3d_preview_scene("world3d-attach-camera", &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, "world3d-attach-camera").world3d_states;
    let state = states.get_mut("world3d-attach-camera").expect("attached world state");
    let before = state.orbit.to_camera().position.to_array();

    let wheel = publish_world3d_intent(state, WorldInteractionIntent::wheel(400.0, 300.0, -120.0, &PointerModifiers::default()));
    let zoomed = wheel.iter().find(|action| action.action == "setCamera").expect("a wheel over the preview publishes setCamera");
    assert_eq!(zoomed.controller_id, "generation3d");
    let args = zoomed.args.clone().expect("camera args");
    assert_eq!(args["surfaceId"].as_str(), Some("world3d-attach-camera"));
    assert!(args["camera"]["position"].as_array().is_some_and(|axes| axes.len() == 3), "generation3d's 📷️set-camera payload carries a 3-axis position");
    assert!(args["camera"]["target"].as_array().is_some_and(|axes| axes.len() == 3));
    assert!(args["camera"]["fov"].as_f64().is_some());
    let after = state.orbit.to_camera().position.to_array();
    let moved = (0..3).any(|axis| (after[axis] - before[axis]).abs() > 1.0e-4);
    assert!(moved, "the wheel actually moved the orbit camera the action reports: {before:?} -> {after:?}");

    // 🖱️ Orbit is `button == 2 && (alt || meta)` — the same chord React's World3dHost binds; a bare
    // right-drag pans instead, and a plain left-drag is a marquee, not a camera move.
    let dragged = publish_world3d_intent(state, WorldInteractionIntent::pointer_move(420.0, 320.0, 20.0, 20.0, true, 2, &PointerModifiers { alt: true, ..PointerModifiers::default() }));
    let orbited = dragged.iter().find(|action| action.action == "setCamera").expect("an orbit drag publishes setCamera");
    let orbit_args = orbited.args.clone().expect("orbit camera args");
    assert_eq!(orbit_args["surfaceId"].as_str(), Some("world3d-attach-camera"));
    let orbited_position = state.orbit.to_camera().position.to_array();
    let turned = (0..3).any(|axis| (orbited_position[axis] - after[axis]).abs() > 1.0e-4);
    assert!(turned, "the orbit drag actually turned the camera: {after:?} -> {orbited_position:?}");
    drop_world3d_states(states);
}

#[test]
fn tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam() {
    let fixture = fixture();
    for surface_id in ["tiled-map-attach", "board2d-attach"] {
        drop_engine_surface(surface_id);
    }
    let map_scene = tiled_map_scene("tiled-map-attach", &fixture);
    let board_scene = board2d_scene("board2d-attach", &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };

    let map_frame = paint_scene(&map_scene, bounds, crate::scenes::AdmittedSurfaceMap::default());
    drop_world3d_states(map_frame.world3d_states);
    let map_draw = map_frame.draw;
    let map_expect = &fixture["tiledMap"]["expect"];
    let (positions, routes, selected, hovered) = with_map_host("tiled-map-attach", |host| {
        (host.features.positions.len(), host.features.routes.len(), host.selected_position_ids().map(str::to_owned).collect::<Vec<_>>(), host.hovered_id().map(str::to_owned))
    })
    .expect("the first painted frame constructs the map engine");
    assert_eq!(positions, map_expect["positions"].as_u64().expect("positions") as usize, "the descriptor reaches the host whole");
    assert_eq!(routes, map_expect["routes"].as_u64().expect("routes") as usize);
    assert_eq!(selected, map_expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_owned()).collect::<Vec<_>>(), "React's resolveMapInteractionSync granularity rule, reproduced");
    assert_eq!(hovered.as_deref(), map_expect["hoveredId"].as_str());
    let map_key = engine_raster_key("tiled-map-attach").expect("bounded engine raster key");
    assert!(map_draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).any(|(key, _)| key == &map_key), "the painted map is composited into the window draw list under {map_key}");

    let board_frame = paint_scene(&board_scene, bounds, crate::scenes::AdmittedSurfaceMap::default());
    drop_world3d_states(board_frame.world3d_states);
    let board_draw = board_frame.draw;
    let board_expect = &fixture["board2d"]["expect"];
    let (nodes, edges, board_selection) = with_board_host("board2d-attach", |host| (host.nodes.len(), host.edges.len(), host.selection.iter().cloned().collect::<Vec<String>>())).expect("the first painted frame constructs the board engine");
    assert_eq!(nodes, board_expect["nodes"].as_u64().expect("nodes") as usize, "the fixture reaches the board host whole");
    assert_eq!(edges, board_expect["edges"].as_u64().expect("edges") as usize);
    assert_eq!(board_selection, board_expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_owned()).collect::<Vec<_>>(), "parse_fixture_json resets selection, so the sync re-applies it silently right after — React's applyFixtureToSession rule");
    let board_key = engine_raster_key("board2d-attach").expect("bounded engine raster key");
    assert!(board_draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).any(|(key, _)| key == &board_key), "the painted board is composited into the window draw list under {board_key}");

    let registrations = take_engine_surface_registrations();
    let kinds: Vec<&EngineSurfaceKindDetail> = registrations.iter().map(|entry| &entry.detail).collect();
    assert!(kinds.iter().any(|detail| matches!(detail, EngineSurfaceKindDetail::TiledMap { .. })), "the map surface is recorded on the shared registration list the shell mirrors");
    assert!(kinds.iter().any(|detail| matches!(detail, EngineSurfaceKindDetail::Board2d { .. })), "the board surface is recorded on the shared registration list the shell mirrors");

    for surface_id in ["tiled-map-attach", "board2d-attach"] {
        drop_engine_surface(surface_id);
    }
}
