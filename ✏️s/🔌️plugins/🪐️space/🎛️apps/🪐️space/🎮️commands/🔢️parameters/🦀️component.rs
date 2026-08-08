//! 🔢️ S Studio app — workflow parameter commands: patch/add/remove + field binding.
//!
//! One nested `pub mod` per payload (the `app_commands!` shape — see `apps::space::🦀️component.rs`'s
//! `🔖️SpaceCommand` region, which `use`s each of these modules flat).

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowDocument, WorkflowMutation, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterType};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️PatchParameter
pub mod patch_parameter {
    use super::*;
    use crate::apps::space::engine::parameter_entity_id;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-parameter")]
    pub struct PatchParameter {
        pub parameter_id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchParameter, doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let projection = doc.projection;
        let value_json: Value = serde_json::from_str(&payload.value).unwrap_or_else(|_| Value::String(payload.value.clone()));
        let patch = if payload.field == "addOption" {
            value_json.as_str().map(str::to_string).and_then(|option| {
                projection.parameters.iter().find(|entry| parameter_entity_id(entry) == payload.parameter_id).and_then(|entry| match entry {
                    WorkflowParameter::Categorical { options, .. } => {
                        let mut next_options = options.clone();
                        if !next_options.iter().any(|row| row == &option) {
                            next_options.push(option.clone());
                        }
                        Some(json!({ "options": next_options, "value": option }))
                    }
                    _ => None,
                })
            })
        } else if payload.field == "removeOption" {
            value_json.as_str().map(str::to_string).and_then(|option| {
                projection.parameters.iter().find(|entry| parameter_entity_id(entry) == payload.parameter_id).and_then(|entry| match entry {
                    WorkflowParameter::Categorical { options, value, .. } => {
                        let next_options: Vec<_> = options.iter().filter(|row| row.as_str() != option).cloned().collect();
                        let next_value = if next_options.iter().any(|row| row.as_str() == value.as_str()) { value.clone() } else { next_options.first().cloned().unwrap_or_default() };
                        Some(json!({ "options": next_options, "value": next_value }))
                    }
                    _ => None,
                })
            })
        } else {
            Some(json!({ payload.field.clone(): value_json }))
        };
        match patch.and_then(|patch| crate::apps::space::engine::patch_parameter_operation(projection, &payload.parameter_id, &patch)) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchParameter

//#region 🔖️AddParameter
pub mod add_parameter {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-parameter")]
    pub struct AddParameter {
        pub name: String,
        pub kind: String,
    }

    pub fn handle(payload: &AddParameter, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let parameter_type = match payload.kind.as_str() {
            "categorical" => WorkflowParameterType::Categorical,
            "toggle" => WorkflowParameterType::Toggle,
            "text" => WorkflowParameterType::Text,
            _ => WorkflowParameterType::Numeric,
        };
        Ok(Emit::mutations(vec![crate::apps::space::engine::add_parameter_operation(&parameter_type, &payload.name)]))
    }
}
//#endregion 🔖️AddParameter

//#region 🔖️RemoveParameter
pub mod remove_parameter {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-parameter")]
    pub struct RemoveParameter {
        pub parameter_id: String,
    }

    pub fn handle(payload: &RemoveParameter, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![WorkflowMutation::RemoveParameter { parameter_id: payload.parameter_id.clone() }]))
    }
}
//#endregion 🔖️RemoveParameter

//#region 🔖️BindParameterField
pub mod bind_parameter_field {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "bind-parameter-field")]
    pub struct BindParameterField {
        pub node_id: String,
        pub field_path: String,
        pub parameter_id: String,
    }

    pub fn handle(payload: &BindParameterField, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        if payload.parameter_id.is_empty() || payload.parameter_id == "__direct__" {
            Ok(Emit::mutations(vec![WorkflowMutation::UnbindParameterField { node_id: payload.node_id.clone(), field_path: payload.field_path.clone() }]))
        } else {
            Ok(Emit::mutations(vec![WorkflowMutation::BindParameterField {
                binding: WorkflowParameterBinding { parameter_id: payload.parameter_id.clone(), node_id: payload.node_id.clone(), field_path: payload.field_path.clone() },
            }]))
        }
    }
}
//#endregion 🔖️BindParameterField

//#region 🔖️UnbindParameterField
pub mod unbind_parameter_field {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "unbind-parameter-field")]
    pub struct UnbindParameterField {
        pub node_id: String,
        pub field_path: String,
    }

    pub fn handle(payload: &UnbindParameterField, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![WorkflowMutation::UnbindParameterField { node_id: payload.node_id.clone(), field_path: payload.field_path.clone() }]))
    }
}
//#endregion 🔖️UnbindParameterField

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::engine::parameter_entity_id;
    use crate::apps::space::testkit::apply_mutations;
    use crate::demo_space_projection;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchParameter(patch_parameter::PatchParameter { parameter_id: "p1".into(), field: "value".into(), value: "48".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::AddParameter(add_parameter::AddParameter { name: "Parameter".into(), kind: "numeric".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveParameter(remove_parameter::RemoveParameter { parameter_id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::BindParameterField(bind_parameter_field::BindParameterField { node_id: "n1".into(), field_path: "label".into(), parameter_id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::UnbindParameterField(unbind_parameter_field::UnbindParameterField { node_id: "n1".into(), field_path: "label".into() }));
    }

    #[test]
    fn patch_parameter_action_updates_value() {
        let projection = demo_space_projection();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = SpaceConfig::default();
        let cfg = ConfigView { projection: &config };
        let emit = patch_parameter::handle(&patch_parameter::PatchParameter { parameter_id: "param-brush-size".into(), field: "value".into(), value: "48".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.document_mutations.len(), 1);
        let next = apply_mutations(&projection, &emit.document_mutations);
        use crate::apps::space::engine::OsParameterId;
        match next.parameters.iter().find(|entry| entry.id() == "param-brush-size").expect("parameter") {
            WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn unbind_parameter_field_removes_binding() {
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.graph.nodes.first().expect("node").clone();
        let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).to_string();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let cfg = ConfigView { projection: &config };
        let emit = bind_parameter_field::handle(&bind_parameter_field::BindParameterField { node_id: node.id.clone(), field_path: "label".into(), parameter_id }, &doc, &cfg).expect("handle");
        projection = apply_mutations(&projection, &emit.document_mutations);
        assert!(projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = unbind_parameter_field::handle(&unbind_parameter_field::UnbindParameterField { node_id: node.id.clone(), field_path: "label".into() }, &doc, &cfg).expect("handle");
        projection = apply_mutations(&projection, &emit.document_mutations);
        assert!(!projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
    }
}
//#endregion 🧪️Tests
