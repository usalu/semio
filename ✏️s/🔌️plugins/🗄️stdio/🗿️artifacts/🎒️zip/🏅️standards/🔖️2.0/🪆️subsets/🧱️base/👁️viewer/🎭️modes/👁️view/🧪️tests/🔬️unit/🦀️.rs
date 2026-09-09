use super::*;

#[semio_framework_async_macros::async_test]
async fn the_view_layout_lists_the_one_read_only_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
}
