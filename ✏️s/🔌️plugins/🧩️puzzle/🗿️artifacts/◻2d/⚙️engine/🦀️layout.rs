//! 📐️ Puzzle 2d artifact engine — the redraw layout dispatcher: picks the undirected-force-graph
//! path for mindmap/wires fixtures and the ported-redraw path for everything else, plus the
//! force-graph / hierarchical-tree / edge-handle-snap laws every layout mode must satisfy.

fn is_undirected_fixture_schema(schema: &str) -> bool {
    matches!(schema, "reasoning.mindmap.fixture" | "reasoning.wires.fixture")
}

/// 🔀️ Picks the undirected-force-graph vs. ported-redraw layout path for a fixture — pure compute,
/// called by the `framework/surface/board-2d` wasm session's `boardRedrawLayoutFixtureJson` export.
pub fn redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
    let fixture: serde_json::Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
    let schema = fixture.get("schema").and_then(|v| v.as_str()).unwrap_or("");
    let opts: serde_json::Value = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
    let mode = opts.get("mode").and_then(|v| v.as_str()).unwrap_or("force-graph");
    if mode == "force-graph" && is_undirected_fixture_schema(schema) {
        apply_normal_undirected_redraw_layout_to_fixture_v1_json(fixture_json, options_json).map_err(|e| e.to_string())
    } else {
        apply_ported_redraw_layout_to_fixture_v1_json(fixture_json, options_json)
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::artifacts::puzzle2d::engine::graph;
    use crate::artifacts::puzzle2d::engine::{apply_force_graph_layout_to_fixture_v1_json, apply_normal_undirected_redraw_layout_to_fixture_v1_json};
    use graph::apply_edge_handle_snap_to_fixture_v1_json;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn force_graph_spreads_two_linked_circles_along_x() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 8000.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected horizontal separation, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_pins_locked_node_positions() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 160.0,
            "repulsionStrength": 7500.0,
            "springStrength": 0.045,
            "gravity": 0.0,
            "randomSeed": 101,
            "lockedNodeIds": ["a"]
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        assert!((ax - 0.0).abs() < 1e-9 && (ay - 0.0).abs() < 1e-9);
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - 40.0).abs() > 25.0, "expected free node to move, bx={bx}");
    }

    #[test]
    fn redraw_force_graph_top_level_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "lockedNodeIds": ["a"],
            "randomSeed": 101,
            "redrawHandlesAfter": false,
            "forceGraph": {
                "iterations": 180,
                "idealEdgeLength": 160.0,
                "repulsionStrength": 7500.0,
                "springStrength": 0.045,
                "gravity": 0.0
            }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        assert!((nodes[0]["x"].as_f64().unwrap() - 0.0).abs() < 1e-9);
        assert!((nodes[0]["y"].as_f64().unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn redraw_force_graph_mindmap_schema_uses_undirected_layout() {
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0 },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0 }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 0.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = apply_normal_undirected_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected mindmap undirected springs, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_normal_mode_node_id_edges_apply_spring_forces() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0, "handles": [] },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0, "handles": [] }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 0.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected node-id edge springs to spread nodes, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_rejects_bad_schema() {
        let err = apply_force_graph_layout_to_fixture_v1_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();
        assert!(err.contains("schema"));
    }

    #[test]
    fn force_graph_barnes_hut_many_bodies_yields_finite_coordinates() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..64 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 8) as f64 * 12.0,
                "y": (k / 8) as f64 * 12.0,
                "radius": 8.0,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 90.0,
            "repulsionStrength": 6000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 91,
            "barnesHutTheta": 0.72,
            "pairwiseRepulsionMaxBodies": 12
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            let x = row["x"].as_f64().unwrap();
            let y = row["y"].as_f64().unwrap();
            assert!(x.is_finite() && y.is_finite());
        }
        let xs: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["x"].as_f64().unwrap()).collect();
        let ys: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["y"].as_f64().unwrap()).collect();
        let x_span = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min);
        let y_span = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - ys.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(x_span > 40.0 && y_span > 35.0, "expected BH layout to spread graph, x_span={x_span} y_span={y_span}");
    }

    #[test]
    fn force_graph_bh_layout_is_deterministic_for_fixed_seed() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..36 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 6) as f64 * 9.0,
                "y": (k / 6) as f64 * 9.0,
                "radius": 6.5,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 120,
            "idealEdgeLength": 88.0,
            "repulsionStrength": 5400.0,
            "springStrength": 0.047,
            "gravity": 0.013,
            "randomSeed": 4041,
            "barnesHutTheta": 0.55,
            "pairwiseRepulsionMaxBodies": 8
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b, "BH path must be bitwise reproducible for identical inputs");
    }

    #[test]
    fn force_graph_pairwise_layout_is_deterministic_for_fixed_seed() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 30.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 3.0, "y": 1.0, "radius": 30.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": -2.0, "y": 4.0, "radius": 28.0, "handles": [{ "id": "c:h0", "angle": 1.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 90,
            "idealEdgeLength": 110.0,
            "repulsionStrength": 6200.0,
            "springStrength": 0.042,
            "gravity": 0.011,
            "randomSeed": 909,
            "pairwiseRepulsionMaxBodies": 80
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b);
    }

    #[test]
    fn force_graph_clamped_barnes_hut_theta_runs_without_error() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 5.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": 2.0, "y": 8.0, "radius": 18.0, "handles": [{ "id": "c:h0", "angle": 0.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 40,
            "idealEdgeLength": 100.0,
            "repulsionStrength": 5000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 3,
            "barnesHutTheta": 500.0,
            "pairwiseRepulsionMaxBodies": 2
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            assert!(row["x"].as_f64().unwrap().is_finite());
            assert!(row["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn redraw_force_graph_wraps_flat_options() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0);
    }

    #[test]
    fn edge_handle_snap_sets_circle_handle_angles_on_center_line() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let out = graph::apply_edge_handle_snap_to_fixture_v1_json(&fixture.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        assert!((ang_a - 0.0).abs() < 1e-6, "expected east on a, got {ang_a}");
        assert!((ang_b - std::f64::consts::PI).abs() < 1e-6, "expected west on b, got {ang_b}");
    }

    #[test]
    fn redraw_force_graph_with_snap_sets_handle_angles() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "redrawHandlesAfter": true,
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        let by = nodes[1]["y"].as_f64().unwrap();
        let exp_a = f64::atan2(by - ay, bx - ax);
        let exp_b = f64::atan2(ay - by, ax - bx);
        let wrap_diff = |a: f64, b: f64| {
            let mut d = (a - b).rem_euclid(std::f64::consts::TAU);
            if d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            d.abs()
        };
        assert!(wrap_diff(ang_a, exp_a) < 0.03, "a angle {ang_a} vs exp {exp_a}");
        assert!(wrap_diff(ang_b, exp_b) < 0.03, "b angle {ang_b} vs exp {exp_b}");
    }

    #[test]
    fn force_graph_accepts_logical_nodes_without_xy() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "centerX": 0.0,
            "centerY": 0.0,
            "randomSeed": 3,
            "forceGraph": { "iterations": 120, "idealEdgeLength": 160.0, "gravity": 0.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for n in parsed["nodes"].as_array().unwrap() {
            assert!(n["x"].as_f64().unwrap().is_finite());
            assert!(n["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "r", "root": true, "radius": 18.0, "handles": [] },
                { "id": "c1", "radius": 18.0, "handles": [] },
                { "id": "c2", "radius": 18.0, "handles": [] }
            ],
            "edges": [
                { "id": "e1", "source": "r", "target": "c1" },
                { "id": "e2", "source": "r", "target": "c2" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_pins_locked_root_coordinates() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 120.0,
                    "y": -33.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "x": 5.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "lockedNodeIds": ["r"],
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 120.0).abs() < 1e-3 && (ry + 33.0).abs() < 1e-3, "locked root moved: {rx},{ry}");
        let (_c1x, c1y) = *by_id.get("c1").unwrap();
        let (_c2x, c2y) = *by_id.get("c2").unwrap();
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
        assert!((c1y - ry).abs() > 40.0, "children laid relative to tree, root stayed pinned");
    }

    #[test]
    fn redraw_hierarchical_tree_nested_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 77.0,
                    "y": 12.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": {
                "direction": "downwards",
                "layerSpacing": 90.0,
                "siblingGap": 12.0,
                "lockedNodeIds": ["r"]
            }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 77.0).abs() < 1e-3 && (ry - 12.0).abs() < 1e-3, "nested locked list ignored: {rx},{ry}");
    }

    #[test]
    fn hierarchical_tree_right_places_children_larger_x_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "right", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut xs: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            xs.insert(id, n["x"].as_f64().unwrap());
        }
        let rx = *xs.get("r").unwrap();
        let c1x = *xs.get("c1").unwrap();
        assert!(c1x > rx + 40.0, "expected child to the right of root");
    }

    #[test]
    fn hierarchical_tree_upwards_places_children_smaller_y_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "upwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        assert!(c1y < ry - 40.0, "expected child above root (smaller y)");
    }

    #[test]
    fn hierarchical_tree_rejects_unknown_direction() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": []
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "hierarchicalTree": { "direction": "sideways" }
        });
        let err = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap_err();
        assert!(err.contains("unknown hierarchical tree direction"));
    }

    #[test]
    fn redraw_rejects_unknown_mode() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [],
            "edges": []
        });
        let err = graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"nope"}"#).unwrap_err();
        assert!(err.contains("unknown redraw mode"));
    }
}
//#endregion 🧪️Tests
