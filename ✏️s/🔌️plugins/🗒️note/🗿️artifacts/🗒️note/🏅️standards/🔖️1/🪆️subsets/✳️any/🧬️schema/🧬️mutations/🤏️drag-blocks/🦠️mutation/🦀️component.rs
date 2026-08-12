//! 🤏 Note mutation — `DragBlocks`: offsets several blocks by the same relative amount (multi-select drag/nudge).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🤏 `drag-blocks` payload — offsets several blocks by the same relative amount (multi-select drag/nudge).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "drag-blocks")]
pub struct DragBlocks {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_blocks(ids: Vec<String>, dx: f64, dy: f64) -> NoteMutation {
    NoteMutation::DragBlocks(DragBlocks { ids, dx, dy })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DragBlocks {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "blocks", kind: "drag-blocks", record: "DraggedBlocks" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Drag {} blocks", self.ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Mutation
