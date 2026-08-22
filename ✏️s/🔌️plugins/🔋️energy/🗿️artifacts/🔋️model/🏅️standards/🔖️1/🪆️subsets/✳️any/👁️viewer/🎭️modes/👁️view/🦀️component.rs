//! 👁️ Energy model viewer — the `view` mode: the read-only twin of the editor's split layout
//! (structure tree left, zone table right), same two composed children, no edit affordances.

use crate::viewer::model::modes::view::windows::{structure, zones};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ENERGY_MODEL_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ENERGY_MODEL_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One column of the split layout: a stack holding a single window kind.
fn model_window_stack(window_kind_id: &str, title: &str) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size: Some(0.5),
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// 🪟️ Same split as the sibling mutation-capable surface's own layout — read-only twin, no quadrant
/// to allocate for edit affordances.
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![model_window_stack(structure::WINDOW_KIND_ID, "Structure"), model_window_stack(zones::WINDOW_KIND_ID, "Zones")] }) }
}
//#endregion 🔖️Definition
