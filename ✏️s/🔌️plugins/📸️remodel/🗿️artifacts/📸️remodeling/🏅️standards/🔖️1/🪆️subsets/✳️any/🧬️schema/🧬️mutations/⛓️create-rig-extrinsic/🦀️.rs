//! ⛓️ Remodeling mutation — `CreateRigExtrinsic`: brings a new camera-id-keyed rig pose into existence.
//! No app call site writes to `calibration.rig` today; schema-complete but unexercised.

use crate::artifacts::remodeling::{RemodelingSnapshot, RigExtrinsic};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⛓️ `create-rig-extrinsic` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-rig-extrinsic")]
pub struct CreateRigExtrinsic {
    #[dsl(block)]
    pub extrinsic: RigExtrinsic,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rig_extrinsic(extrinsic: RigExtrinsic) -> RemodelingMutation {
    RemodelingMutation::CreateRigExtrinsic(CreateRigExtrinsic { extrinsic })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for CreateRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "rig-extrinsic", kind: "create-rig-extrinsic", record: "CreatedRigExtrinsic" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
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
