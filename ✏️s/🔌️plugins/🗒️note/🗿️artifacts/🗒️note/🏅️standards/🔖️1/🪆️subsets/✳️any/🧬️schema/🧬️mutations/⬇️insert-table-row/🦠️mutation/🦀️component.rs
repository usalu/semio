//! ⬇️ Note mutation — `InsertTableRow`: appends a blank row to a table block (width matches the current column count).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬇️ `insert-table-row` payload — appends a blank row to a table block (width matches the current column count).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "insert-table-row")]
pub struct InsertTableRow {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn insert_table_row(id: String) -> NoteMutation {
    NoteMutation::InsertTableRow(InsertTableRow { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for InsertTableRow {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "table-row", kind: "insert-table-row", record: "InsertedTableRow" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert row into table \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
