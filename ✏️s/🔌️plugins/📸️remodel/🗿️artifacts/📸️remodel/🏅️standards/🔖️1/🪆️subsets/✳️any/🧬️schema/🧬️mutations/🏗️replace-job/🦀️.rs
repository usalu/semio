//! 🏗️ Remodel mutation — `ReplaceJob`: whole-value swap of the engine-owned live-run-state blob,
//! rewritten wholesale by the reconstruction engine, never field-edited by a user.

use crate::artifacts::remodel::{ReconstructionJob, RemodelSnapshot};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏗️ `replace-job` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-job")]
pub struct ReplaceJob {
    #[dsl(block)]
    pub job: ReconstructionJob,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_job(job: ReconstructionJob) -> RemodelMutation {
    RemodelMutation::ReplaceJob(ReplaceJob { job })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceJob {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "job", kind: "replace-job", record: "ReplacedJob" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace reconstruction job".to_string()
    }
}
//#endregion 🔖️Mutation
