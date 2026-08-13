//! 📏️ `scale-object` — sets the object's `transform.scale`, keeping translation/rotation.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleObject {
    pub scale: SemioPoint3,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for ScaleObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "object", kind: "scale-object", record: "ScaledObject" };

    fn diff(&self, base: &SemioObjectSnapshot) -> <SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale object to ({}, {}, {})", self.scale.x, self.scale.y, self.scale.z)
    }
    fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
