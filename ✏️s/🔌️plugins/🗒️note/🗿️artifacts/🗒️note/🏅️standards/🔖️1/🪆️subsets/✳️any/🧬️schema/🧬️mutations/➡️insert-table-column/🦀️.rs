//! ➡️ Note mutation — `InsertTableColumn`: appends a lettered column to a table block.

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::{NoteMutation, RemoveTableColumn};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➡️ `insert-table-column` payload — appends a lettered column to a table block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "insert-table-column")]
pub struct InsertTableColumn {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn insert_table_column(id: String) -> NoteMutation {
    NoteMutation::InsertTableColumn(InsertTableColumn { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for InsertTableColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "table-column", kind: "insert-table-column", record: "InsertedTableColumn" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Insert column into table \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
