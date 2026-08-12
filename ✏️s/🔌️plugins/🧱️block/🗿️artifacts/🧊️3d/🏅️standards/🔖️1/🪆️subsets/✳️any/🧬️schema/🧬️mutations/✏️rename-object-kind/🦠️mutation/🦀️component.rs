//! ✏️ Block3d mutation — `RenameObjectKind`: the object kind's identity `name` field.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `rename-object-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-object-kind")]
pub struct RenameObjectKind {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_object_kind(new_name: String) -> Block3dMutation {
    Block3dMutation::RenameObjectKind(RenameObjectKind { new_name })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RenameObjectKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "object-kind", kind: "rename-object-kind", record: "RenamedObjectKind" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename object kind to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Mutation
