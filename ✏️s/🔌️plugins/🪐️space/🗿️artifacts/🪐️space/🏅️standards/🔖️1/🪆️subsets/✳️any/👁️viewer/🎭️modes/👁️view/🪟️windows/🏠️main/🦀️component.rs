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
pub async fn render(document: &SSpaceSnapshot) -> UiNode {
    let columns = SPACE_INDEX_TABLE_COLUMNS.iter().map(|s| s.to_string()).collect();
    // 👁️ The viewer folds no `fold-directory-events`/`presence-heartbeat` commands of its own (no
    // `Config` state to fold into — `NoConfig`), so its presence cell is always empty; the editor's
    // window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main`) is the one live presence source.
    let rows = document.artifacts.iter().map(|row| TableRow { id: format!("artifact:{}", row.id), cells: space_index_table_row(row, ""), actions: Vec::new() }).collect();
    // 🆔️ No row has an action (the viewer has no mutating affordance), so `render_rows` never appends
    // the trailing actions column — `actions_label` is inert here, kept empty.
    TableWindowKit::render_rows(&TableRowsView { columns, rows, actions_label: String::new() })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn render_produces_a_node_for_the_default_document() {
        let _node = render(&SSpaceSnapshot::default());
    }

    /// 🆔️ Contract §C0: the read-only viewer's rows must still carry `data-row-id="artifact:<id>"` —
    /// it just never attaches row action buttons to it.
    #[test]
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
