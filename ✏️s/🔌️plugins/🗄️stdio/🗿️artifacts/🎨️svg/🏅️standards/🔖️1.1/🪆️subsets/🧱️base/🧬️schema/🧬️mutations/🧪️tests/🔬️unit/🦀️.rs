use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(SvgMutation::kinds().len(), 9);
}
