//! 🧮 DAG mutation — `ChangeNodeOperatorKind`: sets the node's optional compute-operator binding.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ChangeNodeOperatorKind {
    pub id: String,
    pub new_operator_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_operator_kind(id: String, new_operator_kind: Option<String>) -> DagMutation {
    DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id, new_operator_kind })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeOperatorKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-operator-kind", record: "ChangedNodeOperatorKind" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" operator kind", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
