//! 👁️ Json viewer — the `view` mode: a single full-pane Tree window, the read-only counterpart of
//! the editor's `edit` mode.

use crate::viewer::json_i_json::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const JSON_I_JSON_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::json_i_json::create_json_i_json_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: JSON_I_JSON_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Tree window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Tree".into()), instance_id: None, template_id: None }] }) }
}
//#endregion 🔖️Definition
