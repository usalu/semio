use super::super::{RunArtifact, RunDiff, RunMutation};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "append-run-log")]
pub struct AppendRunLog { pub node_id: String, pub level: String, pub message: String, pub at: String }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for AppendRunLog {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "append", entity: "run-log", kind: "append-run-log", record: "AppendedRunLog" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> { protocol::MutationOutcome::new(RunDiff::Log { node_id: self.node_id.clone(), level: self.level.clone(), message: self.message.clone(), at: self.at.clone() }) }
    fn inverse(&self, _base: &RunArtifact) -> Vec<RunMutation> { Vec::new() }
    fn label(&self) -> String { format!("Append run log for {}", self.node_id) }
    fn target(&self) -> Vec<String> { vec!["logs".into(), self.node_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_append_identity() { assert_eq!(<AppendRunLog as MutationLeaf>::DESCRIPTOR.semantic_kind, "append-run-log"); }
}
