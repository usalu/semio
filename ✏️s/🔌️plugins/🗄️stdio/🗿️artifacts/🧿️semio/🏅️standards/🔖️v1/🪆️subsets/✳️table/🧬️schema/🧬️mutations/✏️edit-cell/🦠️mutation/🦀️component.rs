//! ✏️ `edit-cell` — replaces one cell's authored value, addressed by BASE-state `row_index` and
//! the column's `column_name` (`📓️taxonomy.md`'s `edit` row literally lists "cell" as an example:
//! "Replace an authored content body (text, cell, code)").

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditCell {
    pub row_index: usize,
    pub column_name: String,
    pub new_value: SemioValue,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for EditCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "cell", kind: "edit-cell", record: "EditedCell" };

    fn diff(&self, base: &SemioTableSnapshot) -> <SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit cell #{} {}", self.row_index, self.column_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.row_index.to_string(), self.column_name.clone()]
    }
}
//#endregion 🔖️Payload
