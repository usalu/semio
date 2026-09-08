//! 🪢 Block5d mutation — `ChangeGripKindDefaultRopeKind`: a grip-kind catalog row's `defaultRopeKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🪢 `change-grip-kind-default-rope-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-grip-kind-default-rope-kind")]
pub struct ChangeGripKindDefaultRopeKind {
    pub id: String,
    pub new_default_rope_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_default_rope_kind(id: String, new_default_rope_kind: String) -> Block5dMutation {
    Block5dMutation::ChangeGripKindDefaultRopeKind(ChangeGripKindDefaultRopeKind { id, new_default_rope_kind })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeGripKindDefaultRopeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-default-rope-kind", record: "ChangedGripKindDefaultRopeKind" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grip kind \"{}\" default rope kind to \"{}\"", self.id, self.new_default_rope_kind)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
