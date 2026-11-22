
use semio_plugin::{Axis, WindowLayoutNode};
use crate::editor::puzzle5d::create_puzzle5d_app;
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d;

#[test]
fn default_layout_is_board_left_two_fifths_and_world_right_three_fifths() {
    let app = create_puzzle5d_app();
    let layout = app.default_layout.as_ref().expect("default layout");
    let WindowLayoutNode::Split { axis: Axis::Horizontal, children, .. } = &layout.root else {
        panic!("default layout root must be a row split");
    };
    assert_eq!(children.len(), 2);
    let WindowLayoutNode::Stack { size: board_size, children: board_windows, .. } = &children[0] else {
        panic!("left pane must be a stack");
    };
    let WindowLayoutNode::Stack { size: world_size, children: world_windows, .. } = &children[1] else {
        panic!("right pane must be a stack");
    };
    assert!((board_size.unwrap() - 0.4).abs() < 1e-9);
    assert!((world_size.unwrap() - 0.6).abs() < 1e-9);
    let WindowLayoutNode::Window { window_kind_id: board_kind, title: board_title, .. } = &board_windows[0] else { panic!("board tab must be a window") };
    let WindowLayoutNode::Window { window_kind_id: world_kind, title: world_title, .. } = &world_windows[0] else { panic!("world tab must be a window") };
    assert_eq!(board_kind, board2d::WINDOW_KIND_ID);
    assert_eq!(world_kind, world3d::WINDOW_KIND_ID);
    assert_eq!(board_title.as_deref(), Some("Puzzle 2D"));
    assert_eq!(world_title.as_deref(), Some("Puzzle 3D"));
}
