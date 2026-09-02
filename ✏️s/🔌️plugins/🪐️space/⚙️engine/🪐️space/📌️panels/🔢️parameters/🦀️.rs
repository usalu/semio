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
    ActionId, Buildable, HasBase, HasChildren, IconName, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
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

fn numeric_constraint_field(id: &str, code: &'static str, label: &str, field: &'static str, value: f64, step: f64) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action = patch_parameter_action([("parameterId", ui_value_text(id)?), ("field", ui_value_text(field)?)])?;
    let control = semio_framework_ui_contract::input(InputKind::Number)
        .value(fixed_text(&value.to_string(), "ui.parameters.constraint-text")?)
        .step(step)
        .try_id(format!("s-play-parameters.{id}.{field}.stepper"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-id", "constraint input id admission failed"))?;
    let control = bind_action(control, Trigger::Change, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.constraint", "constraint input admission failed"))?;
    let mut children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
    children.try_push(control).map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-children", "constraint field child admission failed"))?;
    semio_framework_ui_contract::field(fixed_label(label, code)?)
        .try_id(format!("s-play-parameters.{id}.{field}"))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-field-id", "constraint field id admission failed"))?
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-field-children", "constraint field child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.parameters.constraint-field", "constraint field admission failed"))
}

fn parameter_constraint_fields(parameter: &WorkflowParameter, labels: &SStudioLabels) -> UiAssemblyResult<Vec<semio_framework_plugin::BuiltNode>> {
    match parameter {
        WorkflowParameter::Numeric { id, min, max, step, .. } => Ok(vec![
            numeric_constraint_field(id, "ui.parameters.min-label", labels.min.as_str(), "min", min.unwrap_or(0.0), 1.0)?,
            numeric_constraint_field(id, "ui.parameters.max-label", labels.max.as_str(), "max", max.unwrap_or(0.0), 1.0)?,
            numeric_constraint_field(id, "ui.parameters.step-label", labels.step.as_str(), "step", step.unwrap_or(0.0), 0.1)?,
        ]),
        WorkflowParameter::Categorical { id, options, .. } => {
            let mut fields = Vec::new();
            for option in options {
                let action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("removeOption")?), ("value", ui_value_text(option.as_str())?)])?;
                let remove_button = semio_framework_ui_contract::button(fixed_label(labels.remove.as_str(), "ui.parameters.option-remove-label")?)
                    .icon(fixed_text(IconName::Trash2.as_str(), "ui.parameters.option-remove-icon")?)
                    .try_id(format!("s-play-parameters.{id}.option.{option}.remove"))
                    .map_err(|_| PluginAssemblyError::new("ui.parameters.option-remove-id", "option remove button id admission failed"))?;
                let remove_button = bind_action(remove_button, Trigger::Activate, action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.option-remove", "option remove button admission failed"))?;
                let mut children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
                children.try_push(remove_button).map_err(|_| PluginAssemblyError::new("ui.parameters.option-children", "option field child admission failed"))?;
                fields.push(
                    semio_framework_ui_contract::field(fixed_label(option, "ui.parameters.option-label")?)
                        .try_id(format!("s-play-parameters.{id}.option.{option}"))
                        .map_err(|_| PluginAssemblyError::new("ui.parameters.option-field-id", "option field id admission failed"))?
                        .try_children(children)
                        .map_err(|_| PluginAssemblyError::new("ui.parameters.option-field-children", "option field child admission failed"))?
                        .try_build()
                        .map_err(|_| PluginAssemblyError::new("ui.parameters.option-field", "option field admission failed"))?,
                );
            }
            let add_action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("addOption")?)])?;
            let add_input = semio_framework_ui_contract::input(InputKind::Text)
                .placeholder(fixed_label(labels.new_option_placeholder.as_str(), "ui.parameters.add-option-placeholder")?)
                .try_id(format!("s-play-parameters.{id}.add-option.input"))
                .map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-id", "add-option input id admission failed"))?;
            let add_input = bind_action(add_input, Trigger::Change, add_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.add-option", "add-option input admission failed"))?;
            let mut add_children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
            add_children.try_push(add_input).map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-children", "add-option field child admission failed"))?;
            fields.push(
                semio_framework_ui_contract::field(fixed_label(labels.add_option.as_str(), "ui.parameters.add-option-label")?)
                    .try_id(format!("s-play-parameters.{id}.add-option"))
                    .map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-field-id", "add-option field id admission failed"))?
                    .try_children(add_children)
                    .map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-field-children", "add-option field child admission failed"))?
                    .try_build()
                    .map_err(|_| PluginAssemblyError::new("ui.parameters.add-option-field", "add-option field admission failed"))?,
            );
            Ok(fields)
        }
        _ => Ok(Vec::new()),
    }
}

