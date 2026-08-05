//! ✏️ Imperative play app — the `edit` mode: the app's only mode, a tabbed layout over the steps table
//! and the compiled script.

use crate::apps::imperative::modes::edit::windows::{main, script};
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const IMPERATIVE_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::imperative::create_imperative_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: IMPERATIVE_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_stack_layout(&[main::IMPERATIVE_PLAY_WINDOW_MAIN.into(), script::IMPERATIVE_PLAY_WINDOW_SCRIPT.into()], Some(&["Imperative".into(), "Script".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::IMPERATIVE_PLAY_WINDOW_MAIN) && json.contains(script::IMPERATIVE_PLAY_WINDOW_SCRIPT), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
