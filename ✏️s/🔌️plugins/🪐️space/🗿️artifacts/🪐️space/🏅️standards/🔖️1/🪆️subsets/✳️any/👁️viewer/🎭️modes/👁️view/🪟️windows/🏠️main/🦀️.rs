//! 👁️ SpaceIndexViewer — the `main` window: the same read-only table of the space's artifacts. Uses
//! the shared `TableWindowKit`'s read-only `window_kind()` (no `set-cell` action) — never imports
//! anything from the sibling `✏️editor` (`policyViewerPurityBreaches`).

use crate::standards::v1::subsets::any::schema::snapshot::{space_index_table_row, SSpaceSnapshot, SPACE_INDEX_TABLE_COLUMNS};
use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};
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
pub fn render(document: &SSpaceSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let empty = semio_framework_plugin::UiText::try_from_str("").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.actions-label", "fixed table label admission failed"))?;
    let mut view = TableRowsView::new(empty);
    for column in SPACE_INDEX_TABLE_COLUMNS {
        let column = semio_framework_plugin::UiText::try_from_str(column).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.column", "fixed table column admission failed"))?;
        view.try_push_column(column).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.columns", "fixed table column admission failed"))?;
    }
    // 👁️ The viewer folds no `fold-directory-events`/`presence-heartbeat` commands of its own (no
    // `Config` state to fold into — `NoConfig`), so its presence cell is always empty; the editor's
    // window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main`) is the one live presence source.
    for row in &document.artifacts {
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("artifact:{}", row.id)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in space_index_table_row(row, "") {
            let cell = semio_framework_plugin::UiText::try_from_string(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cell", "fixed table cell admission failed"))?;
            table_row.try_push_cell(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        }
        view.try_push_row(table_row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.rows", "fixed table row admission failed"))?;
    }
    // 🆔️ No row has an action (the viewer has no mutating affordance), so `render_rows` never appends
    // the trailing actions column — `actions_label` is inert here, kept empty.
    TableWindowKit::render_rows(view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
