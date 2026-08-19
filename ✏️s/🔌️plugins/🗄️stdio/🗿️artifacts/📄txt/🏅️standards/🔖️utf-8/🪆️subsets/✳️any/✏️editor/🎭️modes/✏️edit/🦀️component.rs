//! ✏️ Txt editor — the `edit` mode: a single full-pane Text window over the whole document buffer.

use crate::editor::txt::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TXT_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::txt::create_txt_editor`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: TXT_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Text window — one whole-document buffer, no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Text".into()), instance_id: None, template_id: None, corner: None }] }) }
}
//#endregion 🔖️Definition
