//! 🚫 Remodeling mutation — `DeleteCameraCalibration`: removes an id-keyed camera calibration record.
//! No app call site removes a camera today; included for id-keyed collection completeness.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🚫 `delete-camera-calibration` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-camera-calibration")]
pub struct DeleteCameraCalibration {
    pub camera_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_camera_calibration(camera_id: String) -> RemodelingMutation {
    RemodelingMutation::DeleteCameraCalibration(DeleteCameraCalibration { camera_id })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteCameraCalibration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "camera-calibration", kind: "delete-camera-calibration", record: "DeletedCameraCalibration" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
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
