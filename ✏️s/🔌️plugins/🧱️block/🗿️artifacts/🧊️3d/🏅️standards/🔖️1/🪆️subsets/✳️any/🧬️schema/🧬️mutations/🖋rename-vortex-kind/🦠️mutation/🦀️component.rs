//! 🖋 Block3d mutation — `RenameVortexKind`: a vortex-kind catalog row's `name`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖋 `rename-vortex-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-vortex-kind")]
pub struct RenameVortexKind {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_vortex_kind(id: String, new_name: String) -> Block3dMutation {
    Block3dMutation::RenameVortexKind(RenameVortexKind { id, new_name })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RenameVortexKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "vortex-kind", kind: "rename-vortex-kind", record: "RenamedVortexKind" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename vortex kind \"{}\" to \"{}\"", self.id, self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
