//! ✏️ Raster play app — the `edit` mode: the default two-window layout (composite + navigator).

use crate::editor::raster::modes::edit::windows::{composite, navigator};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const RASTER_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::raster::create_raster_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: RASTER_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), navigator::RASTER_PLAY_WINDOW_NAVIGATOR.into()], "row", Some(&[72.0, 28.0]), Some(&["Composite".into(), "Navigator".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(composite::RASTER_PLAY_WINDOW_COMPOSITE) && json.contains(navigator::RASTER_PLAY_WINDOW_NAVIGATOR), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
