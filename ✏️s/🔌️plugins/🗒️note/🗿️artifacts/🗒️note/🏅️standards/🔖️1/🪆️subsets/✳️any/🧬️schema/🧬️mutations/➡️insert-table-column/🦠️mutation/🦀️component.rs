//! ➡️ Note mutation — `InsertTableColumn`: appends a lettered column to a table block.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➡️ `insert-table-column` payload — appends a lettered column to a table block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "insert-table-column")]
pub struct InsertTableColumn {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn insert_table_column(id: String) -> NoteMutation {
    NoteMutation::InsertTableColumn(InsertTableColumn { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for InsertTableColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "table-column", kind: "insert-table-column", record: "InsertedTableColumn" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert column into table \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
