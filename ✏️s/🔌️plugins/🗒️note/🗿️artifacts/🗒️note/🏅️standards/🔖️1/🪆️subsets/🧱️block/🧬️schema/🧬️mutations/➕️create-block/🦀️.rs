//! ➕ Note mutation — `CreateBlock`: brings a new block into existence at an addressed position.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ➕ `create-block` payload — brings a new block into existence at an addressed position.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create block \"{}\"", crate::schema::block_id(&self.block)), &format!("Block \"{}\" erstellen", crate::schema::block_id(&self.block)))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::block_id(&self.block).to_string()]
    }
}
//#endregion 🔖️Mutation
