//! 📋️ Imperative viewer — the main window: a read-only table of the document's top-level steps, built
//! from the framework's `TableWindowKit` (contract §2.6) rather than hand-rolling a scene the way the
//! sibling editor window's `build_table_scene` call does — a viewer table has no run-output row and no
//! localized column labels (no `Config`, so no locale to read them from).

use crate::ProcedureSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedure::create_imperative_viewer`. The
/// window's own label stays "Steps" — `TableWindowKit::window_kind()`'s generic "Table" label is
/// overridden here the same way every kit consumer is expected to (contract §2.6 gives the kit's id/
/// body-key/surface-kind, not a fixed per-app label).
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Steps", "Schritte"), icon_id: "list".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `ProcedureSnapshot -> UiNode` read: one row per top-level step (`index`, `id`, `kind`),
/// English-only headers (a viewer has no persisted locale — `Config = NoConfig`), no run-output row
/// (the editor's own `run` view-action is a `Command`, and the viewer declares none).
pub fn render(document: &ProcedureSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let path = crate::procedure_working_scene(document).path;
    let rows = path.steps.iter().enumerate().map(|(index, step)| vec![(index + 1).to_string(), step.id.clone(), step.kind.clone()]).collect();
    TableWindowKit::render(&TableView { columns: vec!["#".into(), "Id".into(), "Kind".into()], rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
