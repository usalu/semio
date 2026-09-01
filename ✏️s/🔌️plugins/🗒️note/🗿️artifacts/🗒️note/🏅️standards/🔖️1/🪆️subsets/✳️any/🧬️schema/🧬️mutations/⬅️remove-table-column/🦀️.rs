//! ⬅️ Note mutation — `RemoveTableColumn`: removes a table block's last column (a table always keeps at least one column).

use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::schema::mutations::{InsertTableColumn, NoteMutation};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬅️ `remove-table-column` payload — removes a table block's last column (a table always keeps at least one column).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-table-column")]
pub struct RemoveTableColumn {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_table_column(id: String) -> NoteMutation {
    NoteMutation::RemoveTableColumn(RemoveTableColumn { id })
}

impl MutationKind<NoteSnapshot, NoteMutation> for RemoveTableColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "table-column", kind: "remove-table-column", record: "RemovedTableColumn" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove column from table \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
