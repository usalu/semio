
use super::*;
use crate::demo_space_projection;
use crate::engine::space::engine::parameter_entity_id;
use crate::engine::space::testkit::apply_mutations;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchParameter(PatchParameter { parameter_id: "p1".into(), field: "value".into(), value: "48".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::AddParameter(crate::engine::space::commands::add_parameter::AddParameter { name: "Parameter".into(), kind: "numeric".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveParameter(crate::engine::space::commands::remove_parameter::RemoveParameter { parameter_id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::BindParameterField(crate::engine::space::commands::bind_parameter_field::BindParameterField { node_id: "n1".into(), field_path: "label".into(), parameter_id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::UnbindParameterField(crate::engine::space::commands::unbind_parameter_field::UnbindParameterField { node_id: "n1".into(), field_path: "label".into() }));
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
    use crate::engine::space::engine::OsParameterId;
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
        crate::engine::space::commands::bind_parameter_field::handle(&crate::engine::space::commands::bind_parameter_field::BindParameterField { node_id: node.id.clone(), field_path: "label".into(), parameter_id }, &doc, &cfg).expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    assert!(projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
    let doc = ArtifactView::new(&projection, &history);
    let emit = crate::engine::space::commands::unbind_parameter_field::handle(&crate::engine::space::commands::unbind_parameter_field::UnbindParameterField { node_id: node.id.clone(), field_path: "label".into() }, &doc, &cfg).expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    assert!(!projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
}
