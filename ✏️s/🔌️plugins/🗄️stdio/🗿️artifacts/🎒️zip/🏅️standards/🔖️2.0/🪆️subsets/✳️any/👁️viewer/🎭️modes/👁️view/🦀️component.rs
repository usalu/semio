//! 👁️ Zip viewer (2.0/✳️any) — the `view` mode: a single read-only window over the archive tree, the
//! read-only counterpart of the editor's `edit` mode.

use crate::viewer::zip::any::modes::view::windows::main;
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const ZIP_ANY_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::zip::any::create_zip_any_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: ZIP_ANY_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window, one layout slot — mirrors the editor's own default layout shape minus the
/// mutation-shaped window kind.
pub async fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into()], Some(&["Archive".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_view_layout_lists_the_one_read_only_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
