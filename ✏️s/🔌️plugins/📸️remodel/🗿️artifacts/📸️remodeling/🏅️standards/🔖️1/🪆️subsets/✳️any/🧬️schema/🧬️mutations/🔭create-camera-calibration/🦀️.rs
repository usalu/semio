//! 🔭 Remodeling mutation — `CreateCameraCalibration`: brings a new id-keyed camera calibration record
//! into existence (used when `EditCalibration`/`CalibrateCameras` targets a camera id not yet present).

use crate::artifacts::remodeling::{CameraCalibration, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔭 `create-camera-calibration` payload — full initial `CameraCalibration` record (the properties
/// form always submits every field together — same `update` reasoning applies to creation here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-camera-calibration")]
pub struct CreateCameraCalibration {
    #[dsl(block)]
    pub camera: CameraCalibration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_camera_calibration(camera: CameraCalibration) -> RemodelingMutation {
    RemodelingMutation::CreateCameraCalibration(CreateCameraCalibration { camera })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for CreateCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "camera-calibration", kind: "create-camera-calibration", record: "CreatedCameraCalibration" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create camera calibration \"{}\"", self.camera.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.camera.id.clone()]
    }
}
//#endregion 🔖️Mutation
