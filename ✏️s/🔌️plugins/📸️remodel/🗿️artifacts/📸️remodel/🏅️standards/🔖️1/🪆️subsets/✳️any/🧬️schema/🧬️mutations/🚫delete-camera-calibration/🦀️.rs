//! 🚫 Remodel mutation — `DeleteCameraCalibration`: removes an id-keyed camera calibration record.
//! No app call site removes a camera today; included for id-keyed collection completeness.

use crate::artifacts::remodel::RemodelSnapshot;
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚫 `delete-camera-calibration` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-camera-calibration")]
pub struct DeleteCameraCalibration {
    pub camera_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_camera_calibration(camera_id: String) -> RemodelMutation {
    RemodelMutation::DeleteCameraCalibration(DeleteCameraCalibration { camera_id })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for DeleteCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "camera-calibration", kind: "delete-camera-calibration", record: "DeletedCameraCalibration" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete camera calibration \"{}\"", self.camera_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
