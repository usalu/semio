//! ✏️ Block 3D play app — the `edit` mode: the single-window world authoring layout.

use crate::editor::block3d::modes::edit::windows::world;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const BLOCK3D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::block3d::create_block3d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: BLOCK3D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`. A single full-width window (block3d has exactly one window kind).
pub fn layout() -> WindowLayout {
    create_default_layout(&[world::BLOCK3D_WINDOW_WORLD.into()], "row", Some(&[100.0]), Some(&["Object Kind".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_the_world_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(world::BLOCK3D_WINDOW_WORLD), "layout must reference the world window kind: {json}");
    }
}
//#endregion 🧪️Tests
