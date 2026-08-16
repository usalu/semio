//! 🎨️ Block2d mutation — `ChangeHandleKindColor`: a handle-kind catalog row's `color`.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎨️ `change-handle-kind-color` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-handle-kind-color")]
pub struct ChangeHandleKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_kind_color(id: String, new_color: String) -> Block2dMutation {
    Block2dMutation::ChangeHandleKindColor(ChangeHandleKindColor { id, new_color })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeHandleKindColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-color", record: "ChangedHandleKindColor" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change handle kind \"{}\" color to \"{}\"", self.id, self.new_color)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
