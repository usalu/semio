//! 🌱️ TrinityGraph mutation — `CreateNode`: brings a new id-keyed node into existence.
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{JackSnapshot, Node};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ `create-node` payload — full initial node payload (id/kind/name/geometry/ports fixed at
/// creation; `properties` always starts empty, set afterward via `change-data-property`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNode {
    pub node: Node,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_node(node: Node) -> TrinityGraphMutation {
    TrinityGraphMutation::CreateNode(CreateNode { node })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
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
