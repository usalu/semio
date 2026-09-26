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
use infinite_world::world::{
    begin_world3d_dynamic_retirement, enqueue_world3d_event, step_world3d_draw_rebuild, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, world3d_dynamic_retirement_terminal_is_empty,
    World3dBuildContext, World3dSceneBridgeStep, World3dSnapshotApplyStep, World3dState, WorldCursorWakeAuthority, WorldDrawRebuildStep, WorldInteractionAuthorityStep, WorldInteractionIntent,
};
use ui_wgpu::wgpu::{Board2dScene, DrawList, FontAtlas, IconAtlas, InputState, SurfaceKind, TiledMapScene, UiPresence, World3dScene};

const ENGINE_SURFACES_FIXTURE: &str = include_str!("../../🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json");
const MAP_GESTURE_LIFECYCLE_FIXTURE: &str = include_str!("../../../../🧫️fixtures/♻️tiled-map-gesture-lifecycle/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(ENGINE_SURFACES_FIXTURE).expect("committed engine-surface fixture parses")
}

fn json_text(value: &Value) -> String {
    serde_json::to_string(value).expect("fixture payload re-encodes")
}

fn scene_shell(surface_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
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
    scene.tiled_map = Some(TiledMapScene { selection_json: json_text(&map["selectionJson"]), hover_json: json_text(&map["hoverJson"]), ..TiledMapScene::base(json_text(&map["mapFixtureJson"]), json_text(&map["cameraJson"])) });
    scene
}

fn retained_map_record(id: u64, key: &str, scene: &TiledMapScene) -> ui_contract::UiNodeRecord {
    let mut record = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded map node key"),
        component: ui_contract::Component::Surface(ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::TiledMap, scene).expect("bounded Map scene encodes")),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    };
    record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
    record
}

fn retained_surface_record(scene: &UiComponentSceneNode) -> ui_contract::UiNodeRecord {
    let component = match scene.component_kind {
        SurfaceKind::NodeGraph => ui_contract::Component::Surface(ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::NodeGraph, scene.node_graph.as_ref().expect("NodeGraph scene payload")).expect("bounded NodeGraph scene encodes")),
        SurfaceKind::TiledMap => ui_contract::Component::Surface(ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::TiledMap, scene.tiled_map.as_ref().expect("TiledMap scene payload")).expect("bounded TiledMap scene encodes")),
        SurfaceKind::Board2d => ui_contract::Component::Surface(ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Board2d, scene.board2d.as_ref().expect("Board2d scene payload")).expect("bounded Board2d scene encodes")),
        kind => panic!("retained EngineCanvas fixture does not support {kind:?}"),
    };
    let mut record = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(1),
        key: ui_contract::UiText::try_from_str("engine-surface").expect("bounded engine-surface node key"),
        component,
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    };
    record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
    record
}

fn retained_separator_record(id: u64, key: &str) -> ui_contract::UiNodeRecord {
    ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded replacement key"),
        component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    }
}

fn retained_map_row_record(id: u64, children: &[u64]) -> ui_contract::UiNodeRecord {
    let mut mounted_children = ui_contract::UiFixedList::default();
    for child in children {
        mounted_children.try_push(ui_contract::UiNodeId(*child)).expect("Map row children fit the fixed document record");
    }
    let mut record = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str("map-row").expect("bounded Map row key"),
        component: ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: mounted_children,
    };
    record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
    record
}

fn retained_document(surface_id: &str, generation: u64, root: u64, records: Vec<ui_contract::UiNodeRecord>) -> ui_contract::UiDocumentLease {
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(generation), root: Some(ui_contract::UiNodeId(root)), layout_epoch: 0 };
    ui_contract::UiDocumentLease::try_publish(ui_contract::SurfaceId::try_from(surface_id).expect("bounded Map surface id"), identity, records).expect("Map lifecycle document publishes")
}

fn map_fixture_owner(window_id: &str, surface_id: &str) -> crate::interpreter::ScenePointerTarget {
    crate::interpreter::fixture_scene_pointer_target(window_id, surface_id, "map.fixture")
}

