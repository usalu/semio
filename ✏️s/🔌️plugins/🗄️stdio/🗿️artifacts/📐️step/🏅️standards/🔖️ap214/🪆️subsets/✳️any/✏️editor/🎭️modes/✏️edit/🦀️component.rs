//! ✏️ Step Any editor — the `edit` mode: a single full-pane Main window.

use crate::editor::step_any::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const STEP_ANY_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: STEP_ANY_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Main window.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Main".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
