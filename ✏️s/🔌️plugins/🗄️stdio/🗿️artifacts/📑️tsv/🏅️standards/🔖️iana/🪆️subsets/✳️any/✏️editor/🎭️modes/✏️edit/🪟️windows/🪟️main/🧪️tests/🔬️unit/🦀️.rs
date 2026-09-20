use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_record() {
    let document = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["a".into(), "b".into()]], trailing_newline: false, line_ending: Default::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");
    // 📑️ `TableWindowKit` contract: `columnsJson` is `{id, label}` records, `rowsJson` is
    // `{id, <column id>: cell}` records keyed by column position.
    let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
    assert_eq!(columns, vec![serde_json::json!({ "id": "0", "label": "Column 1" }), serde_json::json!({ "id": "1", "label": "Column 2" })]);
    let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
    assert_eq!(rows, vec![serde_json::json!({ "id": "0", "0": "a", "1": "b" })]);
}
