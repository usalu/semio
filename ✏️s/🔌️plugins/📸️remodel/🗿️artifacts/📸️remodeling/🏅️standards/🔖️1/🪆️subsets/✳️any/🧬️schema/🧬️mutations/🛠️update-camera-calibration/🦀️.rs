//! 🛠️ Remodeling mutation — `UpdateCameraCalibration`: full-record replace of an EXISTING camera
//! calibration (the properties form always submits every intrinsics/distortion field together —
//! the `update` verb's inseparable-facet exception, not a scalar `change`).

use crate::artifacts::remodeling::{CameraCalibration, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🛠️ `update-camera-calibration` payload — full FINAL-state `CameraCalibration` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-camera-calibration")]
pub struct UpdateCameraCalibration {
    #[dsl(block)]
    pub camera: CameraCalibration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_camera_calibration(camera: CameraCalibration) -> RemodelingMutation {
    RemodelingMutation::UpdateCameraCalibration(UpdateCameraCalibration { camera })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "camera-calibration", kind: "update-camera-calibration", record: "UpdatedCameraCalibration" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Update camera calibration \"{}\"", self.camera.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.camera.id.clone()]
    }
}
//#endregion 🔖️Mutation
