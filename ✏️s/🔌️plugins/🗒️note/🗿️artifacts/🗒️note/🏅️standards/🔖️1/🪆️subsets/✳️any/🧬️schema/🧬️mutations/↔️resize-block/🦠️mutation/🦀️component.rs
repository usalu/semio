//! ↔️ Note mutation — `ResizeBlock`: changes a block's extent.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `resize-block` payload — changes a block's extent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-block")]
pub struct ResizeBlock {
    pub id: String,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_block(id: String, new_width: f64, new_height: f64) -> NoteMutation {
    NoteMutation::ResizeBlock(ResizeBlock { id, new_width, new_height })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ResizeBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "block", kind: "resize-block", record: "ResizedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize block \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
