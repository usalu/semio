//! 📐 Block3d mutation — `ChangeObjectKindUnit`: the object kind's optional `unit`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐 `change-object-kind-unit` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-kind-unit")]
pub struct ChangeObjectKindUnit {
    pub new_unit: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_object_kind_unit(new_unit: Option<String>) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindUnit(ChangeObjectKindUnit { new_unit })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindUnit {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-unit", record: "ChangedObjectKindUnit" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change object kind unit to {:?}", self.new_unit)
    }
}
//#endregion 🔖️Mutation
