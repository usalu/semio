//! 🎛️ Trinity Rewrite app — Parameters window (editable form over the RHS's declared parameters).

use crate::apps::rewrite::terminology::TrinityRewriteLabels;
use crate::artifacts::rewrite::engine::{ParameterKind, Rhs};
use crate::artifacts::rewrite::RewriteRuleModel;
use crate::artifacts::jack::PropertyValue;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, UiFieldNode, UiNode, UiPresence, UiSectionNode};

trait ParameterKindLabel {
    fn kind_label(&self) -> String;
}

impl ParameterKindLabel for crate::artifacts::rewrite::engine::ParameterSpec {
    fn kind_label(&self) -> String {
        match self.kind {
            ParameterKind::String => "string".into(),
            ParameterKind::Number => "number".into(),
            ParameterKind::Boolean => "boolean".into(),
        }
    }
}

pub(crate) fn render(state: &RewriteRuleModel, labels: &TrinityRewriteLabels) -> UiNode {
    let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return ui_text(Label::data("Invalid RHS"));
    };
    let jack_action = crate::apps::rewrite::rewrite_action;
    let mut children: Vec<UiNode> = Vec::new();
    for param in &rhs.parameters {
        let value = state.parameter_bindings.get(&param.name).cloned().unwrap_or_else(|| param.default.clone());
        let display = match value {
            PropertyValue::String(text) => text,
            PropertyValue::Number(number) => number.to_string(),
            PropertyValue::Bool(flag) => flag.to_string(),
            _ => String::new(),
        };
        children.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: format!("trinity-rewrite.param.{}", param.name),
            label: Label::data(param.name.clone()),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                presence: UiPresence::default(),
                id: format!("trinity-rewrite.param.{}.input", param.name),
                input_kind: match param.kind {
                    ParameterKind::Number => "number",
                    ParameterKind::Boolean => "text",
                    ParameterKind::String => "text",
                }
                .into(),
                value: display,
                placeholder: Some(Label::data(param.kind_label())),
                commit: Some("blur".into()),
                on_change: jack_action("setParameter", Some(serde_json::json!({ "name": param.name }))),
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
    }
    if children.is_empty() {
        children.push(ui_text(Label::data("No parameters declared on RHS.")));
    }
    ui_declarative_sections_to_tree(&[UiSectionNode { id: "trinity-rewrite.parameters".into(), label: Some(labels.parameters.into()), default_open: Some(true), presence: UiPresence::default(), children, menu: None }])
}
