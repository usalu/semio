use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;

#[test]
fn renders_puzzle2d_board_scene() {
    let mut app = app();
    let body = render_body(&mut app, BODY_KEY);
    close_app(&mut app);
    assert!(body.contains("board-2d"));
}
