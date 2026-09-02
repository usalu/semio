//! 🗂️ Sourcing curation app — the `curation` mode: the three-column pool/curated+preview/grid workspace.
//! Sourcing has exactly one mode, so this is also the app's `default_mode_id`/`default_layout`.

use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const SOURCING_CURATION_MODE_CURATION: &str = "curation";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::sourcing::create_sourcing_curation_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SOURCING_CURATION_MODE_CURATION.into(), label: LocalizedLabel::native("Curation", "Kuratierung"), icon_id: "folder-open".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

fn sourcing_window(window_kind_id: &str, title: &str) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }
}

fn sourcing_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size, active_window_kind_id: None, children: vec![sourcing_window(window_kind_id, title)] })
}

/// 🪟️ Three-column layout: pool | curated over preview | grid.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.34), children: vec![sourcing_stack(pool::SOURCING_CURATION_WINDOW_POOL, "Pool", None)] }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.33),
                    children: vec![sourcing_stack(curated::SOURCING_CURATION_WINDOW_CURATED, "Curated", Some(0.55)), sourcing_stack(preview::SOURCING_CURATION_WINDOW_PREVIEW, "Preview", Some(0.45))],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.33), children: vec![sourcing_stack(grid::SOURCING_CURATION_WINDOW_GRID, "Grid", None)] }),
            ],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_every_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        for id in [pool::SOURCING_CURATION_WINDOW_POOL, curated::SOURCING_CURATION_WINDOW_CURATED, preview::SOURCING_CURATION_WINDOW_PREVIEW, grid::SOURCING_CURATION_WINDOW_GRID] {
            assert!(json.contains(id), "layout must reference window kind {id}: {json}");
        }
    }
}
//#endregion 🧪️Tests
