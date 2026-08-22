//! 🔩 Remodel mutation — `UpdateRigExtrinsic`: full-record replace of an EXISTING rig pose (one
//! rigid pose = `{rotation_wxyz, translation_m}`, inseparable — same `update` reasoning as
//! `update-camera-calibration`).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, RigExtrinsic};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔩 `update-rig-extrinsic` payload — full FINAL-state `RigExtrinsic` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-rig-extrinsic")]
pub struct UpdateRigExtrinsic {
    #[dsl(block)]
    pub extrinsic: RigExtrinsic,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_rig_extrinsic(extrinsic: RigExtrinsic) -> RemodelMutation {
    RemodelMutation::UpdateRigExtrinsic(UpdateRigExtrinsic { extrinsic })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "rig-extrinsic", kind: "update-rig-extrinsic", record: "UpdatedRigExtrinsic" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
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
