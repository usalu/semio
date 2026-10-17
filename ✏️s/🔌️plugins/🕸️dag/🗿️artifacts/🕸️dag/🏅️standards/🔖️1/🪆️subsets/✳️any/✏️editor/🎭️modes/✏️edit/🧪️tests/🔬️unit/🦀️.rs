use crate::{editor::dag::modes::edit::windows::compiled, editor::dag::modes::edit::windows::main};
use crate::editor::dag::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::DAG_PLAY_WINDOW_MAIN) && json.contains(compiled::DAG_PLAY_WINDOW_COMPILED), "layout must reference both window kinds: {json}");
}
