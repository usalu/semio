//! ➕ Note mutation — `CreateBlock`: brings a new block into existence at an addressed position.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ➕ `create-block` payload — brings a new block into existence at an addressed position.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-block")]
pub struct CreateBlock {
    #[dsl(statements, block)]
    pub block: Box<crate::NoteBlockNode>,
    pub parent_id: Option<String>,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_block(block: crate::NoteBlockNode, parent_id: Option<String>, index: Option<usize>) -> NoteMutation {
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
        format!("Create block \"{}\"", crate::schema::block_id(&self.block))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation
