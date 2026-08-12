//! 🖼️ DAG mutation — `ChangeNodeIcon`: sets the node's icon reference.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNodeIcon {
    pub id: String,
    pub new_icon: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_icon(id: String, new_icon: String) -> DagMutation {
    DagMutation::ChangeNodeIcon(ChangeNodeIcon { id, new_icon })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-icon", record: "ChangedNodeIcon" };

    fn diff(&self, base: &DagSnapshot) -> DagDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node \"{}\" icon to \"{}\"", self.id, self.new_icon)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
