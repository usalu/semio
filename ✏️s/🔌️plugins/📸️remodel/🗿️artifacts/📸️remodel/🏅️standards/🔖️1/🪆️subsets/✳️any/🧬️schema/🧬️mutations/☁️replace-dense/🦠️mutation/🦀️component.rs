//! 🔁 Remodel mutation — `ReplaceDense`: whole-value swap of `ReconstructionResults.dense`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, DenseCloud};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-dense` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-dense")]
pub struct ReplaceDense {
    #[serde(default)]
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_dense(dense: Option<DenseCloud>) -> RemodelMutation {
    RemodelMutation::ReplaceDense(ReplaceDense { dense })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceDense {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "dense", kind: "replace-dense", record: "ReplacedDense" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace dense".to_string()
    }
}
//#endregion 🔖️Mutation
