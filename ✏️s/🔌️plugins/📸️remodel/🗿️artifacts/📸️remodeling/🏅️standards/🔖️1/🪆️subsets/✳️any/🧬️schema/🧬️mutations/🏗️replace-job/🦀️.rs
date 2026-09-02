//! 🏗️ Remodeling mutation — `ReplaceJob`: whole-value swap of the engine-owned live-run-state blob,
//! rewritten wholesale by the reconstruction engine, never field-edited by a user.

use crate::artifacts::remodeling::{ReconstructionJob, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🏗️ `replace-job` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-job")]
pub struct ReplaceJob {
    #[dsl(block)]
    pub job: ReconstructionJob,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_job(job: ReconstructionJob) -> RemodelingMutation {
    RemodelingMutation::ReplaceJob(ReplaceJob { job })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceJob {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "job", kind: "replace-job", record: "ReplacedJob" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace reconstruction job".to_string()
    }
}
//#endregion 🔖️Mutation
