//! ✏️ Xlsx editor (ecma-376/✳️any) — the `edit` mode: a single-window layout hosting the `🪟️main`
//! table over the workbook's flattened cells (see the surface root's `xlsx_flat_cells` doc comment).

use crate::editor::xlsx::standards::v_ecma_376::subsets::any::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const XLSX_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_xlsx_editor` (subset root).
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: XLSX_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window filling the whole canvas — this subset has exactly one real window.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Cells".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
