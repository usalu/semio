

use crate::demo_space_projection;
use semio_hub_space_operations::parameter_entity_id;
use semio_hub_space_app::unit_tests::context::apply_mutations;
use semio_framework_plugin::HistoryView;
use semio_framework_plugin::{ArtifactView, ConfigView};
use semio_hub_space_commands_authoring::patch_parameter::{PatchParameter, handle};
use semio_hub_space_foundation::config::SpaceConfig;
use semio_framework_os::WorkflowParameter;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use semio_hub_space_app::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchParameter(PatchParameter { parameter_id: "p1".into(), field: "value".into(), value: "48".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::AddParameter(semio_hub_space_commands_authoring::add_parameter::AddParameter { name: "Parameter".into(), kind: "numeric".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveParameter(semio_hub_space_commands_authoring::remove_parameter::RemoveParameter { parameter_id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::BindParameterField(semio_hub_space_commands_authoring::bind_parameter_field::BindParameterField { node_id: "n1".into(), field_path: "label".into(), parameter_id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::UnbindParameterField(semio_hub_space_commands_authoring::unbind_parameter_field::UnbindParameterField { node_id: "n1".into(), field_path: "label".into() }));
}

#[semio_framework_async_macros::async_test]
async fn patch_parameter_action_updates_value() {
    let projection = demo_space_projection().await;
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = SpaceConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PatchParameter { parameter_id: "param-brush-size".into(), field: "value".into(), value: "48".into() }, &doc, &cfg).expect("handle");
    assert_eq!(emit.artifact_mutations.len(), 1);
    let next = apply_mutations(&projection, &emit.artifact_mutations).await;
    use semio_hub_space_operations::OsParameterId;
    let mut found = None;
    for entry in &next.parameters {
        if entry.id().await == "param-brush-size" {
            found = Some(entry);
            break;
        }
    }
    match found.expect("parameter") {
        WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
        _ => panic!("expected numeric"),
    }
}

#[semio_framework_async_macros::async_test]
async fn unbind_parameter_field_removes_binding() {
    let mut projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node = projection.graph.nodes.first().expect("node").clone();
    let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).await.to_string();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit =
        semio_hub_space_commands_authoring::bind_parameter_field::handle(&semio_hub_space_commands_authoring::bind_parameter_field::BindParameterField { node_id: node.id.clone(), field_path: "label".into(), parameter_id }, &doc, &cfg).expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    assert!(projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
    let doc = ArtifactView::new(&projection, &history);
    let emit = semio_hub_space_commands_authoring::unbind_parameter_field::handle(&semio_hub_space_commands_authoring::unbind_parameter_field::UnbindParameterField { node_id: node.id.clone(), field_path: "label".into() }, &doc, &cfg).expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    assert!(!projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
}
