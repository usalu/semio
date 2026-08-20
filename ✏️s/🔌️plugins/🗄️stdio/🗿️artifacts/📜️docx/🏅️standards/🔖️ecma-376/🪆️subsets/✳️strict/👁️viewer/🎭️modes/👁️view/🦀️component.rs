//! 👁️ Docx strict viewer — the `view` mode: the read-only twin of the mutation-capable surface's
//! single full-pane Document window, same page-per-block content, no edit affordances.

use crate::viewer::docx::standards::v_ecma_376::subsets::strict::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const DOCX_STRICT_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_docx_strict_viewer` (this subset's surface
/// root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: DOCX_STRICT_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Same single-window layout as the sibling surface's own layout — read-only twin, no quadrant
/// to allocate for edit affordances.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Document".into()), instance_id: None, template_id: None, corner: None }] }) }
}
//#endregion 🔖️Definition
