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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