fn paint_retained_map_document(document: &ui_contract::UiDocumentLease, surface_id: &str, bounds: Rect) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = crate::scenes::AdmittedSurfaceMap::default();
    let mut world_resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut cursor = crate::interpreter::UiDocumentFrameCursor::default();
    let complete = (0..(1 << 20)).any(|_| {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, bounds.h);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface_id };
        let done = crate::interpreter::render_ui_document_step(&mut cursor, document, bounds, &mut ctx, surface_id, "map.lifecycle.fixture", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
        crate::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
        assert!(done || !cursor.terminal_is_fault(), "Map lifecycle document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "Map lifecycle document painted within its opportunity ceiling");
    drop_world3d_states(world3d_states);
    input
}

fn publish_retained_map_fixture(document: &ui_contract::UiDocumentLease, window_id: &str, bounds: Rect, witness: u64) -> InputState<ActionDescriptor> {
    let mut input = paint_retained_map_document(document, window_id, bounds);
    crate::interpreter::begin_accessibility_visible_documents();
    crate::interpreter::note_accessibility_visible_document(window_id);
    crate::interpreter::register_clipped_retained_hit_targets(window_id, bounds, &mut input);
    assert!(crate::interpreter::seal_presented_input_candidate(witness));
    assert!(crate::interpreter::acknowledge_presented_input(witness));
    input.publish_hits();
    crate::interpreter::publish_accessibility_visible_documents();
    input
}

fn close_retained_map_fixture(window_id: &str) {
    close_retained_surface_fixture(window_id);
}

pub(super) fn close_retained_surface_fixture(window_id: &str) {
    assert!(crate::interpreter::request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !crate::interpreter::ui_document_close_pending() {
            break;
        }
        crate::interpreter::close_ui_document_one();
        crate::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
    }
    assert!(!crate::interpreter::ui_document_close_pending(), "the EngineCanvas fixture returns every retained owner");
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
    STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().remove_surface(surface_id));
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

pub(super) struct RetainedSurfacePaint {
    pub(super) draw: DrawList,
    pub(super) owner: crate::interpreter::ScenePointerTarget,
}

/// 🪟️ Drives an EngineCanvas scene through the retained document and Interpreter admission seam.
pub(super) fn paint_retained_surface_scene(scene: &UiComponentSceneNode, bounds: Rect) -> RetainedSurfacePaint {
    paint_retained_surface_in_window(scene, bounds, &scene.surface_id)
}

pub(super) fn paint_retained_surface_in_window(scene: &UiComponentSceneNode, bounds: Rect, window_id: &str) -> RetainedSurfacePaint {
    let document = retained_document(window_id, 1, 1, vec![retained_surface_record(scene)]);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = crate::scenes::AdmittedSurfaceMap::default();
    let mut world_resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut cursor = crate::interpreter::UiDocumentFrameCursor::default();
    let complete = (0..(1 << 20)).any(|_| {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, bounds.h);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id };
        let done = crate::interpreter::render_ui_document_step(&mut cursor, &document, bounds, &mut ctx, window_id, &scene.controller_id, ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
        crate::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
        assert!(done || !cursor.terminal_is_fault(), "retained EngineCanvas document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "retained EngineCanvas document painted within its opportunity ceiling");
    drop_world3d_states(world3d_states);
    let owner = crate::interpreter::retained_scene_target_at(window_id, bounds.x + 1.0, bounds.y + 1.0).expect("retained EngineCanvas surface exposes its exact mounted target");
    assert_eq!(owner.surface_id, scene.surface_id);
    assert_eq!(owner.kind, scene.component_kind);
    RetainedSurfacePaint { draw, owner }
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: "law-window" };
        let mut cursor = ui_wgpu::wgpu::ScenePaintCursor::default();
        for _ in 0..4096 {
            match crate::scenes::render_component_scene_step(scene, bounds, &mut ctx, &mut cursor, &mut hosts, ui_wgpu::wgpu::UiDriverDrag::Handle, 1) {
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

/// 🧱️ The paint law, keyed by surface id so it can run twice in one binary — once on libtest's own
/// thread and once on the deliberately 2 MiB lane below, without the two sharing a surface.
fn world3d_preview_window_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 24.0, y: 36.0, w: 800.0, h: 600.0 };

    let painted = attach_and_settle_world3d(&scene, bounds, surface_id);

    let expect = &fixture["world3d"]["expect"];
    let state = painted.world3d_states.get(surface_id).expect("attached world state");
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
fn world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid() {
    world3d_preview_window_law("world3d-attach-draw");
}

/// 🖱️ The pick law, keyed by surface id for the same reason as `world3d_preview_window_law`.
///
/// 🎯️ `targets` is a JSON TEXT arg, never a nested array — the world authority writes it through
/// bounded string credits (`INTERACTION_TARGETS_OPEN`, `🌍️world/🦀️.rs:6343`), so the law parses it.
///
/// ⚖️ A pick reports the scene's interaction granularity, matching the shared React gesture oracle.
fn world3d_pointer_down_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene_with_selection(surface_id, &fixture, json!({ "method": "pick", "mode": "replace", "ids": [] }).to_string());
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, surface_id).world3d_states;
    let state = states.get_mut(surface_id).expect("attached world state");

    // 🎯️ Aim through the fixture camera's own target — the centre of the prism's base face, the one
    // point guaranteed both inside the solid and inside the 45° frustum.
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(bounds.w, bounds.h), ui_wgpu::wgpu::Vec3::ZERO, bounds.w, bounds.h).expect("the camera target projects");
    // 🖱️ A left press opens the marquee gesture and the RELEASE is what picks — the same
    // press-then-click contract React's World3dHost binds, so the lane drives both halves.
    let mut actions = publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], true, 0, &PointerModifiers::default()));
    actions.extend(publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], false, 0, &PointerModifiers::default())));
    drop_world3d_states(states);

    let expected_id = fixture["world3d"]["expect"]["instanceIds"][0].as_str().expect("instance id");
    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("a pointer-down over an instance publishes interactionSelect");
    assert_eq!(select.controller_id, "generation3d");
    let args = select.args.clone().expect("select args");
    assert_eq!(args["domainId"].as_str(), Some("graph"), "the window's own bound domain, never the OS `world` fallback");
    assert_eq!(args["merge"].as_str(), Some("replace"));
    assert_eq!(args["method"].as_str(), Some("pick"));
    let targets: Value = serde_json::from_str(args["targets"].as_str().expect("targets is written as JSON text")).expect("targets text parses");
    assert_eq!(targets[0]["granularity"], fixture["world3d"]["domainGranularityId"]);
    let expected_channel_id = expected_id.split_once('#').map_or(expected_id, |(channel, _)| channel);
    assert_eq!(targets[0]["id"].as_str(), Some(expected_channel_id), "the target is the bare channel-qualified id React's World3dHost dispatches — never `surfaceId/id` and never the render id");
}

