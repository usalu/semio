//! ⚙️ Remodeling mutation — `UpdateDenseParams`: full-record replace of `ReconstructionParams.dense` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{DenseParams, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-dense-params` payload — full FINAL-state `DenseParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-dense-params")]
pub struct UpdateDenseParams {
    #[dsl(block)]
    pub params: DenseParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_dense_params(params: DenseParams) -> RemodelingMutation {
    RemodelingMutation::UpdateDenseParams(UpdateDenseParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateDenseParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "dense-params", kind: "update-dense-params", record: "UpdatedDenseParams" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update dense params".to_string()
    }
}
//#endregion 🔖️Mutation
