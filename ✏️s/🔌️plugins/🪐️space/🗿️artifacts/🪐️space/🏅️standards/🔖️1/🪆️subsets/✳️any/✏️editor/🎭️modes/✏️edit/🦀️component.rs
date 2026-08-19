//! ✏️ SpaceIndexEditor — the `edit` mode: the app's only mode, a single full-pane artifact table.

use crate::editor::space_index::modes::edit::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout};

pub const SPACE_INDEX_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: SPACE_INDEX_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "table".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub async fn layout() -> WindowLayout {
    semio_framework_plugin::create_default_layout(&[main::WINDOW_KIND_ID.into()], "row", Some(&[100.0]), Some(&["Artifacts".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_mode_is_the_apps_default() {
        assert_eq!(definition().id, SPACE_INDEX_MODE_EDIT);
    }
}
//#endregion 🧪️Tests
