//! 🗑️ Direct space-checkpoint removal mutation.
use super::super::{CommitSpaceCheckpoint, SpaceHistoryMutation};
use super::super::{SpaceHistoryDiff, SpaceHistorySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveSpaceCheckpoint { pub checkpoint_id: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for RemoveSpaceCheckpoint {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "remove", entity: "space-checkpoint", kind: "remove-space-checkpoint", record: "RemovedSpaceCheckpoint" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> { crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { remove_checkpoint_id: Some(self.checkpoint_id.clone()), ..Default::default() }) }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> { base.checkpoints.iter().find(|value| value.id == self.checkpoint_id).map(|checkpoint| vec![SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() })]).unwrap_or_default() }
    fn label(&self) -> String { format!("Remove space checkpoint {}", self.checkpoint_id) }
    fn target(&self) -> Vec<String> { vec!["checkpoints".into(), self.checkpoint_id.clone()] }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{MutationKind, MutationLeaf};
    #[test]
    fn metadata_and_target_subregion_are_leaf_owned() {
        let payload = RemoveSpaceCheckpoint { checkpoint_id: "cp".into() };
        assert_eq!(<RemoveSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
        assert!(<RemoveSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/🗑️remove-space-checkpoint"));
        assert_eq!(payload.target(), vec!["checkpoints", "cp"]);
    }
}
//#endregion 🧪️Tests
