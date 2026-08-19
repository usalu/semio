//! ✏️ TrinityGraph mutation — `RenameNode`: changes a node's identity `name` field.
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `rename-node` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameNode {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_node(id: String, new_name: String) -> TrinityGraphMutation {
    TrinityGraphMutation::RenameNode(RenameNode { id, new_name })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for RenameNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename node \"{}\" to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
