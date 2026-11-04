//! 🧊 STEP window — the document's own cartesian points on the shared `MeshWindowKit`; it never mutates (edits arrive through the Details pane and the transform gesture) and the viewer variant imports nothing from the sibling editor module (`policyViewerPurityBreaches`).

use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
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
/// 🧊️ The document's own `CARTESIAN_POINT` entities as point markers on the shared mesh scene, framed on their bounds, never a placeholder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &StepSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    semio_s_artifact_stdio_contract::mesh_scene("STEP", &[], &[], &document.scene_points())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
