//! ✂️ Remodel mutation — `DeleteRigExtrinsic`: removes a camera-id-keyed rig pose.

use crate::artifacts::remodel::RemodelSnapshot;
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `delete-rig-extrinsic` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-rig-extrinsic")]
pub struct DeleteRigExtrinsic {
    pub camera_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rig_extrinsic(camera_id: String) -> RemodelMutation {
    RemodelMutation::DeleteRigExtrinsic(DeleteRigExtrinsic { camera_id })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for DeleteRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "rig-extrinsic", kind: "delete-rig-extrinsic", record: "DeletedRigExtrinsic" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
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
