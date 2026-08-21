//! 🗑️ `delete-solid` — removes an id-keyed solid. Nothing else in this collection set references a solid, so no cascade question arises. Absent `id` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSolid {
    pub id: String,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for DeleteSolid {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "solid", kind: "delete-solid", record: "DeletedSolid" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete solid \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
