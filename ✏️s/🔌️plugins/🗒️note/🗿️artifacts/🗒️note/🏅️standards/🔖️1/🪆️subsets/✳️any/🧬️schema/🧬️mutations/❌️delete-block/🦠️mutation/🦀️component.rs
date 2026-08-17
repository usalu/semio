//! ❌ Note mutation — `DeleteBlock`: removes a block (and its subtree, if it is a group).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ❌ `delete-block` payload — removes a block (and its subtree, if it is a group).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-block")]
pub struct DeleteBlock {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_block(id: String) -> NoteMutation {
    NoteMutation::DeleteBlock(DeleteBlock { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DeleteBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "block", kind: "delete-block", record: "DeletedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete block \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
