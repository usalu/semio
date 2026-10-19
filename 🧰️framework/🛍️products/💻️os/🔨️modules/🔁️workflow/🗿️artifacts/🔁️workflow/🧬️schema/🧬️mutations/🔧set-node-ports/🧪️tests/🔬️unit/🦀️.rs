
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_identity() {
    assert_eq!(<SetNodePorts as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-node-ports");
}

/// 🎛️ Three nodes whose middle node `b` has a parameter binding that was never resynced into its ports; `a` and `c` are in sync.
async fn stale_middle_document() -> WorkflowSnapshot {
    use crate::{WorkflowNode, WorkflowParameter, WorkflowParameterBinding};
    let mut document = crate::empty_workflow_snapshot().await;
    for id in ["a", "b", "c"] {
        document.graph.nodes.push(WorkflowNode { id: id.into(), plugin_id: "plugin".into(), app_id: "app".into(), label: id.into(), yields: String::new(), artifact_ref: String::new(), config_ref: String::new(), x: 0.0, y: 0.0, width: 220.0, height: 74.0, inputs: Vec::new(), outputs: Vec::new() });
    }
    document.parameters.push(WorkflowParameter::Toggle { id: "p".into(), name: "P".into(), value: true });
    document.parameter_bindings.push(WorkflowParameterBinding { parameter_id: "p".into(), node_id: "b".into(), field_path: "/flag".into() });
    document
}

/// ➕️ L3: installing ports on the middle node, and on a first and a last node, inverts to the base ports.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let document = stale_middle_document().await;
    for id in ["a", "b", "c"] {
        let mutation = WorkflowMutation::SetNodePorts(SetNodePorts { nodes: vec![WorkflowNodePorts { node_id: id.into(), inputs: Vec::new(), height: 92.0 }] });
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &document).await;
    }
}
