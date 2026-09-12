use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;

#[test]
fn renders_puzzle2d_board_scene() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("board-2d"));
}
