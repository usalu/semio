//! ⛓️ Remodeling mutation — `CreateRigExtrinsic`: brings a new camera-id-keyed rig pose into existence.
//! No app call site writes to `calibration.rig` today; schema-complete but unexercised.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{RemodelingSnapshot, RigExtrinsic};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

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

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create rig extrinsic \"{}\"", self.extrinsic.camera_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.extrinsic.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