#[test]
fn world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches() {
    world3d_pointer_down_law("world3d-attach-pick");
}

/// 📷️ The camera law, keyed by surface id for the same reason as `world3d_preview_window_law`.
///
/// 🪟️ `windowId`, never `surfaceId` — `worldCameraSetCameraDispatchArgs` addresses the WINDOW
/// instance, and the guest camera value has no `fov` member: React sends `{position, target,
/// zoom, up?}` and a stray key fails the whole `camera` deserialization.
fn world3d_orbit_and_wheel_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, surface_id).world3d_states;
    let state = states.get_mut(surface_id).expect("attached world state");
    let before = state.orbit.to_camera().position.to_array();

    let wheel = publish_world3d_intent(state, WorldInteractionIntent::wheel(400.0, 300.0, -120.0, &PointerModifiers::default()));
    let zoomed = wheel.iter().find(|action| action.action == "setCamera").expect("a wheel over the preview publishes setCamera");
    assert_eq!(zoomed.controller_id, "generation3d");
    let args = zoomed.args.clone().expect("camera args");
    assert_eq!(args["windowId"].as_str(), Some(surface_id));
    assert!(args.get("surfaceId").is_none(), "the pre-fix `surfaceId` address is gone, not merely joined by `windowId`");
    assert!(args["camera"]["position"].as_array().is_some_and(|axes| axes.len() == 3), "generation3d's 📷️set-camera payload carries a 3-axis position");
    assert!(args["camera"]["target"].as_array().is_some_and(|axes| axes.len() == 3));
    assert_eq!(args["camera"]["zoom"].as_f64(), Some(1.0), "a perspective orbit reports React's identity zoom");
    assert_eq!(args["camera"]["up"].as_array().map(<[_]>::len), Some(3));
    assert!(args["camera"].get("fov").is_none(), "`fov` is not part of the React `setCamera` camera value");
    let after = state.orbit.to_camera().position.to_array();
    let moved = (0..3).any(|axis| (after[axis] - before[axis]).abs() > 1.0e-4);
    assert!(moved, "the wheel actually moved the orbit camera the action reports: {before:?} -> {after:?}");

    // 🖱️ Orbit is `button == 2 && (alt || meta)` — the same chord React's World3dHost binds; a bare
    // right-drag pans instead, and a plain left-drag is a marquee, not a camera move.
    let dragged = publish_world3d_intent(state, WorldInteractionIntent::pointer_move(420.0, 320.0, 20.0, 20.0, true, 2, &PointerModifiers { alt: true, ..PointerModifiers::default() }));
    let orbited = dragged.iter().find(|action| action.action == "setCamera").expect("an orbit drag publishes setCamera");
    let orbit_args = orbited.args.clone().expect("orbit camera args");
    assert_eq!(orbit_args["windowId"].as_str(), Some(surface_id));
    let orbited_position = state.orbit.to_camera().position.to_array();
    let turned = (0..3).any(|axis| (orbited_position[axis] - after[axis]).abs() > 1.0e-4);
    assert!(turned, "the orbit drag actually turned the camera: {after:?} -> {orbited_position:?}");
    drop_world3d_states(states);
}

