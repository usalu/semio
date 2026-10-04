//! 🧪️ Laws of the empty DAG parent vocabulary (design §20.15): content edits are child-lane leaves only.

use super::*;

/// ⚖️ LAW: the parent vocabulary declares no kind and no descriptor, and refuses every operation line and record.
#[test]
fn the_parent_vocabulary_is_empty_and_refuses_every_operation() {
    assert!(<DagMutation as protocol::SemanticMutation<DagSnapshot>>::kinds().is_empty());
    assert!(<DagMutation as protocol::Mutation<DagSnapshot>>::DESCRIPTORS.is_empty());
    assert!(<DagMutation as protocol::OpText>::parse_op("moveNode:6e31,0,0").is_err());
    assert!(<DagMutation as protocol::OpBinary>::decode_op(&[0]).is_err());
}
