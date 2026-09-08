
use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDeclarationMutation as protocol::MutationKind<XmlSnapshot, super::super::XmlMutation>>::SEMANTICS.kind, "set-declaration");
}
