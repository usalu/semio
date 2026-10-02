//! ✏️ `svg` edit (any) — the `edit` mode: a single
//! full-pane Main window, the only mode this thin surface declares.

use crate::editor::svg_any::modes::edit::windows::main;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;

pub const MODE_ID: &str = "edit";

//#region 🔖️Definition
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main")
}
//#endregion 🔖️Definition
