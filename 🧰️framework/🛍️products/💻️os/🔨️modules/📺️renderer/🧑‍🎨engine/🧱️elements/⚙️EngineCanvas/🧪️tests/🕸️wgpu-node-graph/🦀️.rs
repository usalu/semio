//! 🕸️ Wgpu node-graph attach — the production path that constructs a `NodeGraphEngine` behind a
//! `SurfaceKind::NodeGraph` window, feeds it the scene, paints it through the host's own vector
//! renderer, and routes pointer/wheel input into the `graph` interaction domain.
//!
//! The scene is generation3d's own Flow window payload for the bundled `hexagonal-mushroom-column`
//! example (7 widgets, 6 synapses) — committed as data in `🔣️.json` rather than reached through the
//! plugin crate, and re-validated field by field by `FlowHost::parse_host_snapshot_json` in every lane
//! below. The action payloads are pinned against React's `NodeGraphHost`, the other implementation of
//! this same host (`🧱️elements/🕸️NodeGraph/🟦️.tsx` — `nodeGraphSelectionActionArgs`,
//! `nodeGraphHoverActionArgs`, `nodeGraphViewportActionArgs`).

use super::*;
use semio_framework_os_kernel::Viewport2d;
use ui_wgpu::wgpu::{DrawList, FontAtlas, IconAtlas, InputState, NodeGraphScene, SurfaceKind, UiPresence};

const NODE_GRAPH_SCENE_FIXTURE: &str = include_str!("../../🧫️fixtures/🕸️wgpu-node-graph/🔣️.json");

pub(super) fn hexagonal_mushroom_column_fixture_json() -> String {
    let value: Value = serde_json::from_str(NODE_GRAPH_SCENE_FIXTURE).expect("committed generation3d flow fixture parses");
    serde_json::to_string(value.get("hostSnapshot").expect("host document payload")).expect("host document re-encodes")
}

pub(super) fn flow_window_scene(surface_id: &str) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "generation3d".into(),
        component_kind: SurfaceKind::NodeGraph,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        menu: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: Some(NodeGraphScene {
            editable: Some(true),
            host_snapshot_json: Some(hexagonal_mushroom_column_fixture_json()),
            capabilities_json: Some(json!({ "engine": "flow", "spotlight": true }).to_string()),
            lod_json: Some(json!({ "automatic": true }).to_string()),
            ..NodeGraphScene::base(Vec::new(), Vec::new(), Viewport2d { x: 94.755_815_717_374_45, y: -97.508_331_346_796_68, zoom: 1.784_432_561_601_109_9 })
        }),
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

/// 🧹️ Drains one attached surface through the registry's OWN retirement ladder. `FlowHostSnapshot`'s
/// ordered maps refuse to drop unretired, so a plain `remove` panics inside the registry mutex and
/// poisons it for every later lane — the ladder is the only correct teardown.
pub(super) fn drop_engine_surface(surface_id: &str) {
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

fn paint_scene_into_draw_list(scene: &UiComponentSceneNode, bounds: Rect) -> DrawList {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut world3d_states = crate::scenes::AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: "law-window" };
        let mut cursor = ui_wgpu::wgpu::ScenePaintCursor::default();
        for _ in 0..4096 {
            match crate::scenes::render_component_scene_step(scene, bounds, &mut ctx, &mut cursor, &mut hosts) {
                ui_wgpu::wgpu::ScenePaintStep::Pending => continue,
                ui_wgpu::wgpu::ScenePaintStep::Complete => break,
                ui_wgpu::wgpu::ScenePaintStep::Fault => panic!("node-graph scene paint faulted"),
            }
        }
    }
    draw
}

/// 🔤️ One action's string fields, key-sorted — the emitted value contract is order-preserving, and a
/// key-order assertion would pin the writer's statement order rather than the payload React sends.
pub(super) fn action_fields(action: &ActionDescriptor) -> Vec<(String, String)> {
    let mut fields: Vec<(String, String)> = action
        .args
        .as_ref()
        .and_then(dsl::DslValue::as_object)
        .expect("action args are an object")
        .iter()
        .map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_owned()))
        .collect();
    fields.sort_by(|left, right| left.0.cmp(&right.0));
    fields
}

