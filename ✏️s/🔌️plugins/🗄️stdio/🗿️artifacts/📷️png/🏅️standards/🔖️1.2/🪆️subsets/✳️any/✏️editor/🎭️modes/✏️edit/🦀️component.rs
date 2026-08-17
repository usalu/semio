//! ✏️ `png` edit (any) — the `edit` mode: a single
//! full-pane Main window, the only mode this thin surface declares.

use crate::editor::png::modes::edit::windows::main as main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const MODE_ID: &str = "edit";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Main".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
