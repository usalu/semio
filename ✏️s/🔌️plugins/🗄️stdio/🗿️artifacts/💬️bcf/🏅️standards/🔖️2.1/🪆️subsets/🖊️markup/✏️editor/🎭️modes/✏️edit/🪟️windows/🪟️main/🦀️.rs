//! 📊 BCF editor — the Main window: the SAME topic table as the sibling viewer window,
//! built with the shared `TableWindowKit`'s EDITABLE variant (contract §2.6, action id `set-cell`).
//! Render is identical to the viewer's read; mutation is the surface root's `handle()` responsibility.

use crate::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use semio_framework_plugin::app::{editable_table_window_row, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::Locale;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    semio_s_artifact_stdio_contract::revision_addressed_table_window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn columns(locale: Locale) -> [&'static str; 5] {
    match locale {
        Locale::De => ["GUID", "Titel", "Status", "Priorität", "Autor"],
        Locale::En => ["GUID", "Title", "Status", "Priority", "Author"],
    }
}

pub fn render(document: &BcfSnapshot, locale: Locale, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(document);
    render_revisioned(document, &revision, publication_revision, locale, &TreeWindows::unhosted())
}

/// 🔐️ Renders only the host-requested topic rows against the captured store revision.
pub fn render_revisioned(document: &BcfSnapshot, revision: &str, publication_revision: semio_framework_plugin::UiPublicationRevision, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = columns(locale);
    TableWindowKit::render_indexed_rows(
        windows,
        match locale {
            Locale::De => "BCF-Themen",
            Locale::En => "BCF topics",
        },
        &columns,
        None,
        document.topics.len(),
        |row| {
            let topic = &document.topics[row];
            let values = [&topic.guid, &topic.title, &topic.status, &topic.priority, &topic.creation_author];
            let mut cells = Vec::with_capacity(values.len());
            for (column, value) in values.into_iter().enumerate() {
                if column == 0 {
                    cells.push(WindowedEditableTableCell::read_only(value.clone(), columns[column]));
                } else {
                    let arguments = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_arguments(row, column, revision)?;
                    cells.push(WindowedEditableTableCell::new(value.clone(), columns[column], "set-cell", arguments).publication_revision(publication_revision));
                }
            }
            editable_table_window_row(&format!("topic-{row}"), "s.stdio.bcf@2.1/*#editor", locale, cells, std::iter::empty(), None)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
