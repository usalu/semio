//! 🎫 Block3d mutation — `ChangeVortexKindLabel`: a vortex-kind catalog row's `label`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎫 `change-vortex-kind-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-vortex-kind-label")]
pub struct ChangeVortexKindLabel {
    pub id: String,
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_vortex_kind_label(id: String, new_label: String) -> Block3dMutation {
    Block3dMutation::ChangeVortexKindLabel(ChangeVortexKindLabel { id, new_label })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeVortexKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vortex-kind", kind: "change-vortex-kind-label", record: "ChangedVortexKindLabel" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change vortex kind \"{}\" label to \"{}\"", self.id, self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
