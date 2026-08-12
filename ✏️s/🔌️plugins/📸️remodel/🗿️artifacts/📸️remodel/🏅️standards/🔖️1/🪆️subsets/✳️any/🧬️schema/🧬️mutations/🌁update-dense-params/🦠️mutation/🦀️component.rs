//! ⚙️ Remodel mutation — `UpdateDenseParams`: full-record replace of `ReconstructionParams.dense` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{DenseParams, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-dense-params` payload — full FINAL-state `DenseParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-dense-params")]
pub struct UpdateDenseParams {
    #[dsl(block)]
    pub params: DenseParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_dense_params(params: DenseParams) -> RemodelMutation {
    RemodelMutation::UpdateDenseParams(UpdateDenseParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateDenseParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "dense-params", kind: "update-dense-params", record: "UpdatedDenseParams" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update dense params".to_string()
    }
}
//#endregion 🔖️Mutation
