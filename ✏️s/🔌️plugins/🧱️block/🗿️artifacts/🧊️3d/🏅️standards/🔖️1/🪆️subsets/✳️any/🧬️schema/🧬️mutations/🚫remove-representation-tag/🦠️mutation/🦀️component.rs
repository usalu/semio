//! 🚫 Block3d mutation — `RemoveRepresentationTag`: a member of a representation's `tags` set.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚫 `remove-representation-tag` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-representation-tag")]
pub struct RemoveRepresentationTag {
    pub id: String,
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_representation_tag(id: String, tag: String) -> Block3dMutation {
    Block3dMutation::RemoveRepresentationTag(RemoveRepresentationTag { id, tag })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RemoveRepresentationTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "representation-tag", kind: "remove-representation-tag", record: "RemovedRepresentationTag" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove tag \"{}\" from representation \"{}\"", self.tag, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
