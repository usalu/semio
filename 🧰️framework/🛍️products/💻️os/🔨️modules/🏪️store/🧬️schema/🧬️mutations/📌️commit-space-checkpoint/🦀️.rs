//! 📌️ Direct space-checkpoint commit mutation.
use crate::space_history_mutations::remove_space_checkpoint::RemoveSpaceCheckpoint;
use crate::space_history_mutations::SpaceHistoryMutation;
use crate::space_history_document::{SpaceCheckpoint, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 📌️ Commits one canonical space checkpoint and its exact member pins.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CommitSpaceCheckpoint {
    pub checkpoint: SpaceCheckpoint,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl semio_framework_os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CommitSpaceCheckpoint {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "commit", entity: "space-checkpoint", kind: "commit-space-checkpoint", record: "CommittedSpaceCheckpoint" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> semio_framework_os_spr::MutationOutcome<SpaceHistoryDiff> {
        semio_framework_os_spr::MutationOutcome::new(SpaceHistoryDiff::step(SpaceHistoryStep::AddCheckpoint(self.checkpoint.clone())))
    }
    fn inverse(&self, _base: &SpaceHistorySnapshot) -> Result<Vec<SpaceHistoryMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: self.checkpoint.id.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Commit space checkpoint {}", self.checkpoint.id), &format!("Space-Sicherungspunkt {} festschreiben", self.checkpoint.id))
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
