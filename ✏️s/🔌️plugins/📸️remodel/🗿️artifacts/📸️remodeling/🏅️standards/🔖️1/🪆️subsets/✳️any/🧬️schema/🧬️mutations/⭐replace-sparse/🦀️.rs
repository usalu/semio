//! 🔁 Remodeling mutation — `ReplaceSparse`: whole-value swap of `ReconstructionResults.sparse`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{RemodelingSnapshot, SparseCloud};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-sparse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-sparse")]
pub struct ReplaceSparse {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_sparse(sparse: Option<SparseCloud>) -> RemodelingMutation {
    RemodelingMutation::ReplaceSparse(ReplaceSparse { sparse })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceSparse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "sparse", kind: "replace-sparse", record: "ReplacedSparse" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace sparse".to_string()
    }
}
//#endregion 🔖️Mutation
