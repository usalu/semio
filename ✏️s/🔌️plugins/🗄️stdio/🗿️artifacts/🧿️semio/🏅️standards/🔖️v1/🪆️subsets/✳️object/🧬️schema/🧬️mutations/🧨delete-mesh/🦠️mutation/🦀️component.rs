//! 🧨️ `delete-mesh` — clears the object's `mesh` CHILD slot. Idempotent; inverse escrows from BASE.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteMesh {}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for DeleteMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "mesh", kind: "delete-mesh", record: "DeletedMesh" };

    async fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Delete mesh child".to_string()
    }
    async fn target(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }
}
//#endregion 🔖️Payload
