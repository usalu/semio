//! 🌱 Puzzle3d mutation — `CreateObject`: brings a new id-keyed object into existence.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dObject, Puzzle3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-object` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `object.id` is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-object")]
pub struct CreateObject {
    #[dsl(block)]
    pub object: Puzzle3dObject,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_object(object: Puzzle3dObject, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::CreateObject(CreateObject { object, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for CreateObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create object \"{}\"", self.object.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object.id.clone()]
    }
}
//#endregion 🔖️Mutation
