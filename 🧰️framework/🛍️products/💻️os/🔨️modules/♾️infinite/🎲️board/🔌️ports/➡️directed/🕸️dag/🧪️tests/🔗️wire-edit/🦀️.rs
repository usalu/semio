//! 🔗️ LAW: a port-to-port pointer gesture creates a wire and journals it for the guest in the
//! guest's own four-id vocabulary; replacing a wire journals the removal too; a minimap click
//! navigates; and the host says which of its two pointer paths owns a given gesture.
//!
//! The defect: `DagHost`'s port/handle hit-test and `InteractionMode::DrawEdge` are only reachable
//! through `pointer_down_screen`/`_move_screen`/`_up_screen`. The wgpu renderer drove ONLY the
//! bounded `plan_pointer`/`commit_pointer` entry, whose `bounded_node_hit_index` answers
//! `DagInteractionPlanFault::Unsupported` for exactly the minimap, handle, port-insert and widget
//! hits — so a press on a port aborted the dispatch with a `Structure` fault instead of starting a
//! wire, and clicking the minimap navigated nowhere (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️audit-wgpu-parity-2026-09-13.md` gaps #2/#5).
//!
//! Oracle: `🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json`; its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts`.

use super::*;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔗️wire-edit/🔣️.json")).expect("wire edit fixture")
}

fn ports(value: &Value, key: &str) -> Vec<IoPortSpec> {
    value[key].as_array().map(|items| items.iter().map(|item| IoPortSpec { id: item.as_str().expect("port id").to_string(), label: item.as_str().expect("port id").to_string(), ..Default::default() }).collect()).unwrap_or_default()
}

/// 🕸️ The oracle's own graph, built through the same node constructor the engine's own tests use.
fn wire_host(law: &Value) -> DagHost {
    let graph = &law["graph"];
    let nodes: Vec<DagNodeSpec> = graph["nodes"]
        .as_array()
        .expect("fixture nodes")
        .iter()
        .map(|node| {
            let id = node["id"].as_str().expect("node id").to_string();
            let label = node["label"].as_str().expect("node label");
            let inputs = ports(node, "inputs");
            let outputs = ports(node, "outputs");
            let width = computation_node_width(label, &inputs, &outputs);
            let height = computation_node_height(inputs.len(), outputs.len(), false, false);
            DagNodeSpec::computation(id, label, label, "emoji:🔢️".into(), inputs, outputs, false, false, node["x"].as_f64().expect("node x"), node["y"].as_f64().expect("node y"), width, height)
        })
        .collect();
    let viewport = &graph["viewport"];
    let mut host = DagHost::from_fixture_without_layout(DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges: vec![] });
    host.set_viewport(viewport["width"].as_u64().expect("viewport width") as u32, viewport["height"].as_u64().expect("viewport height") as u32, viewport["dpr"].as_f64().expect("viewport dpr"));
    host
}

/// 📍️ The SCREEN point a `"nodeId@portId"` endpoint sits at, read off the engine's own render
/// snapshot — never a second derivation of where a port is drawn.
fn port_screen_point(host: &DagHost, endpoint: &str) -> (f64, f64) {
    let handle = host.handle_key_map.iter().find(|(_, key)| key.as_str() == endpoint).map(|(id, _)| *id).unwrap_or_else(|| panic!("no handle for {endpoint}"));
    let world = host.engine.render_snapshot().handles.iter().find(|(id, _, _)| *id == handle).map(|(_, point, _)| *point).unwrap_or_else(|| panic!("no painted handle for {endpoint}"));
    (world.x + f64::from(host.width) * 0.5, world.y + f64::from(host.height) * 0.5)
}

fn draw_wire(host: &mut DagHost, gesture: &Value) {
    let (from_x, from_y) = port_screen_point(host, gesture["from"].as_str().expect("wire from"));
    let (to_x, to_y) = port_screen_point(host, gesture["to"].as_str().expect("wire to"));
    host.pointer_down_screen(from_x, from_y, 0, false, false, false, false);
    host.pointer_move_screen(to_x, to_y, false, false, false);
    host.pointer_up_screen(to_x, to_y, false, false, false);
}

