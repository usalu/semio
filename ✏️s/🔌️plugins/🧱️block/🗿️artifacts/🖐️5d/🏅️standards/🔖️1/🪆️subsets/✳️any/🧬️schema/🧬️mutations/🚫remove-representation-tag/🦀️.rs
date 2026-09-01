//! 🚫 Block5d mutation — `RemoveRepresentationTag`: a member of a representation's `tags` set.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dRepresentationsDelta, Block5dRepresentationsPatch, Block5dRepresentationsPatchEntry};
use crate::artifacts::block5d::mutations::Block5dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚫 `remove-representation-tag` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-representation-tag")]
pub struct RemoveRepresentationTag {
    pub id: String,
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_representation_tag(id: String, tag: String) -> Block5dMutation {
    Block5dMutation::RemoveRepresentationTag(RemoveRepresentationTag { id, tag })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for RemoveRepresentationTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "representation-tag", kind: "remove-representation-tag", record: "RemovedRepresentationTag" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
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
