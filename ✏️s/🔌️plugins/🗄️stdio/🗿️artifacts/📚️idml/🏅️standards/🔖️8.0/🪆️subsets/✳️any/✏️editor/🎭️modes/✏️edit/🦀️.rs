//! ✏️ Idml editor — the `edit` mode: one full-pane Document window over the story runs.

use semio_plugin::{ModeDefinition, WindowLayout, WindowLayoutNode};
use semio_ui_locale::LocalizedLabel;

#[path = "🪟️windows/🪟️main/🦀️.rs"]
pub mod main;

pub const IDML_EDIT_MODE_ID: &str = "edit";

/// 🧱️ Stitched into the editor manifest by `create_idml_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: IDML_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Document window.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutNode::Stack { size: None,
            active_window_id: None,
            children: vec![WindowLayoutNode::Window { window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Document".into()), instance_id: None, template_id: None, corner: None }],
        },
    }
}
