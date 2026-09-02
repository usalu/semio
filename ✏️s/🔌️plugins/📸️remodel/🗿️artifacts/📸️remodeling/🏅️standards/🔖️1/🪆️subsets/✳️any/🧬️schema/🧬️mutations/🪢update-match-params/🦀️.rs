//! ⚙️ Remodeling mutation — `UpdateMatchParams`: full-record replace of `ReconstructionParams.matching` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::artifacts::remodeling::{MatchParams, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⚙️ `update-match-params` payload — full FINAL-state `MatchParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-match-params")]
pub struct UpdateMatchParams {
    #[dsl(block)]
    pub params: MatchParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_match_params(params: MatchParams) -> RemodelingMutation {
    RemodelingMutation::UpdateMatchParams(UpdateMatchParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateMatchParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "matching-params", kind: "update-match-params", record: "UpdatedMatchParams" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update matching params".to_string()
    }
}
//#endregion 🔖️Mutation
