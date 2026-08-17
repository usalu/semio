//! ⬅️ Note mutation — `RemoveTableColumn`: removes a table block's last column (a table always keeps at least one column).
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬅️ `remove-table-column` payload — removes a table block's last column (a table always keeps at least one column).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-table-column")]
pub struct RemoveTableColumn {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_table_column(id: String) -> NoteMutation {
    NoteMutation::RemoveTableColumn(RemoveTableColumn { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for RemoveTableColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "table-column", kind: "remove-table-column", record: "RemovedTableColumn" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove column from table \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