pub(super) fn entity_screen_rect(surface_id: &str, domain: &str, id: &str) -> [f64; 4] {
    let geometry = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id)?.node_graph.as_ref() else { return None };
        serde_json::from_str::<Value>(&host.entity_screen_json(domain, id)).ok()
    })
    .expect("live flow host answers the entity's screen geometry");
    assert_eq!(geometry.get("visible").and_then(Value::as_bool), Some(true), "{domain} {id} is on screen for the fixture camera");
    let rect = geometry.get("rect").and_then(Value::as_array).expect("entity screen rect").iter().filter_map(Value::as_f64).collect::<Vec<_>>();
    [rect[0], rect[1], rect[2], rect[3]]
}

pub(super) fn node_center_screen(surface_id: &str, node_id: &str, inner: Rect) -> (f32, f32) {
    let rect = entity_screen_rect(surface_id, "node", node_id);
    (inner.x + (rect[0] + rect[2] * 0.5) as f32, inner.y + (rect[1] + rect[3] * 0.5) as f32)
}

/// 🔌️ The screen point of one input channel's handle dot: the engine hit-tests a handle against the
/// dot on the node's own boundary (`hit_test_pick_targets`, radius + 6), not against the port row —
/// so the pick point is the node rect's left edge at the row's vertical centre.
pub(super) fn input_handle_screen(surface_id: &str, node_id: &str, channel_id: &str, inner: Rect) -> (f32, f32) {
    let node = entity_screen_rect(surface_id, "node", node_id);
    let row = entity_screen_rect(surface_id, "handle", channel_id);
    (inner.x + node[0] as f32, inner.y + (row[1] + row[3] * 0.5) as f32)
}

#[test]
fn node_graph_window_attaches_the_flow_engine_and_paints_a_non_empty_draw_list() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "node-graph-attach-draw";
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    let bounds = Rect { x: 24.0, y: 36.0, w: 966.0, h: 836.0 };

    let draw = paint_scene_into_draw_list(&scene, bounds);

    let (nodes, edges, engine_is_flow) = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id).expect("attach published the engine surface");
        match entry.node_graph.as_ref().expect("attach constructed the node-graph engine") {
            NodeGraphEngine::Flow(host) => (host.dag.host_snapshot.nodes.len(), host.dag.host_snapshot.edges.len(), true),
            NodeGraphEngine::Dag(host) => (host.dag.host_snapshot.nodes.len(), host.dag.host_snapshot.edges.len(), false),
        }
    });
    assert!(engine_is_flow, "a scene carrying hostSnapshotJson selects the flow engine, exactly as React's NodeGraphHost does");
    assert_eq!((nodes, edges), (7, 6), "the hexagonal-mushroom-column fixture reaches the engine whole");

    let key = engine_raster_key(surface_id).expect("bounded engine raster key");
    let raster = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).find(|(instance_key, _)| instance_key == &key);
    assert!(raster.is_some(), "the painted graph is composited into the window's draw list under {key}");

    let mut resources = EngineCanvasBuildContext::default();
    let mut turns = 0;
    while !stage_engine_packet_step(&mut resources) {
        turns += 1;
        assert!(turns < 512, "the staged paint reaches the frame's packet ledger");
    }
    let mut packet = resources.take_packet_step().unwrap_or_else(|_| panic!("ready packet destination")).expect("the attach staged one engine packet");
    assert!(!packet.scene.retirement_is_empty(), "the flow host painted real vector commands, not an empty scene");
    assert_eq!((packet.width, packet.height), (966, 836), "the packet is sized to the window body");
    let mut retirement_turns = 0usize;
    while !packet.close_step() {
        retirement_turns += 1;
        assert!(retirement_turns < 262_144, "the transferred engine packet reaches terminal release");
    }
    assert!(packet.terminal_is_empty());
    assert!(resources.terminal_is_empty());
    drop(packet);
    drop(resources);
    drop_engine_surface(surface_id);
}

