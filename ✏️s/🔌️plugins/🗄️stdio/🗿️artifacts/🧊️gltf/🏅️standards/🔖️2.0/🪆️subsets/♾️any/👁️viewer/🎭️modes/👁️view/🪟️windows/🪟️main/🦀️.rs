//! 🧊 glTF window — the document's own geometry on the shared `MeshWindowKit`; it never mutates (edits arrive through the Details pane and the transform gesture) and the viewer variant imports nothing from the sibling editor module (`policyViewerPurityBreaches`).

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
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
/// 🧊️ The document's own triangles on the shared mesh scene: accessor geometry placed by the node hierarchy, framed on its bounds, never a placeholder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &GltfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (positions, triangles, markers) = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::scene_geometry(&document.document, &document.buffers);
    semio_s_artifact_stdio_contract::mesh_scene("glTF", &positions, &triangles, &markers)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
