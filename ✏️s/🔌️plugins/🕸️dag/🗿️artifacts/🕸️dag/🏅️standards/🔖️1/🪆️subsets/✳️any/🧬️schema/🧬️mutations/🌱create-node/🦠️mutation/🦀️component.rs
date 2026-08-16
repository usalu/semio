//! 🌱 DAG mutation — `CreateNode`: brings a new id-keyed node into existence.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{DagNodeSpec, DagSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — full initial payload (position/size/kind/properties all fixed at
/// creation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNode {
    pub node: DagNodeSpec,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_node(node: DagNodeSpec) -> DagMutation {
    DagMutation::CreateNode(CreateNode { node })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create node \"{}\"", self.node.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node.id.clone()]
    }
}
//#endregion 🔖️Mutation
