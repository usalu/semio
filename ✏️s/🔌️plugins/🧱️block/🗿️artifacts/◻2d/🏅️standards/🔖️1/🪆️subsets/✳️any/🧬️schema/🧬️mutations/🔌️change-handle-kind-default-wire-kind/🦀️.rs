//! 🔌️ Block2d mutation — `ChangeHandleKindDefaultWireKind`: a handle-kind catalog row's `defaultWireKind`.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::diff::{Block2dDiff, Block2dHandleKindsDelta, Block2dHandleKindsPatch, Block2dHandleKindsPatchEntry};
use crate::artifacts::block2d::mutations::Block2dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔌️ `change-handle-kind-default-wire-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-handle-kind-default-wire-kind")]
pub struct ChangeHandleKindDefaultWireKind {
    pub id: String,
    pub new_default_wire_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_handle_kind_default_wire_kind(id: String, new_default_wire_kind: String) -> Block2dMutation {
    Block2dMutation::ChangeHandleKindDefaultWireKind(ChangeHandleKindDefaultWireKind { id, new_default_wire_kind })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeHandleKindDefaultWireKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "handle-kind", kind: "change-handle-kind-default-wire-kind", record: "ChangedHandleKindDefaultWireKind" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change handle kind \"{}\" default wire kind to \"{}\"", self.id, self.new_default_wire_kind)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
