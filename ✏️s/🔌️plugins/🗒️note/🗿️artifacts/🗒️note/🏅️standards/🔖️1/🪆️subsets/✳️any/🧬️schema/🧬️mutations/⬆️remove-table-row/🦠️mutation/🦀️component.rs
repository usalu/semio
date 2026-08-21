//! ⬆️ Note mutation — `RemoveTableRow`: removes a table block's last row (a table always keeps at least one row).
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬆️ `remove-table-row` payload — removes a table block's last row (a table always keeps at least one row).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-table-row")]
pub struct RemoveTableRow {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_table_row(id: String) -> NoteMutation {
    NoteMutation::RemoveTableRow(RemoveTableRow { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for RemoveTableRow {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "table-row", kind: "remove-table-row", record: "RemovedTableRow" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove row from table \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