fn edit_rows(edits: &[DagGraphEdit]) -> Vec<Value> {
    edits
        .iter()
        .map(|edit| match edit {
            DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                serde_json::json!({ "operation": "connect", "sourceNodeId": source_node_id, "sourcePortId": source_port_id, "targetNodeId": target_node_id, "targetPortId": target_port_id })
            }
            DagGraphEdit::Disconnect { synapse_id } => serde_json::json!({ "operation": "disconnect", "synapseId": synapse_id }),
            DagGraphEdit::Move { node_id, x, y } => serde_json::json!({ "operation": "move", "nodeId": node_id, "x": x, "y": y }),
        })
        .collect()
}

/// 📐️ The port rect the host itself PUBLISHES for an endpoint, in viewport pixels — the one geometry
/// `entity_screen_json("handle", …)` hands a script, a demonstration or an assistive caller. Its
/// centre is where anything that trusts the host aims.
fn published_port_rect(host: &DagHost, endpoint: &str) -> (f64, f64, f64, f64) {
    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", endpoint)).expect("entity screen json");
    assert_eq!(geometry["visible"].as_bool(), Some(true), "{endpoint} must be published as a visible port");
    let rect = geometry["rect"].as_array().unwrap_or_else(|| panic!("{endpoint} must publish a rect"));
    (rect[0].as_f64().expect("rect x"), rect[1].as_f64().expect("rect y"), rect[2].as_f64().expect("rect w"), rect[3].as_f64().expect("rect h"))
}

fn published_port_centre(host: &DagHost, endpoint: &str) -> (f64, f64) {
    let (x, y, width, height) = published_port_rect(host, endpoint);
    (x + width * 0.5, y + height * 0.5)
}

/// 🗑️ A screen point with no node, no port row and no minimap panel under it at any of the oracle's
/// zoom bands — where a detached wire is dropped.
const EMPTY_CANVAS_SCREEN: (f64, f64) = (8.0, 8.0);

/// 📐️ World point to viewport pixels through the host's own camera — the projection every published
/// rect is reported in.
fn world_point_to_screen(host: &DagHost, point: canvas::Point) -> (f64, f64) {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    let camera = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: host.width, height: host.height, dpr: host.dpr };
    let screen = world_to_screen(&camera, &viewport, point);
    (screen.x, screen.y)
}

