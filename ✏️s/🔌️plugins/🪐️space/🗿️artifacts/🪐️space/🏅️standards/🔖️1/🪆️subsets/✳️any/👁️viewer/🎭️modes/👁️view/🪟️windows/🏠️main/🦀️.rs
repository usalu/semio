//! 👁️ SpaceIndexViewer — the `main` window: the same read-only table of the space's artifacts. Uses
//! the shared `TableWindowKit`'s read-only `window_kind()` (no `set-cell` action) — never imports
//! anything from the sibling `✏️editor` (`policyViewerPurityBreaches`).

use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceIndexTableLabels};
use semio_framework_plugin::app::{table_window_row, TableWindowKit, TreeWindows, WindowKit};
use semio_framework_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ The viewer folds no `fold-directory-events`/`presence-heartbeat` commands of its own (no `Config`
/// state to fold into — `NoConfig`), so its presence cell is always empty; the editor's window is the one
/// live presence source. One `TableRow` record per artifact, no row actions: the viewer has no mutating
/// affordance.
pub fn render(document: &SSpaceSnapshot, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = semio_framework_plugin::resolve_labels::<SpaceIndexTableLabels>(view_state);
    TableWindowKit::render_rows(&TreeWindows::for_body(view_state, BODY_KEY), labels.table_name.as_str(), &labels.columns(), None, &document.artifacts, |row| {
        let cells = labels.row(row, "");
        table_window_row(&format!("artifact:{}", row.id), &cells.each_ref().map(String::as_str), std::iter::empty(), None)
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
