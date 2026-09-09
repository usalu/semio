use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetMemberMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "set-member");
}
