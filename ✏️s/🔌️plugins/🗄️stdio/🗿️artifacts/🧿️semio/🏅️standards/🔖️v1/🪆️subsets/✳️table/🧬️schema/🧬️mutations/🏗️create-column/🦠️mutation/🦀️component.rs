//! 🏗️ `create-column` — brings a new named column into existence at an optional FINAL-state
//! index, per `📓️taxonomy.md`'s `create` row ("full initial payload (+ optional `index`)").
//! Inserting `SemioValue::Null` at the same index into every row keeps the CRITICAL row/column
//! alignment invariant (see `📸️snapshot/🦀️component.rs`'s own doc comment).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateColumn {
    pub name: String,
    pub kind: SemioTableCellKind,
    pub index: Option<usize>,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for CreateColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "column", kind: "create-column", record: "CreatedColumn" };

    async fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Create column {}", self.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
