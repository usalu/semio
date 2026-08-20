//! 🔄️ `rotate-object` — sets the object's `transform.rotation`, keeping translation/scale.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioQuaternion;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RotateObject {
    pub rotation: SemioQuaternion,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for RotateObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "object", kind: "rotate-object", record: "RotatedObject" };

    async fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        "Rotate object".to_string()
    }
    async fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
