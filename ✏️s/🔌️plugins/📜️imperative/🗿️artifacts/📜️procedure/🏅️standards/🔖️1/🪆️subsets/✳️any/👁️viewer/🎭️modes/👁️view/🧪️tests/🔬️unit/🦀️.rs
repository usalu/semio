use super::*;

#[semio_framework_async_macros::async_test]
async fn the_view_layout_lists_both_read_only_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WINDOW_KIND_ID) && json.contains(script::WINDOW_KIND_ID), "layout must reference both window kinds: {json}");
}
