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
    host.set_automatic_lod(false);
    host.set_forced_draw_lod(Some(DagDrawLod::Full));
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
            DagGraphEdit::Disconnect { edge_id } => serde_json::json!({ "operation": "disconnect", "edgeId": edge_id }),
        })
        .collect()
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
        let (x, y) = match case["at"].as_str().expect("minimap point") {
            "panelTopLeftQuarter" => (layout.panel.0 + layout.panel.2 * 0.15, layout.panel.1 + layout.panel.3 * 0.15),
            "viewportCentre" => (layout.viewport.0 + layout.viewport.2 * 0.5, layout.viewport.1 + layout.viewport.3 * 0.5),
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
