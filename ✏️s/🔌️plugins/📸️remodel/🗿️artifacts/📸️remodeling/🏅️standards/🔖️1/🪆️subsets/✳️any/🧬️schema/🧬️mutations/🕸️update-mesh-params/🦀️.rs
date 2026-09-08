//! ⚙️ Remodeling mutation — `UpdateMeshParams`: full-record replace of `ReconstructionParams.mesh` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{MeshParams, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

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

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update mesh params".to_string()
    }
}
//#endregion 🔖️Mutation
