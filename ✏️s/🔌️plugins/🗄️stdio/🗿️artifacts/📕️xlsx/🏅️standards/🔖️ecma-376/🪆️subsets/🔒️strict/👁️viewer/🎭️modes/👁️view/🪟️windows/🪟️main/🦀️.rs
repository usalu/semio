//! 📊️ Xlsx viewer (ecma-376/🔒️strict) — `main` window: a real, READ-ONLY flat table of every cell
//! in the workbook (same projection the sibling mutation-capable surface's own window renders —
//! independent read, no edit affordances).

use crate::XlsxSnapshot;
use crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::{render_xlsx_cell_value, xlsx_flat_cells};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_xlsx_strict_viewer` (subset root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Cells", "Zellen"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `XlsxSnapshot -> BuiltNode` read: one row per cell, columns `sheet`/`row`/`col`/`value` —
/// no command-driven cell edits (a viewer declares none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XlsxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let shared_strings = &document.workbook.shared_strings;
    let columns = vec!["sheet".to_string(), "row".to_string(), "col".to_string(), "value".to_string()];
    let rows = xlsx_flat_cells(document).into_iter().map(|(sheet, row, col, value)| vec![sheet, row.to_string(), col.to_string(), render_xlsx_cell_value(&value, shared_strings)]).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
