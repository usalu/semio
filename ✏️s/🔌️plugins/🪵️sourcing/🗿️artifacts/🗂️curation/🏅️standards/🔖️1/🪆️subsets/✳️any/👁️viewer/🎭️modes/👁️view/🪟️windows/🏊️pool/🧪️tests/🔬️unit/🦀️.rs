
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
    let json = serde_json::to_string(&render(&document).expect("bounded table")).expect("render json");
    assert!(json.contains("table"), "expected a table UiNode: {json}");
}
