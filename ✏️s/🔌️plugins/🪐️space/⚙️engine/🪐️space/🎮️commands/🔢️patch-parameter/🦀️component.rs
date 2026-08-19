//! 🔢️ 🔢️ S Studio app command — `patch-parameter`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowParameter, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::engine::space::engine::parameter_entity_id;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-parameter")]
pub struct PatchParameter {
    pub parameter_id: String,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchParameter, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
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
    match patch.and_then(|patch| crate::engine::space::engine::patch_parameter_operation(projection, &payload.parameter_id, &patch)) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::engine::parameter_entity_id;
    use crate::engine::space::testkit::apply_mutations;
    use crate::demo_space_projection;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        use crate::engine::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchParameter(PatchParameter { parameter_id: "p1".into(), field: "value".into(), value: "48".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::AddParameter(crate::engine::space::commands::add_parameter::AddParameter { name: "Parameter".into(), kind: "numeric".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveParameter(crate::engine::space::commands::remove_parameter::RemoveParameter { parameter_id: "p1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::BindParameterField(crate::engine::space::commands::bind_parameter_field::BindParameterField {
            node_id: "n1".into(),
            field_path: "label".into(),
            parameter_id: "p1".into(),
        }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::UnbindParameterField(crate::engine::space::commands::unbind_parameter_field::UnbindParameterField { node_id: "n1".into(), field_path: "label".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_parameter_action_updates_value() {
        let projection = demo_space_projection();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let config = SpaceConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&PatchParameter { parameter_id: "param-brush-size".into(), field: "value".into(), value: "48".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.artifact_mutations.len(), 1);
        let next = apply_mutations(&projection, &emit.artifact_mutations);
        use crate::engine::space::engine::OsParameterId;
        match next.parameters.iter().find(|entry| entry.id() == "param-brush-size").expect("parameter") {
            WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
            _ => panic!("expected numeric"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn unbind_parameter_field_removes_binding() {
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.graph.nodes.first().expect("node").clone();
        let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).to_string();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let cfg = ConfigView { snapshot: &config };
        let emit =
            crate::engine::space::commands::bind_parameter_field::handle(&crate::engine::space::commands::bind_parameter_field::BindParameterField { node_id: node.id.clone(), field_path: "label".into(), parameter_id }, &doc, &cfg).expect("handle");
        projection = apply_mutations(&projection, &emit.artifact_mutations);
        assert!(projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
        let doc = ArtifactView::new(&projection, &history);
        let emit = crate::engine::space::commands::unbind_parameter_field::handle(&crate::engine::space::commands::unbind_parameter_field::UnbindParameterField { node_id: node.id.clone(), field_path: "label".into() }, &doc, &cfg).expect("handle");
        projection = apply_mutations(&projection, &emit.artifact_mutations);
        assert!(!projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
    }
}
//#endregion 🧪️Tests
