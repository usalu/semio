//! 🎨 Block3d mutation — `ChangeVortexKindColor`: a vortex-kind catalog row's `color`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎨 `change-vortex-kind-color` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-vortex-kind-color")]
pub struct ChangeVortexKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_vortex_kind_color(id: String, new_color: String) -> Block3dMutation {
    Block3dMutation::ChangeVortexKindColor(ChangeVortexKindColor { id, new_color })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexKindColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex-kind", kind: "change-vortex-kind-color", record: "ChangedVortexKindColor" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change vortex kind \"{}\" color to \"{}\"", self.id, self.new_color)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
