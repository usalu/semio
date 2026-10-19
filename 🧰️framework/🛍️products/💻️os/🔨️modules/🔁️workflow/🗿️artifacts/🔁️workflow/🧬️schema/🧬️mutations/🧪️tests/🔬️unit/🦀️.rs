
use super::*;
use protocol::{Mutation, SemanticMutation};
#[test]
fn descriptors_follow_the_canonical_workflow_roster() {
    assert_eq!(
        WorkflowMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(),
        vec![
            "add-node",
            "remove-node",
            "connect-ports",
            "disconnect-edge",
            "move-node",
            "rename-node",
            "add-parameter",
            "remove-parameter",
            "change-parameter",
            "bind-parameter-field",
            "unbind-parameter-field",
            "update-node-ports",
            "add-input",
            "remove-input",
            "bind-input",
            "unbind-input",
            "bind-output",
            "unbind-output",
            "move-nodes",
            "set-node-positions",
            "set-node-ports"
        ]
    );
    assert_eq!(
        <WorkflowMutation as Mutation<WorkflowSnapshot>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(),
        vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]
    );
}

/// ➕️ L3 for every workflow kind: the concrete inverse's diffs sum to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn every_workflow_kind_inverse_diffs_sum_to_the_negative_diff() {
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    use crate::tests::{sample_workflow_snapshot, stale_ports_snapshot, workflow_edge, workflow_node};
    use crate::*;
    let document = sample_workflow_snapshot().await;
    let node_c = workflow_node("c", Vec::new(), Vec::new()).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::AddNode(AddNode { node: node_c }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::RemoveNode(RemoveNode { node_id: "b".into() }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::MoveNode(MoveNode { node_id: "a".into(), x: 99.0, y: -1.0 }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::RenameNode(RenameNode { node_id: "a".into(), label: "Renamed".into() }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::MoveNodes(MoveNodes { node_ids: vec!["a".into(), "b".into()], dx: 40.0, dy: -12.5 }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::SetNodePositions(SetNodePositions { positions: vec![WorkflowNodePosition { node_id: "a".into(), x: 7.0, y: 8.0 }] }), &document).await;
    let mut unwired = document.clone();
    unwired.graph.edges.clear();
    assert_mutation_inverse_sum_law(&WorkflowMutation::ConnectPorts(ConnectPorts { edge: workflow_edge("e9", "a", "a:out:out", "b", "b:in:in").await }), &unwired).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: "e1".into() }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(WorkflowParameter::Toggle { id: "p9".into(), name: "New".into(), value: true }) }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: "p4".into() }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: "p3".into(), parameter: Box::new(WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: false }) }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::BindParameterField(BindParameterField { binding: WorkflowParameterBinding { parameter_id: "p2".into(), node_id: "a".into(), field_path: "/mode".into() } }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: "a".into(), field_path: "/zoom".into() }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}), &document).await;
    let stale = stale_ports_snapshot().await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}), &stale).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::SetNodePorts(SetNodePorts { nodes: vec![WorkflowNodePorts { node_id: "b".into(), inputs: Vec::new(), height: 56.0 }] }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::AddInput(AddInput { input: WorkflowInput { id: "in-2".into(), kind_id: "kind.b".into(), selector: "**/*".into(), required: false, multiplicity: PortMultiplicity::One } }), &document).await;
    let mut bound = document.clone();
    bound.input_bindings.push(WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() });
    assert_mutation_inverse_sum_law(&WorkflowMutation::RemoveInput(RemoveInput { input_id: "in-1".into() }), &bound).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::BindInput(BindInput { binding: WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() } }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::UnbindInput(UnbindInput { input_id: "in-1".into() }), &bound).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::BindOutput(BindOutput { binding: WorkflowOutputBinding { node_id: "a".into(), port_id: "a:out:out".into(), path_template: "renders/other.out".into() } }), &document).await;
    assert_mutation_inverse_sum_law(&WorkflowMutation::UnbindOutput(UnbindOutput { node_id: "a".into(), port_id: "a:out:out".into() }), &document).await;
}
