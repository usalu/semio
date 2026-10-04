//! 🔩 Remodeling mutation — `UpdateRigExtrinsic`: full-record replace of an EXISTING rig pose (one
//! rigid pose = `{rotation_wxyz, translation_m}`, inseparable — same `update` reasoning as
//! `update-camera-calibration`).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{RemodelingSnapshot, RigExtrinsic};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔩 `update-rig-extrinsic` payload — full FINAL-state `RigExtrinsic` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-rig-extrinsic")]
pub struct UpdateRigExtrinsic {
    #[dsl(block)]
    pub extrinsic: RigExtrinsic,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_rig_extrinsic(extrinsic: RigExtrinsic) -> RemodelingMutation {
    RemodelingMutation::UpdateRigExtrinsic(UpdateRigExtrinsic { extrinsic })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "rig-extrinsic", kind: "update-rig-extrinsic", record: "UpdatedRigExtrinsic" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update rig extrinsic \"{}\"", self.extrinsic.camera_id), &format!("Extrinsische Rig-Kalibrierung \"{}\" aktualisieren", self.extrinsic.camera_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.extrinsic.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
