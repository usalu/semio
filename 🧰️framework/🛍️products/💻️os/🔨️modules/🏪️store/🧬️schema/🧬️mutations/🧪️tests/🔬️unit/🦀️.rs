use super::*;
use crate::os_spr::SemanticMutation;

#[test]
fn aggregate_roster_is_structural_and_exact() {
    assert_eq!(
        SpaceHistoryMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(),
        ["commit-space-checkpoint", "create-space-alternative", "switch-space-alternative", "remove-space-checkpoint", "remove-space-alternative", "restore-active-space-alternative"]
    );
}
