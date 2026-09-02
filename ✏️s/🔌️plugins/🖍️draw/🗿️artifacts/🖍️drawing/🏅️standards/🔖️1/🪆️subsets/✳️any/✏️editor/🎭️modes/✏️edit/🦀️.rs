//! ✏️ Drawing play app — the `edit` mode: the single-window canvas authoring layout.

use crate::editor::drawing::modes::edit::windows::canvas;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const DRAWING_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::drawing::create_drawing_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: DRAWING_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[canvas::DRAWING_PLAY_WINDOW_CANVAS.into()], "row", Some(&[100.0]), Some(&["Canvas".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_the_canvas_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(canvas::DRAWING_PLAY_WINDOW_CANVAS), "layout must reference the canvas window kind: {json}");
    }
}
//#endregion 🧪️Tests
