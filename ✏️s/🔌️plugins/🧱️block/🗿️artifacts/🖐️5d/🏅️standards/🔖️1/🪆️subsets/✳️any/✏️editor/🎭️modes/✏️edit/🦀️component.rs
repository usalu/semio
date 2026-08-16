//! ✏️ Block 5D play app — the `edit` mode: the two-window (board + world) authoring layout.

use crate::editor::block5d::modes::edit::windows::{board, world};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const BLOCK5D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block5d::create_block5d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: BLOCK5D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`. A 50/50 split between the board and world windows.
pub fn layout() -> WindowLayout {
    create_default_layout(&[board::BLOCK5D_WINDOW_BOARD.into(), world::BLOCK5D_WINDOW_WORLD.into()], "row", Some(&[50.0, 50.0]), Some(&["Board".into(), "World".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(board::BLOCK5D_WINDOW_BOARD));
        assert!(json.contains(world::BLOCK5D_WINDOW_WORLD));
    }
}
//#endregion 🧪️Tests
