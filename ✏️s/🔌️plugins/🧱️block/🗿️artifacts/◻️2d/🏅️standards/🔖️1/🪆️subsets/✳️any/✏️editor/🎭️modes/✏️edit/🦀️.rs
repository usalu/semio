//! ✏️ Block 2D play app — the `edit` mode: the single-window board authoring layout.

use crate::editor::block2d::modes::edit::windows::board;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const BLOCK2D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block2d::create_block2d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: BLOCK2D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`. A single full-width window (block2d has exactly one window kind).
pub fn layout() -> WindowLayout {
    create_default_layout(&[board::BLOCK2D_WINDOW_BOARD.into()], "row", Some(&[100.0]), Some(&["Node Kind".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_the_board_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(board::BLOCK2D_WINDOW_BOARD), "layout must reference the board window kind: {json}");
    }
}
//#endregion 🧪️Tests
