//! ➕ Note mutation — `CreateBlock`: brings a new block into existence at an addressed position.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `create-block` payload — brings a new block into existence at an addressed position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-block")]
pub struct CreateBlock {
    #[dsl(statements, block)]
    pub block: Box<crate::artifacts::note::NoteBlockNode>,
    pub parent_id: Option<String>,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_block(block: crate::artifacts::note::NoteBlockNode, parent_id: Option<String>, index: Option<usize>) -> NoteMutation {
    NoteMutation::CreateBlock(CreateBlock { block: Box::new(block), parent_id, index })
}

impl MutationKind<NoteSnapshot, NoteMutation> for CreateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "block", kind: "create-block", record: "CreatedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create block \"{}\"", crate::artifacts::note::schema::block_id(&self.block))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::note::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation
