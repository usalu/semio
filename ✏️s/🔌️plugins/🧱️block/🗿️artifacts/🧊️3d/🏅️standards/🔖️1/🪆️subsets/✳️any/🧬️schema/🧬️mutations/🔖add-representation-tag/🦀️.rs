//! 🔖 Block3d mutation — `AddRepresentationTag`: a member of a representation's `tags` set.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🔖 `add-representation-tag` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-representation-tag")]
pub struct AddRepresentationTag {
    pub id: String,
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_representation_tag(id: String, tag: String) -> Block3dMutation {
    Block3dMutation::AddRepresentationTag(AddRepresentationTag { id, tag })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddRepresentationTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-tag", kind: "add-representation-tag", record: "AddedRepresentationTag" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add tag \"{}\" to representation \"{}\"", self.tag, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
