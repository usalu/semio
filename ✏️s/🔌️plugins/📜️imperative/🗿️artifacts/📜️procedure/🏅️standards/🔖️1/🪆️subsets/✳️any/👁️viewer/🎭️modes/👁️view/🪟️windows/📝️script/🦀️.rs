//! 📝️ Imperative viewer — the script window: a read-only compiled textual form of the document, built
//! from the framework's `TextWindowKit` (contract §2.6). Compiles straight off the pure shared-kernel
//! `imperative_engine::compile_to_text` free function — the sibling editor window's `ImperativeHost`
//! wrapper adds no logic this call needs, it just owns `&mut self` execution state a read-only render
//! never touches, so this file never reaches into the editor module for it.

use crate::ProcedureSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedure::create_imperative_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Script", "Skript"), icon_id: "file-code".into(), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `ProcedureSnapshot -> UiNode` read: the compiled text of the document's own working
/// `Path`, always `read_only: true` (a viewer never emits a `replace-text` command).
pub fn render(document: &ProcedureSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let path = crate::procedure_working_scene(document).path;
    TextWindowKit::render(&TextView { text: imperative_engine::compile_to_text(&path), language: Some("imperative".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
