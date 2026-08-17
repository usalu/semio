//! ✏️ Json editor — the `edit` mode: a single full-pane Tree window over the whole `JsonValue`.

use crate::editor::json_i_json::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const JSON_I_JSON_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::json_i_json::create_json_i_json_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: JSON_I_JSON_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Tree window — one document tree, no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Tree".into()), instance_id: None, template_id: None }] }) }
}
//#endregion 🔖️Definition
