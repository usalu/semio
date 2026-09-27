use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_table_window_kit() {
    let def = definition();
    assert_eq!(def.id, TableWindowKit::KIND_ID);
    assert!(def.actions.iter().any(|action| action.id == "set-cell"));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_node_for_the_default_document() {
    let document = BcfSnapshot::default();
    let _node = render(&document);
}

#[semio_framework_async_macros::async_test]
async fn render_binds_each_topic_cell_to_its_address_and_snapshot_revision() {
    use semio_framework_plugin::Component;
    let mut document = BcfSnapshot::default();
    document.topics.push(Default::default());
    document.topics[0].title = "Issue".into();
    let node = render_revisioned(&document, "store-revision").expect("render table");
    let Component::Surface(props) = node.component else { panic!("expected table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode scene");
    let rows: serde_json::Value = serde_json::from_str(&scene.rows_json).expect("rows JSON");
    assert_eq!(rows[0]["1"]["value"], "Issue");
    assert_eq!(rows[0]["1"]["action"]["args"]["row"], 0);
    assert_eq!(rows[0]["1"]["action"]["args"]["column"], 1);
    assert_eq!(rows[0]["1"]["action"]["args"]["revision"], "store-revision");
}
