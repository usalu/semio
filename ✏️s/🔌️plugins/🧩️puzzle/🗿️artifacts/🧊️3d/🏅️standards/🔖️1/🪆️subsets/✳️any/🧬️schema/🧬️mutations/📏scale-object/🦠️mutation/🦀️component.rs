//! Puzzle3d mutation — `ScaleObject`: changes an object's freeform pose scale.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `scale-object` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-object")]
pub struct ScaleObject {
    pub id: String,
    pub new_scale: Option<crate::artifacts::puzzle3d::Puzzle3dScale>,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ScaleObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "object", kind: "scale-object", record: "ScaledObject" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale object \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_object(id: String, new_scale: Option<crate::artifacts::puzzle3d::Puzzle3dScale>) -> Puzzle3dMutation {
    Puzzle3dMutation::ScaleObject(ScaleObject { id, new_scale })
}
