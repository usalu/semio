
use crate::editor::puzzle5d::create_puzzle5d_app;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use semio_framework_plugin::{WindowLayoutChild, WindowLayoutRoot};

#[test]
fn default_layout_is_board_left_two_fifths_and_world_right_three_fifths() {
    let app = create_puzzle5d_app();
    let layout = app.default_layout.as_ref().expect("default layout");
    let WindowLayoutRoot::Axis(root) = &layout.root else {
        panic!("default layout root must be a row axis");
    };
    assert_eq!(root.kind, "row");
    assert_eq!(root.children.len(), 2);
    let WindowLayoutChild::Stack(board) = &root.children[0] else {
        panic!("left pane must be a stack");
    };
    let WindowLayoutChild::Stack(world) = &root.children[1] else {
        panic!("right pane must be a stack");
    };
    assert!((board.size.unwrap() - 40.0).abs() < 1e-9);
    assert!((world.size.unwrap() - 60.0).abs() < 1e-9);
    assert_eq!(board.children[0].window_kind_id, board2d::WINDOW_KIND_ID);
    assert_eq!(world.children[0].window_kind_id, world3d::WINDOW_KIND_ID);
    assert_eq!(board.children[0].title.as_deref(), Some("Puzzle 2D"));
    assert_eq!(world.children[0].title.as_deref(), Some("Puzzle 3D"));
}
