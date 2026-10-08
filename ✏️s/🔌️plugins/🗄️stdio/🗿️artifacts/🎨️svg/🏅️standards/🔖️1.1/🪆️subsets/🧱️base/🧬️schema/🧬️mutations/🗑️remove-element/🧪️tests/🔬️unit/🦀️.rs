use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveElementPayload as protocol::MutationKind<SvgSnapshot, super::super::SvgMutation>>::SEMANTICS.kind, "remove-element");
}
