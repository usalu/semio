//! 🏗️ `create-column` — brings a new named column into existence at an optional FINAL-state
//! index, per `📓️taxonomy.md`'s `create` row ("full initial payload (+ optional `index`)").
//! Inserting `SemioValue::Null` at the same index into every row keeps the CRITICAL row/column
//! alignment invariant (see `📸️snapshot/🦀️.rs`'s own doc comment).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{SemioTableMutation, delete_column};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumn {
    pub name: String,
    pub kind: SemioTableCellKind,
    pub index: Option<usize>,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for CreateColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "column", kind: "create-column", record: "CreatedColumn" };

    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create column {}", self.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
