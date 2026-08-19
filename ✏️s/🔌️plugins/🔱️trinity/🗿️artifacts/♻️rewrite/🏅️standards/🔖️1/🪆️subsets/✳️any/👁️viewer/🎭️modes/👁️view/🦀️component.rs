//! ♻️ Trinity Rewrite viewer — the `view` mode: a single full-pane Rule window, the read-only
//! counterpart of the editor's six-window `edit` mode. A first-pass viewer needs only the compiled
//! rule text — Before/After/LHS/RHS graph panes and the Jack query preview are editor-only tooling,
//! not part of the read-only surface.

use crate::viewer::rewrite::modes::view::windows::rule;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TRINITY_REWRITE_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::rewrite::create_trinity_rewrite_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: TRINITY_REWRITE_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Rule window — the read-only viewer has no multi-window arrangement to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: rule::WINDOW_KIND_ID.into(), title: Some("Rule".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
