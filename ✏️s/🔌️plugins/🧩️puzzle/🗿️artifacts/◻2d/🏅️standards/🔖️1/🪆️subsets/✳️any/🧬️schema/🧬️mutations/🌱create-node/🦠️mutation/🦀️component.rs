//! 🌱 Puzzle2d mutation — `CreateNode`: brings a new id-keyed node into existence.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::{Puzzle2dNode, Puzzle2dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `node.id` is a no-op (an id-keyed entity that already exists cannot be
/// re-created).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    #[dsl(block)]
    pub node: Puzzle2dNode,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_node(node: Puzzle2dNode, index: Option<usize>) -> Puzzle2dMutation {
    Puzzle2dMutation::CreateNode(CreateNode { node, index })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create node \"{}\"", self.node.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node.id.clone()]
    }
}
//#endregion 🔖️Mutation
