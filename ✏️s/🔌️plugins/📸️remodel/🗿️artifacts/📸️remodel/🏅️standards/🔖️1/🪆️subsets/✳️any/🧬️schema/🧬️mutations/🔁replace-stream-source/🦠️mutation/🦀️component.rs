//! 🔁 Remodel mutation — `ReplaceStreamSource`: whole-value swap of one media stream's `VideoSource`
//! provenance, written wholesale by `ImportVideoDone`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, VideoSource};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-stream-source` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace stream \"{}\" source", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
