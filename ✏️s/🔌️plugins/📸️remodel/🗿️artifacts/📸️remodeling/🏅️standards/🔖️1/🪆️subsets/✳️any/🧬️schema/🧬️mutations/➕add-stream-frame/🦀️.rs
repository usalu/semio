//! ➕ Remodeling mutation — `AddStreamFrame`: appends one `FrameRef` to an EXISTING media stream and
//! sets its `kind` (image import handlers append to a stream that already exists; creating a brand
//! new stream instead goes through `create-stream`).

use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{FrameRef, MediaKind, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-stream-frame` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-stream-frame")]
pub struct AddStreamFrame {
    pub id: String,
    #[dsl(block)]
    pub frame: FrameRef,
    pub kind: MediaKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_stream_frame(id: String, frame: FrameRef, kind: MediaKind) -> RemodelingMutation {
    RemodelingMutation::AddStreamFrame(AddStreamFrame { id, frame, kind })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for AddStreamFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "stream", kind: "add-stream-frame", record: "AddedStreamFrame" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add frame {} to stream \"{}\"", self.frame.index, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
