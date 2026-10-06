
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_framework_table_window_kit() {
    let def = definition();
    assert_eq!(def.id, TableWindowKit::KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn view_model_lists_every_stock_row_with_five_columns() {
    let document = crate::schema::default_document();
    let stock = stock_of(&document);
    let view = view_model(&document);
    assert_eq!(view.columns.len(), 5);
    assert_eq!(view.rows.len(), stock.len());
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_ui_node() {
    let document = crate::schema::default_document();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&document).expect("bounded table"))).expect("bounded retained fixture projection");
    assert!(json.contains("table"), "expected a table UiNode: {json}");
}
