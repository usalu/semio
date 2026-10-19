use semio_framework_os_spr::ColdMutationKind;
use semio_framework_os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use semio_framework_os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_target_subregion_are_leaf_owned() {
    let payload = RemoveSpaceCheckpoint { checkpoint_id: "cp".into() };
    assert_eq!(<RemoveSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<RemoveSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/🗑️remove-space-checkpoint"));
    assert_eq!(payload.target(), vec!["checkpoints", "cp"]);
}

fn checkpoint(id: &str) -> super::super::super::SpaceCheckpoint {
    super::super::super::SpaceCheckpoint { id: id.into(), parent_id: None, message: id.into(), authors: Vec::new(), timestamp: semio_framework_os_spr::HybridLogicalTimestamp::new(0, 1), members: Vec::new() }
}

/// ➕️ L3: removing the first, a middle and the last of three checkpoints inverts to the same position and sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = SpaceHistorySnapshot { checkpoints: vec![checkpoint("a"), checkpoint("b"), checkpoint("c")], ..Default::default() };
    for id in ["a", "b", "c"] {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: id.into() }), &base).await;
    }
}

#[test]
fn removing_an_absent_checkpoint_is_refused_with_the_frozen_target_missing_code() {
    let outcome = RemoveSpaceCheckpoint { checkpoint_id: "nope".into() }.diff(&SpaceHistorySnapshot::default());
    assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), ["mutation.target-missing"]);
}
