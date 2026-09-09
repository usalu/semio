//! 📊️ Xlsx editor (ecma-376/🌉️transitional) — `main` window: a real, directly editable flat table
//! of every cell in the workbook, one row per `(sheet, row, col, value)`, built from the framework
//! `TableWindowKit` (contract §2.6). Columns `sheet`/`row`/`col` are the cell's identity (read-only,
//! same "id column not an edit target" convention `🔋️energy`'s own `zones` window establishes);
//! `value` is the sole `set-cell` edit target, addressed by table row index into the subset root's
//! own `xlsx_flat_cells` flattening.

use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::{render_xlsx_cell_value, xlsx_flat_cells};
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_xlsx_transitional_editor` (subset root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Cells", "Zellen"), icon_id: "table-2".into(), ..TableWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `XlsxSnapshot -> BuiltNode`: one row per cell, columns `sheet`/`row`/`col`/`value`.
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
