//! ⚙️ Remodel mutation — `UpdateSfmParams`: full-record replace of `ReconstructionParams.sfm` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{SfmParams, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-sfm-params` payload — full FINAL-state `SfmParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-sfm-params")]
pub struct UpdateSfmParams {
    #[dsl(block)]
    pub params: SfmParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_sfm_params(params: SfmParams) -> RemodelMutation {
    RemodelMutation::UpdateSfmParams(UpdateSfmParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateSfmParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "sfm-params", kind: "update-sfm-params", record: "UpdatedSfmParams" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update sfm params".to_string()
    }
}
//#endregion 🔖️Mutation
