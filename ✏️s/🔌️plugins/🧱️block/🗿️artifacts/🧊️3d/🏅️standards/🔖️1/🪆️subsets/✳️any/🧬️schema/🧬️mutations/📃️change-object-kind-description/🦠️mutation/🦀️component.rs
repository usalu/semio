//! 📃️ Block3d mutation — `ChangeObjectKindDescription`: the object kind's `description`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📃️ `change-object-kind-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind-description")]
pub struct ChangeObjectKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_object_kind_description(new_description: String) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindDescription(ChangeObjectKindDescription { new_description })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-description", record: "ChangedObjectKindDescription" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Change object kind description".to_string()
    }
}
//#endregion 🔖️Mutation
