use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<InsertElementPayload as protocol::MutationKind<SvgSnapshot, super::super::SvgMutation>>::SEMANTICS.kind, "insert-element");
}
