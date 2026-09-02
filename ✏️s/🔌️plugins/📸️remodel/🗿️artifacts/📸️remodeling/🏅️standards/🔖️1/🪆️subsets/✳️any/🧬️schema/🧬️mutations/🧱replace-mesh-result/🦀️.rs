//! 🧱 Remodeling mutation — `ReplaceMeshResult`: whole-value swap of the reconstructed (or
//! placeholder/imported) mesh. Boxed: `RemodelingMesh` is far larger than any sibling payload, and
//! `clippy::large_enum_variant` flags the resulting size disparity across `RemodelingMutation` otherwise.

use crate::artifacts::remodeling::{RemodelingMesh, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧱 `replace-mesh-result` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-mesh-result")]
pub struct ReplaceMeshResult {
    #[dsl(block)]
    pub mesh: Box<RemodelingMesh>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_mesh_result(mesh: Box<RemodelingMesh>) -> RemodelingMutation {
    RemodelingMutation::ReplaceMeshResult(ReplaceMeshResult { mesh })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceMeshResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "mesh-result", kind: "replace-mesh-result", record: "ReplacedMeshResult" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace mesh result".to_string()
    }
}
//#endregion 🔖️Mutation