#[test]
fn pointer_down_on_a_node_emits_the_graph_domain_selection_react_dispatches() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "node-graph-attach-pick";
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    assert!(sync_node_graph_scene(&scene, "law-window", bounds, Theme::default().panel), "attach");

    let (x, y) = node_center_screen(surface_id, "extrude", bounds);
    let actions = node_graph_pointer_down(surface_id, &scene.controller_id, bounds, x, y, 0, false, false, false, false);

    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("pointer-down publishes interactionSelect");
    assert_eq!(select.controller_id, "generation3d");
    assert_eq!(
        action_fields(select),
        vec![
            ("domainId".to_owned(), "graph".to_owned()),
            ("merge".to_owned(), "replace".to_owned()),
            ("method".to_owned(), "pick".to_owned()),
            ("targets".to_owned(), r#"[{"granularity":"node","id":"extrude"}]"#.to_owned()),
        ],
        "byte-identical to React's nodeGraphSelectionActionArgs with nodeIds [extrude]"
    );
    let hover = actions.iter().find(|action| action.action == "interactionHover").expect("pointer-down publishes interactionHover");
    assert_eq!(
        action_fields(hover),
        vec![
            ("channel".to_owned(), "pointer".to_owned()),
            ("domainId".to_owned(), "graph".to_owned()),
            ("targets".to_owned(), r#"[{"granularity":"node","id":"extrude"}]"#.to_owned()),
        ],
        "byte-identical to React's nodeGraphHoverActionArgs(\"extrude\") - a node-body pick carries no channel, so the target stays node-granular"
    );

    let (px, py) = input_handle_screen(surface_id, "extrude", "extrude@wire", bounds);
    let moved = node_graph_pointer_move(surface_id, &scene.controller_id, bounds, px, py, false, false, false);
    let channel_hover = moved.iter().find(|action| action.action == "interactionHover").expect("pointer-move publishes interactionHover");
    assert_eq!(
        action_fields(channel_hover),
        vec![
            ("channel".to_owned(), "pointer".to_owned()),
            ("domainId".to_owned(), "graph".to_owned()),
            ("targets".to_owned(), r#"[{"granularity":"handle","id":"extrude@wire"}]"#.to_owned()),
        ],
        "byte-identical to React's nodeGraphHoverActionArgs(\"extrude\", \"wire\") - a port pick qualifies the target by channel"
    );
    drop_engine_surface(surface_id);
}

#[test]
fn wheel_zoom_emits_the_node_graph_viewport_action_with_the_moved_camera() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "node-graph-attach-viewport";
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    assert!(sync_node_graph_scene(&scene, "law-window", bounds, Theme::default().panel), "attach");

    let actions = node_graph_wheel(surface_id, &scene.controller_id, bounds, 480.0, 400.0, -120.0, false);
    let viewport = actions.iter().find(|action| action.action == "nodeGraphViewport").expect("wheel publishes nodeGraphViewport");

    let committed = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id)?.node_graph.as_ref() else { return None };
        Some([host.host_snapshot.camera.x, host.host_snapshot.camera.y, host.host_snapshot.camera.zoom])
    })
    .expect("live flow host");
    assert!(committed[2] > 1.784_432_561_601_109_9, "a wheel-up zooms in from the fixture camera, got {}", committed[2]);
    assert_eq!(
        viewport.args,
        semio_framework::optional_json_to_dsl(Some(json!({
            "surfaceId": surface_id,
            "viewport": { "x": committed[0], "y": committed[1], "zoom": committed[2] },
        }))),
        "the published viewport is exactly the camera the host committed"
    );
    drop_engine_surface(surface_id);
}

