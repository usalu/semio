//! 🧊 OBJ viewer — the Main window: a read-only world-3d scene of the document's own geometry on the shared `MeshWindowKit`. This file imports nothing from the sibling editor module (`policyViewerPurityBreaches`).

use crate::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot;
use semio_framework_plugin::{BuiltNode, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by the surface root's `create_*_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧊️ The document's own geometry on the shared mesh scene: real triangles or points, framed on their bounds, never a placeholder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &ObjSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (positions, triangles, markers) = crate::engine::scene_geometry(document);
    semio_s_artifact_stdio_contract::mesh_scene("OBJ", &positions, &triangles, &markers)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
