use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_read_only_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_record_with_35_columns() {
    let mut document = EpwSnapshot::default();
    document.records.push(Default::default());
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");
    // 🌦️ `TableWindowKit` contract: `columnsJson` is `{id, label}` records, `rowsJson` is
    // `{id, <column id>: cell}` records keyed by column position.
    let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
    assert_eq!(columns.len(), 35);
    assert_eq!(columns[0]["id"], serde_json::json!("0"));
    let rows: Vec<serde_json::Map<String, serde_json::Value>> = serde_json::from_str(&scene.rows_json).expect("rows json");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 36);
}
