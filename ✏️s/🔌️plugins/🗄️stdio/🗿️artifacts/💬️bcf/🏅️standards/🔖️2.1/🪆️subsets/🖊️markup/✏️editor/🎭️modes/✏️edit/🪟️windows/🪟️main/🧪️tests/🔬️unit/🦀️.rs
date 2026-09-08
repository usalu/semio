
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
