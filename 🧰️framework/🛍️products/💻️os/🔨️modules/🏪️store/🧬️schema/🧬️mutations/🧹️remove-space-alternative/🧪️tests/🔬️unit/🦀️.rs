use super::*;
use semio_framework_os_spr::{Mutation, MutationDiff, MutationKind, MutationLeaf};

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
        let post = semio_framework_os_spr::apply_diff(mutation.diff(&before).diff(), &before).expect("remove applies");
        let mut inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
        inverse.reverse();
        let restored = inverse.into_iter().fold(post, |current, step| semio_framework_os_spr::apply_diff(step.diff(&current).diff(), &current).expect("inverse applies"));
        assert_eq!(restored, before, "active {active}");
    }
}
