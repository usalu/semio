//! ✏️ Energy model editor — the `edit` mode: a two-pane layout (structure tree left, zone table
//! right) over the artifact's own composed `structure`/`zones` children. Nothing pane-specific lives
//! here; each window binds its own definition/render in its own file.

use crate::editor::model::modes::edit::windows::{structure, zones};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ENERGY_MODEL_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ENERGY_MODEL_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
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

/// 🪟️ Structure tree left, zone table right — the artifact's two composed children rendered side by
/// side so an edit to either is visible without switching windows.
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![model_window_stack(structure::WINDOW_KIND_ID, "Structure"), model_window_stack(zones::WINDOW_KIND_ID, "Zones")] }) }
}
//#endregion 🔖️Definition
