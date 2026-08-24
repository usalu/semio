//! ⚙️ Puzzle 3d play app panel — the app-wide settings: the brush placement overlap budget, the
//! relocate proximity radius, the viewport chunk size and the grid spacing. Unlike the
//! window-instance chrome in `🎭️modes/✏️edit/🎚️options/*`, these are settings a user tunes once for
//! the whole session rather than per pane. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM:
//! the default selection merge mode dropdown is gone — merge is a per-gesture `interactionSelect` arg
//! (modifier-driven) in the framework's model now, with no equivalent "default" setting.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, Component, HasBase, HasChildren, NumberStepperProps, Trigger};
use semio_framework_plugin::{ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "puzzle3d.panel.settings";
pub const BODY_KEY: &str = "puzzle.3d.play.settings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PANEL_TAB_ID.into()), label: LocalizedLabel::native("Settings", "Einstellungen"), group: PanelGroup::Settings, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn stepper_field(id: &str, label: &str, value: f64, step: f64, action: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (action, args) = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action(action, None)?;
    let control = BuiltNode {
        key: format!("{id}.control"),
        component: Component::NumberStepper(NumberStepperProps { value, step, uniform: false }),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        accessibility: Default::default(),
        bindings: vec![ActionBinding { trigger: Trigger::Change, action, args, capability: None }],
        menu: None,
        children: Vec::new(),
    };
    ui::field(label).id(id).child(control).build()
}

pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let runtime = &envelope.runtime;
    ui::section(labels.settings.as_str())
        .id("puzzle3d-play-settings")
        .default_open(true)
        .children(vec![
            stepper_field("puzzle3d-play-settings.overlap-budget", labels.overlap_budget.as_str(), runtime.overlap_budget, 0.05, "setBrushPlacementOverlapBudget"),
            stepper_field("puzzle3d-play-settings.proximity-radius", labels.proximity_radius.as_str(), runtime.proximity_radius, 0.1, "setProximityRadius"),
            stepper_field("puzzle3d-play-settings.chunk-size", labels.chunk_size.as_str(), runtime.chunk_size, 1.0, "setChunkSize"),
            stepper_field("puzzle3d-play-settings.grid-spacing", labels.spacing.as_str(), runtime.grid_spacing, 0.5, "setGridSpacing"),
        ])
        .build()
}
//#endregion 🔖️Render
