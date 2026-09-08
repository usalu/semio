
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_view_layout_lists_the_main_window() {
    let json = pack::to_json_string(&layout());
    assert!(json.contains(main::S_HOME_VIEW_WINDOW), "layout must reference the main window kind: {json}");
}
