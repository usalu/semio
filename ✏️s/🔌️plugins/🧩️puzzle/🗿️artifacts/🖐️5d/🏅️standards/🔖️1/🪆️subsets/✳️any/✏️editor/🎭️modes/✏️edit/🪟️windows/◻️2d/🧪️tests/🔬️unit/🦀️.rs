
use super::*;
use crate::editor::puzzle5d::testkit::*;

#[test]
fn renders_the_board_scene() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("board-2d"));
}
