//! ⏱️ Remodeling mutation — `ChangeStreamSync`: sets one media stream's `sync_offset_ms`.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⏱️ `change-stream-sync` payload — FINAL-state `sync_offset_ms`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-stream-sync")]
pub struct ChangeStreamSync {
    pub id: String,
    pub new_sync_offset_ms: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_stream_sync(id: String, new_sync_offset_ms: f64) -> RemodelingMutation {
    RemodelingMutation::ChangeStreamSync(ChangeStreamSync { id, new_sync_offset_ms })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ChangeStreamSync {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stream", kind: "change-stream-sync", record: "ChangedStreamSync" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change stream \"{}\" sync offset to {}ms", self.id, self.new_sync_offset_ms)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
