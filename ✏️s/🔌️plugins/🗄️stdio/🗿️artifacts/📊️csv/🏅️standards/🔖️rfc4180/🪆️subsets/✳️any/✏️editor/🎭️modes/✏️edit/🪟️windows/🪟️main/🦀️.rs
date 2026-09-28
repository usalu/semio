//! 📊️ Csv editor — `main` window: a real, directly editable table of every `CsvRecord`, built
//! from the framework `TableWindowKit` (contract §2.6). When `has_header` is set, `records[0]`
//! supplies the column labels and only `records[1..]` are rendered as editable rows (RFC 4180
//! draws no structural distinction between a header and a data record, only this convention).

use crate::CsvSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, table_row_action, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::{BuiltNode, UiLabel, Locale, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::csv::create_csv_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..semio_s_artifact_stdio_contract::structural_table_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `CsvSnapshot -> BuiltNode`: header row (if any) supplies column labels, every remaining
/// record is one editable row — `set-cell`'s `row`/`column` index this rendered grid directly
/// (see the surface root's `CsvEditorCommand::SetCell` for the row-offset math back to
/// `CsvMutation::SetField`'s `record_index`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &CsvSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(document);
    render_revisioned(document, &revision, locale, &TreeWindows::unhosted())
}

/// 🔐️ Renders only the host-requested rows against the captured store revision.
pub fn render_revisioned(document: &CsvSnapshot, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let data_rows = if document.has_header && !document.records.is_empty() { &document.records[1..] } else { &document.records[..] };
    let width = document.records.iter().map(|record| record.fields.len()).max().unwrap_or(0);
    let column_name = |index: usize| {
        document
            .has_header
            .then(|| document.records.first()?.fields.get(index).map(|field| field.value.clone()))
            .flatten()
            .unwrap_or_else(|| match locale {
                Locale::De => format!("Spalte {}", index + 1),
                Locale::En => format!("Column {}", index + 1),
            })
    };
    let controller_id = "s.stdio.csv@rfc4180/*#editor";
    let table = || TableWindowKit::render_indexed_matrix_with_id(
        windows,
        TableWindowKit::KIND_ID,
        match locale {
            Locale::De => "CSV-Tabelle",
            Locale::En => "CSV table",
        },
        match locale {
            Locale::De => "Zeile",
            Locale::En => "Row",
        },
        match locale {
            Locale::De => "Spalte",
            Locale::En => "Column",
        },
        width,
        |column| UiLabel::try_from(column_name(column)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.csv.column-label", "column label admission")),
        Some(match locale {
            Locale::De => "Aktionen",
            Locale::En => "Actions",
        }),
        data_rows.len(),
        |row, columns| {
            let record = &data_rows[row];
            let column_offset = columns.start;
            let cells = columns
                .map(|column| {
                    let label = column_name(column);
                    match record.fields.get(column) {
                        Some(field) => semio_s_artifact_stdio_contract::window_kit_revisioned_cell_arguments(row, column, revision).map(|arguments| WindowedEditableTableCell::new(field.value.clone(), label, "set-cell", arguments)),
                        None => Ok(WindowedEditableTableCell::read_only(String::new(), label)),
                    }
                })
                .collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
            let remove = table_row_action(
                "trash-2",
                match locale {
                    Locale::De => "Zeile entfernen",
                    Locale::En => "Remove row",
                },
                (
                    semio_framework_plugin::ActionId::try_v1(controller_id, semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("stdio.csv.remove-row", "invalid action"))?,
                    Some(semio_s_artifact_stdio_contract::window_kit_indexed_revision_arguments("row", row, revision)?),
                ),
            )?;
            editable_table_window_row_at(&format!("row-{row}"), controller_id, locale, column_offset, cells, [remove])
        },
    );
    semio_s_artifact_stdio_contract::render_structural_table(width, column_name, true, controller_id, revision, locale, windows, table)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
