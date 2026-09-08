//! 📐️ Puzzle 2d app engine — the redraw layout dispatcher: picks the undirected-force-graph
//! path for mindmap/wires fixtures and the ported-redraw path for everything else, plus the
//! force-graph / hierarchical-tree / edge-handle-snap laws every layout mode must satisfy.

use crate::editor::puzzle2d::engine::{apply_normal_undirected_redraw_layout_to_fixture_v1_json, apply_ported_redraw_layout_to_fixture_v1_json};

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
#[allow(
    clippy::approx_constant,
    reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input."
)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
