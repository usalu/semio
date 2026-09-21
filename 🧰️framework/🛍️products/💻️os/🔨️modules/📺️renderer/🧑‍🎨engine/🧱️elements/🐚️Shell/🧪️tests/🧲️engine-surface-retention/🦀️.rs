//! 🧲️ LAW: an engine surface's pointer state is owned by its WINDOW INSTANCE, not by a painted
//! frame. A frame whose paint drain does not mention the node-graph surface must still resolve a
//! pointer into it; the frame in which its window leaves the layout must drop it.
//!
//! The defect: `ShellState::sync_engine_surface_states` evicted every `node_graph_states` /
//! `tiled_map_states` / `board2d_states` entry absent from `take_engine_surface_registrations()`.
//! Retained painting means an unchanged document does not repaint, so the graph fell out of the map
//! in nearly every frame — and `node_graph_pointer_down_into` is only ever reached for a surface
//! that is IN that map, so no press, wheel, context menu, catalogue drop or `Fit graph` control
//! could address the graph at all. `world3d_states` was never evicted that way and its input worked
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-retained-controls-wires-2026-09-13.md`
//! §7.3(b)).
//!
//! Oracle: `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`; its TypeScript twin is
//! `🧑‍🎨engine/🧪️tests/🧲️engine-surface-retention/🟦️.ts`.

use super::*;
use serde_json::Value;

#[test]
fn retained_engine_hit_provenance_reaches_each_dedicated_pointer_and_wheel_route() {
    let fixture = law();
    let case = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == "every-composited-kind-is-retained-alike")
        .expect("cross-kind retained-surface case");
    let rows = case["frames"][0]["drain"].as_array().expect("painted surface rows");
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs = ShellDock::default();
    let mut documents = Vec::new();
    for row in rows {
        let surface = row["surfaceId"].as_str().expect("surface id");
        let window = row["windowId"].as_str().expect("window id");
        let controller = row["controllerId"].as_str().expect("controller id");
        let surface_doc = match row["kind"].as_str().expect("surface kind") {
            "nodeGraph" => {
                let scene: ui_wgpu::wgpu::NodeGraphScene = serde_json::from_value(serde_json::json!({
                    "nodes": [],
                    "edges": [],
                    "viewport": { "x": 0.0, "y": 0.0, "zoom": 1.0 }
                }))
                .expect("NodeGraph pointer fixture decodes");
                ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::NodeGraph, &scene).expect("bounded NodeGraph scene encodes")
            }
            "tiledMap" => {
                let mut scene = ui_wgpu::wgpu::TiledMapScene::base("{}".into(), "{}".into());
                scene.selection_method = row["selectionMethod"].as_str().unwrap_or("rectangle").into();
                ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::TiledMap, &scene).expect("bounded TiledMap scene encodes")
            }
            "board2d" => ui_wgpu::wgpu::encode_surface_doc(
                ui_contract::SurfaceKind::Board2d,
                &ui_wgpu::wgpu::Board2dScene::base(row["fixtureJson"].as_str().unwrap_or("{}").into(), "{}".into(), true),
            )
            .expect("bounded Board2d scene encodes"),
            kind => panic!("unsupported retained surface kind {kind}"),
        };
        let records = vec![super::shell_input_tests::tree_pointer_record(1, surface, ui_contract::Component::Surface(surface_doc), &[], None)];
        let document = shell.publish_surface_records(window, records).expect("component scene document publishes");
        documents.push((surface.to_string(), window.to_string(), controller.to_string(), rect(&row["bounds"]), document));
    }
    shell.dock_window_plan = documents.iter().map(|(_, window, _, bounds, _)| (window.clone(), *bounds)).collect();
    let paint_rows: Vec<_> = documents.iter().map(|(_, window, controller, bounds, document)| (window.as_str(), controller.as_str(), document, *bounds)).collect();
    let mut input = super::shell_input_tests::paint_component_pointer_documents(&mut shell, &paint_rows);
    shell.sync_engine_surface_states();
    let theme = Theme::default();
    for probe in case["expectedResolve"].as_array().unwrap() {
        let surface = probe["surfaceId"].as_str().expect("live surface id");
        let x = probe["at"][0].as_f64().unwrap() as f32;
        let y = probe["at"][1].as_f64().unwrap() as f32;
        let (control, (bounds, target)) = shell
            .retained_scene_hits
            .iter()
            .find(|(_, (_, target))| target.surface_id == surface)
            .map(|(control, retained)| (control.clone(), retained.clone()))
            .unwrap_or_else(|| panic!("accepted ComponentScene hit for {surface}"));
        assert!(shell.node_graph_states.contains_key(&target.host_id) || shell.tiled_map_states.contains_key(&target.host_id) || shell.board2d_states.contains_key(&target.host_id), "{surface} state is keyed by its accepted component host");
        assert_eq!(shell.pointer_owner_at(x, y, &input, &theme), PointerHitOwner::Surface, "{} live {surface}", case["name"]);
        assert!(shell.wheel_reaches_scene_surface(x, y, &input, &theme), "{} live wheel {surface}", case["name"]);
        assert!(!shell.handle_pointer_wheel(x, y, 0.0, 1.0, &mut input), "{} generic retained scroll must yield to {surface}", case["name"]);
        shell.retained_hit_windows.insert(control, ("unrelated-window".into(), bounds));
        assert_eq!(shell.pointer_owner_at(x, y, &input, &theme), PointerHitOwner::Chrome, "{} forged owner {surface}", case["name"]);
    }
    println!("[DEBUG] retained Graph, Map and Board hits reached only their live surface owner's dedicated ingress");
}

fn law() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🧲️engine-surface-retention/🔣️.json")).expect("engine surface retention fixture")
}

fn rect(value: &Value) -> Rect {
    let items = value.as_array().expect("bounds quad");
    Rect { x: items[0].as_f64().expect("bounds x") as f32, y: items[1].as_f64().expect("bounds y") as f32, w: items[2].as_f64().expect("bounds w") as f32, h: items[3].as_f64().expect("bounds h") as f32 }
}

/// 🧩️ One authored drain entry, built as the very `EngineSurfaceRegistration` the paint pass pushes.
fn registration(value: &Value) -> crate::engine_canvas::EngineSurfaceRegistration {
    let kind = value["kind"].as_str().expect("surface kind");
    let detail = match kind {
        "nodeGraph" => crate::engine_canvas::EngineSurfaceKindDetail::NodeGraph,
        "tiledMap" => crate::engine_canvas::EngineSurfaceKindDetail::TiledMap { selection_method: value["selectionMethod"].as_str().unwrap_or("rectangle").to_string() },
        "board2d" => crate::engine_canvas::EngineSurfaceKindDetail::Board2d { fixture_json: value["fixtureJson"].as_str().unwrap_or_default().to_string() },
        "world3d" => crate::engine_canvas::EngineSurfaceKindDetail::World3d { status_json: value["statusJson"].as_str().map(str::to_owned) },
        other => panic!("unknown surface kind {other}"),
    };
    crate::engine_canvas::EngineSurfaceRegistration {
        host_id: value["hostId"].as_str().unwrap_or_else(|| value["surfaceId"].as_str().expect("surface id")).to_string(),
        surface_id: value["surfaceId"].as_str().expect("surface id").to_string(),
        engine_token: None,
        window_id: value["windowId"].as_str().expect("window id").to_string(),
        bounds: rect(&value["bounds"]),
        controller_id: value["controllerId"].as_str().expect("controller id").to_string(),
        detail,
        created: value["created"].as_bool().unwrap_or(false),
    }
}

/// 🗺️ The three bespoke pointer-state maps the OS event loop hit-tests, driven through the SAME
/// production body `sync_engine_surface_states` drives — never a second derivation of the rule.
#[derive(Default)]
struct SurfaceMirror {
    node_graph_states: AdmittedSurfaceMap<NodeGraphSurface>,
    tiled_map_states: AdmittedSurfaceMap<TiledMapSurface>,
    board2d_states: AdmittedSurfaceMap<Board2dSurface>,
    world3d_status: HashMap<String, String>,
    world3d_window_ids: HashMap<String, String>,
}

impl SurfaceMirror {
    fn frame(&mut self, frame: &Value) -> Vec<String> {
        let registrations: Vec<crate::engine_canvas::EngineSurfaceRegistration> = frame["drain"].as_array().expect("frame drain").iter().map(registration).collect();
        let live: Vec<String> = frame["liveWindows"].as_array().expect("live windows").iter().map(|id| id.as_str().expect("window id").to_string()).collect();
        let live: Vec<&str> = live.iter().map(String::as_str).collect();
        ShellState::mirror_engine_surface_states(&mut self.node_graph_states, &mut self.tiled_map_states, &mut self.board2d_states, &mut self.world3d_status, &mut self.world3d_window_ids, registrations, &live)
    }

    /// 🖱️ Exactly the resolution the OS event loop performs: the first surface, in any bespoke map,
    /// whose retained bounds contain the point.
    fn resolve(&self, x: f32, y: f32) -> Option<(String, String)> {
        if let Some((id, surface)) = self.node_graph_states.iter().find(|(_, surface)| surface.bounds.contains(x, y)) {
            return Some((id.clone(), surface.controller_id.clone()));
        }
        if let Some((id, surface)) = self.tiled_map_states.iter().find(|(_, surface)| surface.bounds.contains(x, y)) {
            return Some((id.clone(), surface.controller_id.clone()));
        }
        self.board2d_states.iter().find(|(_, surface)| surface.bounds.contains(x, y)).map(|(id, surface)| (id.clone(), surface.controller_id.clone()))
    }
}

fn run_case(case: &Value) {
    let name = case["name"].as_str().expect("case name");
    let mut mirror = SurfaceMirror::default();
    let mut created_per_frame: Vec<Vec<String>> = Vec::new();
    for frame in case["frames"].as_array().expect("case frames") {
        created_per_frame.push(mirror.frame(frame));
    }
    if let Some(expected) = case["expectedCreatedPerFrame"].as_array() {
        let expected: Vec<Vec<String>> = expected.iter().map(|frame| frame.as_array().expect("frame created").iter().map(|id| id.as_str().expect("surface id").to_string()).collect()).collect();
        assert_eq!(created_per_frame, expected, "{name}: which frame announced a constructed host");
    }
    if let Some(expected) = case["expectedCreated"].as_array() {
        let last = created_per_frame.last().cloned().unwrap_or_default();
        let expected: Vec<String> = expected.iter().map(|id| id.as_str().expect("surface id").to_string()).collect();
        assert_eq!(last, expected, "{name}: the LAST frame's constructed hosts");
    }
    for probe in case["expectedResolve"].as_array().expect("case resolutions") {
        let at = probe["at"].as_array().expect("probe point");
        let x = at[0].as_f64().expect("probe x") as f32;
        let y = at[1].as_f64().expect("probe y") as f32;
        match probe["surfaceId"].as_str() {
            Some(surface_id) => {
                let resolved = mirror.resolve(x, y).unwrap_or_else(|| panic!("{name}: a pointer at ({x}, {y}) resolved to no engine surface at all"));
                assert_eq!(resolved.0, surface_id, "{name}: which surface a pointer at ({x}, {y}) reaches");
                if let Some(controller_id) = probe["controllerId"].as_str() {
                    assert_eq!(resolved.1, controller_id, "{name}: which controller answers for ({x}, {y})");
                }
            }
            None => assert_eq!(mirror.resolve(x, y), None, "{name}: a pointer at ({x}, {y}) must reach nothing"),
        }
    }
}

#[test]
fn an_engine_surface_is_retained_by_its_window_instance_not_by_a_painted_frame() {
    let law = law();
    let cases = law["cases"].as_array().expect("fixture cases");
    assert_eq!(cases.len(), 6, "every authored retention case runs");
    for case in cases {
        run_case(case);
    }
}

/// 🌍️ The parity statement the whole oracle is derived from: `world3d_states` is NOT a drain
/// projection, so the mirror never touches it. If a future edit ever routes World3d through the
/// eviction path, this fails before the graph's does.
#[test]
fn the_mirror_never_evicts_a_world3d_surface() {
    let mut world3d_status: HashMap<String, String> = HashMap::new();
    let mut world3d_window_ids: HashMap<String, String> = HashMap::new();
    let mut node_graph_states = AdmittedSurfaceMap::<NodeGraphSurface>::default();
    let mut tiled_map_states = AdmittedSurfaceMap::<TiledMapSurface>::default();
    let mut board2d_states = AdmittedSurfaceMap::<Board2dSurface>::default();
    let registration = crate::engine_canvas::EngineSurfaceRegistration {
        host_id: "scene.1.0.0.1".into(),
        surface_id: "procedural-preview-world".into(),
        engine_token: None,
        window_id: "procedural-preview".into(),
        bounds: Rect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 },
        controller_id: "procedural".into(),
        detail: crate::engine_canvas::EngineSurfaceKindDetail::World3d { status_json: Some("{\"phase\":\"evaluating\"}".into()) },
        created: true,
    };
    ShellState::mirror_engine_surface_states(&mut node_graph_states, &mut tiled_map_states, &mut board2d_states, &mut world3d_status, &mut world3d_window_ids, vec![registration], &["procedural-preview"]);
    assert_eq!(world3d_status.get("scene.1.0.0.1").map(String::as_str), Some("{\"phase\":\"evaluating\"}"));
    ShellState::mirror_engine_surface_states(&mut node_graph_states, &mut tiled_map_states, &mut board2d_states, &mut world3d_status, &mut world3d_window_ids, Vec::new(), &["procedural-preview"]);
    assert_eq!(world3d_status.get("scene.1.0.0.1").map(String::as_str), Some("{\"phase\":\"evaluating\"}"), "a frame that did not repaint the preview does not retire its compute status");
}
