
use crate::editor::puzzle5d::create_puzzle5d_app;
use semio_framework_plugin::WindowLayoutRoot;

#[test]
fn default_layout_is_world_three_fifths_and_board_two_fifths() {
    let app = create_puzzle5d_app();
    let layout = app.default_layout.as_ref().expect("default layout");
    let WindowLayoutRoot::Axis(root) = &layout.root else {
        panic!("default layout root must be a row axis");
    };
    assert_eq!(root.kind, "row");
    assert_eq!(root.children.len(), 2);
}
