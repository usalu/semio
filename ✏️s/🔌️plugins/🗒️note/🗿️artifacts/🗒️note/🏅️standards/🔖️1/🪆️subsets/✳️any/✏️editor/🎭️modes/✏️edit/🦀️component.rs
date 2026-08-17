//! ✏️ Note play app — the `edit` mode: the default two-window canvas layout (composite + navigator).

use crate::editor::note::modes::edit::windows::{composite, navigator};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const NOTE_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::note::create_note_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: NOTE_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[composite::NOTE_PLAY_WINDOW_COMPOSITE.into(), navigator::NOTE_PLAY_WINDOW_NAVIGATOR.into()], "row", Some(&[72.0, 28.0]), Some(&["Canvas".into(), "Navigator".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(composite::NOTE_PLAY_WINDOW_COMPOSITE) && json.contains(navigator::NOTE_PLAY_WINDOW_NAVIGATOR), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
