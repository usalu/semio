//! 🆕️ `add-design` — appends a new, empty DESIGN (no pieces/connections yet — populate via
//! `edit-design`) to the kit's catalog.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, remove_design};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddDesign {
    pub id: String,
    pub name: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for AddDesign {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "design", kind: "add-design", record: "AddedDesign" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add design {}", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
