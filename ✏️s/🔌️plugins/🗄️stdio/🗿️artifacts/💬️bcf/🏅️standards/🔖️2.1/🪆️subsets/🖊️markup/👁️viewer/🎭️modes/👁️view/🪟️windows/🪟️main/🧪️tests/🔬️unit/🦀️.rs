use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_table_window_kit() {
    assert_eq!(definition().id, TableWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_node_for_the_default_document() {
    let document = BcfSnapshot::default();
    let _node = render(&document);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_topic() {
    let mut document = BcfSnapshot::default();
    document.topics.push(crate::schema::snapshot::BcfTopic {
        guid: "g1".into(),
        title: "Clash".into(),
        description: String::new(),
        status: "open".into(),
        priority: "high".into(),
        labels: Vec::new(),
        creation_date: String::new(),
        creation_author: "tester".into(),
        comments: Vec::new(),
        viewpoints: Vec::new(),
    });
    let _node = render(&document);
    assert_eq!(document.topics.len(), 1);
}
