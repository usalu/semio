//! 🧪️ Laws of the empty procedure parent vocabulary (design §20.15): content edits are child-lane leaves only.

use super::*;

/// ⚖️ LAW: the parent vocabulary declares no kind and no descriptor, and refuses every operation line and record.
#[test]
fn the_parent_vocabulary_is_empty_and_refuses_every_operation() {
    assert!(<ProcedureMutation as protocol::SemanticMutation<ProcedureSnapshot>>::kinds().is_empty());
    assert!(<ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::DESCRIPTORS.is_empty());
    assert!(<ProcedureMutation as protocol::OpText>::parse_op("createStep:step-1").is_err());
    assert!(<ProcedureMutation as protocol::OpBinary>::decode_op(&[0]).is_err());
}
