//! ◻️ Puzzle 5d play app — the `Board2d` window kind: the flat projection of the unified 5d document.
//! Owns the board scene payload (nodes/handles/edges plus the glyph catalogs and placement
//! compatibility the board host reads), binds the pointer utilities (`🪛️utilities/*`) and scopes the
//! board-event/2D-camera actions (`🎬️actions`). Its only genuinely 2D-specific chrome measure is the
//! LOD select in `🎚️options/🔭️lod`; the brush/fill Utility Options it shares with the 3D window come
//! from the mode's own `🎚️options/*`.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dRuntime};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::options as mode_options;
use crate::editor::puzzle5d::modes::edit::windows::board2d::{options, utilities};
use crate::editor::puzzle5d::precompute::Puzzle5dPrecomputeSession;
use crate::editor::puzzle5d::terminology::{puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::{puzzle5d_grip_full_id, puzzle5d_scene_mode, Puzzle5dDocument, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_BOARD_FIXTURE_SCHEMA, PUZZLE5D_DEFAULT_PART_RADIUS, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{build_board2d_scene, Board2dScene, SurfaceKind, UiNode, WindowEngagement, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle5d-2d";
pub const BODY_KEY: &str = "puzzle.5d.play.2d";
pub const SURFACE_ID: &str = "puzzle.5d.play.2d";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`. Like puzzle2d,
/// puzzle5d freezes the first `window_measures()` frame into `options.measures` so the shell has LOD
/// and utility chrome before the first `refreshUi` tick; every later frame comes from
/// `ArtifactApp::window_measures`.
pub async fn definition(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle5d_localized(|l| l.window_2d),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: window_measures(envelope, precompute, labels), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: Vec::new(),
        utilities: vec![utilities::select::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into(), utilities::fill::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window: its own LOD select plus the mode-level brush/fill
/// Utility Options groups it shares with the 3D window.
pub async fn window_measures(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    vec![options::lod::measure(&envelope.runtime, labels), mode_options::fill::measure(envelope, labels), mode_options::brush::measure(envelope, precompute, labels)]
}

pub async fn engagement(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowEngagement {
    edit::puzzle5d_engagement(envelope, WINDOW_KIND_ID, labels)
}
//#endregion 🔖️Definition

//#region 🔖️BoardJson
pub async fn board_camera_value(camera: &Puzzle5dCamera2d) -> Value {
    json!({ "x": camera.x, "y": camera.y, "zoom": camera.zoom })
}

async fn board_node_value(part: &Puzzle5dPart) -> Value {
    let shape = if part.part_2d.shape.is_empty() { "circle" } else { part.part_2d.shape.as_str() };
    let handles: Vec<Value> = part
        .grips
        .iter()
        .map(|grip| {
            json!({
                "id": puzzle5d_grip_full_id(&part.id, &grip.id),
                "handleKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                "angle": grip.grip_2d.angle,
                "radius": if grip.grip_2d.radius > 0.0 { grip.grip_2d.radius } else { 3.0 },
            })
        })
        .collect();
    let mut node = json!({
        "id": part.id,
        "nodeKind": part.part_kind,
        "shape": shape,
        "x": part.part_2d.x,
        "y": part.part_2d.y,
        "text": part.part_2d.text,
        "handles": handles,
    });
    if shape == "rectangle" {
        node["width"] = json!(part.part_2d.width.unwrap_or(48.0));
        node["height"] = json!(part.part_2d.height.unwrap_or(48.0));
    } else {
        node["radius"] = json!(if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS });
    }
    if let Some(icon) = part.part_2d.icon_kind.as_ref() {
        node["iconKind"] = json!(icon);
    }
    if let Some(hidden) = part.part_2d.hidden {
        node["hidden"] = json!(hidden);
    }
    if let Some(locked) = part.part_2d.locked {
        node["locked"] = json!(locked);
    }
    node
}

/// 🗂️ Projects the unified 5d kind bundle (`parts/grips/fasteners/ropes`) to the board's `nodes/handles/edges/wires` naming.
pub async fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
    let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
    json!({
        "nodes": catalogs.get("parts").cloned().unwrap_or(json!([])),
        "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "edges": catalogs.get("fasteners").cloned().unwrap_or(json!([])),
        "wires": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    })
}

async fn board_fixture_value(document: &Puzzle5dDocument, camera2d: &Puzzle5dCamera2d) -> Value {
    let nodes: Vec<Value> = document.parts.iter().map(board_node_value).collect();
    let edges: Vec<Value> = document
        .fasteners
        .iter()
        .map(|fastener| {
            json!({
                "id": fastener.id,
                "edgeKind": fastener.fastener_kind.clone().unwrap_or_else(|| "link".into()),
                "source": fastener.source,
                "target": fastener.target,
            })
        })
        .collect();
    json!({
        "schema": PUZZLE5D_BOARD_FIXTURE_SCHEMA,
        "camera": board_camera_value(camera2d),
        "nodes": nodes,
        "edges": edges,
        "wires": [],
        "meta": {
            "kindCatalogs": board_kind_catalogs_value(document),
            "kindCompatibility": document.kind_compatibility.clone().unwrap_or(json!([])),
        },
    })
}

async fn board_brush_weights_json(runtime: &Puzzle5dRuntime) -> String {
    json!({ "nodeWeights": runtime.object_kind_weights, "handleWeights": runtime.vortex_kind_weights }).to_string()
}

async fn puzzle5d_board_scene(envelope: &Puzzle5dScene) -> Board2dScene {
    Board2dScene {
        fixture_json: board_fixture_value(&envelope.document, &envelope.runtime.camera2d).to_string(),
        camera_json: board_camera_value(&envelope.runtime.camera2d).to_string(),
        glyph_catalogs_json: board_kind_catalogs_value(&envelope.document).to_string(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection/hover ids
        // and method used to come from `runtime.selection`/`hovered_part_id`/`selection_method`, now
        // dissolved into the framework-owned `vortex` interaction domain; `render` has no
        // `InteractionView` to read it from (see `puzzle5d_gumball_active`'s doc comment) — this
        // payload carries no live ids until that framework gap closes.
        selection_json: "[]".into(),
        interactive: true,
        hovered_id: None,
        active_utility: Some(puzzle5d_scene_mode(&envelope.active_utility).to_string()),
        selection_method: "rectangle".into(),
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_weights_json: board_brush_weights_json(&envelope.runtime),
        placement_compatibility_json: envelope.document.kind_compatibility.clone().unwrap_or(json!([])).to_string(),
        lod_mode: envelope.runtime.lod_mode.clone(),
    }
}
//#endregion 🔖️BoardJson

//#region 🔖️Render
pub async fn render(envelope: &Puzzle5dScene) -> UiNode {
    build_board2d_scene(SURFACE_ID, PUZZLE5D_PLAY_CONTROLLER_ID, puzzle5d_board_scene(envelope))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_the_board_scene() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("board-2d"));
    }
}
//#endregion 🧪️Tests
