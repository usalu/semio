//! 📊️ Windowed per-worksheet spreadsheet grids with revision-bound occupied and vacant cells.

use crate::editor::xlsx::standards::v_ecma_376::subsets::base::render_xlsx_cell_value;
use crate::standards::v_ecma_376::subsets::base::io::column_letter;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::{xlsx_cell_address, xlsx_worksheet_address, XLSX_MAX_COLUMN, XLSX_MAX_ROW};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet};
use crate::XlsxSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, tree_window_item, tree_window_section, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::plugin_app_close_prelude::HasBase;
use semio_framework_plugin::{BuiltNode, PluginAssemblyError, TreeWindows, UiLabel, UiMapBuilder, UiText, UiValue, WindowKindDefinition};
use semio_framework_ui_locale::{Locale, LocalizedLabel};
use std::collections::BTreeMap;

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
pub const BASE_CONTROLLER_ID: &str = "s.stdio.xlsx@ecma-376/*#editor";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Workbook", "Arbeitsmappe"), icon_id: "table-2".into(), ..semio_s_artifact_stdio_contract::stable_addressed_table_window_kind() }
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

fn cell_arguments(sheet_name: &str, row: u32, column: u32, revision: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("xlsx.cell-arguments", "cell argument map capacity"))?;
    arguments
        .try_insert("sheetName".into(), UiValue::Text(UiText::try_from_str(sheet_name).ok_or_else(|| PluginAssemblyError::new("xlsx.sheet-name", "worksheet name exceeds the UI text bound"))?))
        .map_err(|_| PluginAssemblyError::new("xlsx.cell-arguments", "worksheet argument capacity"))?;
    arguments.try_insert("row".into(), UiValue::Number(row as f64)).map_err(|_| PluginAssemblyError::new("xlsx.cell-arguments", "row argument capacity"))?;
    arguments.try_insert("column".into(), UiValue::Number(column as f64)).map_err(|_| PluginAssemblyError::new("xlsx.cell-arguments", "column argument capacity"))?;
    arguments
        .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| PluginAssemblyError::new("xlsx.cell-revision", "cell revision exceeds the UI text bound"))?))
        .map_err(|_| PluginAssemblyError::new("xlsx.cell-arguments", "revision argument capacity"))?;
    Ok(UiValue::Map(arguments.finish()))
}

fn render_sheet_grid(
    document: &XlsxSnapshot,
    sheet: &XlsxSheet,
    shared_strings: &[String],
    sheet_ordinal: usize,
    locale: Locale,
    windows: &TreeWindows<'_>,
    controller_id: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let cells: BTreeMap<(u32, u32), &XlsxCell> = sheet.cells.iter().map(|cell| ((cell.row, cell.col), cell)).collect();
    let worksheet = xlsx_worksheet_address(document, &sheet.name).map_err(|error| PluginAssemblyError::new("xlsx.worksheet-address", error))?;
    let (row_total, column_total) = grid_extent(sheet);
    let table_id = format!("xlsx-sheet-{sheet_ordinal}-grid");
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
            let mut projected = Vec::with_capacity(columns.len());
            for column_ordinal in columns.clone() {
                let column = column_ordinal as u32;
                let (value, revision) = match cells.get(&(row, column)) {
                    Some(cell) => {
                        let address = xlsx_cell_address(document, &sheet.name, row, column).map_err(|error| PluginAssemblyError::new("xlsx.cell-address", error))?;
                        (render_xlsx_cell_value(&cell.value, shared_strings), address.revision)
                    }
                    None => (String::new(), worksheet.revision.clone()),
                };
                let label = format!("{}{}", column_letter(column), row);
                projected.push(WindowedEditableTableCell::new(value, label, "set-cell", cell_arguments(&sheet.name, row, column, &revision)?).publication_revision(publication_revision));
            }
            editable_table_window_row_at(&format!("xlsx-sheet-{sheet_ordinal}-row-{row}"), controller_id, locale, columns.start, projected, Vec::new(), None)
        },
    )
}

pub(crate) fn render_for_controller(
    document: &XlsxSnapshot,
    locale: Locale,
    windows: &TreeWindows<'_>,
    controller_id: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let workbook = document.project_workbook().map_err(|error| PluginAssemblyError::new("xlsx.projection", error.to_string()))?;
    let ordinals: Vec<usize> = (0..workbook.sheets.len()).collect();
    tree_window_section(windows, "xlsx-sheets", ui::Label::default(), true, &ordinals, |ordinal| {
        let sheet = &workbook.sheets[*ordinal];
        let item_id = format!("xlsx-sheet-{ordinal}");
        let item = ui::tree_item_builder(ui::Label(UiText::clipped(&sheet.name))).try_id(&item_id).map_err(|_| PluginAssemblyError::new("xlsx.sheet-item", "worksheet item id exceeds the UI bound"))?;
        tree_window_item(windows, item, &item_id, *ordinal == 0, &[()], |_| render_sheet_grid(document, sheet, &workbook.shared_strings, *ordinal, locale, windows, controller_id, publication_revision))
    })
}

pub fn render(document: &XlsxSnapshot, locale: Locale, windows: &TreeWindows<'_>, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_for_controller(document, locale, windows, BASE_CONTROLLER_ID, publication_revision)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
