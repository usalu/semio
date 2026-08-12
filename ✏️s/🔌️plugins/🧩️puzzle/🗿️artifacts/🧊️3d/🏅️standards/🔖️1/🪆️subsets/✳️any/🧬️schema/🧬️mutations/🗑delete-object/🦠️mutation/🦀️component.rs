//! 🗑 Puzzle3d mutation — `DeleteObject`: removes an id-keyed object (captures cascade — any
//! attraction touching one of this object's vortices is severed too, re-`connect-vortices`ed by
//! the inverse).
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑 `delete-object` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-object")]
pub struct DeleteObject {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_object(id: String) -> Puzzle3dMutation {
    Puzzle3dMutation::DeleteObject(DeleteObject { id })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DeleteObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "object", kind: "delete-object", record: "DeletedObject" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete object \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
