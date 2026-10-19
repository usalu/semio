use semio_framework_os_spr::ColdMutationKind;
use semio_framework_os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use semio_framework_os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_activation_diff_are_leaf_owned() {
    assert_eq!(<CreateSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<CreateSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🌿️create-space-alternative"));
    let payload = CreateSpaceAlternative { alternative: SpaceAlternative { id: "alt".into(), name: "alt".into(), checkpoint_ids: Vec::new() } };
    assert_eq!(payload.diff(&SpaceHistorySnapshot::default()).diff().steps.last(), Some(&crate::space_history_document::SpaceHistoryStep::SetActive { alternative_id: Some("alt".into()) }));
}

fn alternative(id: &str) -> SpaceAlternative {
    SpaceAlternative { id: id.into(), name: id.into(), checkpoint_ids: Vec::new() }
}

/// ➕️ L3: creating the first, a middle and the last alternative (alternatives stay in ascending id order) with and without an active alternative sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for active in [None, Some("a".to_string()), Some("c".to_string())] {
        let base = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("c")], active_alternative_id: active, ..Default::default() };
        for id in ["0", "b", "z"] {
            semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: alternative(id) }), &base).await;
        }
    }
    semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: alternative("a") }), &SpaceHistorySnapshot::default()).await;
}
