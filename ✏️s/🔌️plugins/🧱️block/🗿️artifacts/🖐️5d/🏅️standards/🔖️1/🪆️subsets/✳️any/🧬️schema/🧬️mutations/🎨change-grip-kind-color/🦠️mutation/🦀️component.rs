//! 🎨 Block5d mutation — `ChangeGripKindColor`: a grip-kind catalog row's `color`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎨 `change-grip-kind-color` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grip-kind-color")]
pub struct ChangeGripKindColor {
    pub id: String,
    pub new_color: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_color(id: String, new_color: String) -> Block5dMutation {
    Block5dMutation::ChangeGripKindColor(ChangeGripKindColor { id, new_color })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeGripKindColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-color", record: "ChangedGripKindColor" };

    fn diff(&self, base: &Block5dSnapshot) -> Block5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grip kind \"{}\" color to \"{}\"", self.id, self.new_color)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
