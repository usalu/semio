//! 🧷️ Block2d mutation — `ChangeHandleHandleKind`: a rim-handle's `handleKind` catalog reference (rebind).

use crate::artifacts::block2d::{Block2dHandleTemplate, Block2dSnapshot};
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandlesDelta, Block2dHandlesPatch, Block2dHandlesPatchEntry};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🧷️ `change-handle-handle-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-handle-handle-kind")]
pub struct ChangeHandleHandleKind {
    pub id: String,
    pub new_handle_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_handle_handle_kind(id: String, new_handle_kind: String) -> Block2dMutation {
    Block2dMutation::ChangeHandleHandleKind(ChangeHandleHandleKind { id, new_handle_kind })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeHandleHandleKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle", kind: "change-handle-handle-kind", record: "ChangedHandleHandleKind" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change handle \"{}\" handle kind to \"{}\"", self.id, self.new_handle_kind)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
