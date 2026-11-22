//! 🗂️ Sourcing curation app — the `curation` mode: the three-column pool/curated+preview/grid workspace.
//! Sourcing has exactly one mode, so this is also the app's `default_mode_id`/`default_layout`.

use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::ModeDefinition;
use semio_plugin::WindowLayout;
use semio_plugin::{WindowLayoutNode, Axis};

pub const SOURCING_CURATION_MODE_CURATION: &str = "curation";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::sourcing::create_sourcing_curation_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SOURCING_CURATION_MODE_CURATION.into(), label: LocalizedLabel::native("Curation", "Kuratierung"), icon_id: "folder-open".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

fn sourcing_window(window_kind_id: &str, title: &str) -> WindowLayoutNode {
    WindowLayoutNode::Window { window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }
}

fn sourcing_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutNode {
    WindowLayoutNode::Stack { size, active_window_id: None, children: vec![sourcing_window(window_kind_id, title)] }
}

/// 🪟️ Three-column layout: pool | curated over preview | grid.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutNode::Split { axis: Axis::Horizontal, size: None,
            children: vec![
                sourcing_stack(pool::SOURCING_CURATION_WINDOW_POOL, "Pool", None),
                WindowLayoutNode::Split { axis: Axis::Vertical, size: Some(0.33),
                    children: vec![sourcing_stack(curated::SOURCING_CURATION_WINDOW_CURATED, "Curated", Some(0.55)), sourcing_stack(preview::SOURCING_CURATION_WINDOW_PREVIEW, "Preview", Some(0.45))],
                },
                sourcing_stack(grid::SOURCING_CURATION_WINDOW_GRID, "Grid", None),
            ],
        },
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
