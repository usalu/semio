//! 🔢️ S Studio app — workflow parameters panel: add/edit/remove the workflow's own parameter set.

use crate::apps::space::engine::parameter_entity_id;
use crate::apps::space::terminology::SStudioLabels;
use crate::apps::space::{s_play_action, S_PLAY_PARAMETERS_BODY_KEY, S_PLAY_PARAMETERS_TAB_ID};
use semio_framework_os::{WorkflowSnapshot, WorkflowParameter};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiNumberStepperNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode, UiToggleNode,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use serde_json::json;

//#region 🔖️Manifest
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(S_PLAY_PARAMETERS_TAB_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, "Parameter"), group: PanelGroup::Workbench, body_key: Some(S_PLAY_PARAMETERS_BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
fn parameter_value_control(parameter: &WorkflowParameter, labels: &SStudioLabels) -> UiNode {
    match parameter {
        WorkflowParameter::Numeric { id, value, step, .. } => UiNode::NumberStepper(UiNumberStepperNode {
            presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            value: *value,
            step: step.unwrap_or(1.0),
            uniform: true,
            on_absolute: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "value" }))),
            on_delta: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "value" }))),
            menu: None,
        }),
        WorkflowParameter::Categorical { id, value, options, .. } => UiNode::Select(UiSelectNode {
            presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            value: value.clone(),
            items: options.iter().map(|option| UiSelectItem { value: option.clone(), label: Label::data(option.clone()) }).collect(),
            placeholder: None,
            on_change: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "value" }))),
            menu: None,
        }),
        WorkflowParameter::Toggle { id, value, .. } => UiNode::Toggle(UiToggleNode {
            id: format!("s-play-parameters.{id}.value"),
            icon_id: "toggle-left".into(),
            presence: UiPresence::selected(*value),
            text: Some(if *value { labels.toggle_on.into() } else { labels.toggle_off.into() }),
            on_change: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "value" }))),
            menu: None,
        }),
        WorkflowParameter::Text { id, value, .. } => UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            input_kind: "text".into(),
            value: value.clone(),
            placeholder: None,
            commit: None,
            on_change: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "value" }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        }),
    }
}

fn parameter_constraint_fields(parameter: &WorkflowParameter, labels: &SStudioLabels) -> Vec<UiNode> {
    match parameter {
        WorkflowParameter::Numeric { id, min, max, step, .. } => vec![
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.min"),
                label: labels.min.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                    presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.min.stepper"),
                    value: min.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "min" }))),
                    on_delta: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "min" }))),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.max"),
                label: labels.max.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                    presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.max.stepper"),
                    value: max.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "max" }))),
                    on_delta: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "max" }))),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.step"),
                label: labels.step.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                    presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.step.stepper"),
                    value: step.unwrap_or(0.0),
                    step: 0.1,
                    uniform: true,
                    on_absolute: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "step" }))),
                    on_delta: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "step" }))),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
        WorkflowParameter::Categorical { id, options, .. } => {
            let mut fields: Vec<UiNode> = options
                .iter()
                .map(|option| {
                    UiNode::Field(UiFieldNode {
                        id: format!("s-play-parameters.{id}.option.{option}"),
                        label: Label::data(option.clone()),
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Button(UiButtonNode {
                            id: Some(format!("s-play-parameters.{id}.option.{option}.remove")),
                            icon_id: "trash-2".into(),
                            label: labels.remove.into(),
                            action: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "removeOption", "value": option }))),
                            style: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    })
                })
                .collect();
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.add-option"),
                label: labels.add_option.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.add-option.input"),
                    input_kind: "text".into(),
                    value: String::new(),
                    placeholder: Some(labels.new_option_placeholder.into()),
                    commit: None,
                    on_change: s_play_action("patchParameter", Some(json!({ "parameterId": id, "field": "addOption" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            fields
        }
        _ => Vec::new(),
    }
}

pub fn render(projection: &WorkflowSnapshot, labels: &SStudioLabels) -> UiNode {
    let mut children = vec![UiSectionNode {
        id: "s-play-parameters.header".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL)),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            UiNode::Button(UiButtonNode {
                id: Some("s-play-parameters.add".into()),
                icon_id: "plus".into(),
                label: labels.add_parameter.into(),
                action: s_play_action("addParameter", Some(json!({ "type": "numeric" }))),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            ui_text(Label::data(format!("{} {}", projection.parameters.len(), labels.parameter_count_suffix.as_str()))),
        ],
        menu: None,
    }];
    for parameter in &projection.parameters {
        let parameter_id = parameter_entity_id(parameter).to_string();
        let mut parameter_children = vec![
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{parameter_id}.name"),
                label: labels.name.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: format!("s-play-parameters.{parameter_id}.name.input"),
                    input_kind: "text".into(),
                    value: match parameter {
                        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
                    },
                    placeholder: None,
                    commit: None,
                    on_change: s_play_action("patchParameter", Some(json!({ "parameterId": parameter_id, "field": "name" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("s-play-parameters.{parameter_id}.value-field"),
                label: labels.value.into(),
                child: Box::new(parameter_value_control(parameter, labels)),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ];
        parameter_children.extend(parameter_constraint_fields(parameter, labels));
        parameter_children.push(UiNode::Button(UiButtonNode {
            id: Some(format!("s-play-parameters.{parameter_id}.remove")),
            icon_id: "trash-2".into(),
            label: labels.remove.into(),
            action: s_play_action("removeParameter", Some(json!({ "parameterId": parameter_id }))),
            style: None,
            presence: UiPresence::default(),
            menu: None,
        }));
        children.push(UiSectionNode {
            id: format!("s-play-parameters.{parameter_id}"),
            label: Some(Label::data(match parameter {
                WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
            })),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: parameter_children,
            menu: None,
        });
    }
    ui_declarative_sections_to_tree(&children)
}
//#endregion 🔖️Render
