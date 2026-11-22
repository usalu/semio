//! 🪟️ The assembled puzzle3d app opens on the edit mode's default layout: a top view left third, a perspective right two thirds.


use semio_plugin::{Axis, WindowLayoutNode};
use semio_s_artifact_puzzle_3d_editor::create_puzzle3d_app;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;

#[test]
fn default_layout_is_top_left_third_and_perspective_right_two_thirds() {
    let app = create_puzzle3d_app();
    let layout = app.default_layout.as_ref().expect("default layout");
    let WindowLayoutNode::Split { axis: Axis::Horizontal, children, .. } = &layout.root else {
        panic!("default layout root must be a row split");
    };
    assert_eq!(children.len(), 2);
    let WindowLayoutNode::Stack { size: top_size, children: top_windows, .. } = &children[0] else {
        panic!("left pane must be a stack");
    };
    let WindowLayoutNode::Stack { size: perspective_size, children: perspective_windows, .. } = &children[1] else {
        panic!("right pane must be a stack");
    };
    assert!((top_size.unwrap() - 1.0 / 3.0).abs() < 1e-9);
    assert!((perspective_size.unwrap() - 2.0 / 3.0).abs() < 1e-9);
    let WindowLayoutNode::Window { window_kind_id: top_kind, instance_id: top_instance, title: top_title, template_id: top_template, .. } = &top_windows[0] else { panic!("top tab must be a window") };
    let WindowLayoutNode::Window { window_kind_id: perspective_kind, instance_id: perspective_instance, title: perspective_title, template_id: perspective_template, .. } = &perspective_windows[0] else { panic!("perspective tab must be a window") };
    assert_eq!(top_kind, main::WINDOW_KIND_ID);
    assert_eq!(perspective_kind, main::WINDOW_KIND_ID);
    assert_eq!(top_instance.as_deref(), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(perspective_instance.as_deref(), Some(main::WINDOW_INSTANCE_PERSPECTIVE));
    assert_eq!(top_title.as_deref(), Some("Top"));
    assert_eq!(perspective_title.as_deref(), Some("Perspective"));
    assert_eq!(top_template.as_deref(), Some(main::TEMPLATE_TOP));
    assert_eq!(perspective_template.as_deref(), Some(main::TEMPLATE_PERSPECTIVE));
}
