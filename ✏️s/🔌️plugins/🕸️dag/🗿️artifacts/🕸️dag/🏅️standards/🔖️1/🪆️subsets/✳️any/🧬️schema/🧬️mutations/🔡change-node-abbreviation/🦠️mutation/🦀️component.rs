//! 🔡 DAG mutation — `ChangeNodeAbbreviation`: sets the node's short label.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNodeAbbreviation {
    pub id: String,
    pub new_abbreviation: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_abbreviation(id: String, new_abbreviation: String) -> DagMutation {
    DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id, new_abbreviation })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeAbbreviation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-abbreviation", record: "ChangedNodeAbbreviation" };

    fn diff(&self, base: &DagSnapshot) -> DagDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node \"{}\" abbreviation to \"{}\"", self.id, self.new_abbreviation)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
