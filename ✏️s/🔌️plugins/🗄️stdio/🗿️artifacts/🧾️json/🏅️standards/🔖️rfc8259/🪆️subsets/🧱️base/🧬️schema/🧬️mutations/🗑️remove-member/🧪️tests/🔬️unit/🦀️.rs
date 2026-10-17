use crate::JsonSnapshot;
use crate::standards::v_rfc8259::subsets::base::schema::mutations::remove_member::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveMemberMutation as protocol::MutationKind<JsonSnapshot, crate::standards::v_rfc8259::subsets::base::schema::mutations::JsonMutation>>::SEMANTICS.kind, "remove-member");
}
