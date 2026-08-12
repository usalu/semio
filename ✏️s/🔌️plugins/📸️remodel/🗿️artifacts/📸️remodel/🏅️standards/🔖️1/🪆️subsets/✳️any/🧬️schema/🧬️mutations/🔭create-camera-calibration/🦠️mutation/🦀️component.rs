//! 🔭 Remodel mutation — `CreateCameraCalibration`: brings a new id-keyed camera calibration record
//! into existence (used when `EditCalibration`/`CalibrateCameras` targets a camera id not yet present).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔭 `create-camera-calibration` payload — full initial `CameraCalibration` record (the properties
/// form always submits every field together — same `update` reasoning applies to creation here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-camera-calibration")]
pub struct CreateCameraCalibration {
    #[dsl(block)]
    pub camera: CameraCalibration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_camera_calibration(camera: CameraCalibration) -> RemodelMutation {
    RemodelMutation::CreateCameraCalibration(CreateCameraCalibration { camera })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for CreateCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "camera-calibration", kind: "create-camera-calibration", record: "CreatedCameraCalibration" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create camera calibration \"{}\"", self.camera.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.camera.id.clone()]
    }
}
//#endregion 🔖️Mutation
