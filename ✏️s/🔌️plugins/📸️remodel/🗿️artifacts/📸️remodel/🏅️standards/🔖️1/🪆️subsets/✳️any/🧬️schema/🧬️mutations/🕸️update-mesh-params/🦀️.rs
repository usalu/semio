//! ⚙️ Remodel mutation — `UpdateMeshParams`: full-record replace of `ReconstructionParams.mesh` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodel::{MeshParams, RemodelSnapshot};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
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
pub fn update_mesh_params(params: MeshParams) -> RemodelMutation {
    RemodelMutation::UpdateMeshParams(UpdateMeshParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateMeshParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "mesh-params", kind: "update-mesh-params", record: "UpdatedMeshParams" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update mesh params".to_string()
    }
}
//#endregion 🔖️Mutation