#[test]
fn world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload() {
    world3d_orbit_and_wheel_law("world3d-attach-camera");
}

/// 🧵️ Runs all three World3d laws on a lane with an EXPLICIT 2 MiB stack — the budget
/// `std::thread` hands a spawned thread, and the budget the native frame-build pool worker that
/// actually drives `AppFrameTransactionPhase::World3dSnapshot` runs on (`WorkerPool` spawns its
/// workers with no `stack_size` at `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1780`, and
/// `🧵️frame-job/🦀️.rs:474` submits the frame build to it).
///
/// `Builder::stack_size` overrides `RUST_MIN_STACK`, so the repo runner's blanket 128 MiB floor
/// (`runCargoTestBudgeted`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`)
/// cannot hide a re-inflated frame here the way it hid the 5 664 768-byte
/// `AdmittedSurfaceMap::<World3dState>::default` array literal until 2026-09-12. Behavioural, not a
/// source-shape assertion: any future frame that grows back past the budget aborts this lane.
#[test]
fn world3d_engine_surface_laws_fit_a_bounded_thread_stack() {
    let lane = std::thread::Builder::new()
        .name("world3d-bounded-stack".to_string())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            world3d_preview_window_law("world3d-bounded-draw");
            world3d_pointer_down_law("world3d-bounded-pick");
            world3d_orbit_and_wheel_law("world3d-bounded-camera");
        })
        .expect("the bounded-stack lane spawns");
    if let Err(panic) = lane.join() {
        std::panic::resume_unwind(panic);
    }
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

    let map_frame = paint_retained_surface_scene(&map_scene, bounds);
    let map_host_id = map_frame.owner.host_id.clone();
    let map_draw = map_frame.draw;
    let map_expect = &fixture["tiledMap"]["expect"];
    let (positions, routes, selected, hovered) =
        with_map_host(&map_host_id, |host| (host.features.positions.len(), host.features.routes.len(), host.selected_position_ids().map(str::to_owned).collect::<Vec<_>>(), host.hovered_id().map(str::to_owned)))
            .expect("the first painted frame constructs the map engine");
    assert_eq!(positions, map_expect["positions"].as_u64().expect("positions") as usize, "the descriptor reaches the host whole");
    assert_eq!(routes, map_expect["routes"].as_u64().expect("routes") as usize);
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
        "parse_fixture_json resets selection, so the sync re-applies it silently right after — React's applyFixtureToSession rule"
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
const BOARD2D_SCENE_LANES: &str = include_str!("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json");

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
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
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
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("scene cancellation contract");
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
    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&owner, controller_id, bounds, 360.0, 260.0, 1, false, false, "rectangle", &mut input), Ok(true));
    let _ = crate::collect_fixture_actions(&mut input);
    assert_eq!(crate::scenes::tiled_map_pointer_move_into(&owner, controller_id, bounds, 410.0, 290.0, true, &mut input), Ok(true));
    let _ = crate::collect_fixture_actions(&mut input);
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(true));
    let pan = crate::collect_fixture_actions(&mut input);
    assert_eq!(pan.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert!(!crate::scenes::tiled_map_drag_active(surface_id));
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(false));
    assert_eq!(crate::scenes::tiled_map_pointer_up_into(surface_id, surface_id, controller_id, bounds, 410.0, 290.0, &mut input), Ok(false));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());

    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&owner, controller_id, bounds, 120.0, 120.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert_eq!(crate::scenes::tiled_map_pointer_move_into(&owner, controller_id, bounds, 220.0, 220.0, true, &mut input), Ok(true));
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 220.0, 220.0, &mut input), Ok(true));
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "marquee cancellation cannot publish interactionSelect");
    assert!(!crate::scenes::tiled_map_drag_active(surface_id));
    assert_eq!(crate::scenes::tiled_map_pointer_up_into(surface_id, surface_id, controller_id, bounds, 220.0, 220.0, &mut input), Ok(false));
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the old outside-up is inert");
    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&owner, controller_id, bounds, 140.0, 140.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 140.0, 140.0, &mut input), Ok(true));
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(surface_id, controller_id, bounds, 140.0, 140.0, &mut input), Ok(false));
    assert!(crate::collect_fixture_actions(&mut input).is_empty());
    drop_engine_surface(surface_id);
}

