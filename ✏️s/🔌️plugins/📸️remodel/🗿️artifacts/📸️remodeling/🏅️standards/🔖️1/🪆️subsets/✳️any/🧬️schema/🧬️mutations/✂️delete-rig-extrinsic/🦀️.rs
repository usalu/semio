//! ✂️ Remodeling mutation — `DeleteRigExtrinsic`: removes a camera-id-keyed rig pose.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ✂️ `delete-rig-extrinsic` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-rig-extrinsic")]
pub struct DeleteRigExtrinsic {
    pub camera_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rig_extrinsic(camera_id: String) -> RemodelingMutation {
    RemodelingMutation::DeleteRigExtrinsic(DeleteRigExtrinsic { camera_id })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "rig-extrinsic", kind: "delete-rig-extrinsic", record: "DeletedRigExtrinsic" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete rig extrinsic \"{}\"", self.camera_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
