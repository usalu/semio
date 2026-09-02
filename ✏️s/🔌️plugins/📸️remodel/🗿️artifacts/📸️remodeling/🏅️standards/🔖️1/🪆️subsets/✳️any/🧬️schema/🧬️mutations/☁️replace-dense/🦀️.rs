//! 🔁 Remodeling mutation — `ReplaceDense`: whole-value swap of `ReconstructionResults.dense`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{DenseCloud, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-dense` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-dense")]
pub struct ReplaceDense {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_dense(dense: Option<DenseCloud>) -> RemodelingMutation {
    RemodelingMutation::ReplaceDense(ReplaceDense { dense })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceDense {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "dense", kind: "replace-dense", record: "ReplacedDense" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace dense".to_string()
    }
}
//#endregion 🔖️Mutation
