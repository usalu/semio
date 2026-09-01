//! 🗑️️ `remove-design` — removes the DESIGN matching `id` (pieces/connections included).
//! Idempotent no-op if absent; inverse escrows the FULL design (via `add-design` + `edit-design`,
//! a real 2-step inverse — `add-design` alone only creates an empty design).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, add_design, edit_design};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveDesign {
    pub id: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for RemoveDesign {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "design", kind: "remove-design", record: "RemovedDesign" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove design {}", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
