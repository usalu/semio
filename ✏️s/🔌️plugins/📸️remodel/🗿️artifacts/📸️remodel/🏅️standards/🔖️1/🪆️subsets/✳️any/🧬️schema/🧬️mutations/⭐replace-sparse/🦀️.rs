//! 🔁 Remodel mutation — `ReplaceSparse`: whole-value swap of `ReconstructionResults.sparse`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodel::{RemodelSnapshot, SparseCloud};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
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
pub fn replace_sparse(sparse: Option<SparseCloud>) -> RemodelMutation {
    RemodelMutation::ReplaceSparse(ReplaceSparse { sparse })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceSparse {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "sparse", kind: "replace-sparse", record: "ReplacedSparse" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace sparse".to_string()
    }
}
//#endregion 🔖️Mutation
