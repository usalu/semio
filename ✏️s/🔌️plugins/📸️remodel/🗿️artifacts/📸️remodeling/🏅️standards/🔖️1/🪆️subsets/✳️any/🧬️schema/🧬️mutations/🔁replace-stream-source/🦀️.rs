//! 🔁 Remodeling mutation — `ReplaceStreamSource`: whole-value swap of one media stream's `VideoSource`
//! provenance, written wholesale by `ImportVideoDone`.

use crate::artifacts::remodeling::{RemodelingSnapshot, VideoSource};
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-stream-source` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-stream-source")]
pub struct ReplaceStreamSource {
    pub id: String,
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_stream_source(id: String, source: Option<VideoSource>) -> RemodelingMutation {
    RemodelingMutation::ReplaceStreamSource(ReplaceStreamSource { id, source })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceStreamSource {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "stream", kind: "replace-stream-source", record: "ReplacedStreamSource" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
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
