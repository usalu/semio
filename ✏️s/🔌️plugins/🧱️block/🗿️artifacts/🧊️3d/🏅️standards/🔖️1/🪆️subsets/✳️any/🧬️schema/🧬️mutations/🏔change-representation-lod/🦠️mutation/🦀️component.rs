//! 🏔 Block3d mutation — `ChangeRepresentationLod`: a representation's `lod`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏔 `change-representation-lod` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-representation-lod")]
pub struct ChangeRepresentationLod {
    pub id: String,
    pub new_lod: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_representation_lod(id: String, new_lod: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeRepresentationLod(ChangeRepresentationLod { id, new_lod })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeRepresentationLod {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-lod", record: "ChangedRepresentationLod" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change representation \"{}\" LOD", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
