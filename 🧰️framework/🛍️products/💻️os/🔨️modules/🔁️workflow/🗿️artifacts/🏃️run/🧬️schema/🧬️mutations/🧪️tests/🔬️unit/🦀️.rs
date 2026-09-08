
use super::*;
use protocol::{Mutation, SemanticMutation};

#[test]
fn descriptors_follow_the_canonical_run_roster() {
    assert_eq!(RunMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(), vec!["start-run", "start-run-node", "finish-run-node", "append-run-log", "seal-run"]);
    assert_eq!(<RunMutation as Mutation<RunArtifact>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(), vec![Some(0), Some(1), Some(2), Some(3), Some(4)]);
}
