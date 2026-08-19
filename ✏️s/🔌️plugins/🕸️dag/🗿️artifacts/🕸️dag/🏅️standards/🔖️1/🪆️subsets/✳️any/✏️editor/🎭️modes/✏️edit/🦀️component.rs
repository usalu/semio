//! ✏️ DAG play app — the `edit` mode: the app's only mode, a two-window authoring layout (node-graph
//! canvas + compiled DSL).

use crate::editor::dag::modes::edit::windows::{compiled, main};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const DAG_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::dag::create_dag_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: DAG_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub async fn layout() -> WindowLayout {
    create_default_layout(&[main::DAG_PLAY_WINDOW_MAIN.into(), compiled::DAG_PLAY_WINDOW_COMPILED.into()], "row", Some(&[68.0, 32.0]), Some(&["DAG".into(), "DSL".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::DAG_PLAY_WINDOW_MAIN) && json.contains(compiled::DAG_PLAY_WINDOW_COMPILED), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
