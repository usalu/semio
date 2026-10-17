use crate::XmlSnapshot;
use crate::standards::v1_0::subsets::base::schema::mutations::insert_element::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<InsertElementMutation as protocol::MutationKind<XmlSnapshot, crate::standards::v1_0::subsets::base::schema::mutations::XmlMutation>>::SEMANTICS.kind, "insert-element");
}
