//! 🔁 Remodel mutation — `ReplaceStreamSource`: whole-value swap of one media stream's `VideoSource`
//! provenance, written wholesale by `ImportVideoDone`.

use crate::artifacts::remodel::{RemodelSnapshot, VideoSource};
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-stream-source` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-stream-source")]
pub struct ReplaceStreamSource {
    pub id: String,
    #[serde(default)]
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_stream_source(id: String, source: Option<VideoSource>) -> RemodelMutation {
    RemodelMutation::ReplaceStreamSource(ReplaceStreamSource { id, source })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceStreamSource {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "stream", kind: "replace-stream-source", record: "ReplacedStreamSource" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace stream \"{}\" source", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