#[test]
fn tiled_map_hover_leave_publishes_one_empty_owner_transition() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("scene cancellation contract");
    let fixture = fixture();
    let surface_id = contract["tiledMap"]["owner"]["surfaceId"].as_str().expect("map surface id");
    let window_id = contract["tiledMap"]["owner"]["windowId"].as_str().expect("map window id");
    let controller_id = "map.hover.fixture";
    let mut scene = tiled_map_scene(surface_id, &fixture);
    scene.controller_id = controller_id.into();
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    drop_engine_surface(surface_id);
    assert!(sync_tiled_map_scene(&scene, window_id, bounds, &Theme::default()));
    let point = (0..600)
        .step_by(4)
        .find_map(|y| (0..800).step_by(4).find_map(|x| with_map_host(surface_id, |host| host.hit_test_feature_json(x as f64, y as f64)).filter(|hit| hit != "null").map(|_| (x as f32, y as f32))))
        .expect("fixture feature has a physical hit point");
    let mut input = InputState::<ActionDescriptor>::default();
    let owner = map_fixture_owner(window_id, surface_id);
    assert_eq!(crate::scenes::tiled_map_pointer_move_into(&owner, controller_id, bounds, point.0, point.1, false, &mut input), Ok(true));
    let entered = crate::collect_fixture_actions(&mut input);
    assert_eq!(entered.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    assert_eq!(crate::scenes::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(true));
    let left = crate::collect_fixture_actions(&mut input);
    assert_eq!(left.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    let args: Value = serde_json::from_str(&dsl::json::from_dsl_value(left[0].args.as_ref().expect("hover clear args")).to_string()).expect("hover clear args decode");
    assert_eq!(args["targets"], contract["tiledMap"]["hoverLeave"]["emptyTargets"]);
    assert_eq!(crate::scenes::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(false));
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a null witness cannot publish a second leave");
    assert!(!crate::scenes::tiled_map_drag_active(surface_id), "hover retirement cannot manufacture a gesture");
    drop_engine_surface(surface_id);
}

#[test]
fn retained_map_same_identity_refresh_preserves_the_active_gesture() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(MAP_GESTURE_LIFECYCLE_FIXTURE).expect("Map gesture lifecycle contract");
    let fixture = fixture();
    let surface_id = "map.lifecycle.refresh";
    let key = contract["owner"]["key"].as_str().expect("Map owner key");
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let base = tiled_map_scene(surface_id, &fixture).tiled_map.expect("fixture Map scene");
    drop_engine_surface(surface_id);
    let mut first = retained_document(surface_id, 1, 1, vec![retained_map_record(1, key, &base)]);
    let _ = publish_retained_map_fixture(&first, surface_id, bounds, 901);
    let mut input = InputState::<ActionDescriptor>::default();
    let owner = crate::interpreter::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("mounted Map exposes its exact retained target");
    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&owner, "map.lifecycle.fixture", bounds, 100.0, 100.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert!(crate::scenes::tiled_map_drag_active(&owner.host_id));
    let mut changed = base.clone();
    changed.camera_json = json!({ "x": 0.5, "y": 0.4, "zoom": 5.0 }).to_string();
    let mut refresh = retained_document(surface_id, 2, 1, vec![retained_map_record(1, key, &changed)]);
    let mut refresh_input = publish_retained_map_fixture(&refresh, surface_id, bounds, 902);
    let refresh_actions = crate::collect_fixture_actions(&mut refresh_input);
    assert!(refresh_actions.is_empty(), "an ordinary refresh does not synthesize Map input");
    assert!(crate::scenes::tiled_map_drag_active(&owner.host_id), "the same retained node/key/kind/surface preserves its active gesture");
    assert_eq!(crate::scenes::tiled_map_pointer_cancel_into(&owner.host_id, "map.lifecycle.fixture", bounds, 100.0, 100.0, &mut input), Ok(true));
    close_retained_map_fixture(surface_id);
    while !first.close_step() {}
    while !refresh.close_step() {}
    assert!(engine_surface_token(&owner.host_id).is_none());
}

