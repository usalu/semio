//! ⚙️ Puzzle 3d play app panel — the app-wide settings: the brush placement overlap budget, the
//! relocate proximity radius, the viewport chunk size and the grid spacing. Unlike the
//! window-instance chrome in `🎭️modes/✏️edit/☑️options/*`, these are settings a user tunes once for
//! the whole session rather than per pane. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM:
//! the default selection merge mode dropdown is gone — merge is a per-gesture `interactionSelect` arg
//! (modifier-driven) in the framework's model now, with no equivalent "default" setting.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{ui_label, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
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
fn stepper_window_args(window_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let text = semio_framework_plugin::UiText::try_from_str(window_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.text", "settings window id admission failed"))?;
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.map", "settings window map admission failed"))?;
    builder.push("windowId".to_owned(), semio_framework_plugin::UiValue::Text(text)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.map.entry", "settings window map entry admission failed"))?;
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

fn stepper_field(id: &str, label: &str, value: f64, step: f64, action: &str, window_id: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (action, args) = ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action(action, Some(stepper_window_args(window_id)?))?;
    // 🔢️ `uniform: true` — one window's own scalar setting, never a multi-selection aggregate.
    // `uniform: false` renders the stepper as MIXED: a blank box with a placeholder instead of the
    // value, and its internal base for a +/− bump is `defaultValue` (0), not the setting. Both halves
    // were browser-visible (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12).
    let mut control = BuiltNode::try_new(format!("{id}.control"), Component::NumberStepper(NumberStepperProps { value, step, uniform: true }))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.number-stepper", "number stepper admission failed"))?;
    control.bindings.try_push(ActionBinding { trigger: Trigger::Change, action, args, capability: None }).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.number-stepper", "number stepper binding admission failed"))?;
    ui::field(ui_label(label)?)
        .try_id(id)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.field", "settings field id admission failed"))?
        .try_child(control)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.field", "settings field child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.field", "settings field admission failed"))
}

/// ⚙️ Renders the app settings for ONE window instance. Every stepper tags that instance as `windowId`
/// so a split layout cannot silently retune the pane the user is not looking at.
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, window_id: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let runtime = &envelope.runtime;
    ui::section(ui_label(if window_id.is_empty() { labels.settings.as_str().to_string() } else { format!("{} — {window_id}", labels.settings.as_str()) })?)
        .try_id("puzzle3d-play-settings")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.section", "settings section id admission failed"))?
        .default_open(true)
        .try_children(crate::editor::puzzle3d::ui_node_list([
            stepper_field("puzzle3d-play-settings.overlap-budget", labels.overlap_budget.as_str(), runtime.overlap_budget, 0.05, "setBrushPlacementOverlapBudget", window_id),
            stepper_field("puzzle3d-play-settings.proximity-radius", labels.proximity_radius.as_str(), runtime.proximity_radius, 0.1, "setProximityRadius", window_id),
            stepper_field("puzzle3d-play-settings.chunk-size", labels.chunk_size.as_str(), runtime.chunk_size, 1.0, "setChunkSize", window_id),
            stepper_field("puzzle3d-play-settings.grid-spacing", labels.spacing.as_str(), runtime.grid_spacing, 0.5, "setGridSpacing", window_id),
        ])?)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.section", "settings children admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.section", "settings section admission failed"))
}
//#endregion 🔖️Render
