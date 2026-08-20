//! 🚚️ `move-object` — sets the object's `transform.translation`, keeping rotation/scale.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveObject {
    pub translation: SemioPoint3,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for MoveObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "object", kind: "move-object", record: "MovedObject" };

    async fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move object to ({}, {}, {})", self.translation.x, self.translation.y, self.translation.z)
    }
    async fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
