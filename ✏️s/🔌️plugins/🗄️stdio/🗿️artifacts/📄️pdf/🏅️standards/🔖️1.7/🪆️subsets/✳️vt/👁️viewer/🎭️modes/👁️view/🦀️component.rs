//! 👁️ PDF/VT Document (1.7) viewer -- the `view` mode: the read-only twin of the mutation-capable surface's
//! single-window layout, hosting `main`'s `DocumentWindowKit` surface, no edit affordances.

use crate::viewer::pdf17vt::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PDF17VT_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::pdf17vt::create_pdf17_vt_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PDF17VT_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Same single-window layout as the mutation-capable counterpart's own -- read-only twin, no
/// quadrant to allocate for edit affordances.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Pages".into()), instance_id: None, template_id: None, corner: None }] }),
    }
}
//#endregion 🔖️Definition
