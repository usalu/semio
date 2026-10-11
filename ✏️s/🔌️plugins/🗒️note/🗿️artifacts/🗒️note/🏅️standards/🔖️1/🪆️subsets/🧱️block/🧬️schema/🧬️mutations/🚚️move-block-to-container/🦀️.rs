//! 🚚 Note mutation — `MoveBlockToContainer`: reparents a block into a new container at an index (hierarchy move).

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚚 `move-block-to-container` payload — reparents a block into a new container at an index (hierarchy move).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-block-to-container")]
pub struct MoveBlockToContainer {
    pub id: String,
    pub new_parent_id: Option<String>,
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_block_to_container(id: String, new_parent_id: Option<String>, index: usize) -> NoteMutation {
    NoteMutation::MoveBlockToContainer(MoveBlockToContainer { id, new_parent_id, index })
}

impl MutationKind<NoteSnapshot, NoteMutation> for MoveBlockToContainer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "block", kind: "move-block-to-container", record: "MovedBlockToContainer" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move block \"{}\"", self.id), &format!("Block \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
