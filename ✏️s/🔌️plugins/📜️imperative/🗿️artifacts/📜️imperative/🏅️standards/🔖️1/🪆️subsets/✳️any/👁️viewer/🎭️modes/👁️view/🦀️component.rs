//! 👁️ Imperative viewer — the `view` mode: a stacked pair of read-only windows (steps table, compiled
//! script), the read-only counterpart of the editor's `edit` mode.

use crate::viewer::imperative::modes::view::windows::{main, script};
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const IMPERATIVE_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::imperative::create_imperative_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: IMPERATIVE_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Stacked steps/script windows — mirrors the editor's own default layout shape, minus any
/// mutation-shaped window.
pub fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into(), script::WINDOW_KIND_ID.into()], Some(&["Steps".into(), "Script".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_view_layout_lists_both_read_only_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::WINDOW_KIND_ID) && json.contains(script::WINDOW_KIND_ID), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
