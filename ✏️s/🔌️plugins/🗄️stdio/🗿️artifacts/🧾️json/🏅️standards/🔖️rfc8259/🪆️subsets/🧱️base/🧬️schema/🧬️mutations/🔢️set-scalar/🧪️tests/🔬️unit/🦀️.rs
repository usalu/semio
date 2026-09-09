use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetScalarMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "set-scalar");
}
