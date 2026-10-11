use super::super::{RunArtifact, RunDiff, RunMutation, RunSealEdit, RunStatus, RunStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🔓️ Sets the absolute seal state — sealed flag, final status and finish time — and is the inverse of `seal-run`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-run-seal")]
pub struct SetRunSeal {
    pub sealed: bool,
    pub status: RunStatus,
    pub finished_at: Option<String>,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for SetRunSeal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-seal", kind: "set-run-seal", record: "SetRunSeal" };
    fn diff(&self, base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        let seal = RunSealEdit { sealed: self.sealed, status: self.status, finished_at: self.finished_at.clone() };
        if seal == RunSealEdit::of(base) {
            return protocol::MutationOutcome::empty();
        }
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Seal(seal)))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![RunMutation::SetRunSeal(Self { sealed: base.sealed, status: base.status, finished_at: base.finished_at.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.sealed {
            true => semio_framework_ui_locale::LocalizedLabel::native("Seal run", "Lauf versiegeln"),
            false => semio_framework_ui_locale::LocalizedLabel::native("Reopen run", "Lauf wieder öffnen"),
        }
    }
    fn target(&self) -> Vec<String> {
        vec!["sealed".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
