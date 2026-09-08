
use super::*;
use crate::editor::puzzle3d::create_puzzle3d_app;

#[test]
fn default_layout_is_top_left_third_and_perspective_right_two_thirds() {
    let app = create_puzzle3d_app();
    let layout = app.default_layout.as_ref().expect("default layout");
    let WindowLayoutRoot::Axis(root) = &layout.root else {
        panic!("default layout root must be a row axis");
    };
    assert_eq!(root.kind, "row");
    assert_eq!(root.children.len(), 2);
    let WindowLayoutChild::Stack(top) = &root.children[0] else {
        panic!("left pane must be a stack");
    };
    let WindowLayoutChild::Stack(perspective) = &root.children[1] else {
        panic!("right pane must be a stack");
    };
    assert!((top.size.unwrap() - 100.0 / 3.0).abs() < 1e-9);
    assert!((perspective.size.unwrap() - 200.0 / 3.0).abs() < 1e-9);
    let top_window = &top.children[0];
    let perspective_window = &perspective.children[0];
    assert_eq!(top_window.window_kind_id, main::WINDOW_KIND_ID);
    assert_eq!(perspective_window.window_kind_id, main::WINDOW_KIND_ID);
    assert_eq!(top_window.instance_id.as_deref(), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(perspective_window.instance_id.as_deref(), Some(main::WINDOW_INSTANCE_PERSPECTIVE));
    assert_eq!(top_window.title.as_deref(), Some("Top"));
    assert_eq!(perspective_window.title.as_deref(), Some("Perspective"));
    assert_eq!(top_window.template_id.as_deref(), Some(main::TEMPLATE_TOP));
    assert_eq!(perspective_window.template_id.as_deref(), Some(main::TEMPLATE_PERSPECTIVE));
}
