//! 🗑️ Direct space-checkpoint removal mutation.
use crate::space_history_mutations::commit_space_checkpoint::CommitSpaceCheckpoint;
use crate::space_history_mutations::SpaceHistoryMutation;
use crate::space_history_document::{SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🗑️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive (its sibling `serde_json` differential test). Production never serializes through serde.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveSpaceCheckpoint {
    pub checkpoint_id: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl semio_framework_os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for RemoveSpaceCheckpoint {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "remove", entity: "space-checkpoint", kind: "remove-space-checkpoint", record: "RemovedSpaceCheckpoint" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove space checkpoint {}", self.checkpoint_id), &format!("Space-Sicherungspunkt {} entfernen", self.checkpoint_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["checkpoints".into(), self.checkpoint_id.clone()]
    }
}

impl semio_framework_os_spr::ColdMutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for RemoveSpaceCheckpoint {
    fn diff(&self, base: &SpaceHistorySnapshot) -> semio_framework_os_spr::MutationOutcome<SpaceHistoryDiff> {
        match base.checkpoints.iter().position(|value| value.id == self.checkpoint_id) {
            Some(index) => semio_framework_os_spr::MutationOutcome::new(SpaceHistoryDiff::step(SpaceHistoryStep::RemoveCheckpoint { checkpoint_id: self.checkpoint_id.clone(), index: index as u64 })),
            None => semio_framework_os_spr::MutationOutcome::error("mutation.target-missing", "checkpoint does not exist", ["checkpoints", self.checkpoint_id.as_str()]),
        }
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Result<Vec<SpaceHistoryMutation>, semio_framework_value::ValueError> {
        Ok(base.checkpoints.iter().position(|value| value.id == self.checkpoint_id).map(|index| vec![SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: base.checkpoints[index].clone(), index: Some(index as u64) })]).unwrap_or_default())
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
