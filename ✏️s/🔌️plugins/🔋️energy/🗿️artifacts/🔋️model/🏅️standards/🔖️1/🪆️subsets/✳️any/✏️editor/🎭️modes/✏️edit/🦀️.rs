//! ✏️ Energy model editor — the `edit` mode: the 3d model viewport on the left with the structure
//! tree, the zone table and the simulation window stacked in a right-hand column, and the energy
//! simulation tool whose run the framework drives. Nothing pane-specific lives here; each window and
//! tool binds its own definition/render in its own file.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_artifact_energy_model_editor_commands as editor_commands;
extern crate semio_s_artifact_energy_model_editor_config as editor_config;
extern crate semio_s_artifact_energy_model_editor_interaction as editor_interaction;
extern crate semio_s_artifact_energy_model_editor_results as editor_results;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_scene as scene;
extern crate semio_s_artifact_energy_model_session as session;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_examples as examples;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_geometry as geometry;




use crate::tools;
use crate::windows::{model as model_window, simulation, structure, zones};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutAxisNode;
use semio_framework_plugin::WindowLayoutChild;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;

pub const ENERGY_MODEL_EDIT_MODE_ID: &str = "edit";

//#region 🪟️SharedWindowActions
/// 🪟️ The verbs a WINDOW renders but any window may have to dispatch — the simulation window's three
/// config/document verbs (its settings tree and the inspector's own result-field select both reach
/// `set-result-field`), plus the two creation verbs the app keybindings `mod+shift+n`/`mod+shift+s`
/// fire. They are declared app-level for exactly the reason `semio_s_artifact_energy_model_editor_commands::inspector_action_definitions` is: an
/// action a window kind lists in its own `actions` is copied onto NO other window, so the chord is
/// refused with `window kind … does not own action …` whenever another window holds focus.
pub fn window_shared_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    vec![simulation::settings_action(), simulation::result_field_action(), simulation::run_period_action(), structure::create_surface_action(), zones::create_zone_action(), zones::rename_zone_action()]
}
//#endregion 🪟️SharedWindowActions

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

//#region 🛠️Tools
pub mod tools {
    #[path = "../🛠️tools/⚡️simulation/🦀️.rs"]
    pub mod simulation;
}
//#endregion 🛠️Tools

//#region 🪟️Windows
pub mod windows {
    #[path = "../🪟️windows/🧊️model/🦀️.rs"]
    pub mod model;
    #[path = "../🪟️windows/⚡️simulation/🦀️.rs"]
    pub mod simulation;
    #[path = "../🪟️windows/🌳️structure/🦀️.rs"]
    pub mod structure;
    #[path = "../🪟️windows/📊️zones/🦀️.rs"]
    pub mod zones;
}
//#endregion 🪟️Windows
