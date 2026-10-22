use semio_framework_os_spr::ColdMutationKind;
use super::*;
use semio_framework_os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_target_subregion_are_leaf_owned() {
    let payload = SwitchSpaceAlternative { alternative_id: "alt".into() };
    assert_eq!(<SwitchSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<SwitchSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🔀️switch-space-alternative"));
    assert_eq!(payload.target(), vec!["activeAlternativeId"]);
}

fn alternative(id: &str) -> crate::space_history_document::SpaceAlternative {
    crate::space_history_document::SpaceAlternative { id: id.into(), name: id.into(), checkpoint_ids: Vec::new() }
}

/// ➕️ L3: switching to another and to the already active alternative sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for active in [None, Some("a".to_string())] {
        let base = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b")], active_alternative_id: active, ..Default::default() };
        for target in ["a", "b"] {
            semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: target.into() }), &base).await;
        }
    }
}
