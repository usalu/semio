//! 📊️ Sparse, windowed, read-only worksheet grids for the XLSX viewer.

use crate::standards::v_ecma_376::subsets::base::io::column_letter;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::{XLSX_MAX_COLUMN, XLSX_MAX_ROW};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet};
use crate::viewer::xlsx::standards::v_ecma_376::subsets::base::render_xlsx_cell_value;
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, tree_window_item, tree_window_section, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::plugin_app_close_prelude::HasBase;
use semio_framework_plugin::{BuiltNode, PluginAssemblyError, TreeWindows, UiLabel, UiText, WindowKindDefinition};
use semio_framework_ui_locale::{Locale, LocalizedLabel};
use std::collections::BTreeMap;

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
const VIEWER_CONTROLLER_ID: &str = "s.stdio.xlsx@ecma-376/*#viewer";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Workbook", "Arbeitsmappe"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}

fn grid_extent(sheet: &XlsxSheet) -> (usize, usize) {
    let max_row = sheet.cells.iter().map(|cell| cell.row).max().unwrap_or(0);
    let max_column = sheet.cells.iter().map(|cell| cell.col).max();
    let rows = match max_row {
        0 => 1,
        XLSX_MAX_ROW => XLSX_MAX_ROW as usize,
        row => row as usize + 1,
    };
    let columns = match max_column {
        None => 1,
        Some(XLSX_MAX_COLUMN) => XLSX_MAX_COLUMN as usize + 1,
        Some(column) => column as usize + 2,
    };
    (rows, columns)
}

fn render_sheet_grid(sheet: &XlsxSheet, shared_strings: &[String], sheet_ordinal: usize, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let cells: BTreeMap<(u32, u32), &XlsxCell> = sheet.cells.iter().map(|cell| ((cell.row, cell.col), cell)).collect();
    let (row_total, column_total) = grid_extent(sheet);
    let table_id = format!("xlsx-viewer-sheet-{sheet_ordinal}-grid");
    let (row_label, column_label) = match locale {
        Locale::En => ("Row", "Column"),
        Locale::De => ("Zeile", "Spalte"),
    };
    TableWindowKit::render_indexed_matrix_with_id(
        windows,
        &table_id,
        &sheet.name,
        row_label,
        column_label,
        column_total,
        |column| UiLabel::try_from(column_letter(column as u32)).map_err(|_| PluginAssemblyError::new("xlsx.column-label", "column label exceeds the UI label bound")),
        None,
        row_total,
        |row_ordinal, columns| {
            let row = row_ordinal as u32 + 1;
            let column_offset = columns.start;
            let projected = columns.map(|column_ordinal| {
                let column = column_ordinal as u32;
                let label = format!("{}{}", column_letter(column), row);
                let value = cells.get(&(row, column)).map_or_else(String::new, |cell| render_xlsx_cell_value(&cell.value, shared_strings));
                WindowedEditableTableCell::read_only(value, label)
            });
            editable_table_window_row_at(&format!("xlsx-viewer-sheet-{sheet_ordinal}-row-{row}"), VIEWER_CONTROLLER_ID, locale, column_offset, projected, Vec::new(), None)
        },
    )
}

pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let workbook = document.project_workbook().map_err(|error| PluginAssemblyError::new("xlsx.projection", error.to_string()))?;
    let ordinals: Vec<usize> = (0..workbook.sheets.len()).collect();
    tree_window_section(windows, "xlsx-viewer-sheets", ui::Label::default(), true, &ordinals, |ordinal| {
        let sheet = &workbook.sheets[*ordinal];
        let item_id = format!("xlsx-viewer-sheet-{ordinal}");
        let item = ui::tree_item_builder(ui::Label(UiText::clipped(&sheet.name))).try_id(&item_id).map_err(|_| PluginAssemblyError::new("xlsx.sheet-item", "worksheet item id exceeds the UI bound"))?;
        tree_window_item(windows, item, &item_id, *ordinal == 0, &[()], |_| render_sheet_grid(sheet, &workbook.shared_strings, *ordinal, locale, windows))
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
