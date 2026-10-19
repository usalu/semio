use semio_framework_os_spr::ColdMutationKind;
use semio_framework_os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use semio_framework_os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_diff_are_leaf_owned() {
    assert_eq!(<CommitSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<CommitSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/📌️commit-space-checkpoint"));
    let payload = CommitSpaceCheckpoint { checkpoint: SpaceCheckpoint { id: "cp".into(), parent_id: None, message: String::new(), authors: Vec::new(), timestamp: semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0), members: Vec::new() } , index: None };
    assert!(matches!(payload.diff(&SpaceHistorySnapshot::default()).diff().steps.as_slice(), [crate::space_history_document::SpaceHistoryStep::AddCheckpoint{ checkpoint:value, index: None }] if value.id == "cp"));
}

fn checkpoint(id: &str) -> SpaceCheckpoint {
    SpaceCheckpoint { id: id.into(), parent_id: None, message: id.into(), authors: Vec::new(), timestamp: semio_framework_os_spr::HybridLogicalTimestamp::new(0, 1), members: Vec::new() }
}

/// ➕️ L3: appending, inserting first, in the middle and at the end of three checkpoints (and into an empty history) sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = SpaceHistorySnapshot { checkpoints: vec![checkpoint("a"), checkpoint("b"), checkpoint("c")], ..Default::default() };
    for index in [None, Some(0), Some(1), Some(3)] {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint("n"), index }), &base).await;
    }
    semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint("n"), index: None }), &SpaceHistorySnapshot::default()).await;
}

#[test]
fn an_insert_past_the_end_or_of_a_duplicate_id_is_refused() {
    let base = SpaceHistorySnapshot { checkpoints: vec![checkpoint("a")], ..Default::default() };
    for (payload, code) in [(CommitSpaceCheckpoint { checkpoint: checkpoint("n"), index: Some(2) }, "mutation.target-missing"), (CommitSpaceCheckpoint { checkpoint: checkpoint("a"), index: None }, "mutation.duplicate-id")] {
        let outcome = payload.diff(&base);
        assert_eq!(outcome.messages().iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), [code]);
    }
}
