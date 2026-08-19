//! 👁️ Raster viewer — the `view` mode: Composite (full document) + Navigator (read-only minimap), the
//! read-only counterpart of the editor's `edit` mode. Both windows render the same real composited
//! pixels through `ImageWindowKit` (contract §2.6) — no editing chrome, no utilities.

use crate::viewer::raster::modes::view::windows::{composite, navigator};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const RASTER_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::raster::create_raster_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: RASTER_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Composite dominant, Navigator a minimap strip — mirrors the editor's own `edit::layout()` ratio.
pub async fn layout() -> WindowLayout {
    create_default_layout(&[composite::RASTER_VIEW_WINDOW_COMPOSITE.into(), navigator::RASTER_VIEW_WINDOW_NAVIGATOR.into()], "row", Some(&[72.0, 28.0]), Some(&["Composite".into(), "Navigator".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_view_layout_lists_both_viewer_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(composite::RASTER_VIEW_WINDOW_COMPOSITE) && json.contains(navigator::RASTER_VIEW_WINDOW_NAVIGATOR), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
