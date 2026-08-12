//! Puzzle5d mutation — `ChangePart3dMesh`: changes a part's 3D-projection geometry reference.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-part-3d-mesh` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-3d-mesh")]
pub struct ChangePart3dMesh {
    pub id: String,
    pub new_mesh_url: Option<String>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePart3dMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part-3d-mesh", record: "ChangedPart3dMesh" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part \"{}\" 3d mesh", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_3d_mesh(id: String, new_mesh_url: Option<String>) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangePart3dMesh(ChangePart3dMesh { id, new_mesh_url })
}
