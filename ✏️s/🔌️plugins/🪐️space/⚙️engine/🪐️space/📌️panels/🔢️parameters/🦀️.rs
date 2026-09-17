//! 🔢️ S Studio app — workflow parameters panel: add/edit/remove the workflow's own parameter set.
//!
//! 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (26/08/20) port: rebuilt on the `semio_framework_ui_
//! contract` builder DSL (`BuiltNode`), replacing the old `ui_wgpu::wgpu::UiNode` struct literals —
//! mirrors the pattern already landed on `🗿️artifacts/🏠️home/…/🏠️main/🦀️.rs` and
//! `🗿️artifacts/🪐️space/…/🏠️main/🦀️.rs` (`fixed_text`/`fixed_label` admission helpers, `try_id`/
//! `try_children`/`try_build` fallible chains, U1-sync `render`). One functional simplification, flagged
//! where it happens: the old `UiNumberStepperNode` carried TWO triggers (`on_absolute` for typed entry,
//! `on_delta` for a stepper +/- affordance) with IDENTICAL dispatched args — the new contract has no
//! stepper component, only `Component::Input(InputKind::Number)` (one `Trigger::Change`) and
//! `Component::Slider` (a continuous range, wrong shape for a typed numeric field), so numeric values
//! render as a plain number input bound to `Trigger::Change` only; no dispatch target or arg shape
//! changed, only the +/- click affordance is gone (a plain number input still supports the browser's
//! native up/down spinner, which fires the same `change` event).

use crate::engine::space::engine::parameter_entity_id;
use crate::engine::space::terminology::SStudioLabels;
use crate::engine::space::{ui_value_map, ui_value_text, S_PLAY_PARAMETERS_TAB_ID};
use semio_framework_os::{WorkflowParameter, WorkflowSnapshot};
use semio_framework_plugin::{
    tree_window_item, ui_node_list, ActionId, BuiltNode, Buildable, HasBase, HasChildren, IconName, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use semio_framework_ui_contract::{InputKind, Label, Trigger};

//#region 🔖️Manifest
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(S_PLAY_PARAMETERS_TAB_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, "Parameter"),
        group: PanelGroup::Workbench,
        body_key: Some(crate::engine::space::S_PLAY_PARAMETERS_BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
fn fixed_text(value: &str, code: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new(code, "fixed parameters text admission failed"))
}

fn fixed_label(value: &str, code: &'static str) -> UiAssemblyResult<Label> {
    Label::try_from(value).map_err(|_| PluginAssemblyError::new(code, "fixed parameters label admission failed"))
}

/// 🎬️ Binds `trigger` to the `(ActionId, Option<UiValue>)` pair `crate::engine::space::s_play_action`
/// returns — generic over any `HasBase` builder, since `try_on`/`try_on_with` are its default methods.
fn bind_action<B: HasBase>(builder: B, trigger: Trigger, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<B> {
    match action.1 {
        Some(args) => builder.try_on_with(trigger, action.0, args),
        None => builder.try_on(trigger, action.0),
    }
    .map_err(|_| PluginAssemblyError::new("ui.parameters.action", "action binding admission failed"))
}

fn patch_parameter_action(entries: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    crate::engine::space::s_play_action("patchParameter", Some(ui_value_map(entries)?))
}

fn parameter_value_control(parameter: &WorkflowParameter, labels: &SStudioLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    match parameter {
        WorkflowParameter::Numeric { id, value, step, .. } => {
            let action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("value")?)])?;
            let builder = semio_framework_ui_contract::input(InputKind::Number)
                .value(fixed_text(&value.to_string(), "ui.parameters.value-text")?)
                .step(step.unwrap_or(1.0))
                .try_id(format!("s-play-parameters.{id}.value"))
                .map_err(|_| PluginAssemblyError::new("ui.parameters.value-id", "value input id admission failed"))?;
            bind_action(builder, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.value", "value input admission failed"))
        }
        WorkflowParameter::Categorical { id, value, options, .. } => {
            let action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("value")?)])?;
            let mut builder = semio_framework_ui_contract::select(fixed_text(value, "ui.parameters.select-value")?)
                .try_id(format!("s-play-parameters.{id}.value"))
                .map_err(|_| PluginAssemblyError::new("ui.parameters.value-id", "value select id admission failed"))?;
            for option in options {
                builder = builder
                    .try_item(fixed_text(option, "ui.parameters.select-item-value")?, fixed_label(option, "ui.parameters.select-item-label")?)
                    .map_err(|_| PluginAssemblyError::new("ui.parameters.select-item", "select item admission failed"))?;
            }
            bind_action(builder, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.value", "value select admission failed"))
        }
        WorkflowParameter::Toggle { id, value, .. } => {
            let action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("value")?)])?;
            let builder = semio_framework_ui_contract::toggle(*value)
                .icon(fixed_text(IconName::ToggleLeft.as_str(), "ui.parameters.toggle-icon")?)
                .text(fixed_label(if *value { labels.toggle_on.as_str() } else { labels.toggle_off.as_str() }, "ui.parameters.toggle-text")?)
                .try_id(format!("s-play-parameters.{id}.value"))
                .map_err(|_| PluginAssemblyError::new("ui.parameters.value-id", "value toggle id admission failed"))?;
            bind_action(builder, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.value", "value toggle admission failed"))
        }
        WorkflowParameter::Text { id, value, .. } => {
            let action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("value")?)])?;
            let builder = semio_framework_ui_contract::input(InputKind::Text)
                .value(fixed_text(value, "ui.parameters.value-text")?)
                .try_id(format!("s-play-parameters.{id}.value"))
                .map_err(|_| PluginAssemblyError::new("ui.parameters.value-id", "value input id admission failed"))?;
            bind_action(builder, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.value", "value input admission failed"))
        }
    }
}

