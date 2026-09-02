//! ⚙️ Remodeling mutation — `UpdateMeshParams`: full-record replace of `ReconstructionParams.mesh` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodeling::{MeshParams, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⚙️ `update-mesh-params` payload — full FINAL-state `MeshParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-mesh-params")]
pub struct UpdateMeshParams {
    #[dsl(block)]
    pub params: MeshParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_mesh_params(params: MeshParams) -> RemodelingMutation {
    RemodelingMutation::UpdateMeshParams(UpdateMeshParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateMeshParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "mesh-params", kind: "update-mesh-params", record: "UpdatedMeshParams" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update mesh params".to_string()
    }
}
//#endregion 🔖️Mutation
