//! 🖼️ Block3d mutation — `ChangeObjectKindIcon`: the object kind's optional `icon`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `change-object-kind-icon` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind-icon")]
pub struct ChangeObjectKindIcon {
    pub new_icon: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_kind_icon(new_icon: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindIcon(ChangeObjectKindIcon { new_icon })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-icon", record: "ChangedObjectKindIcon" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change object kind icon to {:?}", self.new_icon)
    }
}
//#endregion 🔖️Mutation
