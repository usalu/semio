//! 📊️ Xlsx viewer (ecma-376/✳️transitional) — `main` window: a real, READ-ONLY flat table of every
//! cell in the workbook (same projection the sibling mutation-capable surface's own window renders
//! — independent read, no edit affordances).

use crate::artifacts::xlsx::XlsxSnapshot;
use crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::{render_xlsx_cell_value, xlsx_flat_cells};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_xlsx_transitional_viewer` (subset root).
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Cells", "Zellen"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `XlsxSnapshot -> UiNode` read: one row per cell, columns `sheet`/`row`/`col`/`value` —
/// no command-driven cell edits (a viewer declares none).
pub async fn render(document: &XlsxSnapshot) -> UiNode {
    let shared_strings = &document.workbook.shared_strings;
    let columns = vec!["sheet".to_string(), "row".to_string(), "col".to_string(), "value".to_string()];
    let rows = xlsx_flat_cells(document).into_iter().map(|(sheet, row, col, value)| vec![sheet, row.to_string(), col.to_string(), render_xlsx_cell_value(&value, shared_strings)]).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_a_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    async fn render_lists_one_row_per_cell() {
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
        let document = XlsxSnapshot { workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }], ..Default::default() }, ..XlsxSnapshot::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows, vec![vec!["Sheet1".to_string(), "1".to_string(), "0".to_string(), "1".to_string()]]);
    }
}
//#endregion 🧪️Tests
