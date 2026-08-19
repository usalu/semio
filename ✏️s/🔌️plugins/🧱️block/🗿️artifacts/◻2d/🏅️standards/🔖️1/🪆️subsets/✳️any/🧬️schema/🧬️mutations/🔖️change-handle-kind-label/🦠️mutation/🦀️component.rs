//! 🔖️ Block2d mutation — `ChangeHandleKindLabel`: a handle-kind catalog row's `label`.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔖️ `change-handle-kind-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-handle-kind-label")]
pub struct ChangeHandleKindLabel {
    pub id: String,
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_handle_kind_label(id: String, new_label: String) -> Block2dMutation {
    Block2dMutation::ChangeHandleKindLabel(ChangeHandleKindLabel { id, new_label })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeHandleKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-label", record: "ChangedHandleKindLabel" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change handle kind \"{}\" label to \"{}\"", self.id, self.new_label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
