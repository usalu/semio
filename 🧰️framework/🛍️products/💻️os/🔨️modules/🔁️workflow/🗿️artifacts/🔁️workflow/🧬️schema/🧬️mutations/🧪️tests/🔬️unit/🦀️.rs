
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
            "unbind-output"
        ]
    );
    assert_eq!(
        <WorkflowMutation as Mutation<WorkflowSnapshot>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(),
        vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17)]
    );
}
