//! 🚚️ `move-object` — sets the object's `transform.translation`, keeping rotation/scale.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveObject {
    pub translation: SemioPoint3,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for MoveObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "object", kind: "move-object", record: "MovedObject" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move object to ({}, {}, {})", self.translation.x, self.translation.y, self.translation.z)
    }
    fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
