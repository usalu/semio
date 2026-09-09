//! 🎨 Note mutation — `EditBlockInkStroke`: replaces an ink block's authored stroke geometry (points + bounding box, drawn atomically).

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎨 `edit-block-ink-stroke` payload — replaces an ink block's authored stroke geometry (points + bounding box, drawn atomically).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-block-ink-stroke")]
pub struct EditBlockInkStroke {
    pub id: String,
    pub new_points: Vec<[f64; 2]>,
    pub new_x: f64,
    pub new_y: f64,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_block_ink_stroke(id: String, new_points: Vec<[f64; 2]>, new_x: f64, new_y: f64, new_width: f64, new_height: f64) -> NoteMutation {
    NoteMutation::EditBlockInkStroke(EditBlockInkStroke { id, new_points, new_x, new_y, new_width, new_height })
}

impl MutationKind<NoteSnapshot, NoteMutation> for EditBlockInkStroke {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "block-ink-stroke", kind: "edit-block-ink-stroke", record: "EditedBlockInkStroke" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit block \"{}\" ink stroke", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
