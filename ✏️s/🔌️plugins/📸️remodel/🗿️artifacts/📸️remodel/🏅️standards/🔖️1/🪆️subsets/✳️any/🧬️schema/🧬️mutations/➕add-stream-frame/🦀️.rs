//! ➕ Remodel mutation — `AddStreamFrame`: appends one `FrameRef` to an EXISTING media stream and
//! sets its `kind` (image import handlers append to a stream that already exists; creating a brand
//! new stream instead goes through `create-stream`).

use crate::artifacts::remodel::{FrameRef, MediaKind, RemodelSnapshot};
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-stream-frame` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-stream-frame")]
pub struct AddStreamFrame {
    pub id: String,
    #[dsl(block)]
    pub frame: FrameRef,
    pub kind: MediaKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_stream_frame(id: String, frame: FrameRef, kind: MediaKind) -> RemodelMutation {
    RemodelMutation::AddStreamFrame(AddStreamFrame { id, frame, kind })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for AddStreamFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "stream", kind: "add-stream-frame", record: "AddedStreamFrame" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add frame {} to stream \"{}\"", self.frame.index, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
