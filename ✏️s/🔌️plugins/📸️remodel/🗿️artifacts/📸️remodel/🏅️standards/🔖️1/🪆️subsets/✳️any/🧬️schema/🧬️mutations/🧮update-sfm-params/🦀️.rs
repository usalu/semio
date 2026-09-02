//! ⚙️ Remodel mutation — `UpdateSfmParams`: full-record replace of `ReconstructionParams.sfm` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodel::{RemodelSnapshot, SfmParams};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⚙️ `update-sfm-params` payload — full FINAL-state `SfmParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update sfm params".to_string()
    }
}
//#endregion 🔖️Mutation