#[test]
fn retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(MAP_GESTURE_LIFECYCLE_FIXTURE).expect("Map gesture lifecycle contract");
    let fixture = fixture();
    let surface_id = "map.lifecycle.replacement";
    let original = contract["owner"]["key"].as_str().expect("Map owner key");
    let successor = contract["cases"].as_array().expect("lifecycle cases").iter().find(|row| row["name"] == "key-replacement").and_then(|row| row["next"]["key"].as_str()).expect("successor key");
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let map = tiled_map_scene(surface_id, &fixture).tiled_map.expect("fixture Map scene");
    drop_engine_surface(surface_id);
    let mut first = retained_document(surface_id, 1, 1, vec![retained_map_record(1, original, &map)]);
    let _ = publish_retained_map_fixture(&first, surface_id, bounds, 901);
    let mut gesture_input = InputState::<ActionDescriptor>::default();
    let original_owner = crate::interpreter::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("mounted Map exposes its exact retained target");
    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&original_owner, "map.lifecycle.fixture", bounds, 100.0, 100.0, 0, false, false, "rectangle", &mut gesture_input), Ok(true));
    assert!(crate::scenes::tiled_map_drag_active(&original_owner.host_id));
    let mut replacement = retained_document(surface_id, 2, 1, vec![retained_map_record(1, successor, &map)]);
    let mut replacement_input = publish_retained_map_fixture(&replacement, surface_id, bounds, 902);
    let replacement_actions = crate::collect_fixture_actions(&mut replacement_input);
    assert!(replacement_actions.is_empty(), "key replacement cannot synthesize PointerUp or selection");
    assert!(!crate::interpreter::scene_pointer_target_is_live(&original_owner), "acknowledgement revokes the old Map receiver before successor input");
    let mut retirement_input = publish_retained_map_fixture(&replacement, surface_id, bounds, 903);
    assert!(crate::collect_fixture_actions(&mut retirement_input).is_empty(), "bounded Map retirement cannot synthesize input");
    assert!(!crate::scenes::tiled_map_drag_active(&original_owner.host_id), "the bounded retirement lane clears the old keyed Map gesture");
    assert!(engine_surface_token(&original_owner.host_id).is_none());
    let successor_owner = crate::interpreter::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("successor exposes its exact retained target");
    assert_eq!(crate::scenes::tiled_map_pointer_down_into(&successor_owner, "map.lifecycle.fixture", bounds, 140.0, 140.0, 0, false, false, "rectangle", &mut gesture_input), Ok(true));
    assert!(crate::scenes::tiled_map_drag_active(&successor_owner.host_id), "the keyed successor starts a fresh gesture");
    let mut removed = retained_document(surface_id, 3, 2, vec![retained_separator_record(2, "map.removed")]);
    let mut removal_input = publish_retained_map_fixture(&removed, surface_id, bounds, 904);
    let removal_actions = crate::collect_fixture_actions(&mut removal_input);
    assert!(removal_actions.is_empty(), "node removal cannot synthesize PointerUp or selection");
    assert!(!crate::interpreter::scene_pointer_target_is_live(&successor_owner), "acknowledgement revokes the removed Map receiver");
    let mut retirement_input = publish_retained_map_fixture(&removed, surface_id, bounds, 905);
    assert!(crate::collect_fixture_actions(&mut retirement_input).is_empty());
    assert!(!crate::scenes::tiled_map_drag_active(&successor_owner.host_id), "Retire clears the removed Map gesture");
    close_retained_map_fixture(surface_id);
    while !first.close_step() {}
    while !replacement.close_step() {}
    while !removed.close_step() {}
    assert!(engine_surface_token(&original_owner.host_id).is_none());
    assert!(engine_surface_token(&successor_owner.host_id).is_none());
}

