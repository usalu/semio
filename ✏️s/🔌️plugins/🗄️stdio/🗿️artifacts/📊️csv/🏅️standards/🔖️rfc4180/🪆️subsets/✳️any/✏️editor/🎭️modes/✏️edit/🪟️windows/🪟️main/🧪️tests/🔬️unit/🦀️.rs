use super::*;
use semio_framework_plugin::Component;

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
    let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");
    let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][0], "ada");
}
