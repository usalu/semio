//! 📑️ Tsv editor — `main` window: a real, directly editable table of `TsvSnapshot.records`, built
//! from the framework `TableWindowKit` (contract §2.6). IANA TSV draws no header/data structural
//! distinction, so every record remains an editable data row and localized synthetic labels name
//! the positional columns.

use crate::TsvSnapshot;
use semio_framework_plugin::app::{editable_table_window_row_at, table_row_action, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::{BuiltNode, UiLabel, Locale, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::tsv::create_tsv_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = semio_s_artifact_stdio_contract::structural_table_window_kind();
    definition.actions.retain(|action| action.id != semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID);
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..definition }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `TsvSnapshot -> BuiltNode`: every record remains data and `set-cell`'s row ordinal
/// addresses the snapshot directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TsvSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(document);
    render_revisioned(document, &revision, locale, &TreeWindows::unhosted())
}

/// 🔐️ Renders only the host-requested rows against the captured store revision.
pub fn render_revisioned(document: &TsvSnapshot, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let width = document.records.iter().map(|record| record.len()).max().unwrap_or(0);
    let column_name = |index: usize| match locale {
            Locale::De => format!("Spalte {}", index + 1),
            Locale::En => format!("Column {}", index + 1),
        };
    let controller_id = "s.stdio.tsv@iana/*#editor";
    let table = TableWindowKit::render_indexed_matrix_with_id(
        windows,
        TableWindowKit::KIND_ID,
        match locale {
            Locale::De => "TSV-Tabelle",
            Locale::En => "TSV table",
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
        |column| UiLabel::try_from(column_name(column)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.tsv.column-label", "column label admission")),
        Some(match locale {
            Locale::De => "Aktionen",
            Locale::En => "Actions",
        }),
        document.records.len(),
        |row, columns| {
            let column_offset = columns.start;
            let cells = columns
                .map(|column| {
                    let label = column_name(column);
                    match document.records[row].get(column) {
                        Some(value) => semio_s_artifact_stdio_contract::window_kit_revisioned_cell_arguments(row, column, revision).map(|arguments| WindowedEditableTableCell::new(value.clone(), label, "set-cell", arguments)),
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
                    semio_framework_plugin::ActionId::try_v1(controller_id, semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("stdio.tsv.remove-row", "invalid action"))?,
                    Some(semio_s_artifact_stdio_contract::window_kit_indexed_revision_arguments("row", row, revision)?),
                ),
            )?;
            editable_table_window_row_at(&format!("row-{row}"), controller_id, locale, column_offset, cells, [remove])
        },
    )?;
    semio_s_artifact_stdio_contract::render_structural_table(table, width, column_name, false, controller_id, revision, locale, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
