//! 🗃️ DAG mutation — `ReplaceNodeProperties`: whole-value swap of the node's `PropertyBag`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use graph::manifest::PropertyBag;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceNodeProperties {
    pub id: String,
    pub new_properties: PropertyBag,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_node_properties(id: String, new_properties: PropertyBag) -> DagMutation {
    DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id, new_properties })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReplaceNodeProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node-properties", record: "ReplacedNodeProperties" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace node \"{}\" properties", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
