//! ✏️ Energy model editor — the `edit` mode: the 3d model viewport on the left with the structure
//! tree, the zone table and the simulation window stacked in a right-hand column, and the energy
//! simulation tool whose run the framework drives. Nothing pane-specific lives here; each window and
//! tool binds its own definition/render in its own file.

use crate::editor::model::modes::edit::tools;
use crate::editor::model::modes::edit::windows::{model as model_window, simulation, structure, zones};
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ENERGY_MODEL_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ENERGY_MODEL_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: vec![tools::simulation::TOOL_ID.into()], layout_id: None, commands: Vec::new() }
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

/// 🪟️ The 3d model viewport left (≈55 % of the row), the structure tree, the zone table and the
/// simulation window stacked in a right-hand column — geometry and data side by side, so an edit to
/// either is visible without switching windows. Every window kind named here is declared by
/// `create_energy_model_editor`; `try_build_definition` refuses a layout that names an undeclared one.
pub fn layout() -> WindowLayout {
    let data_column = WindowLayoutChild::Axis(WindowLayoutAxisNode {
        kind: "column".into(),
        size: Some(MODEL_DATA_COLUMN_SHARE),
        children: vec![
            model_window_stack(structure::WINDOW_KIND_ID, "Structure", 1.0 / 3.0),
            model_window_stack(zones::WINDOW_KIND_ID, "Zones", 1.0 / 3.0),
            model_window_stack(simulation::WINDOW_KIND_ID, "Energy simulation", 1.0 / 3.0),
        ],
    });
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![model_window_stack(model_window::WINDOW_KIND_ID, "Model", MODEL_VIEWPORT_SHARE), data_column] }) }
}

/// 🪟️ The 3d viewport's share of the edit-mode row.
pub const MODEL_VIEWPORT_SHARE: f64 = 0.55;
/// 🪟️ The data column's share of the edit-mode row — the remainder, spelled so the two always sum to one.
pub const MODEL_DATA_COLUMN_SHARE: f64 = 1.0 - MODEL_VIEWPORT_SHARE;
//#endregion 🔖️Definition
