use super::super::{RunArtifact, RunDiff, RunHeaderEdit, RunMutation, RunParameterValue, RunStatus, RunStep, RunTrigger, SetRunHeader};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "start-run")]
pub struct StartRun {
    pub workflow_ref: String,
    pub workflow_checkpoint_id: String,
    pub input_collection_ref: String,
    pub input_snapshot_id: String,
    #[dsl(table)]
    pub parameter_values: Vec<RunParameterValue>,
    pub output_collection_ref: String,
    #[dsl(block)]
    pub trigger: RunTrigger,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for StartRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "start", entity: "run", kind: "start-run", record: "StartedRun" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        protocol::MutationOutcome::new(RunDiff::step(RunStep::Header(RunHeaderEdit {
            workflow_ref: self.workflow_ref.clone(),
            workflow_checkpoint_id: self.workflow_checkpoint_id.clone(),
            input_collection_ref: self.input_collection_ref.clone(),
            input_snapshot_id: self.input_snapshot_id.clone(),
            parameter_values: self.parameter_values.clone(),
            output_collection_ref: self.output_collection_ref.clone(),
            trigger: self.trigger.clone(),
            status: RunStatus::Running,
            started_at: store::now_iso(),
        })))
    }
    fn inverse(&self, base: &RunArtifact) -> Result<Vec<RunMutation>, semio_framework_value::ValueError> {
        Ok(vec![RunMutation::SetRunHeader(SetRunHeader {
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
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Start run for {}", self.workflow_ref), &format!("Lauf für {} starten", self.workflow_ref))
    }
    fn target(&self) -> Vec<String> {
        vec!["run".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
