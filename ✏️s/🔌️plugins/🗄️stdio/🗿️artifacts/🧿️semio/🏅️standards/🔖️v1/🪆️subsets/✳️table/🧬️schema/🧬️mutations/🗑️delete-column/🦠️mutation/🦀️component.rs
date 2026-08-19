//! 🗑️ `delete-column` — removes a named column and its aligned cell from every row (captures the
//! full cascade for its inverse, per `📓️taxonomy.md`'s `delete` row).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteColumn {
    pub name: String,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for DeleteColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "column", kind: "delete-column", record: "DeletedColumn" };

    async fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete column {}", self.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
