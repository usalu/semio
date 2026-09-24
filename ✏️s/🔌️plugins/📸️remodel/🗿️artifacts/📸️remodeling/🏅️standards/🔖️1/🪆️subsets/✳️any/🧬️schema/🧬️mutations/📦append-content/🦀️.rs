//! 📦️ Remodeling mutation — `AppendContent`: appends raw 4 KiB leaves to one durable content entry
//! (`durable_artifacts[content_id]`). A reconstruction run publishes every sparse cloud, mesh and raster
//! it produced through these leaves, and `commit-reconstruction` then names the complete content by id.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{RemodelingContentKind, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📦️ `append-content` payload: `chunks` are base64 raw leaves placed at leaf index `first`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "append-content")]
pub struct AppendContent {
    pub content_id: String,
    pub kind: RemodelingContentKind,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    pub first: u64,
    pub chunks: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn append_content(payload: AppendContent) -> RemodelingMutation {
    RemodelingMutation::AppendContent(payload)
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for AppendContent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "append", entity: "content", kind: "append-content", record: "AppendedContent" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Append {} content leaves to \"{}\"", self.chunks.len(), self.content_id), &format!("{} Inhaltsblätter an \"{}\" anhängen", self.chunks.len(), self.content_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.content_id.clone()]
    }
}
//#endregion 🔖️Mutation
