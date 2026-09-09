use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveArrayElementMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "remove-array-element");
}
