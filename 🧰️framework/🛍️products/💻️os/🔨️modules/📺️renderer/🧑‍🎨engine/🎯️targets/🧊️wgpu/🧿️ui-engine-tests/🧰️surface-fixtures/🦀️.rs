//! 🧰️ Retained-surface helpers of the EngineCanvas laws: paint a scene through the retained document and the Interpreter admission seam, then close it through the production ladder. Shared by the engine-surfaces, node-graph and paint2d laws, which live in two bins.

use infinite_world::world::World3dBuildContext;
use infinite_world::world::World3dState;
use infinite_world::world::WorldCursorWakeAuthority;
use infinite_world::world::begin_world3d_dynamic_retirement;
use infinite_world::world::step_world3d_dynamic_retirement;
use infinite_world::world::world3d_dynamic_retirement_terminal_is_empty;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap;
use semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget;
use semio_framework_os_renderer_wgpu_engine_registry::with_engine_close_context;
use semio_framework_os_renderer_wgpu_host_winit::os_host;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading;
use semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels;
use semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::UiDocumentFrameCursor;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::render_ui_document_step;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_draw_contract::IconAtlas;
use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_theme::geometry::Rect;
use std::collections::HashMap;
use ui_contract::SurfaceId;
use ui_contract::UiDocumentLease;
use ui_contract::UiText;
use semio_framework_ui_wgpu_input::InputState;
use semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;

pub fn retained_surface_record(scene: &UiComponentSceneNode) -> ui_contract::UiNodeRecord {
    let component = match scene.component_kind {
        SurfaceKind::NodeGraph => ui_contract::Component::Surface(semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::NodeGraph, scene.node_graph.as_ref().expect("NodeGraph scene payload")).expect("bounded NodeGraph scene encodes")),
        SurfaceKind::TiledMap => ui_contract::Component::Surface(semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::TiledMap, scene.tiled_map.as_ref().expect("TiledMap scene payload")).expect("bounded TiledMap scene encodes")),
        SurfaceKind::Board2d => ui_contract::Component::Surface(semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::Board2d, scene.board2d.as_ref().expect("Board2d scene payload")).expect("bounded Board2d scene encodes")),
        SurfaceKind::TextEditor => ui_contract::Component::Surface(semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::TextEditor, scene.text_editor.as_ref().expect("TextEditor scene payload")).expect("bounded TextEditor scene encodes")),
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

pub fn retained_document(surface_id: &str, generation: u64, root: u64, records: Vec<ui_contract::UiNodeRecord>) -> ui_contract::UiDocumentLease {
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(generation), root: Some(ui_contract::UiNodeId(root)), layout_epoch: 0 };
    ui_contract::UiDocumentLease::try_publish(ui_contract::SurfaceId::try_from(surface_id).expect("bounded Map surface id"), identity, records).expect("Map lifecycle document publishes")
}

pub fn close_retained_surface_fixture(window_id: &str) {
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
            break;
        }
        caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn");
        semio_framework_os_renderer_wgpu_host_winit::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending(), "the EngineCanvas fixture returns every retained owner");
    caller.finish();
}

/// 🧹️ Drains every attached `World3dState` through its OWN dynamic retirement ladder. The state's
/// `WorldDynamicRegistry`s refuse to drop unretired, so letting the map fall out of scope aborts the
/// whole test binary in a destructor — the ladder is the only correct teardown, exactly as
/// `close_engine_surface_step` is for the vello-composited kinds.
pub fn drop_world3d_states(mut states: semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap<World3dState>) {
    while let Some(owner) = states.close_step() {
        let mut state = owner.value;
        begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
        let mut turns = 0usize;
        while !world3d_dynamic_retirement_terminal_is_empty(&state) {
            with_world_step(64, |context| {
                let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
                let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,terrain_grant,context);
                assert!(terrain.ownership.fits(terrain_grant));
                assert!(!matches!(terrain.step,infinite_world::world::WorldTerrainMeshPublicationStep::Fault(_)));
                if terrain.step==infinite_world::world::WorldTerrainMeshPublicationStep::Idle {step_world3d_dynamic_retirement(&mut state,context);}
                true
            });
            turns += 1;
            assert!(turns < 262_144, "the attached world state reaches a fixed terminal witness");
        }
    }
}

pub struct RetainedSurfacePaint {
    pub draw: DrawList,
    pub owner: semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget,
}

/// 🪟️ Drives an EngineCanvas scene through the retained document and Interpreter admission seam.
pub fn paint_retained_surface_scene(scene: &UiComponentSceneNode, bounds: Rect) -> RetainedSurfacePaint {
    paint_retained_surface_in_window(scene, bounds, &scene.surface_id)
}

pub fn paint_retained_surface_in_window(scene: &UiComponentSceneNode, bounds: Rect, window_id: &str) -> RetainedSurfacePaint {
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let document = retained_document(window_id, 1, 1, vec![retained_surface_record(scene)]);
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
        let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id };
        let done = caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::render_ui_document_step(&mut cursor, &document, bounds, &mut ctx, window_id, &scene.controller_id, semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle, &mut hosts, cx));
        semio_framework_os_renderer_wgpu_host_winit::os_host::finish_cpu_only_fixture_component_close(|owner| drop_engine_surface(&owner.host_id));
        assert!(done || !cursor.terminal_is_fault(), "retained EngineCanvas document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "retained EngineCanvas document painted within its opportunity ceiling");
    drop_world3d_states(world3d_states);
    let owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(window_id, bounds.x + 1.0, bounds.y + 1.0).expect("retained EngineCanvas surface exposes its exact mounted target");
    assert_eq!(owner.surface_id, window_id);
    assert_eq!(owner.kind, scene.component_kind);
    caller.retire_world(&mut world_resources);
    caller.finish();
    RetainedSurfacePaint { draw, owner }
}

pub fn with_world_step(fuel: u64, apply: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> bool) -> bool {
    with_engine_close_context(fuel, apply)
}
