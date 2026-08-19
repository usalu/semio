//! ✏️ EPW editor — the `edit` mode: a single window over every hourly weather record.

use crate::editor::epw::modes::edit::windows::main;
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const EPW_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::epw::create_epw_editor`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: EPW_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window, one layout slot.
pub async fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into()], Some(&["Weather Records".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_edit_layout_lists_the_one_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
