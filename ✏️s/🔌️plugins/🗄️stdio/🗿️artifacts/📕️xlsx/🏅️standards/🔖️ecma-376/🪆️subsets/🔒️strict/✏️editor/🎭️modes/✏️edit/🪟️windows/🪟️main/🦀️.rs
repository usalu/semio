//! 📊️ Xlsx editor (ecma-376/🔒️strict) — `main` window: a real, directly editable flat table of every
//! cell in the workbook, one row per `(sheet, row, col, value)`, built from the framework
//! `TableWindowKit` (contract §2.6). Columns `sheet`/`row`/`col` are the cell's identity (read-only,
//! same "id column not an edit target" convention `🔋️energy`'s own `zones` window establishes);
//! `value` is the sole `set-cell` edit target, addressed by worksheet identity plus native row and
//! column with an optimistic revision guard.

use crate::editor::xlsx::standards::v_ecma_376::subsets::strict::render_xlsx_cell_value;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address;
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::{BuiltNode, Locale, LocalizedLabel, TreeWindows, UiLabel, UiMapBuilder, UiText, UiValue, WindowKindDefinition};

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
pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let workbook = document.project_workbook().map_err(|error| semio_framework_plugin::PluginAssemblyError::new("xlsx.projection", error.to_string()))?;
    let shared_strings = &workbook.shared_strings;
    let controller_id = "s.stdio.xlsx@ecma-376/strict#editor";
    let total = workbook.sheets.iter().map(|sheet| sheet.cells.len()).sum();
    let labels = match locale {
        Locale::De => ["Arbeitsblatt", "Zeile", "Spalte", "Wert"],
        Locale::En => ["Sheet", "Row", "Column", "Value"],
    };
    TableWindowKit::render_indexed_matrix_with_id(
        windows,
        TableWindowKit::KIND_ID,
        match locale {
            Locale::De => "Zellen",
            Locale::En => "Cells",
        },
        labels[1],
        labels[2],
        labels.len(),
        |column| UiLabel::try_from(labels[column]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.column-label", "column label exceeds the UI label bound")),
        None,
        total,
        |ordinal, columns| {
            let mut remaining = ordinal;
            let (sheet, cell) = workbook
                .sheets
                .iter()
                .find_map(|sheet| {
                    if remaining < sheet.cells.len() {
                        Some((sheet.name.as_str(), &sheet.cells[remaining]))
                    } else {
                        remaining -= sheet.cells.len();
                        None
                    }
                })
                .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-address", "windowed cell ordinal is outside the workbook"))?;
            let mut arguments = UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "cell argument map capacity"))?;
            let address = xlsx_cell_address(document, sheet, cell.row, cell.col).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-address", error))?;
            arguments
                .try_insert("sheetName".into(), UiValue::Text(UiText::try_from_str(sheet).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.sheet-name", "worksheet name exceeds the UI text bound"))?))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "worksheet argument capacity"))?;
            arguments.try_insert("row".into(), UiValue::Number(cell.row as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "row argument capacity"))?;
            arguments.try_insert("column".into(), UiValue::Number(cell.col as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "column argument capacity"))?;
            arguments
                .try_insert("revision".into(), UiValue::Text(UiText::try_from_string(address.revision).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-revision", "cell revision exceeds the UI text bound"))?))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "revision argument capacity"))?;
            let mut arguments = Some(UiValue::Map(arguments.finish()));
            let cells = columns
                .clone()
                .map(|column| {
                    let value = match column {
                        0 => sheet.to_owned(),
                        1 => cell.row.to_string(),
                        2 => cell.col.to_string(),
                        3 => render_xlsx_cell_value(&cell.value, shared_strings),
                        _ => String::new(),
                    };
                    Ok(if column == 3 {
                        WindowedEditableTableCell::new(value, labels[column], "set-cell", arguments.take().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.strict.cell-arguments", "value column was projected more than once"))?)
                    } else {
                        WindowedEditableTableCell::read_only(value, labels[column])
                    })
                })
                .collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
            editable_table_window_row_at(&format!("workbook-cell-{ordinal}"), controller_id, locale, columns.start, cells, Vec::new())
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
