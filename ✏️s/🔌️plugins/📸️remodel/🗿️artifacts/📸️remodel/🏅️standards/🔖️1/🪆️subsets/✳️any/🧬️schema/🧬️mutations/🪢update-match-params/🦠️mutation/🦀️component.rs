//! ⚙️ Remodel mutation — `UpdateMatchParams`: full-record replace of `ReconstructionParams.matching` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{MatchParams, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-match-params` payload — full FINAL-state `MatchParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-match-params")]
pub struct UpdateMatchParams {
    #[dsl(block)]
    pub params: MatchParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_match_params(params: MatchParams) -> RemodelMutation {
    RemodelMutation::UpdateMatchParams(UpdateMatchParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateMatchParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "matching-params", kind: "update-match-params", record: "UpdatedMatchParams" };

    fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update matching params".to_string()
    }
}
//#endregion 🔖️Mutation
