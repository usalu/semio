//! 👁️ Note viewer — the `view` mode: a single read-only canvas window, the read-only counterpart of
//! the editor's two-window `edit` mode. A first-pass viewer needs only the composite canvas render —
//! the Navigator window is an editing-overview aid (camera minimap for the interactive canvas next to
//! it) with no independent read-only value of its own.

use crate::viewer::note::modes::view::windows::composite;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const NOTE_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::note::create_note_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: NOTE_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Composite window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: composite::WINDOW_KIND_ID.into(), title: Some("Canvas".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