#[test]
fn a_port_to_port_drag_creates_a_wire_and_journals_it_for_the_guest() {
    let law = law();
    for case in law["cases"].as_array().expect("fixture cases") {
        let name = case["name"].as_str().expect("case name");
        let gesture = &case["gesture"];
        let mut host = wire_host(&law);
        for setup in case["setup"].as_array().unwrap_or(&Vec::new()) {
            draw_wire(&mut host, setup);
            // 🔗️ A setup wire is the graph the CASE starts from, not part of what it measures.
            let _ = host.take_graph_edits();
        }
        match gesture["kind"].as_str().expect("gesture kind") {
            "wire" => {
                draw_wire(&mut host, gesture);
                let edits = host.take_graph_edits();
                let rows = edit_rows(&edits);
                let expected = case["expectedEdits"].as_array().expect("expected edits");
                for row in expected {
                    assert!(rows.contains(row), "{name}: expected {row} among {rows:?}");
                }
                if let Some(edges) = case["expectedEdges"].as_array() {
                    for edge in edges {
                        let source = edge["source"].as_str().expect("edge source");
                        let target = edge["target"].as_str().expect("edge target");
                        assert!(host.fixture.edges.iter().any(|candidate| candidate.source == source && candidate.target == target), "{name}: {source} -> {target} must be a live wire, got {:?}", host.fixture.edges.iter().map(|edge| (edge.source.as_str(), edge.target.as_str())).collect::<Vec<_>>());
                    }
                    assert_eq!(host.fixture.edges.len(), edges.len(), "{name}: an input port holds exactly one incoming wire");
                }
                if case["expectedSecondDrain"].is_array() {
                    assert!(host.take_graph_edits().is_empty(), "{name}: a drained journal must not report the same wire twice");
                }
            }
            "probe" => {
                let (x, y) = port_screen_point(&host, gesture["at"].as_str().expect("probe endpoint"));
                assert_eq!(host.screen_pointer_gesture_begins_at(x, y), case["expectedScreenPath"].as_bool().expect("expected screen path"), "{name}");
            }
            // 🖐️ The discriminator and the bounded fault set must be the SAME set, for every phase:
            // wherever `derive_pointer_plan` answers `Unsupported`, hover and release over that point
            // belong to the screen path too — the renderer asks `screen_pointer_gesture_begins_at`
            // on all three phases and must never hand one of these points to the bounded path.
            "probePhases" => {
                let (x, y) = match gesture["at"].as_str() {
                    Some(endpoint) => port_screen_point(&host, endpoint),
                    None => (gesture["x"].as_f64().expect("probe x"), gesture["y"].as_f64().expect("probe y")),
                };
                let expected = case["expectedScreenPath"].as_bool().expect("expected screen path");
                assert_eq!(host.screen_pointer_gesture_begins_at(x, y), expected, "{name}: the point's own path");
                let projection = host.bounded_interaction_projection(0).expect("bounded projection");
                for phase in [DagPointerPhase::Down, DagPointerPhase::Move, DagPointerPhase::Up] {
                    let intent = DagPointerIntent { phase, x, y, button: 0, shift: false, ctrl_or_meta: false, alt: false, pan: false };
                    let fault = host.derive_pointer_plan(projection, intent).err();
                    match case["expectedBoundedFault"].as_str() {
                        Some("Unsupported") => assert_eq!(fault, Some(DagInteractionPlanFault::Unsupported), "{name}: the bounded path must refuse {phase:?} here, which is why the screen path owns it"),
                        Some(other) => panic!("{name}: unhandled expected fault {other}"),
                        None => assert_eq!(fault, None, "{name}: the bounded path must describe {phase:?} here without a fault"),
                    }
                }
            }
            "probeScreenPoint" => {
                let x = gesture["x"].as_f64().expect("probe x");
                let y = gesture["y"].as_f64().expect("probe y");
                assert_eq!(host.screen_pointer_gesture_begins_at(x, y), case["expectedScreenPath"].as_bool().expect("expected screen path"), "{name}");
            }
            other => panic!("fixture gesture kind {other}"),
        }
    }
}

/// 🖱️ While a wire is being drawn the host owns the gesture — the bounded plan path must not claim
/// the follow-up move/up and cancel it.
#[test]
fn a_wire_in_flight_holds_the_screen_pointer_path() {
    let law = law();
    let mut host = wire_host(&law);
    assert!(!host.screen_pointer_gesture_active(), "an idle graph is on the bounded path");
    let (from_x, from_y) = port_screen_point(&host, "src@out");
    host.pointer_down_screen(from_x, from_y, 0, false, false, false, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "a press on an output port draws a wire");
    assert!(host.screen_pointer_gesture_active(), "the follow-up move/up belongs to the screen path");
    let (to_x, to_y) = port_screen_point(&host, "tgt@in");
    host.pointer_up_screen(to_x, to_y, false, false, false);
    assert!(!host.screen_pointer_gesture_active(), "a released gesture hands the graph back to the bounded path");
}

