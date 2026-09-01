//! ⚙️ Remodel mutation — `UpdateFeatureParams`: full-record replace of `ReconstructionParams.feature` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodel::{FeatureParams, RemodelSnapshot};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-feature-params` payload — full FINAL-state `FeatureParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-feature-params")]
pub struct UpdateFeatureParams {
    #[dsl(block)]
    pub params: FeatureParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_feature_params(params: FeatureParams) -> RemodelMutation {
    RemodelMutation::UpdateFeatureParams(UpdateFeatureParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateFeatureParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "feature-params", kind: "update-feature-params", record: "UpdatedFeatureParams" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update feature params".to_string()
    }
}
//#endregion 🔖️Mutation
