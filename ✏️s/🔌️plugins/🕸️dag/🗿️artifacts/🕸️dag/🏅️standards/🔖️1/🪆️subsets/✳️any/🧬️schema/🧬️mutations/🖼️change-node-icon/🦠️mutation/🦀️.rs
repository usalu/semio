//! 🖼️ DAG mutation — `ChangeNodeIcon`: sets the node's icon reference.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ChangeNodeIcon {
    pub id: String,
    pub new_icon: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_icon(id: String, new_icon: String) -> DagMutation {
    DagMutation::ChangeNodeIcon(ChangeNodeIcon { id, new_icon })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ChangeNodeIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-icon", record: "ChangedNodeIcon" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node \"{}\" icon to \"{}\"", self.id, self.new_icon)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
