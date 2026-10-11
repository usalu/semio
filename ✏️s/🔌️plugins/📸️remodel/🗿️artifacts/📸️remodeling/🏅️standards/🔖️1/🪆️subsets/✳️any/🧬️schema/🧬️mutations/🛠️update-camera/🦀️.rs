//! 🛠️ Remodeling mutation — `UpdateCameraCalibration`: full-record replace of an EXISTING camera
//! calibration (the properties form always submits every intrinsics/distortion field together —
//! the `update` verb's inseparable-facet exception, not a scalar `change`).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{CameraCalibration, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🛠️ `update-camera-calibration` payload — full FINAL-state `CameraCalibration` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update camera calibration \"{}\"", self.camera.id), &format!("Kamerakalibrierung \"{}\" aktualisieren", self.camera.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.camera.id.clone()]
    }
}
//#endregion 🔖️Mutation
