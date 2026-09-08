
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::IMPERATIVE_PLAY_WINDOW_MAIN) && json.contains(script::IMPERATIVE_PLAY_WINDOW_SCRIPT), "layout must reference both window kinds: {json}");
}
