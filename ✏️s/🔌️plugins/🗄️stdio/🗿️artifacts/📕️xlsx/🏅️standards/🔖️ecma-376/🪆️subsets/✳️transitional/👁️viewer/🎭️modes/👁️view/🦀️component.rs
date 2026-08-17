//! 👁️ Xlsx viewer (ecma-376/✳️transitional) — the `view` mode: read-only twin of the sibling
//! mutation-capable surface's own single-window layout, same `🪟️main` table, no edit affordances.

use crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const XLSX_TRANSITIONAL_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_xlsx_transitional_viewer` (subset root).
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: XLSX_TRANSITIONAL_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window filling the whole canvas — same single-window shape as the sibling mutation-
/// capable surface's own layout.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Cells".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
