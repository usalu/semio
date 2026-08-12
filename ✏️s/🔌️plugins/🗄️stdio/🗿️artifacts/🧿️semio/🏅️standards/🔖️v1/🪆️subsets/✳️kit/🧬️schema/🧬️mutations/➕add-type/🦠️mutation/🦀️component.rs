//! ➕️ `add-type` — appends a new TYPE to the kit's catalog (id-keyed, no positional meaning).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddType {
    pub id: String,
    pub name: String,
    pub category: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for AddType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "type", kind: "add-type", record: "AddedType" };

    fn diff(&self, base: &SemioKitSnapshot) -> <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add type {}", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
