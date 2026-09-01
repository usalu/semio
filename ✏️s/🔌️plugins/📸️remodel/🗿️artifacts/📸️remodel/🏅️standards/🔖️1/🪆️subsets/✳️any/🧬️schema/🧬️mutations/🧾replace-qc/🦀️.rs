//! 🔁 Remodel mutation — `ReplaceQc`: whole-value swap of `ReconstructionResults.qc`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodel::{QcReportSnapshot, RemodelSnapshot};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-qc` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-qc")]
pub struct ReplaceQc {
    #[serde(default)]
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_qc(qc: Option<QcReportSnapshot>) -> RemodelMutation {
    RemodelMutation::ReplaceQc(ReplaceQc { qc })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceQc {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "qc", kind: "replace-qc", record: "ReplacedQc" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace qc".to_string()
    }
}
//#endregion 🔖️Mutation
