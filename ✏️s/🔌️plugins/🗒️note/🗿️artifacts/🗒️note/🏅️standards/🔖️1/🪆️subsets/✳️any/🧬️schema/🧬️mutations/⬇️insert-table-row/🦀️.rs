//! ⬇️ Note mutation — `InsertTableRow`: appends a blank row to a table block (width matches the current column count).

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::{NoteMutation, RemoveTableRow};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬇️ `insert-table-row` payload — appends a blank row to a table block (width matches the current column count).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "insert-table-row")]
pub struct InsertTableRow {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn insert_table_row(id: String) -> NoteMutation {
    NoteMutation::InsertTableRow(InsertTableRow { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for InsertTableRow {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "table-row", kind: "insert-table-row", record: "InsertedTableRow" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Insert row into table \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
