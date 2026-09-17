//! ◻️ Puzzle 5D viewer — the Board2d window: a read-only render of the unified 5d document's flat
//! projection (part nodes with their rim grips, fastener edges), built directly on the
//! artifact-level `Puzzle5dSnapshot`. This file imports nothing from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright), so the board fixture is re-derived here rather
//! than reached for across that boundary.
//!
//! 👁️ Read-only by construction: `interactive: false`, no selection, no hover, no utility, no
//! suggestion popup and no snapping — a viewer emits no mutations (`ViewEmit`). Glyph catalogs stay
//! empty: the kind bundle is a composed child handle the viewer does not mount, so nodes render with
//! the engine's own default palette — the same documented simplification the sibling world pane makes
//! with its fixed camera.

use crate::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot};
use semio_framework_plugin::{Board2dScene, LocalizedLabel, SurfaceKind, WindowEngagementSlot, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::BuiltNode;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle5d-view-2d";
pub const BODY_KEY: &str = "puzzle.5d.view.2d";
pub const SURFACE_ID: &str = "puzzle.5d.view.2d";
/// 🧬️ The board fixture schema tag the host's parser keys on — the same one the editor's board pane emits.
const BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
/// 🔵️ Node radius for a part whose 2D projection never authored one.
const DEFAULT_PART_RADIUS: f64 = 20.0;
/// 🔘️ Handle radius for a grip whose 2D projection never authored one.
const DEFAULT_GRIP_RADIUS: f64 = 3.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle5d::create_puzzle5d_viewer`.
/// No utilities, no actions, no measures: everything this window offers is reading.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Puzzle 2D", "Puzzle 2D"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️BoardJson
/// 🔗️ `{partId}:{gripId}`, unless the authored grip id already carries its owner.
fn grip_full_id(part_id: &str, grip_id: &str) -> String {
    if grip_id.contains(':') {
        grip_id.to_string()
    } else {
        format!("{part_id}:{grip_id}")
    }
}

fn node_value(part: &Puzzle5dPart) -> Value {
    let shape = part.part_2d.shape.clone().unwrap_or_else(|| "circle".into());
    let handles: Vec<Value> = part
        .grips
        .iter()
        .map(|grip| {
            json!({
                "id": grip_full_id(&part.id, &grip.id),
                "handleKind": grip.grip_2d.grip_kind.clone().or_else(|| grip.grip_kind.clone()).unwrap_or_default(),
                "angle": grip.grip_2d.angle,
                "radius": grip.grip_2d.radius.filter(|radius| *radius > 0.0).unwrap_or(DEFAULT_GRIP_RADIUS),
            })
        })
        .collect();
    let mut node = json!({
        "id": part.id,
        "nodeKind": part.part_kind.clone().unwrap_or_default(),
        "shape": shape,
        "x": part.part_2d.x,
        "y": part.part_2d.y,
        "text": part.part_2d.text.clone().unwrap_or_default(),
        "handles": handles,
    });
    if node["shape"] == json!("rectangle") {
        node["width"] = json!(part.part_2d.width.unwrap_or(48.0));
        node["height"] = json!(part.part_2d.height.unwrap_or(48.0));
    } else {
        node["radius"] = json!(part.part_2d.radius.filter(|radius| *radius > 0.0).unwrap_or(DEFAULT_PART_RADIUS));
    }
    if let Some(icon) = part.part_2d.icon_kind.as_ref() {
        node["iconKind"] = json!(icon);
    }
    if let Some(hidden) = part.part_2d.hidden {
        node["hidden"] = json!(hidden);
    }
    node
}

fn edge_value(fastener: &Puzzle5dFastener) -> Value {
    json!({
        "id": fastener.id,
        "edgeKind": fastener.fastener_kind.clone().unwrap_or_else(|| "link".into()),
        "source": fastener.source,
        "target": fastener.target,
    })
}

/// 👁️ Default camera — a viewer has no persisted per-session camera (`Config = NoConfig`), so this is
/// a fixed, documented simplification, the flat twin of the sibling world pane's fixed orbit.
fn camera_value() -> Value {
    json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 })
}

pub fn board_fixture_json(document: &Puzzle5dSnapshot) -> String {
    let nodes: Vec<Value> = document.parts.iter().map(node_value).collect();
    let edges: Vec<Value> = document.fasteners.iter().map(edge_value).collect();
    json!({
        "schema": BOARD_FIXTURE_SCHEMA,
        "camera": camera_value(),
        "nodes": nodes,
        "edges": edges,
        "wires": [],
        "meta": { "kindCatalogs": { "nodes": [], "handles": [], "edges": [], "wires": [] }, "kindCompatibility": [] },
    })
    .to_string()
}

fn board_scene(document: &Puzzle5dSnapshot) -> Board2dScene {
    Board2dScene {
        fixture_json: board_fixture_json(document),
        camera_json: camera_value().to_string(),
        glyph_catalogs_json: json!({ "nodes": [], "handles": [], "edges": [], "wires": [] }).to_string(),
        selection_json: "[]".into(),
        interactive: false,
        hovered_id: None,
        active_utility: None,
        selection_method: "rectangle".into(),
        grid_visible: true,
        grid_snap_enabled: false,
        grid_factor: 1.0,
        selectable_nodes: false,
        selectable_edges: false,
        selectable_handles: false,
        suggestion_offset: 0.0,
        brush_weights_json: json!({ "nodeWeights": {}, "handleWeights": {} }).to_string(),
        placement_compatibility_json: "[]".into(),
        lod_mode: "automatic".into(),
        transform_flags: None,
        area_brush_size: None,
        domain_id: None,
        suggestion_menu_json: None,
        tool_run_trace: None,
        lanes: Vec::new(),
    }
}
//#endregion 🔖️BoardJson

//#region 🔖️Render
/// 👁️ Pure `Puzzle5dSnapshot -> UiNode` read of the flat projection.
pub fn render(document: &Puzzle5dSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Board2d, &board_scene(document))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
