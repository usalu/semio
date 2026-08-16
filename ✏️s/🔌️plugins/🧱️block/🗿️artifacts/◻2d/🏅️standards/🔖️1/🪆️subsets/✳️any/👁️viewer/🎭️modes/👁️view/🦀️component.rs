//! 👁️ Block 2D viewer — the `view` mode: a single full-pane board window, the read-only
//! counterpart of the editor's single `edit` mode.

use crate::viewer::block2d::modes::view::windows::board;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const BLOCK2D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::block2d::create_block2d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: BLOCK2D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The viewer's default window layout — a single full-width board window (the viewer has no
/// quadrant/edit chrome to allocate).
pub fn layout() -> WindowLayout {
    create_default_layout(&[board::WINDOW_KIND_ID.into()], "row", Some(&[100.0]), Some(&["Node Kind".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_board_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(board::WINDOW_KIND_ID), "layout must reference the board window kind: {json}");
    }
}
//#endregion 🧪️Tests
