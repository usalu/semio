//! 🔤 DAG mutation — `ChangeNodeName`: sets the node's display `name` (distinct from its `id`,
//! which `rename-node` governs).
use serde::{Deserialize, Serialize};
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ChangeNodeName {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_name(id: String, new_name: String) -> DagMutation {
    DagMutation::ChangeNodeName(ChangeNodeName { id, new_name })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-name", record: "ChangedNodeName" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename node \"{}\" label to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
