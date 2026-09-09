//! 👀 Note mutation — `ChangeBlockVisible`: sets a block's visibility.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👀 `change-block-visible` payload — sets a block's visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-visible")]
pub struct ChangeBlockVisible {
    pub id: String,
    pub new_visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_block_visible(id: String, new_visible: bool) -> NoteMutation {
    NoteMutation::ChangeBlockVisible(ChangeBlockVisible { id, new_visible })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeBlockVisible {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-visible", kind: "change-block-visible", record: "ChangedBlockVisible" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change block \"{}\" visible to {}", self.id, self.new_visible)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
