//! 👁️ Tsv viewer — the `view` mode: a single full-pane Table window, the read-only counterpart of
//! the editor's `edit` mode.

use crate::viewer::tsv::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const TSV_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::tsv::create_tsv_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: TSV_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Table window — the read-only viewer has no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Table".into()), instance_id: None, template_id: None, corner: None }] }) }
}
//#endregion 🔖️Definition
