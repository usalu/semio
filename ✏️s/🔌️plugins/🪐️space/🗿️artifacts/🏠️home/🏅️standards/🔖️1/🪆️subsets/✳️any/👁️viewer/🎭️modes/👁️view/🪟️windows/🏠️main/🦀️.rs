//! 🏠️ S Home viewer — the main window: a READ-ONLY render of the SAME overview table the editor's own
//! main window renders (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS), built from
//! the SAME plugin-root `crate::home_space_rows` helper AND the SAME plugin-root `crate::HomeTableLabels`
//! bilingual label set the editor's own main window uses — this file itself imports nothing from the
//! sibling editor surface (`policyViewerPurityBreaches` forbids it outright); both live at the plugin
//! root precisely so both surfaces can reach them without either importing through the other. Six of
//! the editor's seven columns render here (name/kind/visibility/members/updated/origin) — the trailing
//! "Actions" column is dropped, not left empty: a viewer has no row-scoped affordances to summarize
//! there (contract §2.2, `HomeViewer::handle` is structurally `ViewEmit`-only). No row commands, no
//! create/delete/rename/share affordances: a viewer has no utilities that mutate and emits nothing but
//! `ViewEmit` by construction.

use crate::HomeTableLabels;
use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const S_HOME_VIEW_WINDOW: &str = "s-home-view-main";
pub const S_HOME_VIEW_BODY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::home::create_home_viewer`.
pub fn definition() -> WindowKindDefinition {
    let mut def = TableWindowKit::window_kind();
    def.id = S_HOME_VIEW_WINDOW.into();
    def.label = LocalizedLabel::native("Studios", "Studios");
    def
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::home_space_rows`'s internal union with
/// `crate::list_all_space_catalog_entries()`'s process-global catalog singleton (shared across every
/// test in this crate's test binary — a `DirectoryReadModel::default()` alone is NOT enough to reach an
/// empty row list, since the local catalog half is unconditionally unioned in and never guaranteed
/// empty once any other test has created a studio).
fn render_rows(rows: &[crate::HomeSpaceRow], labels: &HomeTableLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(labels.empty_message.as_str().to_string()))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let empty = semio_framework_plugin::UiText::try_from_str("").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.actions-label", "fixed table label admission failed"))?;
    let mut view = TableRowsView::new(empty);
    for column in [labels.column_name, labels.column_kind, labels.column_visibility, labels.column_members, labels.column_updated, labels.column_origin] {
        let column = semio_framework_plugin::UiText::try_from_str(column.as_str()).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.column", "fixed table column admission failed"))?;
        view.try_push_column(column).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.columns", "fixed table column admission failed"))?;
    }
    for row in rows {
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("space:{}", row.id)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in [&row.name, &row.kind, &row.visibility, &row.members, &row.updated] {
            let cell = semio_framework_plugin::UiText::try_from_str(cell).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.cell", "fixed table cell admission failed"))?;
            table_row.try_push_cell(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        }
        let origin = if row.origin == "hub" { labels.origin_hub.as_str() } else { labels.origin_local.as_str() };
        let origin = semio_framework_plugin::UiText::try_from_str(origin).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.origin", "fixed table origin admission failed"))?;
        table_row.try_push_cell(origin).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        view.try_push_row(table_row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.rows", "fixed table row admission failed"))?;
    }
    // 🆔️ No row has an action (the viewer never renders row affordances), so
    // `TableWindowKit::render_rows` never appends the trailing actions column — `actions_label` is
    // therefore inert here, kept empty rather than reaching for a label nothing displays.
    TableWindowKit::render_rows(view)
}

/// 👁️ No `SHomeSnapshot` argument: exactly like the editor's own main-window render, Home's table rows
/// are derived entirely from `HomeConfig.directory` + the live studio catalog, never from the artifact
/// document itself — see `HomeApp::handle`'s doc comment in the editor for the same observation.
pub fn render(directory: &store::os_directory::DirectoryReadModel, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = semio_framework_plugin::resolve_labels::<HomeTableLabels>(view_state);
    // 🌉️ `crate::home_space_rows` is a plugin-root async fn (outside this lease); `render` must
    // stay sync (called synchronously by `HomeViewer::render`) — bridged via `resolve_ready`.
    let identity = crate::home_session_identity(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("s.home.session-identity-required", "current host session identity is required"))?;
    let rows = semio_framework_plugin::resolve_ready(crate::home_space_rows(directory, &identity.user_id));
    render_rows(&rows, labels)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
