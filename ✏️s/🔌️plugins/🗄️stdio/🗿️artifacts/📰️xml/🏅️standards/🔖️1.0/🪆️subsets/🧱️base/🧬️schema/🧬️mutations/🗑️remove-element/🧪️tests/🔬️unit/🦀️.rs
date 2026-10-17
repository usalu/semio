use crate::XmlSnapshot;
use crate::standards::v1_0::subsets::base::schema::mutations::remove_element::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveElementMutation as protocol::MutationKind<XmlSnapshot, crate::standards::v1_0::subsets::base::schema::mutations::XmlMutation>>::SEMANTICS.kind, "remove-element");
}
