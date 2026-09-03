//! 🌱 DAG mutation — `CreateNode`: brings a new id-keyed node into existence.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{DagNodeSpec, DagSnapshot};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — full initial payload (position/size/kind/properties all fixed at
/// creation).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateNode {
    pub node: DagNodeSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_node(node: DagNodeSpec) -> DagMutation {
    DagMutation::CreateNode(CreateNode { node })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
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
