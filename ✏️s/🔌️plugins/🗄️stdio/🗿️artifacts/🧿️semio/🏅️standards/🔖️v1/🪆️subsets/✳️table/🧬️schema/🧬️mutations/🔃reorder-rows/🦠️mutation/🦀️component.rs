//! 🔃️ `reorder-rows` — repositions one row within the sequence (never spatial — `SemioTableRow`
//! carries no position of its own, only sequence order).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderRows {
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for ReorderRows {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "rows", kind: "reorder-rows", record: "ReorderedRows" };

    async fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move row #{} to #{}", self.from, self.to)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}
//#endregion 🔖️Payload