/// 🎛️ One tree row whose single non-`treeItem` child is its inline control — the row shape the host's
/// `treeItemToTreeData` mounts through `TreeDataItem.control` (the History panel's own idiom).
fn control_row(id: &str, label: Label, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    semio_framework_ui_contract::tree_item(label)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("ui.parameters.row-id", "parameter row id admission failed"))?
        .try_child(control)
        .map_err(|_| PluginAssemblyError::new("ui.parameters.row-child", "parameter row control admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.parameters.row", "parameter row admission failed"))
}

/// 🔢️ One editable row under a parameter — the entry slice a [`tree_window_item`] windows.
enum ParameterField<'a> {
    Name,
    Value,
    Constraint { field: &'static str, label: &'a str, value: f64, step: f64 },
    Option(&'a str),
    AddOption,
    Remove,
}

fn parameter_fields<'a>(parameter: &'a WorkflowParameter, labels: &'a SStudioLabels) -> Vec<ParameterField<'a>> {
    let mut fields = vec![ParameterField::Name, ParameterField::Value];
    match parameter {
        WorkflowParameter::Numeric { min, max, step, .. } => {
            fields.push(ParameterField::Constraint { field: "min", label: labels.min.as_str(), value: min.unwrap_or(0.0), step: 1.0 });
            fields.push(ParameterField::Constraint { field: "max", label: labels.max.as_str(), value: max.unwrap_or(0.0), step: 1.0 });
            fields.push(ParameterField::Constraint { field: "step", label: labels.step.as_str(), value: step.unwrap_or(0.0), step: 0.1 });
        }
        WorkflowParameter::Categorical { options, .. } => {
            fields.extend(options.iter().map(|option| ParameterField::Option(option.as_str())));
            fields.push(ParameterField::AddOption);
        }
        _ => {}
    }
    fields.push(ParameterField::Remove);
    fields
}

