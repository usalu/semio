//! 🧱 Remodel mutation — `ReplaceMeshResult`: whole-value swap of the reconstructed (or
//! placeholder/imported) mesh. Boxed: `RemodelMesh` is far larger than any sibling payload, and
//! `clippy::large_enum_variant` flags the resulting size disparity across `RemodelMutation` otherwise.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelMesh, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧱 `replace-mesh-result` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-mesh-result")]
pub struct ReplaceMeshResult {
    #[dsl(block)]
    pub mesh: Box<RemodelMesh>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_mesh_result(mesh: Box<RemodelMesh>) -> RemodelMutation {
    RemodelMutation::ReplaceMeshResult(ReplaceMeshResult { mesh })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceMeshResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "mesh-result", kind: "replace-mesh-result", record: "ReplacedMeshResult" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace mesh result".to_string()
    }
}
//#endregion 🔖️Mutation
