//! 💥️ `delete-brep` — clears the object's `brep` CHILD slot. Idempotent (a no-op if already
//! empty); the inverse captures the escrowed handle from BASE so undo restores it exactly.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteBrep {}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for DeleteBrep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "brep", kind: "delete-brep", record: "DeletedBrep" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Delete brep child".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["brep".to_string()]
    }
}
//#endregion 🔖️Payload
