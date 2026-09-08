
use super::*;
use crate::os_spr::{Mutation, MutationDiff, MutationKind, MutationLeaf};

fn alternative(id: &str) -> super::super::super::SpaceAlternative {
    super::super::super::SpaceAlternative { id: id.into(), name: id.into(), checkpoint_ids: Vec::new() }
}

#[test]
fn inverse_restores_inactive_and_active_alternatives() {
    assert_eq!(<RemoveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-space-alternative");
    assert!(<RemoveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🧹️remove-space-alternative"));
    for active in ["a", "b"] {
        let before = SpaceHistorySnapshot { alternatives: vec![alternative("a"), alternative("b")], active_alternative_id: Some(active.into()), ..Default::default() };
        let mutation = RemoveSpaceAlternative { alternative_id: "b".into() };
        let post = mutation.diff(&before).diff().apply(&before).expect("remove applies");
        let mut inverse = mutation.inverse(&before);
        inverse.reverse();
        let restored = inverse.into_iter().fold(post, |current, step| step.diff(&current).diff().apply(&current).expect("inverse applies"));
        assert_eq!(restored, before, "active {active}");
    }
}
