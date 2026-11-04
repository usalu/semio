//! 👁️ Idml viewer — the `view` mode: the read-only twin of the editor's single Document window.

use semio_framework_plugin::{ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
use semio_framework_ui_locale::LocalizedLabel;

#[path = "🪟️windows/🪟️main/🦀️.rs"]
pub mod main;

pub const IDML_VIEW_MODE_ID: &str = "view";

/// 🧱️ Stitched into the viewer manifest by `create_idml_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: IDML_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Document window.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Document".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
