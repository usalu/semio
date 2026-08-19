//! 🔀️ Block3d mutation — `ChangeObjectKindVariant`: the object kind's optional `variant`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔀️ `change-object-kind-variant` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind-variant")]
pub struct ChangeObjectKindVariant {
    pub new_variant: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_object_kind_variant(new_variant: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindVariant(ChangeObjectKindVariant { new_variant })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindVariant {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-variant", record: "ChangedObjectKindVariant" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change object kind variant to {:?}", self.new_variant)
    }
}
//#endregion 🔖️Mutation
