//! 🔱️ Trinity Jack viewer — the `view` mode: a single full-pane Graph window, the read-only
//! counterpart of the editor's three-window `edit` mode. A first-pass viewer needs only the graph
//! render — the Jack Query / Results windows are editor-only tooling, not part of the read-only
//! surface.

use crate::viewer::jack::modes::view::windows::graph;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TRINITY_JACK_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::jack::create_trinity_jack_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: TRINITY_JACK_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Graph window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: graph::WINDOW_KIND_ID.into(), title: Some("Nakagin Graph".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