fn numeric_constraint_row(id: &str, label: &str, field: &'static str, value: f64, step: f64) -> UiAssemblyResult<BuiltNode> {
    let action = patch_parameter_action([("parameterId", ui_value_text(id)?), ("field", ui_value_text(field)?)])?;
    let control = semio_framework_ui_contract::input(InputKind::Number)
        .value(fixed_text(&value.to_string(), "ui.parameters.constraint-text")?)
        .step(step)
        .try_id(format!("s-play-parameters.{id}.{field}.stepper"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-id", "constraint input id admission failed"))?;
    let control = bind_action(control, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.constraint", "constraint input admission failed"))?;
    control_row(&format!("s-play-parameters.{id}.{field}"), fixed_label(label, "ui.parameters.constraint-label")?, control)
}

fn option_remove_row(id: &str, option: &str, labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    let action = patch_parameter_action([("parameterId", ui_value_text(id)?), ("field", ui_value_text("removeOption")?), ("value", ui_value_text(option)?)])?;
    let remove_button = semio_framework_ui_contract::button(fixed_label(labels.remove.as_str(), "ui.parameters.option-remove-label")?)
        .icon(fixed_text(IconName::Trash2.as_str(), "ui.parameters.option-remove-icon")?)
        .try_id(format!("s-play-parameters.{id}.option.{option}.remove"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.option-remove-id", "option remove button id admission failed"))?;
    let remove_button = bind_action(remove_button, Trigger::Activate, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.option-remove", "option remove button admission failed"))?;
    control_row(&format!("s-play-parameters.{id}.option.{option}"), fixed_label(option, "ui.parameters.option-label")?, remove_button)
}

fn add_option_row(id: &str, labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    let add_action = patch_parameter_action([("parameterId", ui_value_text(id)?), ("field", ui_value_text("addOption")?)])?;
    let add_input = semio_framework_ui_contract::input(InputKind::Text)
        .placeholder(fixed_label(labels.new_option_placeholder.as_str(), "ui.parameters.add-option-placeholder")?)
        .try_id(format!("s-play-parameters.{id}.add-option.input"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-id", "add-option input id admission failed"))?;
    let add_input = bind_action(add_input, Trigger::Change, add_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.add-option", "add-option input admission failed"))?;
    control_row(&format!("s-play-parameters.{id}.add-option"), fixed_label(labels.add_option.as_str(), "ui.parameters.add-option-label")?, add_input)
}

fn remove_parameter_row(id: &str, labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    let remove_action = crate::engine::space::s_play_action("removeParameter", Some(ui_value_map([("parameterId", ui_value_text(id)?)])?))?;
    let remove_button = semio_framework_ui_contract::button(fixed_label(labels.remove.as_str(), "ui.parameters.remove-label")?)
        .icon(fixed_text(IconName::Trash2.as_str(), "ui.parameters.remove-icon")?)
        .try_id(format!("s-play-parameters.{id}.remove"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.remove-id", "remove button id admission failed"))?;
    let remove_button = bind_action(remove_button, Trigger::Activate, remove_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.remove", "remove button admission failed"))?;
    control_row(&format!("s-play-parameters.{id}.remove-row"), fixed_label(labels.remove.as_str(), "ui.parameters.remove-label")?, remove_button)
}

fn parameter_name_row(parameter: &WorkflowParameter, id: &str, labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    let name = match parameter {
        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.as_str(),
    };
    let name_action = patch_parameter_action([("parameterId", ui_value_text(id)?), ("field", ui_value_text("name")?)])?;
    let name_input = semio_framework_ui_contract::input(InputKind::Text)
        .value(fixed_text(name, "ui.parameters.name-text")?)
        .try_id(format!("s-play-parameters.{id}.name.input"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.name-id", "name input id admission failed"))?;
    let name_input = bind_action(name_input, Trigger::Change, name_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.name", "name input admission failed"))?;
    control_row(&format!("s-play-parameters.{id}.name"), fixed_label(labels.name.as_str(), "ui.parameters.name-label")?, name_input)
}

fn parameter_field_row(parameter: &WorkflowParameter, id: &str, field: &ParameterField<'_>, labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    match field {
        ParameterField::Name => parameter_name_row(parameter, id, labels),
        ParameterField::Value => control_row(&format!("s-play-parameters.{id}.value-field"), fixed_label(labels.value.as_str(), "ui.parameters.value-label")?, parameter_value_control(parameter, labels)?),
        ParameterField::Constraint { field, label, value, step } => numeric_constraint_row(id, label, field, *value, *step),
        ParameterField::Option(option) => option_remove_row(id, option, labels),
        ParameterField::AddOption => add_option_row(id, labels),
        ParameterField::Remove => remove_parameter_row(id, labels),
    }
}

/// 🌉️ `parameter_entity_id` is `⚙️engine`'s async fn (kernel-shape convention); this whole render
/// tree stays U1-sync (matches the contract builder's own sync-only ruling, `📌️important.md`), so it
/// is bridged the same way `🎮️commands/🔍️open-instance/🦀️.rs`'s `handle` bridges its own callee.
fn parameter_id(parameter: &WorkflowParameter) -> String {
    crate::engine::space::engine::resolve_future(parameter_entity_id(parameter)).to_string()
}

/// 🪟️ The node key the host addresses this panel's one open-ended container by.
pub const S_PLAY_PARAMETERS_LIST_KEY: &str = "s-play-parameters.list";

/// 🪟️ The parameter list is a **windowed tree**: `PanelTreeBuilder` root, one `window_section` over
/// `projection.parameters` keyed [`S_PLAY_PARAMETERS_LIST_KEY`], and every parameter is a
/// [`tree_window_item`] over its own editable field rows. Each field row is a `treeItem` carrying its
/// control as its single non-`treeItem` child — the shape the host's `treeItemToTreeData` mounts
/// through `TreeDataItem.control` (`collectTreeItemControls`, the History panel's `.run` idiom). Both
/// levels therefore stamp `TreeWindow::total`, so the host learns the full extent and streams the rest
/// on scroll; the old `column` of `Component::Container(Section)` nodes (which had no `window` carrier
/// and hard-failed past the 32nd child) is gone.
pub fn render(projection: &WorkflowSnapshot, labels: &SStudioLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let add_action = crate::engine::space::s_play_action("addParameter", Some(ui_value_map([("type", ui_value_text("numeric")?)])?))?;
    let add_button = semio_framework_ui_contract::button(fixed_label(labels.add_parameter.as_str(), "ui.parameters.add-label")?)
        .icon(fixed_text(IconName::Plus.as_str(), "ui.parameters.add-icon")?)
        .try_id("s-play-parameters.add")
        .map_err(|_| PluginAssemblyError::new("ui.parameters.add-id", "add button id admission failed"))?;
    let add_button = bind_action(add_button, Trigger::Activate, add_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.add", "add button admission failed"))?;
    let header_rows = ui_node_list([control_row("s-play-parameters.add-row", fixed_label(labels.add_parameter.as_str(), "ui.parameters.add-label")?, add_button)])?;
    let count_label = fixed_label(&format!("{} {}", projection.parameters.len(), labels.parameter_count_suffix.as_str()), "ui.parameters.count")?;

    PanelTreeBuilder::new("s-play-parameters")?
        .section("s-play-parameters.header", Some(fixed_label(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, "ui.parameters.header-label")?), true, header_rows)?
        .window_section(windows, S_PLAY_PARAMETERS_LIST_KEY, Some(count_label), true, &projection.parameters, |parameter| {
            let id = parameter_id(parameter);
            let name = match parameter {
                WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.as_str(),
            };
            let node_key = format!("s-play-parameters.{id}");
            let item = semio_framework_ui_contract::tree_item(fixed_label(name, "ui.parameters.parameter-label")?)
                .try_id(&node_key)
                .map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-id", "parameter row id admission failed"))?;
            let fields = parameter_fields(parameter, labels);
            tree_window_item(windows, item, &node_key, true, &fields, |field| parameter_field_row(parameter, &id, field, labels))
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