#[test]
fn retained_map_same_host_sibling_rebase_retires_engine_interaction_owner() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).expect("scene pointer owner contract");
    let fixture = fixture();
    let law = &contract["siblingInsertion"];
    let receiver = law["receiver"].as_u64().expect("receiver protocol id");
    let window_id = "map.lifecycle.sibling-rebase";
    let wire_surface_id = "map.lifecycle.shared-wire";
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let map = tiled_map_scene(wire_surface_id, &fixture).tiled_map.expect("fixture Map scene");
    let mut documents = Vec::new();
    let mut presented = None;
    let mut accepted = None;
    let mut hosts = Vec::new();
    for (index, ids) in law["records"].as_array().expect("sibling histories").iter().enumerate() {
        let ids: Vec<_> = ids.as_array().expect("protocol node ids").iter().map(|id| id.as_u64().expect("protocol node id")).collect();
        let mut records = vec![retained_map_row_record(1, &ids)];
        records.extend(ids.iter().map(|id| retained_map_record(*id, &format!("map-{id}"), &map)));
        let document = retained_document(window_id, index as u64 + 1, 1, records);
        let mut input = paint_retained_map_document(&document, window_id, bounds);
        crate::interpreter::begin_accessibility_visible_documents();
        crate::interpreter::note_accessibility_visible_document(window_id);
        let registrations = crate::interpreter::register_clipped_retained_hit_targets(window_id, bounds, &mut input);
        for (_, target, _) in &registrations {
            if let Some(target) = target {
                if !hosts.contains(&target.host_id) {
                    hosts.push(target.host_id.clone());
                }
            }
        }
        let witness = 840 + index as u64;
        assert!(crate::interpreter::seal_presented_input_candidate(witness));
        assert!(crate::interpreter::acknowledge_presented_input(witness));
        input.publish_hits();
        crate::interpreter::publish_accessibility_visible_documents();
        if index > 0 {
            let (target, rect) = registrations.into_iter().find_map(|(_, target, rect)| target.filter(|target| target.document_id == ui_contract::UiNodeId(receiver)).map(|target| (target, rect))).expect("the receiver Map remains mounted");
            assert!(crate::interpreter::scene_pointer_target_is_live(&target), "the accepted receiver registration names the live Map host");
            if index == 1 {
                let mut interaction = InputState::<ActionDescriptor>::default();
                assert_eq!(crate::scenes::tiled_map_pointer_down_into(&target, "map.lifecycle.fixture", rect, rect.x + rect.w * 0.5, rect.y + rect.h * 0.5, 0, false, false, "rectangle", &mut interaction), Ok(true));
                assert!(crate::scenes::tiled_map_drag_active(&target.host_id));
                presented = Some(target);
            } else {
                accepted = Some(target);
            }
        }
        documents.push(document);
    }
    let presented = presented.expect("the displayed Map owns the interaction");
    let accepted = accepted.expect("the accepted sibling candidate retains the Map");
    let engine_retired = retire_map_interaction_owner(&accepted);
    let gesture_retired = crate::scenes::retire_tiled_map_scene_identity(&accepted);
    let gesture_live = crate::scenes::tiled_map_drag_active(&accepted.host_id);
    let duplicate_retired = retire_map_interaction_owner(&presented);
    eprintln!("[DEBUG] Map presented={:?} accepted={:?} engineRetired={engine_retired} gestureRetired={gesture_retired}", presented.node, accepted.node);
    for host in hosts {
        drop_engine_surface(&host);
    }
    assert!(crate::interpreter::request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !crate::interpreter::ui_document_close_pending() {
            break;
        }
        crate::interpreter::close_ui_document_one();
        crate::os_host::finish_retired_cpu_only_map_fixture_close();
    }
    assert!(!crate::interpreter::ui_document_close_pending(), "the retained Map window returns every bounded owner");
    for mut document in documents {
        while !document.close_step() {}
    }
    assert!(presented.same_component_host(&accepted));
    assert_ne!(presented.node, accepted.node, "the fixture exercises a real arena-node rebase");
    assert!(engine_retired, "the accepted same-host identity retires the EngineCanvas interaction owner");
    assert!(gesture_retired, "the accepted same-host identity retires the local Map gesture");
    assert!(!gesture_live, "the accepted retirement clears the local Map gesture");
    assert!(!duplicate_retired, "the same retirement already consumed the EngineCanvas interaction owner");
}

