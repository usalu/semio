//! ➕️ `insert-row` — places a new row at a FINAL-state index in the row sequence (an intrinsically
//! ordered, anonymous collection — no stable id on `SemioTableRow`).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableRow, SemioTableSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertRow {
    pub index: usize,
    pub row: SemioTableRow,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertedRow" };

    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Insert row at #{}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
