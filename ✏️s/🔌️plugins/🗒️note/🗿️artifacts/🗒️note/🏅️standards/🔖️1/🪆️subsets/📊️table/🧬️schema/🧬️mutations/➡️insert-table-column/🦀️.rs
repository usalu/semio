//! ➡️ Note mutation — `InsertTableColumn`: appends a lettered column to a table block.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➡️ `insert-table-column` payload — appends a lettered column to a table block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "insert-table-column")]
pub struct InsertTableColumn {
    pub id: String,
    /// 🏷️ The header of the appended column; absent derives the next letter.
    #[value(default)]
    #[serde(default)]
    pub name: Option<String>,
    /// 🧱️ One cell per row for the appended column; absent appends blank cells.
    #[value(default)]
    #[serde(default)]
    pub cells: Option<Vec<crate::NoteTableCell>>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn insert_table_column(id: String) -> NoteMutation {
    NoteMutation::InsertTableColumn(InsertTableColumn { id, name: None, cells: None })
}

impl MutationKind<NoteSnapshot, NoteMutation> for InsertTableColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "table-column", kind: "insert-table-column", record: "InsertedTableColumn" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert column into table \"{}\"", self.id), &format!("Spalte in Tabelle \"{}\" einfügen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
