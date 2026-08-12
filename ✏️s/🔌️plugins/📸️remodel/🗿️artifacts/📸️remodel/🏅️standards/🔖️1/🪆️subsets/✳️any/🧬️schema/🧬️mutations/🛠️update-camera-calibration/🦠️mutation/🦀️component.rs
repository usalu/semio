//! 🛠️ Remodel mutation — `UpdateCameraCalibration`: full-record replace of an EXISTING camera
//! calibration (the properties form always submits every intrinsics/distortion field together —
//! the `update` verb's inseparable-facet exception, not a scalar `change`).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🛠️ `update-camera-calibration` payload — full FINAL-state `CameraCalibration` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-camera-calibration")]
pub struct UpdateCameraCalibration {
    #[dsl(block)]
    pub camera: CameraCalibration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_camera_calibration(camera: CameraCalibration) -> RemodelMutation {
    RemodelMutation::UpdateCameraCalibration(UpdateCameraCalibration { camera })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "camera-calibration", kind: "update-camera-calibration", record: "UpdatedCameraCalibration" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update camera calibration \"{}\"", self.camera.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.camera.id.clone()]
    }
}
//#endregion 🔖️Mutation
