use super::super::{RunArtifact, RunDiff, RunMutation, RunSealEdit, RunStatus, RunStep, SetRunSeal};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "seal-run")]
pub struct SealRun {
    pub status: RunStatus,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for SealRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "seal", entity: "run", kind: "seal-run", record: "SealedRun" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Seal(RunSealEdit { sealed: true, status: self.status, finished_at: Some(store::now_iso()) })))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![RunMutation::SetRunSeal(SetRunSeal { sealed: base.sealed, status: base.status, finished_at: base.finished_at.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Seal run", "Lauf versiegeln")
    }
    fn target(&self) -> Vec<String> {
        vec!["sealed".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
