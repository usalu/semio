use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_zone() {
    let document = EnergyModelSnapshot::default();
    let table = render(&document).expect("the table window assembles");
    assert_eq!(table.key.as_str(), WINDOW_KIND_ID);
    assert!(table.children.is_empty());
}
