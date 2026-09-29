use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_splits_header_from_data_rows() {
    let document = CsvSnapshot {
        schema: "stdio.csv".into(),
        has_header: true,
        records: vec![crate::CsvRecord { fields: vec![crate::CsvField { value: "name".into(), quoted: false }] }, crate::CsvRecord { fields: vec![crate::CsvField { value: "ada".into(), quoted: false }] }],
    };
    let node = render(&document).expect("render");
    let scene: semio_framework_ui_scene::TableScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the table scene with its lanes");
    // 📊️ `TableWindowKit` contract: `columnsJson` is `{id, label}` records, `rowsJson` is
    // `{id, <column id>: cell}` records keyed by column position.
    let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
    assert_eq!(columns, vec![serde_json::json!({ "id": "0", "label": "name" })]);
    let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
    assert_eq!(rows, vec![serde_json::json!({ "id": "0", "0": "ada" })]);
}
