//! 👁️ SpaceIndexViewer — the `main` window: the same read-only table of the space's artifacts. Uses
//! the shared `TableWindowKit`'s read-only `window_kind()` (no `set-cell` action) — never imports
//! anything from the sibling `✏️editor` (`policyViewerPurityBreaches`).

use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{space_index_table_row, SSpaceSnapshot, SPACE_INDEX_TABLE_COLUMNS};
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
mod tests {
    use super::*;

    fn project(node: semio_framework_plugin::BuiltNode) -> String {
        semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Space viewer tree projection")
    }

    fn observe<R>(node: semio_framework_plugin::BuiltNode, inspect: impl FnOnce(&semio_framework_plugin::BuiltNode) -> R) -> R {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inspect(&node)));
        let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
        while !retirement.terminal_is_empty() {
            let step = retirement.close_step(1, 4096).expect("Space viewer fixture tree remains valid");
            if !step.progressed {
                std::thread::yield_now();
            }
        }
        match result {
            Ok(result) => result,
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_node_for_the_default_document() {
        let _ = project(render(&SSpaceSnapshot::default()).expect("default Space viewer rows"));
    }

    /// 🆔️ Contract §C0: the read-only viewer's rows must still carry `data-row-id="artifact:<id>"` —
    /// it just never attaches row action buttons to it.
    #[semio_framework_async_macros::async_test]
    async fn a_row_stamps_the_artifact_row_id_with_no_actions_cell() {
        use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
        let mut document = SSpaceSnapshot::default();
        document.artifacts.push(SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() });
        observe(render(&document).expect("Space viewer rows"), |root| {
            let row = root.children.iter().find(|node| node.key.as_str() == "artifact:artifact-1").expect("Space viewer row id");
            assert!(!row.children.iter().any(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))), "the viewer never carries a row action button");
        });
    }
}
//#endregion 🧪️Tests
