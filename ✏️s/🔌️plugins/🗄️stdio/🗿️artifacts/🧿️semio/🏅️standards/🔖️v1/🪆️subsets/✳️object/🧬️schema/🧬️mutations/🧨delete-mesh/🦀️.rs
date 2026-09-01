//! 🧨️ `delete-mesh` — clears the object's `mesh` CHILD slot. Idempotent; inverse escrows from BASE.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, create_mesh};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteMesh {}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for DeleteMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "mesh", kind: "delete-mesh", record: "DeletedMesh" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Delete mesh child".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }
}
//#endregion 🔖️Payload
