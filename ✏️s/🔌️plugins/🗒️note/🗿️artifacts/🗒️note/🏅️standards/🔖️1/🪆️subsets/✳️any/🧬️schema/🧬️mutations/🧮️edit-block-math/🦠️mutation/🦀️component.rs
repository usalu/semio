//! 🧮 Note mutation — `EditBlockMath`: replaces a math block's authored TeX source.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧮 `edit-block-math` payload — replaces a math block's authored TeX source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-block-math")]
pub struct EditBlockMath {
    pub id: String,
    pub new_tex: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_block_math(id: String, new_tex: String) -> NoteMutation {
    NoteMutation::EditBlockMath(EditBlockMath { id, new_tex })
}

impl MutationKind<NoteSnapshot, NoteMutation> for EditBlockMath {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "block-math", kind: "edit-block-math", record: "EditedBlockMath" };

    fn diff(&self, base: &NoteSnapshot) -> NoteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit block \"{}\" math", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
