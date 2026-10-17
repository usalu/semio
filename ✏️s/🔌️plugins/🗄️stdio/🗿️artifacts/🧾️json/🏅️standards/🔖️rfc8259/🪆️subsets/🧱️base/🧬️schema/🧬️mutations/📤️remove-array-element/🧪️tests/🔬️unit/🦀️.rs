use crate::JsonSnapshot;
use crate::standards::v_rfc8259::subsets::base::schema::mutations::remove_array_element::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveArrayElementMutation as protocol::MutationKind<JsonSnapshot, crate::standards::v_rfc8259::subsets::base::schema::mutations::JsonMutation>>::SEMANTICS.kind, "remove-array-element");
}
