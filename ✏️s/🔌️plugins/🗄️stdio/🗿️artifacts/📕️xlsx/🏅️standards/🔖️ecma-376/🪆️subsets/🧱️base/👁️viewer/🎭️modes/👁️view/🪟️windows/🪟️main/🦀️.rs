//! 📊️ Xlsx viewer (ecma-376/🧱️base) — `main` window: a real, READ-ONLY flat table of every cell in
//! the workbook (same projection the sibling mutation-capable surface's own window renders —
//! independent read, no edit affordances).

use crate::viewer::xlsx::standards::v_ecma_376::subsets::base::render_xlsx_cell_value;
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::{BuiltNode, UiLabel, Locale, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_xlsx_viewer` (subset root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Cells", "Zellen"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `XlsxSnapshot -> BuiltNode` read: one row per cell, columns `sheet`/`row`/`col`/`value` —
/// no command-driven cell edits (a viewer declares none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let workbook = document.project_workbook().map_err(|error| semio_framework_plugin::PluginAssemblyError::new("xlsx.projection", error.to_string()))?;
    let labels = match locale {
        Locale::De => ["Arbeitsblatt", "Zeile", "Spalte", "Wert"],
        Locale::En => ["Sheet", "Row", "Column", "Value"],
    };
    let total = workbook.sheets.iter().map(|sheet| sheet.cells.len()).sum();
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
        |column| UiLabel::try_from(labels[column]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("xlsx.column-label", "column label exceeds the UI label bound")),
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
                .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("xlsx.cell-address", "windowed cell ordinal is outside the workbook"))?;
            let cells = columns.clone().map(|column| {
                let value = match column {
                    0 => sheet.to_owned(),
                    1 => cell.row.to_string(),
                    2 => cell.col.to_string(),
                    3 => render_xlsx_cell_value(&cell.value, &workbook.shared_strings),
                    _ => unreachable!(),
                };
                WindowedEditableTableCell::read_only(value, labels[column])
            });
            editable_table_window_row_at(&format!("workbook-cell-{ordinal}"), "s.stdio.xlsx@ecma-376/*#viewer", locale, columns.start, cells, Vec::new())
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
