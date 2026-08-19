//! ✒ Block3d mutation — `RenameRepresentation`: a representation's `name`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✒ `rename-representation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-representation")]
pub struct RenameRepresentation {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_representation(id: String, new_name: String) -> Block3dMutation {
    Block3dMutation::RenameRepresentation(RenameRepresentation { id, new_name })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RenameRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "representation", kind: "rename-representation", record: "RenamedRepresentation" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename representation \"{}\" to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
