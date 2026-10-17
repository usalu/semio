use crate::XmlSnapshot;
use crate::standards::v1_0::subsets::base::schema::mutations::set_declaration::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDeclarationMutation as protocol::MutationKind<XmlSnapshot, crate::standards::v1_0::subsets::base::schema::mutations::XmlMutation>>::SEMANTICS.kind, "set-declaration");
}
