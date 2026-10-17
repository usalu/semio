
#[allow(unused_imports)] use semio_s_artifact_block_2d_editor::modes::edit::windows::board;
#[allow(unused_imports)] use semio_s_artifact_block_2d_editor::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_board_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(board::BLOCK2D_WINDOW_BOARD), "layout must reference the board window kind: {json}");
}
