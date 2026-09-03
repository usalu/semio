//! 🚫️ Block2d mutation — `RemoveAttribute`: a free-form key/value attribute attachment.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dAttributesDelta, Block2dDiff};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🚫️ `remove-attribute` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-attribute")]
pub struct RemoveAttribute {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_attribute(key: String) -> Block2dMutation {
    Block2dMutation::RemoveAttribute(RemoveAttribute { key })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for RemoveAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "attribute", kind: "remove-attribute", record: "RemovedAttribute" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
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
