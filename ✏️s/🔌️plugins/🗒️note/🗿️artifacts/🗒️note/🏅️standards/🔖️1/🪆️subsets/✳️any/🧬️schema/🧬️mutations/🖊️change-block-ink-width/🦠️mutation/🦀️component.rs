//! 🖊️ Note mutation — `ChangeBlockInkWidth`: sets an ink block's stroke width.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖊️ `change-block-ink-width` payload — sets an ink block's stroke width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-ink-width")]
pub struct ChangeBlockInkWidth {
    pub id: String,
    pub new_stroke_width: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_block_ink_width(id: String, new_stroke_width: f64) -> NoteMutation {
    NoteMutation::ChangeBlockInkWidth(ChangeBlockInkWidth { id, new_stroke_width })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockInkWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-ink-width", kind: "change-block-ink-width", record: "ChangedBlockInkWidth" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change block \"{}\" ink width to {}", self.id, self.new_stroke_width)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
