//! ⚙️ Puzzle 2d play app panel — the app-wide settings: the fill count, the brush suggestion offset,
//! the grid factor and the grid snap. Unlike the window-instance chrome in `🎭️modes/✏️edit/☑️options/*`,
//! these are settings a user tunes once for the session rather than per pane — every stepper tags
//! the window instance it was rendered for, so a triptych cannot silently retune another pane.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{ui_label, ui_node_list, Puzzle2dScene, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, Component, HasBase, HasChildren, NumberStepperProps, Trigger};
use semio_framework_plugin::{ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, UiMapBuilder, UiText, UiValue};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "puzzle2d.panel.settings";
pub const PUZZLE2D_PLAY_BODY_SETTINGS: &str = "puzzle2d.play.settings";
const ROOT: &str = "puzzle2d-play-settings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PANEL_TAB_ID.into()), label: LocalizedLabel::native("Settings", "Einstellungen"), group: PanelGroup::Settings, body_key: Some(PUZZLE2D_PLAY_BODY_SETTINGS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn window_args(window_id: &str) -> UiAssemblyResult<UiValue> {
    let text = UiText::try_from_str(window_id).ok_or_else(|| PluginAssemblyError::new("ui.action.text", "settings window id admission failed"))?;
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.action.map", "settings window map admission failed"))?;
    builder.push("windowId".to_owned(), UiValue::Text(text)).map_err(|_| PluginAssemblyError::new("ui.action.map.entry", "settings window map entry admission failed"))?;
    Ok(UiValue::Map(builder.finish()))
}

/// 🔢️ One labelled number stepper bound to `action {windowId, value}`. `uniform: true` — one window's own
/// scalar, never a multi-selection aggregate (a `false` renders as MIXED with a blank box).
fn stepper_field(id: &str, label: &str, value: f64, step: f64, action: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action(action, Some(window_args(window_id)?))?;
    let mut control = BuiltNode::try_new(format!("{id}.control"), Component::NumberStepper(NumberStepperProps { value, step, uniform: true })).map_err(|_| PluginAssemblyError::new("ui.number-stepper", "number stepper admission failed"))?;
    control.bindings.try_push(ActionBinding { trigger: Trigger::Change, action, args, capability: None }).map_err(|_| PluginAssemblyError::new("ui.number-stepper", "number stepper binding admission failed"))?;
    ui::field(ui_label(label)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("ui.field", "settings field id admission failed"))?
        .try_child(control)
        .map_err(|_| PluginAssemblyError::new("ui.field", "settings field child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.field", "settings field admission failed"))
}

/// ⚙️ Renders the app settings for ONE window instance.
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let runtime = &envelope.runtime;
    ui::section(ui_label(labels.settings.as_str())?)
        .try_id(ROOT)
        .map_err(|_| PluginAssemblyError::new("ui.section", "settings section id admission failed"))?
        .default_open(true)
        .try_children(ui_node_list([
            stepper_field(&format!("{ROOT}.fill-count"), &format!("{} · {}", labels.fill.as_str(), labels.count.as_str()), f64::from(runtime.fill_count), 1.0, "setFillCount", window_id),
            stepper_field(&format!("{ROOT}.suggestion-offset"), &format!("{} {}", labels.suggestion.as_str(), labels.offset.as_str()), runtime.suggestion_offset, 4.0, "setSuggestionOffset", window_id),
            stepper_field(&format!("{ROOT}.grid-factor"), &format!("{} {}", labels.grid.as_str(), labels.grid_factor.as_str()), runtime.grid_factor, 0.25, "setGridFactor", window_id),
        ])?)
        .map_err(|_| PluginAssemblyError::new("ui.section", "settings children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.section", "settings section admission failed"))
}
//#endregion 🔖️Render
