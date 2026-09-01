use super::super::{RunArtifact, RunDiff, RunMutation, RunStatus};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "seal-run")]
pub struct SealRun { pub status: RunStatus }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<RunArtifact, RunMutation> for SealRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "seal", entity: "run", kind: "seal-run", record: "SealedRun" };
    fn diff(&self, _base: &RunArtifact) -> protocol::MutationOutcome<RunDiff> { protocol::MutationOutcome::new(RunDiff::Seal { status: self.status }) }
    fn inverse(&self, _base: &RunArtifact) -> Vec<RunMutation> { Vec::new() }
    fn label(&self) -> String { "Seal run".into() }
    fn target(&self) -> Vec<String> { vec!["sealed".into()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_seal_identity() { assert_eq!(<SealRun as MutationLeaf>::DESCRIPTOR.semantic_kind, "seal-run"); }
}
