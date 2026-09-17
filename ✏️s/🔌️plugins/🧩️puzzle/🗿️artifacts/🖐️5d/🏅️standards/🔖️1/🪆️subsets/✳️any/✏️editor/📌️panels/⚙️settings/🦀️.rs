//! ⚙️ Puzzle 5d play app panel — the app-wide settings: the fill count, the brush suggestion offset,
//! the placement contact tolerance, the auto-connect proximity radius, the broad-phase chunk size and
//! the two grid scalars. Unlike the window-instance chrome in `🎭️modes/✏️edit/🪟️windows/*/☑️options/*`,
//! these are settings a user tunes once for the session rather than per pane.
//!
//! 🪟️ Every stepper still tags the window instance the panel resolves to (the render's own, else the
//! focused pane, else the first live pane) — and none when the host mounts no pane at all, since an
//! empty `windowId` is refused as an unknown window instance while the commands never need one.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{ui_label, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, Component, HasBase, HasChildren, NumberStepperProps, Trigger};
use semio_framework_plugin::{ui_node_list, ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, UiMapBuilder, UiText, UiValue};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "puzzle5d.panel.settings";
pub const BODY_KEY: &str = "puzzle.5d.play.settings";
const ROOT: &str = "puzzle5d-play-settings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PANEL_TAB_ID.into()), label: LocalizedLabel::native("Settings", "Einstellungen"), group: PanelGroup::Settings, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn window_args(window_id: Option<&str>) -> UiAssemblyResult<Option<UiValue>> {
    let Some(window_id) = window_id.filter(|id| !id.is_empty()) else { return Ok(None) };
    let text = UiText::try_from_str(window_id).ok_or_else(|| PluginAssemblyError::new("ui.action.text", "puzzle5d settings window id admission failed"))?;
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.action.map", "puzzle5d settings window map admission failed"))?;
    builder.push("windowId".to_owned(), UiValue::Text(text)).map_err(|_| PluginAssemblyError::new("ui.action.map.entry", "puzzle5d settings window map entry admission failed"))?;
    Ok(Some(UiValue::Map(builder.finish())))
}

/// 🪟️ The pane an app-level panel addresses: the render's own window, else the focused pane, else the
/// first live instance — `None` when the host mounts no pane at all.
pub fn panel_window_id(view_state: &semio_framework_plugin::ViewModel) -> Option<&str> {
    view_state
        .window_id
        .as_deref()
        .or(view_state.focused_window_id.as_deref())
        .or_else(|| view_state.window_instances.first().map(|window| window.id.as_str()))
        .filter(|id| !id.is_empty())
}

/// 🔢️ One labelled number stepper bound to `action {windowId}`; the host adds the typed value.
/// `uniform: true` — one session scalar, never a multi-selection aggregate (a `false` renders the
/// stepper as MIXED: a blank box whose +/− bump starts from `defaultValue`, not from the setting).
fn stepper_field(id: &str, label: &str, value: f64, step: f64, action: &str, window_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action(action, window_args(window_id)?)?;
    let mut control =
        BuiltNode::try_new(format!("{id}.control"), Component::NumberStepper(NumberStepperProps { value, step, uniform: true })).map_err(|_| PluginAssemblyError::new("ui.number-stepper", "number stepper admission failed"))?;
    control.bindings.try_push(ActionBinding { trigger: Trigger::Change, action, args, capability: None }).map_err(|_| PluginAssemblyError::new("ui.number-stepper", "number stepper binding admission failed"))?;
    ui::field(ui_label(label)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("ui.field", "puzzle5d settings field id admission failed"))?
        .try_child(control)
        .map_err(|_| PluginAssemblyError::new("ui.field", "puzzle5d settings field child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.field", "puzzle5d settings field admission failed"))
}

/// ⚙️ Renders the app settings, addressed at `window_id` when the host has a live pane.
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, window_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let runtime = &envelope.runtime;
    ui::section(ui_label(labels.settings.as_str())?)
        .try_id(ROOT)
        .map_err(|_| PluginAssemblyError::new("ui.section", "puzzle5d settings section id admission failed"))?
        .default_open(true)
        .try_children(ui_node_list([
            stepper_field(&format!("{ROOT}.fill-count"), &format!("{} · {}", labels.fill.as_str(), labels.count.as_str()), f64::from(runtime.fill_count), 1.0, "setFillCount", window_id),
            stepper_field(&format!("{ROOT}.suggestion-offset"), &format!("{} {}", labels.suggestion.as_str(), labels.offset.as_str()), runtime.suggestion_offset, 4.0, "setSuggestionOffset", window_id),
            stepper_field(&format!("{ROOT}.contact-tolerance"), labels.contact_tolerance.as_str(), runtime.contact_tolerance, 0.001, "setBrushPlacementContactTolerance", window_id),
            stepper_field(&format!("{ROOT}.proximity-radius"), labels.proximity_radius.as_str(), runtime.proximity_radius, 0.1, "setProximityRadius", window_id),
            stepper_field(&format!("{ROOT}.chunk-size"), labels.chunk_size.as_str(), runtime.chunk_size, 1.0, "setChunkSize", window_id),
            stepper_field(&format!("{ROOT}.grid-factor"), &format!("{} {}", labels.grid.as_str(), labels.grid_factor.as_str()), runtime.grid_factor, 0.25, "setGridFactor", window_id),
            stepper_field(&format!("{ROOT}.grid-spacing"), &format!("{} · {}", labels.grid.as_str(), labels.spacing.as_str()), runtime.grid_spacing, 0.5, "setGridSpacing", window_id),
        ])?)
        .map_err(|_| PluginAssemblyError::new("ui.section", "puzzle5d settings children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.section", "puzzle5d settings section admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
