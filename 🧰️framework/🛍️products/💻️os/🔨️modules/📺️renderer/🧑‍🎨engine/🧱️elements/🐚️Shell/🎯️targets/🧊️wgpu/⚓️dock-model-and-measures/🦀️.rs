//! ⚓️ Dock model, panel anchors and window measures.
//!
//! Panel anchors, the dock tree and skeleton, dock UI persistence helpers and the window-measures projection
//! with its `impl ShellState` publication half.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle

//#region 📏️WindowMeasures

/// 🪟️ The window instance a Measures overlay surface belongs to, `None` for every other surface.
pub(crate) fn window_measures_owner(surface_id: &str) -> Option<&str> {
    surface_id.strip_suffix(semio_framework::UiRefreshSection::Measures.body_key()).and_then(|owner| owner.strip_suffix('/')).filter(|owner| !owner.is_empty())
}

/// 🌿️ One labelled measure row with a distinct control child and retained disclosure state.
pub(super) fn measure_tree_item(label: ui_contract::Label, default_open: Option<bool>) -> ui_contract::Component {
    ui_contract::Component::TreeItem(ui_contract::TreeItemProps {
        label,
        description: None,
        icon: None,
        default_open,
        draggable: None,
        drag_data: None,
        dimmed: None,
        selected: None,
        window: None,
        granularity: None,
        inline_toolbar: None,
        detail: None,
        row_actions: Default::default(),
        target: None,
    })
}

pub(crate) fn measure_label(value: &str) -> ui_contract::Label {
    ui_contract::Label(UiText::clipped(value))
}

pub(crate) fn measure_container(role: ui_contract::ContainerRole, label: Option<&str>, default_open: Option<bool>) -> ui_contract::ContainerProps {
    ui_contract::ContainerProps { role, label: label.map(measure_label), description: None, required: None, error: None, default_open, drop_overlay: None }
}

pub(crate) fn measure_slider(value: f64, min: f64, max: f64, step: Option<f64>, unit: Option<&str>) -> ui_contract::Component {
    ui_contract::Component::Slider(ui_contract::SliderProps { value, min, max, step: step.unwrap_or(0.0), unit: unit.map(UiText::clipped), ..Default::default() })
}

pub(super) fn measure_activity(loading: Option<bool>, waiting: Option<bool>) -> ui_contract::Activity {
    match (loading, waiting) {
        (Some(true), _) => ui_contract::Activity::Loading,
        (_, Some(true)) => ui_contract::Activity::Waiting,
        _ => ui_contract::Activity::Idle,
    }
}

/// 🔗️ A measure's `on_change` as the record's one binding: the authored args, plus `extra` merged over
/// them. The gesture's own scalar is merged over these again by the retained router.
pub(crate) fn measure_binding(trigger: ui_contract::Trigger, action: &ActionDescriptor, extra: Option<(&str, DslValue)>) -> Result<ui_contract::ActionBinding, String> {
    let mut entries = match action.args.as_ref() {
        Some(semio_framework_value::DslValue::Object(entries)) => entries.clone(),
        _ => Vec::new(),
    };
    if let Some((key, value)) = extra {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }
    let args = if entries.is_empty() {
        None
    } else {
        Some(serde_json::from_value::<ui_contract::UiValue>(dsl_value_as_json(&semio_framework_value::DslValue::Object(entries))).map_err(|error| format!("measure action '{}' args exceed the retained contract: {error}", action.action))?)
    };
    let action_id = ui_contract::ActionId::try_v1(&action.controller_id, &action.action).ok_or_else(|| format!("measure action '{}' exceeds the retained contract", action.action))?;
    Ok(ui_contract::ActionBinding { trigger, action: action_id, args, capability: None })
}

/// 🎯️ A measured row's click as its contract [`ui_contract::RowTarget`]: the descriptor's controller and version, its argument
/// map, and its action as the target's activation verb.
pub(crate) fn measure_row_target(action: &ActionDescriptor) -> Result<ui_contract::RowTarget, String> {
    let binding = measure_binding(ui_contract::Trigger::Activate, action, None)?;
    let args = match binding.args {
        Some(ui_contract::UiValue::Map(args)) => Some(args),
        None => None,
        Some(_) => return Err(format!("measure row action '{}' args are not an argument map", action.action)),
    };
    Ok(ui_contract::RowTarget { scope: binding.action.scope, version: binding.action.version, args, activation: Some(binding.action.name) })
}

pub(crate) fn measure_bindings(trigger: ui_contract::Trigger, action: &ActionDescriptor, extra: Option<(&str, DslValue)>) -> Result<ui_contract::UiNodeBindings, String> {
    let mut bindings = ui_contract::UiNodeBindings::default();
    bindings.try_push(measure_binding(trigger, action, extra)?).map_err(|_| "measure binding admits one action".to_string())?;
    Ok(bindings)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📏️wgpu-window-measures/🦀️.rs"]
mod window_measures_tests;
//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle
