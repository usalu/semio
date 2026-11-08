use semio_os_spr::ColdMutationKind;
use semio_os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use semio_os_spr::{Mutation, MutationDiff, MutationKind, MutationLeaf};

fn alternative(id: &str) -> crate::space_history_document::SpaceAlternative {
    crate::space_history_document::SpaceAlternative { id: id.into(), name: id.into(), checkpoint_ids: Vec::new() }
}

#[test]
fn inverse_restores_inactive_and_active_alternatives() {
    assert_eq!(<RemoveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-space-alternative");
    assert!(<RemoveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🧹️remove-space-alternative"));
    for active in ["a", "b"] {
        let before = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b")], active_alternative_id: Some(active.into()), ..Default::default() };
        let mutation = RemoveSpaceAlternative { alternative_id: "b".into() };
        let post = semio_os_spr::apply_diff_cold(mutation.diff(&before).diff(), &before).expect("remove applies");
        let mut inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
        inverse.reverse();
        let restored = inverse.into_iter().fold(post, |current, step| semio_os_spr::apply_diff_cold(step.diff(&current).diff(), &current).expect("inverse applies"));
        assert_eq!(restored, before, "active {active}");
    }
}

/// ➕️ L3: removing the first, a middle and the last of three alternatives, active or not, sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for active in [None, Some("a".to_string()), Some("b".to_string()), Some("c".to_string())] {
        let base = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b"), alternative("c")], active_alternative_id: active, ..Default::default() };
        for id in ["a", "b", "c"] {
            semio_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: id.into() }), &base).await;
        }
    }
}

/// 🧹️ Removing the active alternative leaves no dangling active id; removing an inactive one keeps the active id.
#[test]
fn removing_the_active_alternative_clears_the_active_id() {
    for (active, expected) in [("a", Some("a")), ("b", None)] {
        let before = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b")], active_alternative_id: Some(active.into()), ..Default::default() };
        let post = semio_os_spr::apply_diff_cold(RemoveSpaceAlternative { alternative_id: "b".into() }.diff(&before).diff(), &before).expect("remove applies");
        assert_eq!(post.active_alternative_id.as_deref(), expected, "active {active}");
        assert!(post.active_alternative_id.as_ref().is_none_or(|id| post.alternatives.iter().any(|alternative| alternative.id == *id)));
    }
}
