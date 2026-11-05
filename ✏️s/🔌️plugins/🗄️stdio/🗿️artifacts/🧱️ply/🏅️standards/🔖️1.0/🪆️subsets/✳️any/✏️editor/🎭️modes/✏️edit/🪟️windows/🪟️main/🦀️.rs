//! 🧊 PLY editor — the Main window: the document's own geometry on the shared `MeshWindowKit` (the same scene as the sibling viewer window). The window itself never mutates; edits arrive through the details pane and the transform gesture handled by the surface root.

use crate::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot;
use semio_framework_plugin::{BuiltNode, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`. The EDITABLE
/// variant — `MeshWindowKit::window_kind()` — carries the frozen `set-vertex` action.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧊️ The document's own geometry on the shared mesh scene: real triangles or points, framed on their bounds, never a placeholder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PlySnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (positions, triangles, markers) = crate::schema::inferences::scene_geometry(document);
    semio_s_artifact_stdio_contract::mesh_scene("PLY", &positions, &triangles, &markers)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
