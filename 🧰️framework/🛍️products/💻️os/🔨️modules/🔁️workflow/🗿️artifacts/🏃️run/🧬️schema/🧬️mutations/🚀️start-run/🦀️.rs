use super::super::{RunArtifact, RunDiff, RunMutation, RunParameterValue, RunTrigger};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> { protocol::MutationOutcome::new(RunDiff::Start { workflow_ref: self.workflow_ref.clone(), workflow_checkpoint_id: self.workflow_checkpoint_id.clone(), input_collection_ref: self.input_collection_ref.clone(), input_snapshot_id: self.input_snapshot_id.clone(), parameter_values: self.parameter_values.clone(), output_collection_ref: self.output_collection_ref.clone(), trigger: self.trigger.clone() }) }
    fn inverse(&self, _base: &RunArtifact) -> Vec<RunMutation> { Vec::new() }
    fn label(&self) -> String { format!("Start run for {}", self.workflow_ref) }
    fn target(&self) -> Vec<String> { vec!["run".into()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_start_identity() { assert_eq!(<StartRun as MutationLeaf>::DESCRIPTOR.semantic_kind, "start-run"); }
}
