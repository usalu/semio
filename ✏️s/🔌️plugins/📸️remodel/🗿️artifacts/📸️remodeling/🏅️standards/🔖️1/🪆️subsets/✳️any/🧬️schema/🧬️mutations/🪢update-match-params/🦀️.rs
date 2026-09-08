//! ⚙️ Remodeling mutation — `UpdateMatchParams`: full-record replace of `ReconstructionParams.matching` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{MatchParams, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

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

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update matching params".to_string()
    }
}
//#endregion 🔖️Mutation
