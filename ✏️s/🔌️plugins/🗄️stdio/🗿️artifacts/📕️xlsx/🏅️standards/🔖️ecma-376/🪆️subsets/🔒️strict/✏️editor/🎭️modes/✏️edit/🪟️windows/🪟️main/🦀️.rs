//! 📊️ Xlsx editor (ecma-376/🔒️strict) — `main` window: a real, directly editable flat table of
//! every cell in the workbook, one row per `(sheet, row, col, value)`, built from the framework
//! `TableWindowKit` (contract §2.6). Columns `sheet`/`row`/`col` are the cell's identity (read-only,
//! same "id column not an edit target" convention `🔋️energy`'s own `zones` window establishes);
//! `value` is the sole `set-cell` edit target, addressed by worksheet identity plus native row and
//! column with an optimistic revision guard.

use crate::editor::xlsx::standards::v_ecma_376::subsets::strict::{render_xlsx_cell_value, xlsx_cell_revision, xlsx_flat_cells};
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{EditableTableCell, TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, UiMapBuilder, UiText, UiValue, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_xlsx_strict_editor` (subset root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Cells", "Zellen"), icon_id: "table-2".into(), ..semio_s_artifact_stdio_contract::stable_addressed_table_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `XlsxSnapshot -> BuiltNode`: one row per cell, columns `sheet`/`row`/`col`/`value`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XlsxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let shared_strings = &document.workbook.shared_strings;
    let columns = vec!["sheet".to_string(), "row".to_string(), "col".to_string(), "value".to_string()];
    let cells = xlsx_flat_cells(document);
    let rows = cells.iter().map(|(sheet, row, col, value)| vec![sheet.clone(), row.to_string(), col.to_string(), render_xlsx_cell_value(value, shared_strings)]).collect();
    let editable_cells = cells
        .iter()
        .enumerate()
        .map(|(row_index, (sheet, row, column, value))| {
            let mut arguments = UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "cell argument map capacity"))?;
            arguments
                .try_insert("sheetName".into(), UiValue::Text(UiText::try_from_str(sheet).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.sheet-name", "worksheet name exceeds the UI text bound"))?))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "worksheet argument capacity"))?;
            arguments.try_insert("row".into(), UiValue::Number(*row as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "row argument capacity"))?;
            arguments.try_insert("column".into(), UiValue::Number(*column as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "column argument capacity"))?;
            arguments
                .try_insert("revision".into(), UiValue::Text(UiText::try_from_string(xlsx_cell_revision(value)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-revision", "cell revision exceeds the UI text bound"))?))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "revision argument capacity"))?;
            Ok(EditableTableCell::new(row_index, 3, "set-cell", UiValue::Map(arguments.finish())))
        })
        .collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
    TableWindowKit::render_editable_cells(&TableView { columns, rows }, "s.stdio.xlsx@ecma-376/strict#editor", &editable_cells)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
