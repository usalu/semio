
use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetTextMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "set-text");
}
