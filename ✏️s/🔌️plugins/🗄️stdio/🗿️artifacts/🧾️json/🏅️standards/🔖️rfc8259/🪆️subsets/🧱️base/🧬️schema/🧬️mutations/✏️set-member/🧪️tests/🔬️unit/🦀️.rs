use crate::JsonSnapshot;
use crate::standards::v_rfc8259::subsets::base::schema::mutations::set_member::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetMemberMutation as protocol::MutationKind<JsonSnapshot, crate::standards::v_rfc8259::subsets::base::schema::mutations::JsonMutation>>::SEMANTICS.kind, "set-member");
}
