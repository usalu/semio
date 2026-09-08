
use super::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDeclarationMutation as protocol::MutationKind<SvgSnapshot, super::super::SvgMutation>>::SEMANTICS.kind, "set-declaration");
}
