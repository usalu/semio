//! ✏️ Puzzle 3d play app — the `edit` mode: the dual-pane default layout (an orthographic Top pane
//! on the left third, a three-point Perspective pane on the right two thirds — two INSTANCES of the
//! one `🪟️windows/🧊️main` window kind) plus the mode-level Fill tool. The per-window chrome options
//! every instance shares live in `☑️options/*`.

use crate::editor::puzzle3d::modes::edit::tools::fill;
use crate::editor::puzzle3d::modes::edit::windows::main;
use semio_framework_plugin::{create_window_layout, LocalizedLabel, ModeDefinition, ToolRef, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};

pub const PUZZLE3D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: PUZZLE3D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![semio_framework::io::resolve_ready(ToolRef::new(fill::TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ Top (left ⅓) + Perspective (right ⅔) — the default dual-pane workbench for Puzzle 3D and the Aggregator.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(100.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(main::WINDOW_KIND_ID, Some("Top".into()), Some(main::WINDOW_INSTANCE_TOP.into()), Some(main::TEMPLATE_TOP.into()))],
                }),
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(200.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(main::WINDOW_KIND_ID, Some("Perspective".into()), Some(main::WINDOW_INSTANCE_PERSPECTIVE.into()), Some(main::TEMPLATE_PERSPECTIVE.into()))],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
