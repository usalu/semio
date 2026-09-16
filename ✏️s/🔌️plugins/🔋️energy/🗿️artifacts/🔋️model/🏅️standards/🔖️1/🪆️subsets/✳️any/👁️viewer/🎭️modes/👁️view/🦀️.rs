//! 👁️ Energy model viewer — the `view` mode: the read-only twin of the editor's split layout — the
//! 3d model viewport left, the structure tree, the zone table and the energy results stacked in a
//! right-hand column. Same windows, no edit affordances and no picking domain.

use crate::viewer::model::modes::view::windows::{model as model_window, simulation, structure, zones};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ENERGY_MODEL_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ENERGY_MODEL_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One pane of the split layout: a stack holding a single window kind. `size` is the pane's share
/// of its own axis — the 3d viewport takes the bigger half of the row, the three data windows split
/// the right-hand column in equal thirds.
fn model_window_stack(window_kind_id: &str, title: &str, size: f64) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size: Some(size),
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// 🪟️ Same split as the sibling mutation-capable surface's own layout — read-only twin, no quadrant
/// to allocate for edit affordances.
pub fn layout() -> WindowLayout {
    let data_column = WindowLayoutChild::Axis(WindowLayoutAxisNode {
        kind: "column".into(),
        size: Some(MODEL_DATA_COLUMN_SHARE),
        children: vec![
            model_window_stack(structure::WINDOW_KIND_ID, "Structure", 1.0 / 3.0),
            model_window_stack(zones::WINDOW_KIND_ID, "Zones", 1.0 / 3.0),
            model_window_stack(simulation::WINDOW_KIND_ID, "Energy results", 1.0 / 3.0),
        ],
    });
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![model_window_stack(model_window::WINDOW_KIND_ID, "Model", MODEL_VIEWPORT_SHARE), data_column] }) }
}

/// 🪟️ The 3d viewport's share of the view-mode row.
pub const MODEL_VIEWPORT_SHARE: f64 = 0.55;
/// 🪟️ The data column's share of the view-mode row — the remainder, spelled so the two always sum to one.
pub const MODEL_DATA_COLUMN_SHARE: f64 = 1.0 - MODEL_VIEWPORT_SHARE;
//#endregion 🔖️Definition
