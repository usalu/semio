//! 👁️ SpaceIndexViewer — the `main` window: the same read-only table of the space's artifacts. Uses
//! the shared `TableWindowKit`'s read-only `window_kind()` (no `set-cell` action) — never imports
//! anything from the sibling `✏️editor` (`policyViewerPurityBreaches`).

use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{space_index_table_row, SSpaceSnapshot, SPACE_INDEX_TABLE_COLUMNS};
use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &SSpaceSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("artifact:{}", row.id))
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in space_index_table_row(row, "") {
            let cell = semio_framework_plugin::UiText::try_from_string(cell)
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cell", "fixed table cell admission failed"))?;
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

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_node_for_the_default_document() {
        let _node = render(&SSpaceSnapshot::default());
    }

    /// 🆔️ Contract §C0: the read-only viewer's rows must still carry `data-row-id="artifact:<id>"` —
    /// it just never attaches row action buttons to it.
    #[semio_framework_async_macros::async_test]
    async fn a_row_stamps_the_artifact_row_id_with_no_actions_cell() {
        use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
        let mut document = SSpaceSnapshot::default();
        document.artifacts.push(SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() });
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows_json parses");
        assert_eq!(rows[0]["id"], serde_json::json!("artifact:artifact-1"));
        assert!(rows[0].get("actions").is_none(), "the viewer never carries a row actions cell: {:?}", rows[0]);
    }
}
//#endregion 🧪️Tests
