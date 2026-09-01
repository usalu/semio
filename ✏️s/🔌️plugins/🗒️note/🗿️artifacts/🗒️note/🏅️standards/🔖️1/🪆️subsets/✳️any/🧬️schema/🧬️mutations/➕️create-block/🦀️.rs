//! ➕ Note mutation — `CreateBlock`: brings a new block into existence at an addressed position.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_added_diff;
use crate::artifacts::note::schema::mutations::{DeleteBlock, NoteMutation};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `create-block` payload — brings a new block into existence at an addressed position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-block")]
pub struct CreateBlock {
    #[dsl(statements, block)]
    pub block: Box<crate::artifacts::note::NoteBlockNode>,
    pub parent_id: Option<String>,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_block(block: crate::artifacts::note::NoteBlockNode, parent_id: Option<String>, index: Option<usize>) -> NoteMutation {
    NoteMutation::CreateBlock(CreateBlock { block: Box::new(block), parent_id, index })
}

impl MutationKind<NoteSnapshot, NoteMutation> for CreateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "block", kind: "create-block", record: "CreatedBlock" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create block \"{}\"", crate::artifacts::note::schema::block_id(&self.block))
    }
    async fn target(&self) -> Vec<String> {
        vec![crate::artifacts::note::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation
