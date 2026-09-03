//! ✏️ Zip editor (2.0/✳️base) — the `edit` mode: a single window over the archive tree.

use crate::editor::zip::base::modes::edit::windows::main;
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const ZIP_ANY_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::zip::base::create_zip_any_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ZIP_ANY_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window, one layout slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into()], Some(&["Archive".into()]))
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
