
use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveElementMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "remove-element");
}