/// 🗺️ A minimap click navigates; a press on its viewport rectangle grabs instead.
#[test]
fn a_minimap_click_moves_the_camera() {
    let law = law();
    let minimap = &law["minimap"];
    let camera = &minimap["camera"];
    for case in minimap["cases"].as_array().expect("minimap cases") {
        let name = case["name"].as_str().expect("case name");
        let mut host = wire_host(&law);
        host.set_minimap_widget_visible(true);
        host.set_camera(camera["x"].as_f64().expect("camera x"), camera["y"].as_f64().expect("camera y"), camera["zoom"].as_f64().expect("camera zoom"));
        let layout = host.minimap_widget_layout(host.width, host.height).unwrap_or_else(|| panic!("{name}: the minimap must be laid out for a camera that does not already show the whole graph"));
        // 📐️ `minimap::layout` reports both rects as CORNERS `(x0, y0, x1, y1)`, not as origin+size —
        // the same convention `minimap::point_in_rect` reads them back in.
        let fraction = |rect: (f64, f64, f64, f64), tx: f64, ty: f64| (rect.0 + (rect.2 - rect.0) * tx, rect.1 + (rect.3 - rect.1) * ty);
        // 🗺️ "Inside the panel, outside the viewport rectangle" is a RELATION between two rects whose
        // sizes depend on the camera, not a fixed corner — so the law searches the panel for it
        // rather than guessing a fraction that a different zoom would put back inside the viewport.
        let outside_viewport = || {
            for step_y in 0..20 {
                for step_x in 0..20 {
                    let point = fraction(layout.panel, 0.02 + f64::from(step_x) * 0.048, 0.02 + f64::from(step_y) * 0.048);
                    if !DagHost::minimap_widget_point_in_rect(layout.viewport, point.0, point.1) {
                        return Some(point);
                    }
                }
            }
            None
        };
        let (x, y) = match case["at"].as_str().expect("minimap point") {
            "panelOutsideViewport" => outside_viewport().unwrap_or_else(|| panic!("{name}: the viewport rectangle must not fill the whole minimap panel")),
            "viewportCentre" => fraction(layout.viewport, 0.5, 0.5),
            other => panic!("fixture minimap point {other}"),
        };
        assert!(host.screen_pointer_gesture_begins_at(x, y), "{name}: a minimap press belongs to the screen pointer path");
        let before = (host.fixture.camera.x, host.fixture.camera.y);
        host.pointer_down_screen(x, y, 0, false, false, false, false);
        let after = (host.fixture.camera.x, host.fixture.camera.y);
        let moved = (after.0 - before.0).abs() > f64::EPSILON || (after.1 - before.1).abs() > f64::EPSILON;
        assert_eq!(moved, case["expectedCameraMoves"].as_bool().expect("expected camera moves"), "{name}: camera {before:?} -> {after:?}");
        assert!(host.screen_pointer_gesture_active(), "{name}: the minimap drag holds the screen path until release");
    }
}

/// 📐️ LAW: the geometry the host PUBLISHES for a port is the geometry that grabs it — at every zoom
/// band the graph draws nodes at, and pressed at the rect's centre, which is the only point a caller
/// that trusts `entity_screen_json` can derive.
///
/// The defect this pins: `entity_screen_json("handle", …)` published the port ROW rect while
/// `rim_handle_anchor_hit` accepted only a disc of `handle.radius + 1.5` WORLD units at the row's
/// outer edge — 6.5 units into a 20-unit-wide row, so the published centre never grabbed at any zoom
/// — and `allows_connection_hit_picking()` switched port hits off entirely below the Normal band.
/// Two authorities for one affordance; the row that is painted, published and hovered was not the
/// row that wires (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-mode-interactions-2026-09-13.md` §4).
/// 📷️ The world centre of every endpoint a case names — its setup wires and its gesture — so the
/// camera can frame exactly what the case is about to aim at.
fn case_endpoint_centre(host: &DagHost, case: &Value) -> (f64, f64) {
    let mut endpoints: Vec<&str> = Vec::new();
    let no_setup = Vec::new();
    for setup in case["setup"].as_array().unwrap_or(&no_setup) {
        endpoints.extend(["from", "to"].iter().filter_map(|key| setup[*key].as_str()));
    }
    endpoints.extend(["at", "from", "to"].iter().filter_map(|key| case["gesture"][*key].as_str()));
    let centres: Vec<(f64, f64)> = endpoints
        .iter()
        .filter_map(|endpoint| endpoint.split('@').next())
        .filter_map(|node_id| host.fixture.nodes.iter().find(|node| node.id == node_id))
        .map(|node| (node.x, node.y))
        .collect();
    assert!(!centres.is_empty(), "a case must name at least one endpoint to frame");
    let count = centres.len() as f64;
    (centres.iter().map(|centre| centre.0).sum::<f64>() / count, centres.iter().map(|centre| centre.1).sum::<f64>() / count)
}

