//! 🎲️ Puzzle 2d viewer — the Board window: a read-only mesh scene built with the shared
//! `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), from the
//! same artifact-level `Puzzle2dSnapshot` the editor's own overview/selection/detail panes render —
//! this file itself imports nothing from the sibling editor surface (`policyViewerPurityBreaches`
//! forbids it outright). No selection, no engagement, no brush/fill utility chrome: a viewer emits no
//! mutations by construction (`ViewEmit`). Circle/rectangle nodes flatten onto the world-3d ground
//! plane (`z = 0`) as sphere/box instances — the same 2-in-3 placeholder-geometry compromise the
//! editor's own board host makes for its canvas, not a new simplification introduced here.

use crate::{Puzzle2dNode, Puzzle2dSnapshot};
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
/// 👁️ Matches the editor's own circle/rectangle shape vocabulary (`Puzzle2dNode.shape`) mapped onto
/// the mesh-engine's built-in placeholder kinds — duplicated on purpose rather than imported through
/// the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
const PUZZLE2D_VIEW_CIRCLE_MESH_KIND: &str = "sphere";
const PUZZLE2D_VIEW_RECTANGLE_MESH_KIND: &str = "box";
const PUZZLE2D_VIEW_DEFAULT_RADIUS: f64 = 24.0;
const PUZZLE2D_VIEW_DEFAULT_WIDTH: f64 = 48.0;
const PUZZLE2D_VIEW_DEFAULT_HEIGHT: f64 = 48.0;
/// 👁️ A thin but visible extrusion along the flattened axis — the fixture itself carries no depth.
const PUZZLE2D_VIEW_FLAT_DEPTH: f64 = 4.0;
/// 👁️ Read-only default overhead camera looking straight down the flattened board — a viewer has no
/// persisted per-session camera (`Config = NoConfig`), unlike the editor's `Puzzle2dConfig` runtime.
const PUZZLE2D_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [0.0, 0.0, 800.0];
const PUZZLE2D_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const PUZZLE2D_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle2d::create_puzzle2d_viewer`.
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn is_rectangle(node: &Puzzle2dNode) -> bool {
    node.shape.as_deref() == Some("rectangle") || (node.shape.is_none() && node.width.is_some())
}

/// 👁️ Read-only twin of the editor's own node placement — real per-node position/kind/label, flattened
/// onto `z = 0` (duplicated shape math, not imported, per `policyViewerPurityBreaches`).
fn world_instances_json(snapshot: &Puzzle2dSnapshot) -> String {
    let instances: Vec<dsl::DslValue> = snapshot
        .nodes
        .iter()
        .map(|node| {
            let rectangle = is_rectangle(node);
            let mesh_id = if rectangle { PUZZLE2D_VIEW_RECTANGLE_MESH_KIND } else { PUZZLE2D_VIEW_CIRCLE_MESH_KIND };
            let scale = if rectangle {
                [node.width.unwrap_or(PUZZLE2D_VIEW_DEFAULT_WIDTH), node.height.unwrap_or(PUZZLE2D_VIEW_DEFAULT_HEIGHT), PUZZLE2D_VIEW_FLAT_DEPTH]
            } else {
                let diameter = node.radius.unwrap_or(PUZZLE2D_VIEW_DEFAULT_RADIUS) * 2.0;
                [diameter, diameter, PUZZLE2D_VIEW_FLAT_DEPTH]
            };
            dsl::DslValue::object([
                ("id".to_string(), dsl::DslValue::String(node.id.clone())),
                ("meshId".to_string(), dsl::DslValue::String(mesh_id.to_string())),
                ("position".to_string(), dsl::ToValue::to_value(&[node.x, node.y, 0.0])),
                ("rotation".to_string(), dsl::ToValue::to_value(&[0.0, 0.0, 0.0, 1.0])),
                ("scale".to_string(), dsl::ToValue::to_value(&scale)),
                ("label".to_string(), dsl::DslValue::String(node.text.clone().unwrap_or_default())),
            ])
        })
        .collect();
    dsl::json::to_json_string(&dsl::DslValue::Array(instances))
}

fn world_meshes_json() -> String {
    let meshes = dsl::DslValue::Array(vec![
        dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(PUZZLE2D_VIEW_CIRCLE_MESH_KIND.to_string())), ("data".to_string(), dsl::ToValue::to_value(&mesh_from_kind(PUZZLE2D_VIEW_CIRCLE_MESH_KIND)))]),
        dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(PUZZLE2D_VIEW_RECTANGLE_MESH_KIND.to_string())), ("data".to_string(), dsl::ToValue::to_value(&mesh_from_kind(PUZZLE2D_VIEW_RECTANGLE_MESH_KIND)))]),
    ]);
    dsl::json::to_json_string(&meshes)
}

/// 👁️ Pure `Puzzle2dSnapshot -> UiNode` read: default overhead camera, no selection/utility/engagement
/// overlay, real node positions/shapes read straight off the document.
pub fn render(document: &Puzzle2dSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::BuiltNode> {
    let view = MeshView {
        camera_json: world3d_camera_json(PUZZLE2D_VIEW_DEFAULT_CAMERA_POSITION, PUZZLE2D_VIEW_DEFAULT_CAMERA_TARGET, PUZZLE2D_VIEW_DEFAULT_CAMERA_FOV),
        meshes_json: world_meshes_json(),
        instances_json: world_instances_json(document),
        selection_json: world3d_selection_json("rectangle", &[], None),
    };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
