use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<InsertArrayElementMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "insert-array-element");
}
