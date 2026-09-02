//! 🔩 Remodel mutation — `UpdateRigExtrinsic`: full-record replace of an EXISTING rig pose (one
//! rigid pose = `{rotation_wxyz, translation_m}`, inseparable — same `update` reasoning as
//! `update-camera-calibration`).

use crate::artifacts::remodel::{RemodelSnapshot, RigExtrinsic};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
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
