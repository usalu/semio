//! 🧹️ Direct space-alternative removal mutation.
use super::super::{CreateSpaceAlternative, RestoreActiveSpaceAlternative, SpaceHistoryMutation};
use super::super::{SpaceHistoryDiff, SpaceHistorySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// @emoji 🧹️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive (its sibling `serde_json` differential test). Production never serializes through serde.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveSpaceAlternative {
    pub alternative_id: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for RemoveSpaceAlternative {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "remove", entity: "space-alternative", kind: "remove-space-alternative", record: "RemovedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> {
        crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { remove_alternative_id: Some(self.alternative_id.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> {
        base.alternatives
            .iter()
            .find(|value| value.id == self.alternative_id)
            .map(|alternative| {
                vec![
                    SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() }),
                    SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: alternative.clone() }),
                ]
            })
            .unwrap_or_default()
    }
    fn label(&self) -> String {
        format!("Remove space alternative {}", self.alternative_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["alternatives".into(), self.alternative_id.clone()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
