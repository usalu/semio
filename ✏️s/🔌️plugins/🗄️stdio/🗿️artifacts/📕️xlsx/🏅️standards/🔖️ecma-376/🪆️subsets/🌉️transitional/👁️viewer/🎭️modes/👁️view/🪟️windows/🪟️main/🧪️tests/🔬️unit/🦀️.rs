use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_cell() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let document = XlsxSnapshot { workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }], ..Default::default() }, ..XlsxSnapshot::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");
    // 📊️ `TableWindowKit` contract: `columnsJson` is `{id, label}` records, `rowsJson` is
    // `{id, <column id>: cell}` records keyed by column position.
    let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
    assert_eq!(columns, vec![serde_json::json!({ "id": "0", "label": "sheet" }), serde_json::json!({ "id": "1", "label": "row" }), serde_json::json!({ "id": "2", "label": "col" }), serde_json::json!({ "id": "3", "label": "value" })]);
    let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
    assert_eq!(rows, vec![serde_json::json!({ "id": "0", "0": "Sheet1", "1": "1", "2": "0", "3": "1" })]);
}
