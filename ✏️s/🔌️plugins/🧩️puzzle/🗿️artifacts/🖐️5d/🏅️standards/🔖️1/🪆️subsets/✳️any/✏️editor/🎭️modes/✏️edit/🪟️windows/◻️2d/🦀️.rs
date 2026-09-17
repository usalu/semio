//! ◻️ Puzzle 5d play app — the `Board2d` window kind: the flat projection of the unified 5d document.
//! Owns the board scene payload (nodes/🐙️handles/edges plus the glyph catalogs and placement
//! compatibility the board host reads), binds the pointer utilities (`🪛️utilities/*`) and scopes the
//! board-event/2D-camera actions (`🎬️actions`). Its only genuinely 2D-specific chrome measure is the
//! LOD select in `☑️options/🔭️lod`; the brush Utility Options it shares with the 3D window come from the
//! mode's own `☑️options/*`, and fill's from the mode-level tool `🛠️tools/🪣️fill`.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dRuntime};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::options as mode_options;
use crate::editor::puzzle5d::modes::edit::windows::board2d::{options, utilities};
use crate::editor::puzzle5d::terminology::{puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::{puzzle5d_grip_full_id, puzzle5d_scene_mode, target_volume_flat_rect, Puzzle5dDocument, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_BOARD_FIXTURE_SCHEMA, PUZZLE5D_DEFAULT_PART_RADIUS};
use semio_framework_plugin::{Board2dScene, SurfaceKind, ToolRunView, WindowEngagement, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_ui_contract::BuiltNode;
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
pub fn definition(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle5d_localized(|l| l.window_2d),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions { measures: window_measures(envelope, labels), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: Vec::new(),
        utilities: vec![utilities::select::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window: its own LOD select, grid group and selection group
/// plus the mode-level brush Utility Options group it shares with the 3D window. Fill is a mode-level
/// TOOL, so its count and distribution measures are the tool options rail's (`🛠️tools/🪣️fill`), never
/// this window's.
pub fn window_measures(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    vec![
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        mode_options::brush::measure(envelope, labels),
    ]
}

pub fn engagement(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowEngagement {
    edit::puzzle5d_engagement(envelope, WINDOW_KIND_ID, labels, None)
}
//#endregion 🔖️Definition

//#region 🔖️BoardJson
pub fn board_camera_value(camera: &Puzzle5dCamera2d) -> Value {
    json!({ "x": camera.x, "y": camera.y, "zoom": camera.zoom })
}

fn board_node_value(part: &Puzzle5dPart) -> Value {
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

/// 🗂️ Projects the unified 5d kind bundle (`parts/grips/fasteners/ropes`) to the board's `nodes/🐙️handles/edges/wires` naming.
pub fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
    let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
    json!({
        "nodes": catalogs.get("parts").cloned().unwrap_or(json!([])),
        "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "edges": catalogs.get("fasteners").cloned().unwrap_or(json!([])),
        "wires": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    })
}

/// 📐️ The flat rectangles the board paints for the document's target volumes — the projection of the
/// 3d-pose boxes through the ONE board↔world map (`target_volume_flat_rect`), under the same
/// `targetRegions` key the board engine reads in the 2d artifact. A hidden volume is dropped here
/// rather than painted transparent, so the board never hit-tests a constraint the operator hid.
fn board_target_regions_value(document: &Puzzle5dDocument) -> Value {
    let regions: Vec<Value> = document
        .target_volumes
        .iter()
        .filter(|volume| !volume.hidden)
        .map(|volume| {
            let [x, y, width, height] = target_volume_flat_rect(volume);
            json!({ "id": volume.id, "x": x, "y": y, "width": width, "height": height, "hidden": volume.hidden, "locked": volume.locked })
        })
        .collect();
    Value::Array(regions)
}

fn board_fixture_value(document: &Puzzle5dDocument, camera2d: &Puzzle5dCamera2d) -> Value {
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
        "targetRegions": board_target_regions_value(document),
        "meta": {
            "kindCatalogs": board_kind_catalogs_value(document),
            "kindCompatibility": document.kind_compatibility.clone().unwrap_or(json!([])),
        },
    })
}

fn board_brush_weights_json(runtime: &Puzzle5dRuntime) -> String {
    json!({ "nodeWeights": runtime.object_kind_weights, "handleWeights": runtime.vortex_kind_weights }).to_string()
}

/// 🎬️ The board host's whole scene payload for one render — `pub` so the cross-pane interaction law
/// can assert the board and world projections of ONE snapshot against each other.
pub fn puzzle5d_board_scene(envelope: &Puzzle5dScene) -> Board2dScene {
    Board2dScene {
        fixture_json: board_fixture_value(&envelope.document, &envelope.runtime.camera2d).to_string(),
        camera_json: board_camera_value(&envelope.runtime.camera2d).to_string(),
        glyph_catalogs_json: board_kind_catalogs_value(&envelope.document).to_string(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection and hover both come
        // from the framework-owned `vortex` domain the ONE 5d interaction snapshot reads — the same
        // domain the world pane projects, so a part picked or hovered in either pane paints in both.
        selection_json: envelope.interaction.selection_json(),
        interactive: true,
        hovered_id: envelope.interaction.hovered_id().map(str::to_string),
        active_utility: Some(puzzle5d_scene_mode(&envelope.active_utility).to_string()),
        selection_method: "rectangle".into(),
        grid_visible: envelope.runtime.grid_visible,
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        // 🎯️ The board's own pick filter, projected from this pane's `selectable_kinds` record in the
        // engine's node/edge/handle vocabulary (part/fastener/grip).
        selectable_nodes: envelope.runtime.selectable_kinds.parts,
        selectable_edges: envelope.runtime.selectable_kinds.fasteners,
        selectable_handles: envelope.runtime.selectable_kinds.grips,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_weights_json: board_brush_weights_json(&envelope.runtime),
        placement_compatibility_json: envelope.document.kind_compatibility.clone().unwrap_or(json!([])).to_string(),
        lod_mode: envelope.runtime.lod_mode.clone(),
        transform_flags: Some(json!({ "move": envelope.runtime.transform_move, "rotate": envelope.runtime.transform_rotate }).to_string()),
        domain_id: Some(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN.into()),
        suggestion_menu_json: None,
        tool_run_trace: None,
        lanes: Vec::new(),
    }
}
//#endregion 🔖️BoardJson

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let scene = puzzle5d_board_scene(envelope);
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Board2d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
