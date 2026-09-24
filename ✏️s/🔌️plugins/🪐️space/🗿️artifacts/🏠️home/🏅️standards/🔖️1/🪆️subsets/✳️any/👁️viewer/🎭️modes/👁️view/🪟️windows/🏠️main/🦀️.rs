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
use semio_framework_plugin::app::{table_window_row, TableWindowKit, TreeWindows, WindowKit};
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
fn render_rows(rows: &[crate::HomeSpaceRow], labels: &HomeTableLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(labels.empty_message.as_str().to_string()))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let columns = [labels.column_name.as_str(), labels.column_kind.as_str(), labels.column_visibility.as_str(), labels.column_members.as_str(), labels.column_updated.as_str(), labels.column_origin.as_str()];
    TableWindowKit::render_rows(windows, labels.table_name.as_str(), &columns, None, rows, |row| {
        let origin = if row.origin == "hub" { labels.origin_hub.as_str() } else { labels.origin_local.as_str() };
        let key = format!("space:{}", row.id);
        table_window_row(&key, &[row.name.as_str(), row.kind.as_str(), row.visibility.as_str(), row.members.as_str(), row.updated.as_str(), origin], std::iter::empty(), None)
    })
}

/// 👁️ No `SHomeSnapshot` argument: exactly like the editor's own main-window render, Home's table rows
/// are derived entirely from `HomeConfig.directory` + the live studio catalog, never from the artifact
/// document itself — see `HomeApp::handle`'s doc comment in the editor for the same observation.
pub fn render(directory: &store::os_directory::DirectoryReadModel, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = semio_framework_plugin::resolve_labels::<HomeTableLabels>(view_state);
    // 🌉️ `crate::home_space_rows` is a plugin-root async fn (outside this lease); `render` must
    // stay sync (called synchronously by `HomeViewer::render`) — bridged via `resolve_ready`.
    // 🪪️ Signed out is a state, not a fault — the editor's twin of this window carries the full
    // reasoning. A signed-out human owns no spaces, so this publishes the same table empty instead of
    // declining to render the product's landing window.
    let rows = match crate::home_session_identity(view_state) {
        Some(identity) => semio_framework_plugin::resolve_ready(crate::home_space_rows(directory, &identity.user_id)),
        None => Vec::new(),
    };
    render_rows(&rows, labels, &TreeWindows::for_body(view_state, S_HOME_VIEW_BODY))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