/// 🖼️ `Fit graph` publishes the camera it COMPUTED, not the one the scene shipped with. The control
/// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `shell.nodeGraph.fit::`) persists whatever
/// `node_graph_fit_camera` answers through `nodeGraphViewport`, so answering the pre-fit camera
/// re-publishes it forever — which is exactly what a Flow surface did while the fit landed on the
/// dag's derived paint copy. The shared rows live in
/// `♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json` (`surfaceRows`); the host half of the law is
/// `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, `a_fitted_flow_surface_publishes_the_camera_it_computed`.
/// Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
#[test]
fn fit_graph_answers_the_fitted_camera_not_the_scene_camera() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "node-graph-attach-fit";
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    let bounds = Rect { x: 0.0, y: 0.0, w: 483.0, h: 814.0 };
    assert!(sync_node_graph_scene(&scene, "law-window", bounds, Theme::default().panel), "attach");

    let scene_camera = [94.755_815_717_374_45, -97.508_331_346_796_68, 1.784_432_561_601_109_9];
    let fitted = crate::engine_canvas::node_graph_fit_camera(surface_id).expect("a live graph can be framed");
    assert_ne!(fitted, scene_camera, "the fit must answer a camera it computed, not the scene's own");

    let (published, painted, coverage) = ENGINE_SURFACES
        .with(|cell| {
            let map = cell.borrow();
            let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id)?.node_graph.as_ref() else { return None };
            Some((host.camera(), [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom], host.dag.camera_content_coverage()))
        })
        .expect("live flow host");
    assert_eq!(fitted, published, "the answer the control persists is the surface's own published camera");
    assert_eq!(published, painted, "the published camera and the painted camera are one camera");
    assert!((coverage - 1.0).abs() <= 1e-6, "a fitted camera frames the whole graph, got {coverage}");
    drop_engine_surface(surface_id);
}

#[test]
fn projection_snapshot_rejects_invalid_node_graph_viewports() {
    let _serialized = engine_surface_law_guard();
    for camera in [[0.0, 0.0, 0.0], [f64::NAN, 0.0, 1.0], [0.0, f64::INFINITY, 1.0]] {
        assert!(matches!(
            graph_projection_snapshot(Vec::new(), None, None, camera),
            Err(ui_wgpu::wgpu::BoundedActionFault::Structure)
        ));
    }
}

/// 🏷️ The node and port CAPTIONS React prints inside every node body (`Polygon`, `ExtrudeCurve`,
/// `! wire`, `? x` …). They are OVERLAY glyphs the shell paints itself after the engine's own vector
/// raster, because the raster is a texture the flow host renders without text.
///
/// 🩸️ Two independent ways this lane can be dead, and the live generation3d flow window on 6118 was
/// the proof that it WAS: the label state can come back empty, or the glyphs can be painted into the
/// SAME draw layer as the engine raster — which the encoder walks `ui → overlay_ui → vector →
/// overlay_vector → raster`, so the opaque graph texture is composited ON TOP of every caption.
/// The window then shows node bodies, ports, wires and the minimap and not one word
/// (`📓️w13e-other-playgrounds-wgpu.md` §2.3.1).
#[test]
fn node_graph_paint_publishes_its_captions_over_the_engine_raster() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "node-graph-attach-labels";
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    let bounds = Rect { x: 3.0, y: 54.0, w: 975.0, h: 814.0 };

    let draw = paint_scene_into_draw_list(&scene, bounds);

    let rows: Value = ENGINE_SURFACES
        .with(|cell| {
            let map = cell.borrow();
            let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id)?.node_graph.as_ref() else { return None };
            serde_json::from_str(&host.label_overlay_paint_state_json().ok()?).ok()
        })
        .expect("the live flow host answers its label overlay state");
    let labels = rows.get("labels").and_then(Value::as_array).expect("label rows").len();
    assert!(labels >= 20, "the seven-node fixture publishes a caption per node and per port, got {labels} at lod {:?}", rows.get("lod"));

    let key = engine_raster_key(surface_id).expect("bounded engine raster key");
    let raster_layer = draw
        .layers
        .iter()
        .position(|layer| layer.raster_instances.iter().any(|(instance_key, _)| instance_key == &key))
        .expect("the painted graph is composited into the window's draw list");
    let caption_layer = draw.layers.iter().position(|layer| !layer.overlay_ui_instances.is_empty()).expect("the caption overlay published glyph instances");
    assert!(caption_layer > raster_layer, "captions paint in a LATER layer than the engine raster ({caption_layer} vs {raster_layer}), or the opaque texture covers them");
    let glyphs: usize = draw.layers.iter().map(|layer| layer.overlay_ui_instances.len()).sum();
    assert!(glyphs >= labels, "every published caption reaches the draw list as glyphs, got {glyphs} for {labels} captions");
    drop_engine_surface(surface_id);
}
