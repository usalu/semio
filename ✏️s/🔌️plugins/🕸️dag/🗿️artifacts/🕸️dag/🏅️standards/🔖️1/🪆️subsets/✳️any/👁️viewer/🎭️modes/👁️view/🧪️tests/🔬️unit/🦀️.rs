use crate::viewer::dag::modes::view::windows::main;
use crate::viewer::dag::modes::view::*;

#[semio_framework_async_macros::async_test]
async fn the_view_layout_lists_the_main_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::DAG_VIEW_WINDOW_MAIN), "layout must reference the main window kind: {json}");
}
