//! 🏷️ Block3d mutation — `ChangeObjectKindLabel`: the object kind's `label`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `change-object-kind-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind-label")]
pub struct ChangeObjectKindLabel {
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_kind_label(new_label: String) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindLabel(ChangeObjectKindLabel { new_label })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-label", record: "ChangedObjectKindLabel" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change object kind label to \"{}\"", self.new_label)
    }
}
//#endregion 🔖️Mutation
