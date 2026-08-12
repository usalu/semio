//! Puzzle3d mutation — `ChangeObjectAnchor`: changes whether a root object keeps its stored plane or resets to default XY.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-object-anchor` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-anchor")]
pub struct ChangeObjectAnchor {
    pub id: String,
    pub new_anchor: crate::artifacts::puzzle3d::Puzzle3dObjectAnchor,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ChangeObjectAnchor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-anchor", record: "ChangedObjectAnchor" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change object \"{}\" anchor", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_anchor(id: String, new_anchor: crate::artifacts::puzzle3d::Puzzle3dObjectAnchor) -> Puzzle3dMutation {
    Puzzle3dMutation::ChangeObjectAnchor(ChangeObjectAnchor { id, new_anchor })
}
