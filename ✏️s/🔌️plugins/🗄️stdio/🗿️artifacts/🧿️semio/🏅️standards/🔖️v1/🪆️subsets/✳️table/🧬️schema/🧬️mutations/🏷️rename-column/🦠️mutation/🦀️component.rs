//! 🏷️ `rename-column` — changes a column's identity field (`name`), addressed by its current
//! name (`📓️taxonomy.md`'s `rename` row).

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameColumn {
    pub name: String,
    pub new_name: String,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for RenameColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "column", kind: "rename-column", record: "RenamedColumn" };

    fn diff(&self, base: &SemioTableSnapshot) -> <SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename column {} to {}", self.name, self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
