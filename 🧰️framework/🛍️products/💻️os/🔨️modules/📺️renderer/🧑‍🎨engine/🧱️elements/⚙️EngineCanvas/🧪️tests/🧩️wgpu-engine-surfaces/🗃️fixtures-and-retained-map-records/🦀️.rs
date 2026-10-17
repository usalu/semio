/// 🗃️ Fixtures and retained map records: the engine surface fixture constants, the scene builders and the retained map, separator and row records with their painting and publishing helpers.

const ENGINE_SURFACES_FIXTURE: &str = include_str!("../../🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json");
const MAP_GESTURE_LIFECYCLE_FIXTURE: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/♻️tiled-map-gesture-lifecycle/🔣️.json");
const SURFACE_PINCH_FIXTURE: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🤏️surface-pinch/🔣️.json");
const MAP_OVERLAY_DESCRIPTOR_FIXTURE: &str = include_str!("../../../../🧭️TiledMapHost/🧪️tests/🧫️fixtures/🗺️overlay-descriptor/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(ENGINE_SURFACES_FIXTURE).expect("committed engine-surface fixture parses")
}

fn map_overlay_fixture() -> Value {
    serde_json::from_str(MAP_OVERLAY_DESCRIPTOR_FIXTURE).expect("committed map overlay descriptor fixture parses")
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
    let overlay = map_overlay_fixture();
    let mut scene = scene_shell(surface_id, SurfaceKind::TiledMap);
    scene.tiled_map = Some(TiledMapScene { selection_json: json_text(&overlay["selection"]), hover_json: json_text(&overlay["hover"]), ..TiledMapScene::base(json_text(&overlay["mapDescriptor"]), json_text(&fixture["tiledMap"]["cameraJson"])) });
    scene
}

fn retained_map_record(id: u64, key: &str, scene: &TiledMapScene) -> ui_contract::UiNodeRecord {
    let mut record = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded map node key"),
        component: ui_contract::Component::Surface(semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::TiledMap, scene).expect("bounded Map scene encodes")),
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

fn map_fixture_owner(window_id: &str, surface_id: &str) -> semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget {
    semio_framework_os_renderer_wgpu_contracts::fixture_scene_pointer_target(window_id, surface_id, "map.fixture")
}

fn paint_retained_map_document(document: &ui_contract::UiDocumentLease, surface_id: &str, bounds: Rect) -> InputState<ActionDescriptor> {
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap::default();
    let mut world_resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut cursor = semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::UiDocumentFrameCursor::default();
    let complete = (0..(1 << 20)).any(|_| {
        let mut ctx = semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, bounds.h);
        let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface_id };
        let done = caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::render_ui_document_step(&mut cursor, document, bounds, &mut ctx, surface_id, "map.lifecycle.fixture", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts, cx));
        semio_framework_os_renderer_wgpu_host_winit::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
        assert!(done || !cursor.terminal_is_fault(), "Map lifecycle document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "Map lifecycle document painted within its opportunity ceiling");
    drop_world3d_states(world3d_states);
    caller.retire_world(&mut world_resources);
    caller.finish();
    input
}

fn publish_retained_map_fixture(document: &ui_contract::UiDocumentLease, window_id: &str, bounds: Rect, witness: u64) -> InputState<ActionDescriptor> {
    let mut input = paint_retained_map_document(document, window_id, bounds);
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_accessibility_visible_document(window_id);
    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::register_clipped_retained_hit_targets(window_id, bounds, &mut input);
    assert!(semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::seal_presented_input_candidate(witness));
    assert!(semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::acknowledge_presented_input(witness));
    input.publish_hits();
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents();
    input
}

fn close_retained_map_fixture(window_id: &str) {
    close_retained_surface_fixture(window_id);
}

fn board2d_scene(surface_id: &str, fixture: &Value) -> UiComponentSceneNode {
    let board = &fixture["board2d"];
    let mut scene = scene_shell(surface_id, SurfaceKind::Board2d);
    scene.board2d = Some(Board2dScene { selection_json: json_text(&board["selectionJson"]), ..Board2dScene::base(json_text(&board["snapshotJson"]), json_text(&board["cameraJson"]), true) });
    scene
}
