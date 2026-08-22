//! ➖ Remodel mutation — `RemoveStreamFrame`: removes one `FrameRef` from a media stream by its
//! BASE-state index. Internal counterpart to `add-stream-frame` (its own inverse): `add-stream-frame`
//! always APPENDS, so `remove-stream-frame` only round-trips exactly when `frame_index` addresses the
//! LAST frame — the only way this mutation is ever emitted (see this facet's report for the reasoning).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖ `remove-stream-frame` payload — `frame_index` is a BASE-state position in `frames`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-stream-frame")]
pub struct RemoveStreamFrame {
    pub id: String,
    pub frame_index: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_stream_frame(id: String, frame_index: u32) -> RemodelMutation {
    RemodelMutation::RemoveStreamFrame(RemoveStreamFrame { id, frame_index })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for RemoveStreamFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "stream", kind: "remove-stream-frame", record: "RemovedStreamFrame" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove frame {} from stream \"{}\"", self.frame_index, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
