//! 🔀️ `reorder-columns` — repositions one named column within the sequence (never spatial —
//! `SemioTableColumn` carries no position of its own, only sequence order).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderColumns {
    pub name: String,
    pub to_index: usize,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for ReorderColumns {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "columns", kind: "reorder-columns", record: "ReorderedColumns" };

    async fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move column {} to #{}", self.name, self.to_index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
