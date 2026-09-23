//! 🌱 DAG mutation — `CreateNode`: brings a new id-keyed node into existence.
use crate::diff::DagDiff;
use crate::mutations::DagMutation;
use crate::{DagNodeSpec, DagSnapshot};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — full initial payload (position/size/kind/properties all fixed at
/// creation).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateNode {
    pub node: DagNodeSpec,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_node(node: DagNodeSpec) -> DagMutation {
    DagMutation::CreateNode(CreateNode { node, index: None })
}

/// 📍️ Builder — inserts the node at `index` of the node roster instead of appending it, which is how
/// `delete-node`'s inverse restores a node to its exact position in ONE row.
pub fn create_node_at(node: DagNodeSpec, index: usize) -> DagMutation {
    DagMutation::CreateNode(CreateNode { node, index: Some(index) })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create node \"{}\"", self.node.id), &format!("Knoten \"{}\" erstellen", self.node.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node.id.clone()]
    }
}
//#endregion 🔖️Mutation
