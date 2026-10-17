use crate::standards::v1_0::subsets::base::schema::mutations::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(XmlMutation::kinds().len(), 6, "one kind per XmlMutation variant");
}
