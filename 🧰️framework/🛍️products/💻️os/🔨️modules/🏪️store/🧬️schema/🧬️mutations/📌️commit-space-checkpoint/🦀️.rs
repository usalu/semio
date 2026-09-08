//! 📌️ Direct space-checkpoint commit mutation.
use super::super::{RemoveSpaceCheckpoint, SpaceHistoryMutation};
use super::super::{SpaceCheckpoint, SpaceHistoryDiff, SpaceHistorySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 📌️ Commits one canonical space checkpoint and its exact member pins.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CommitSpaceCheckpoint {
    pub checkpoint: SpaceCheckpoint,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CommitSpaceCheckpoint {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "commit", entity: "space-checkpoint", kind: "commit-space-checkpoint", record: "CommittedSpaceCheckpoint" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> {
        crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { add_checkpoint: Some(self.checkpoint.clone()), ..Default::default() })
    }
    fn inverse(&self, _base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> {
        vec![SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: self.checkpoint.id.clone() })]
    }
    fn label(&self) -> String {
        format!("Commit space checkpoint {}", self.checkpoint.id)
    }
    fn target(&self) -> Vec<String> {
        vec!["checkpoints".into(), self.checkpoint.id.clone()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
