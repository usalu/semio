//! 👁️ Draw viewer — the `view` mode: a single full-pane Canvas window, the read-only counterpart of
//! the editor's single-window `edit` mode. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! contract §1 requires ≥1 real window per viewer mode — the Canvas window is the draw artifact's
//! natural (and only) surface.

use crate::viewer::draw::modes::view::windows::canvas;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const DRAW_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::draw::create_draw_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: DRAW_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Canvas window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: canvas::WINDOW_KIND_ID.into(), title: Some("Canvas".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
