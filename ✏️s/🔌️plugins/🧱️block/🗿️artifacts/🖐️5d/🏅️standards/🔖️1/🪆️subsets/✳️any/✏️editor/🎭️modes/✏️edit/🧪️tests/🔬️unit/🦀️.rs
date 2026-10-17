
#[allow(unused_imports)] use semio_s_artifact_block_5d_editor::modes::edit::windows::{board, world};
#[allow(unused_imports)] use semio_s_artifact_block_5d_editor::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(board::BLOCK5D_WINDOW_BOARD));
    assert!(json.contains(world::BLOCK5D_WINDOW_WORLD));
}
