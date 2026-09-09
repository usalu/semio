use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDoctypeMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "set-doctype");
}
