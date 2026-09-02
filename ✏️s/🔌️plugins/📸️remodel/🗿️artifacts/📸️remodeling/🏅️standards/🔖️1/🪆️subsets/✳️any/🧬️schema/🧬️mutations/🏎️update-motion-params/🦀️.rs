//! ⚙️ Remodeling mutation — `UpdateMotionParams`: full-record replace of `ReconstructionParams.motion` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodeling::{MotionParams, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⚙️ `update-motion-params` payload — full FINAL-state `MotionParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-motion-params")]
pub struct UpdateMotionParams {
    #[dsl(block)]
    pub params: MotionParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_motion_params(params: MotionParams) -> RemodelingMutation {
    RemodelingMutation::UpdateMotionParams(UpdateMotionParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateMotionParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "motion-params", kind: "update-motion-params", record: "UpdatedMotionParams" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update motion params".to_string()
    }
}
//#endregion 🔖️Mutation