#[test]
fn the_port_geometry_the_host_publishes_is_the_geometry_that_grabs_a_wire() {
    let law = law();
    let grab = &law["grab"];
    for band in grab["zooms"].as_array().expect("grab zooms") {
        let zoom = band["zoom"].as_f64().expect("zoom");
        for case in grab["cases"].as_array().expect("grab cases") {
            let name = format!("{} @ zoom {zoom}", case["name"].as_str().expect("case name"));
            let mut host = wire_host(&law);
            // 📷️ A zoom band tests the LOD, not the FRAMING: the camera is centred on the very nodes
            // this case names, so every endpoint it aims at is on the surface at every band. Fixed at
            // (0, 0) the `tgt` row left the 1280-wide viewport at zoom 2 entirely, and the case pressed
            // a point no browser would ever route to the canvas — which `visible` now says outright.
            let (cam_x, cam_y) = case_endpoint_centre(&host, case);
            host.set_camera(cam_x, cam_y, zoom);
            for setup in case["setup"].as_array().unwrap_or(&Vec::new()) {
                let (from_x, from_y) = published_port_centre(&host, setup["from"].as_str().expect("setup from"));
                let (to_x, to_y) = published_port_centre(&host, setup["to"].as_str().expect("setup to"));
                host.pointer_down_screen(from_x, from_y, 0, false, false, false, false);
                host.pointer_move_screen(to_x, to_y, false, false, false);
                host.pointer_up_screen(to_x, to_y, false, false, false);
                assert!(!host.fixture.edges.is_empty(), "{name}: the setup wire must exist before the case runs");
                let _ = host.take_graph_edits();
            }
            let gesture = &case["gesture"];
            match gesture["kind"].as_str().expect("gesture kind") {
                "grabCentre" => {
                    let endpoint = gesture["at"].as_str().expect("grab endpoint");
                    let (x, y) = published_port_centre(&host, endpoint);
                    assert!(host.screen_pointer_gesture_begins_at(x, y), "{name}: the published rect of {endpoint} must belong to the screen pointer path");
                    host.pointer_down_screen(x, y, 0, false, false, false, false);
                    assert_eq!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), gesture_expects(case, "expectedDrawEdge"), "{name}: pressing the published centre of {endpoint} must draw a wire");
                }
                "wireCentres" => {
                    let (from_x, from_y) = published_port_centre(&host, gesture["from"].as_str().expect("wire from"));
                    let (to_x, to_y) = published_port_centre(&host, gesture["to"].as_str().expect("wire to"));
                    host.pointer_down_screen(from_x, from_y, 0, false, false, false, false);
                    host.pointer_move_screen(to_x, to_y, false, false, false);
                    host.pointer_up_screen(to_x, to_y, false, false, false);
                    assert_case_edits(&host.take_graph_edits(), case, &name);
                    assert_case_edges(&host, case, &name);
                }
                "grabSides" => {
                    let endpoint = gesture["at"].as_str().expect("grab endpoint");
                    let (node_id, port_id) = endpoint.split_once('@').expect("endpoint grammar");
                    let node = host.fixture.nodes.iter().find(|node| node.id == node_id).expect("fixture node").clone();
                    let input_index = node.inputs().iter().position(|port| port.id == port_id).expect("input port");
                    let output_index = node.outputs().iter().position(|port| port.id == port_id).expect("output port");
                    for (side, bounds) in [(true, input_port_connector_bounds(&node, input_index)), (false, output_port_connector_bounds(&node, output_index))] {
                        let (x0, y0, x1, y1) = bounds.expect("connector bounds");
                        let (sx, sy) = world_point_to_screen(&host, canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
                        let InteractionMode::DrawEdge { anchor_handle, .. } = host.engine.interaction else {
                            panic!("{name}: pressing the {} connector of {endpoint} must draw a wire", if side { "input" } else { "output" });
                        };
                        let role = host.engine.handles.get(&anchor_handle).expect("anchor handle").role;
                        assert_eq!(role == HandleRole::Target, side, "{name}: the {} connector of {endpoint} must grab the handle on ITS side, got {role:?}", if side { "input" } else { "output" });
                        host.pointer_up_screen(sx, sy, false, false, false);
                        let _ = host.take_graph_edits();
                    }
                }
                "detach" => {
                    let (x, y) = published_port_centre(&host, gesture["at"].as_str().expect("detach endpoint"));
                    host.pointer_down_screen(x, y, 0, false, false, false, false);
                    host.pointer_move_screen(EMPTY_CANVAS_SCREEN.0, EMPTY_CANVAS_SCREEN.1, false, false, false);
                    host.pointer_up_screen(EMPTY_CANVAS_SCREEN.0, EMPTY_CANVAS_SCREEN.1, false, false, false);
                    assert_case_edits(&host.take_graph_edits(), case, &name);
                    assert_case_edges(&host, case, &name);
                }
                other => panic!("fixture grab gesture kind {other}"),
            }
        }
    }
}

