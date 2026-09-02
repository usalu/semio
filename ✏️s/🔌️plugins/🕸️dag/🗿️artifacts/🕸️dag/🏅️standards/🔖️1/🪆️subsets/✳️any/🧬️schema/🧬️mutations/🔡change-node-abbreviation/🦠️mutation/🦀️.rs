//! 🔡 DAG mutation — `ChangeNodeAbbreviation`: sets the node's short label.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ChangeNodeAbbreviation {
    pub id: String,
    pub new_abbreviation: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_abbreviation(id: String, new_abbreviation: String) -> DagMutation {
    DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id, new_abbreviation })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeAbbreviation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-abbreviation", record: "ChangedNodeAbbreviation" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" abbreviation to \"{}\"", self.id, self.new_abbreviation)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
