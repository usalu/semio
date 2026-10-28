//! ✏️ IFC 2x3 Any editor — the `edit` mode: a single full-pane Main window.

use crate::editor::ifc2x3_any::modes::edit::windows::main;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const IFC2X3_ANY_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: IFC2X3_ANY_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Geometry, selected-instance attributes and full snapshot Details.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    crate::standards::v2x3::subsets::base::io::inspector::layout(main::WINDOW_KIND_ID,true)
}
//#endregion 🔖️Definition
