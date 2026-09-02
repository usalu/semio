//! 🔁 Remodeling mutation — `ReplaceQc`: whole-value swap of `ReconstructionResults.qc`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{QcReportSnapshot, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-qc` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-qc")]
pub struct ReplaceQc {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_qc(qc: Option<QcReportSnapshot>) -> RemodelingMutation {
    RemodelingMutation::ReplaceQc(ReplaceQc { qc })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceQc {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "qc", kind: "replace-qc", record: "ReplacedQc" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace qc".to_string()
    }
}
//#endregion 🔖️Mutation
