//! 🌳️ Energy model viewer — `structure` window: a real, READ-ONLY overview tree of the working
//! `crate::model::Model` behind the artifact's composed `structure` child, built from the framework
//! `TreeWindowKit` (contract §2.6). Independent render from the sibling mutation-capable surface — the
//! same `crate::energy_model` read, no edit affordances (`window_kind()`, the
//! read-only variant, not the editable one).

use crate::EnergyModelSnapshot;
use semio_framework_plugin::app::{TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EnergyModelSnapshot -> UiNode` read: the same grouped tree the editor draws, from
/// `crate::energy_structure_tree` (artifact root, never the editor surface) — grouped so no node
/// exceeds the kit's 32-sibling ceiling.
pub fn render(document: &EnergyModelSnapshot) -> UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render(&crate::energy_structure_tree(&crate::energy_model(document)))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