/// 👁️ LAW: `visible` means "you can aim at this", not "this exists". A port the camera has scrolled
/// past publishes NO geometry, so a caller that trusts the host never presses outside the surface.
///
/// 🩸️ `entity_screen_json` stamped `visible: true` on every entity it could find, whatever the camera.
/// Measured on 6026: the graph canvas starts at page `x = 6.4` and the host published `height@number`
/// at page `x = 1.4` (surface `x = -4.9`) — twice, byte-identically, so not a cache race — and the
/// wire row's redraw press landed outside the canvas, reached nothing, and was graded as "the redraw
/// does not restore the wire" (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
#[test]
fn a_port_the_camera_has_scrolled_past_publishes_no_geometry_to_aim_at() {
    let law = law();
    let endpoint = law["grab"]["cases"].as_array().expect("grab cases")[0]["gesture"]["at"].as_str().expect("grab endpoint").to_string();
    let mut host = wire_host(&law);
    let (on_x, on_y) = published_port_centre(&host, &endpoint);
    assert!(on_x >= 0.0 && on_y >= 0.0 && on_x <= f64::from(host.width) && on_y <= f64::from(host.height), "{endpoint} must start on the surface: ({on_x}, {on_y}) in {}×{}", host.width, host.height);

    let world = host.fixture.nodes.iter().find(|node| node.id == endpoint.split('@').next().expect("node id")).map(|node| (node.x, node.y)).expect("endpoint node");
    host.set_camera(world.0 + f64::from(host.width) * 2.0, world.1, 1.0);
    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");
    println!("[DEBUG] scrolled-past port geometry {geometry}");
    assert_eq!(geometry["visible"].as_bool(), Some(false), "{endpoint} is off the surface, so it must not be published as visible");
    assert!(geometry.get("rect").is_none_or(Value::is_null), "an invisible entity publishes no rect for a caller to aim at");

    host.set_camera(0.0, 0.0, 1.0);
    let back: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");
    assert_eq!(back["visible"].as_bool(), Some(true), "{endpoint} is on the surface again and must publish its rect");
}

fn gesture_expects(case: &Value, key: &str) -> bool {
    case[key].as_bool().unwrap_or_else(|| panic!("case must declare {key}"))
}

/// 🔗️ Asserts a case's declared journal: either the exact rows (`expectedEdits`) or, when only the
/// KIND is knowable ahead of time because the synapse id is the host's own, the operation names.
fn assert_case_edits(edits: &[DagGraphEdit], case: &Value, name: &str) {
    let rows = edit_rows(edits);
    if let Some(operations) = case["expectedEditOperations"].as_array() {
        let actual: Vec<&str> = rows.iter().filter_map(|row| row["operation"].as_str()).collect();
        for operation in operations {
            assert!(actual.contains(&operation.as_str().expect("operation name")), "{name}: expected a {operation} among {actual:?}");
        }
        return;
    }
    let Some(expected) = case["expectedEdits"].as_array() else { return };
    if expected.is_empty() {
        assert!(rows.is_empty(), "{name}: this gesture must journal nothing, got {rows:?}");
        return;
    }
    for row in expected {
        assert!(rows.contains(row), "{name}: expected {row} among {rows:?}");
    }
}

fn assert_case_edges(host: &DagHost, case: &Value, name: &str) {
    let Some(edges) = case["expectedEdges"].as_array() else { return };
    for edge in edges {
        let source = edge["source"].as_str().expect("edge source");
        let target = edge["target"].as_str().expect("edge target");
        assert!(host.fixture.edges.iter().any(|candidate| candidate.source == source && candidate.target == target), "{name}: {source} -> {target} must be a live wire, got {:?}", host.fixture.edges.iter().map(|edge| (edge.source.as_str(), edge.target.as_str())).collect::<Vec<_>>());
    }
    assert_eq!(host.fixture.edges.len(), edges.len(), "{name}: the live wire count must be exactly what the case declares");
}
