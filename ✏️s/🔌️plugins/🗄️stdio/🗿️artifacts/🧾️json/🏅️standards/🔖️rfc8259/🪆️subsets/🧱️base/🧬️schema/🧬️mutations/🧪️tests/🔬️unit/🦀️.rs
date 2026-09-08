
use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(JsonMutation::kinds().len(), 5);
}