/// 🔗️ `map_tile_url` is React's own `urlTemplate.replace("{z}", …).replace("{x}", …).replace("{y}", …)`.
#[test]
fn map_tile_url_substitutes_the_same_three_placeholders_react_does() {
    assert_eq!(map_tile_url("/osm/{z}/{x}/{y}.png", 14, 8192, 5461), "/osm/14/8192/5461.png");
    assert_eq!(map_tile_url("/vt/{z}/{x}/{y}.pbf", 0, 0, 0), "/vt/0/0/0.pbf");
    assert_eq!(map_tile_url("/no/placeholders", 3, 4, 5), "/no/placeholders", "a template without placeholders is passed through, as in JS");
}

/// 🧱️ A revision bump taken while the scene bridge's lease is IN FLIGHT must not lose the draws the
/// apply built from that lease.
///
/// 🩸️ `step_world3d_snapshot`'s completion adopted `lease.revision` verbatim, and that lease carries
/// this world's `interaction_revision` as of the bridge PUBLISH. Anything that bumps the revision
/// between the publish and the apply — a document lane the producer republished, a parallel framing,
/// a camera report — therefore rolled the counter BACKWARDS at completion, and the sealed draw
/// rebuild the same apply had just filled answered `WorldDrawRebuildStep::Stale` on its very next
/// turn, which closes it and throws every draw away. Measured on 6118: generation3d's
/// `procedural-preview` sat at `state-meshes=3 state-instances=0 state-draws=0` forever, three
/// `step=Stale` lines after `world3d delivery applied`, with the React twin showing the tessellated
/// column (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14b-generation3d-labels-preview-layout.md`).
#[test]
fn a_revision_bump_while_the_bridge_lease_is_in_flight_still_publishes_the_draws() {
    let fixture = fixture();
    let surface_id = "world3d-inflight-revision";
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };

    // 1️⃣ The first paint STAGES the mesh-wire bridge; this drive seals its lease.
    let mut frame = paint_scene(&scene, bounds, crate::scenes::AdmittedSurfaceMap::default());
    assert!(frame.world3d_states.contains_key(surface_id), "the first painted frame constructs the World3d host");
    drive_world3d_ladder(frame.world3d_states.get_mut(surface_id).expect("attached world state"));

    // 2️⃣ …and only THEN does the producer republish a document lane, which bumps the revision.
    //    The reference plane is `hidden`, so this is a pure revision bump and never a re-framing.
    let mut moved = world3d_preview_scene(surface_id, &fixture);
    moved.world_3d.as_mut().expect("world scene").references_json = Some(json!([{ "id": "plan", "origin": [0.0, 0.0, 0.0], "widthWorld": 4.0, "hidden": true }]).to_string());

    let mut painted = false;
    for turn in 0..16 {
        frame = paint_scene(&moved, bounds, frame.world3d_states);
        if frame.draw.scene_passes.first().is_some_and(|pass| !pass.draws.is_empty()) {
            painted = true;
            break;
        }
        drive_world3d_ladder(frame.world3d_states.get_mut(surface_id).expect("attached world state"));
        assert!(turn < 15, "the preview settled into a drawn frame within its frame ceiling");
    }
    let state = frame.world3d_states.get(surface_id).expect("attached world state");
    assert_eq!(state.snapshot_fault(), None, "a revision bump is not a snapshot fault");
    assert!(painted, "the bridged draws survive a revision bump taken while their lease was in flight: {}", state.ingest_census());
    drop_world3d_states(frame.world3d_states);
}
