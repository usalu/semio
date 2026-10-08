use super::super::{RunArtifact, RunDiff, RunHeaderEdit, RunMutation, RunParameterValue, RunStatus, RunStep, RunTrigger};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🧷️ Sets the absolute run header — identity, trigger, lifecycle status and start time — and is the inverse of `start-run`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-run-header")]
pub struct SetRunHeader {
    pub workflow_ref: String,
    pub workflow_checkpoint_id: String,
    pub input_collection_ref: String,
    pub input_snapshot_id: String,
    #[dsl(table)]
    pub parameter_values: Vec<RunParameterValue>,
    pub output_collection_ref: String,
    #[dsl(block)]
    pub trigger: RunTrigger,
    pub status: RunStatus,
    pub started_at: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for SetRunHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-header", kind: "set-run-header", record: "SetRunHeader" };
    fn diff(&self, base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        let header = RunHeaderEdit {
            workflow_ref: self.workflow_ref.clone(),
            workflow_checkpoint_id: self.workflow_checkpoint_id.clone(),
            input_collection_ref: self.input_collection_ref.clone(),
            input_snapshot_id: self.input_snapshot_id.clone(),
            parameter_values: self.parameter_values.clone(),
            output_collection_ref: self.output_collection_ref.clone(),
            trigger: self.trigger.clone(),
            status: self.status,
            started_at: self.started_at.clone(),
        };
        if header == RunHeaderEdit::of(base) {
            return protocol::MutationOutcome::empty();
        }
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Header(header)))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![RunMutation::SetRunHeader(Self {
            workflow_ref: base.workflow_ref.clone(),
            workflow_checkpoint_id: base.workflow_checkpoint_id.clone(),
            input_collection_ref: base.input_collection_ref.clone(),
            input_snapshot_id: base.input_snapshot_id.clone(),
            parameter_values: base.parameter_values.clone(),
            output_collection_ref: base.output_collection_ref.clone(),
            trigger: base.trigger.clone(),
            status: base.status,
            started_at: base.started_at.clone(),
        })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set run header for {}", self.workflow_ref), &format!("Laufkopf für {} setzen", self.workflow_ref))
    }
    fn target(&self) -> Vec<String> {
        vec!["run".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
