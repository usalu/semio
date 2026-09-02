//! 🔩 Remodeling mutation — `UpdateRigExtrinsic`: full-record replace of an EXISTING rig pose (one
//! rigid pose = `{rotation_wxyz, translation_m}`, inseparable — same `update` reasoning as
//! `update-camera-calibration`).

use crate::artifacts::remodeling::{RemodelingSnapshot, RigExtrinsic};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔩 `update-rig-extrinsic` payload — full FINAL-state `RigExtrinsic` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Update rig extrinsic \"{}\"", self.extrinsic.camera_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.extrinsic.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
