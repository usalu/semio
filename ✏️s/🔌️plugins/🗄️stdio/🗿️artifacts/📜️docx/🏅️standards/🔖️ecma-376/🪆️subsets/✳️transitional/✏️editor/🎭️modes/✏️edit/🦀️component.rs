//! ✏️ Docx transitional editor — the `edit` mode: a single full-pane Document window over
//! `DocxDocument.body`.

use crate::editor::docx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const DOCX_TRANSITIONAL_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_docx_transitional_editor` (this subset's
/// surface root).
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: DOCX_TRANSITIONAL_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Document window — one page per top-level block, no quadrant layout to
/// allocate.
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Document".into()), instance_id: None, template_id: None, corner: None }] }) }
}
//#endregion 🔖️Definition
