//! ⛓️ Remodel mutation — `CreateRigExtrinsic`: brings a new camera-id-keyed rig pose into existence.
//! No app call site writes to `calibration.rig` today; schema-complete but unexercised.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, RigExtrinsic};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⛓️ `create-rig-extrinsic` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-rig-extrinsic")]
pub struct CreateRigExtrinsic {
    #[dsl(block)]
    pub extrinsic: RigExtrinsic,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_rig_extrinsic(extrinsic: RigExtrinsic) -> RemodelMutation {
    RemodelMutation::CreateRigExtrinsic(CreateRigExtrinsic { extrinsic })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for CreateRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "rig-extrinsic", kind: "create-rig-extrinsic", record: "CreatedRigExtrinsic" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create rig extrinsic \"{}\"", self.extrinsic.camera_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.extrinsic.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
