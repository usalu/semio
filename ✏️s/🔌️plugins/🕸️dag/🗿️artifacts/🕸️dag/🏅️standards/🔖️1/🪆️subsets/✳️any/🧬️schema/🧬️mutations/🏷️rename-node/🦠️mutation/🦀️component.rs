//! 🏷️ DAG mutation — `RenameNode`: changes a node's identity key (its `id`), cascading to every
//! edge endpoint string that referenced it (`"<id>@<port>"`).
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `rename-node` payload — the node's `id` is its identity field (its separate `name` display
/// field has its own `change-node-name` mutation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameNode {
    pub id: String,
    pub new_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_node(id: String, new_id: String) -> DagMutation {
    DagMutation::RenameNode(RenameNode { id, new_id })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for RenameNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };

    fn diff(&self, base: &DagSnapshot) -> DagDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename node \"{}\" to \"{}\"", self.id, self.new_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