/// 🌉️ `parameter_entity_id` is `⚙️engine`'s async fn (kernel-shape convention); this whole render
/// tree stays U1-sync (matches the contract builder's own sync-only ruling, `📌️important.md`), so it
/// is bridged the same way `🎮️commands/🔍️open-instance/🦀️.rs`'s `handle` bridges its own callee.
fn parameter_id(parameter: &WorkflowParameter) -> String {
    crate::engine::space::engine::resolve_future(parameter_entity_id(parameter)).to_string()
}

pub fn render(projection: &WorkflowSnapshot, labels: &SStudioLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut sections = UiFixedList::<semio_framework_plugin::BuiltNode>::default();

    let add_action = crate::engine::space::s_play_action("addParameter", Some(ui_value_map([("type", ui_value_text("numeric")?)])?))?;
    let add_button = semio_framework_ui_contract::button(fixed_label(labels.add_parameter.as_str(), "ui.parameters.add-label")?)
        .icon(fixed_text(IconName::Plus.as_str(), "ui.parameters.add-icon")?)
        .try_id("s-play-parameters.add")
        .map_err(|_| PluginAssemblyError::new("ui.parameters.add-id", "add button id admission failed"))?;
    let add_button = bind_action(add_button, Trigger::Activate, add_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.add", "add button admission failed"))?;
    let count_text = semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(format!("{} {}", projection.parameters.len(), labels.parameter_count_suffix.as_str())))
        .map_err(|_| PluginAssemblyError::new("ui.parameters.count", "parameter count text admission failed"))?;
    let mut header_children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
    header_children.try_push(add_button).map_err(|_| PluginAssemblyError::new("ui.parameters.header-children", "header child admission failed"))?;
    header_children.try_push(count_text).map_err(|_| PluginAssemblyError::new("ui.parameters.header-children", "header child admission failed"))?;
    let header = semio_framework_ui_contract::section(fixed_label(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, "ui.parameters.header-label")?)
        .default_open(true)
        .try_id("s-play-parameters.header")
        .map_err(|_| PluginAssemblyError::new("ui.parameters.header-id", "header section id admission failed"))?
        .try_children(header_children)
        .map_err(|_| PluginAssemblyError::new("ui.parameters.header-section-children", "header section child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.parameters.header-section", "header section admission failed"))?;
    sections.try_push(header).map_err(|_| PluginAssemblyError::new("ui.parameters.sections", "parameters section admission failed"))?;

    for parameter in &projection.parameters {
        let id = parameter_id(parameter);
        let name = match parameter {
            WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
        };
        let name_action = patch_parameter_action([("parameterId", ui_value_text(id.as_str())?), ("field", ui_value_text("name")?)])?;
        let name_input = semio_framework_ui_contract::input(InputKind::Text)
            .value(fixed_text(&name, "ui.parameters.name-text")?)
            .try_id(format!("s-play-parameters.{id}.name.input"))
            .map_err(|_| PluginAssemblyError::new("ui.parameters.name-id", "name input id admission failed"))?;
        let name_input = bind_action(name_input, Trigger::Change, name_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.name", "name input admission failed"))?;
        let mut name_children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
        name_children.try_push(name_input).map_err(|_| PluginAssemblyError::new("ui.parameters.name-field-children", "name field child admission failed"))?;
        let name_field = semio_framework_ui_contract::field(fixed_label(labels.name.as_str(), "ui.parameters.name-label")?)
            .try_id(format!("s-play-parameters.{id}.name"))
            .map_err(|_| PluginAssemblyError::new("ui.parameters.name-field-id", "name field id admission failed"))?
            .try_children(name_children)
            .map_err(|_| PluginAssemblyError::new("ui.parameters.name-field-children", "name field child admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.parameters.name-field", "name field admission failed"))?;

        let value_control = parameter_value_control(parameter, labels)?;
        let mut value_children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
        value_children.try_push(value_control).map_err(|_| PluginAssemblyError::new("ui.parameters.value-field-children", "value field child admission failed"))?;
        let value_field = semio_framework_ui_contract::field(fixed_label(labels.value.as_str(), "ui.parameters.value-label")?)
            .try_id(format!("s-play-parameters.{id}.value-field"))
            .map_err(|_| PluginAssemblyError::new("ui.parameters.value-field-id", "value field id admission failed"))?
            .try_children(value_children)
            .map_err(|_| PluginAssemblyError::new("ui.parameters.value-field-children", "value field child admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.parameters.value-field", "value field admission failed"))?;

        let remove_action = crate::engine::space::s_play_action("removeParameter", Some(ui_value_map([("parameterId", ui_value_text(id.as_str())?)])?))?;
        let remove_button = semio_framework_ui_contract::button(fixed_label(labels.remove.as_str(), "ui.parameters.remove-label")?)
            .icon(fixed_text(IconName::Trash2.as_str(), "ui.parameters.remove-icon")?)
            .try_id(format!("s-play-parameters.{id}.remove"))
            .map_err(|_| PluginAssemblyError::new("ui.parameters.remove-id", "remove button id admission failed"))?;
        let remove_button = bind_action(remove_button, Trigger::Activate, remove_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.parameters.remove", "remove button admission failed"))?;

        let mut parameter_children = UiFixedList::<semio_framework_plugin::BuiltNode>::default();
        parameter_children.try_push(name_field).map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-children", "parameter section child admission failed"))?;
        parameter_children.try_push(value_field).map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-children", "parameter section child admission failed"))?;
        for constraint_field in parameter_constraint_fields(parameter, labels)? {
            parameter_children.try_push(constraint_field).map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-children", "parameter section child admission failed"))?;
        }
        parameter_children.try_push(remove_button).map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-children", "parameter section child admission failed"))?;

        let section = semio_framework_ui_contract::section(fixed_label(&name, "ui.parameters.parameter-label")?)
            .default_open(true)
            .try_id(format!("s-play-parameters.{id}"))
            .map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-section-id", "parameter section id admission failed"))?
            .try_children(parameter_children)
            .map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-section-children", "parameter section child admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.parameters.parameter-section", "parameter section admission failed"))?;
        sections.try_push(section).map_err(|_| PluginAssemblyError::new("ui.parameters.sections", "parameters section admission failed"))?;
    }

    semio_framework_ui_contract::column()
        .try_children(sections)
        .map_err(|_| PluginAssemblyError::new("ui.parameters.children", "parameters panel child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.parameters.build", "parameters panel admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn render_produces_the_add_parameter_header() {
        let projection = semio_framework_os::empty_workflow_snapshot().await;
        let config = crate::engine::space::config::SpaceConfig::default();
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale);
        let node = render(&projection, labels).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("addParameter"), "header must carry the add-parameter action: {json}");
        assert!(json.contains("parameter"), "empty parameter count copy must render: {json}");
    }
}
//#endregion 🧪️Tests
