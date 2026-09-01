//! ⚙️ Remodel mutation — `UpdateMotionParams`: full-record replace of `ReconstructionParams.motion` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodel::{MotionParams, RemodelSnapshot};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-motion-params` payload — full FINAL-state `MotionParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-motion-params")]
pub struct UpdateMotionParams {
    #[dsl(block)]
    pub params: MotionParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_motion_params(params: MotionParams) -> RemodelMutation {
    RemodelMutation::UpdateMotionParams(UpdateMotionParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateMotionParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "motion-params", kind: "update-motion-params", record: "UpdatedMotionParams" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update motion params".to_string()
    }
}
//#endregion 🔖️Mutation
