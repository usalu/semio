//! 🚷 Block3d mutation — `RemoveAttribute`: a free-form key/value attribute attachment.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dAttributesDelta, Block3dDiff};
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚷 `remove-attribute` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-attribute")]
pub struct RemoveAttribute {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_attribute(key: String) -> Block3dMutation {
    Block3dMutation::RemoveAttribute(RemoveAttribute { key })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RemoveAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "attribute", kind: "remove-attribute", record: "RemovedAttribute" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove attribute \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
