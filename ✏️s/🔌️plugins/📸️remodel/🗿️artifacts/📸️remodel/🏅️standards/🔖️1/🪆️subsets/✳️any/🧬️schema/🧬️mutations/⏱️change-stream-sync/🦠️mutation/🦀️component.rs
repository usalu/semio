//! ⏱️ Remodel mutation — `ChangeStreamSync`: sets one media stream's `sync_offset_ms`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⏱️ `change-stream-sync` payload — FINAL-state `sync_offset_ms`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-stream-sync")]
pub struct ChangeStreamSync {
    pub id: String,
    pub new_sync_offset_ms: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_stream_sync(id: String, new_sync_offset_ms: f64) -> RemodelMutation {
    RemodelMutation::ChangeStreamSync(ChangeStreamSync { id, new_sync_offset_ms })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ChangeStreamSync {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stream", kind: "change-stream-sync", record: "ChangedStreamSync" };

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stream \"{}\" sync offset to {}ms", self.id, self.new_sync_offset_ms)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
