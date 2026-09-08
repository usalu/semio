//! 🔀 DAG mutation — `ReorderNodes`: position within the (display-order-meaningful, e.g. z-order
//! layering) node list — never spatial.
use crate::diff::DagDiff;
use crate::mutations::DagMutation;
use crate::DagSnapshot;

//#region 🔖️Mutation
/// 🔀 `reorder-nodes` payload — FINAL-state full id order.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReorderNodes {
    pub order: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_nodes(order: Vec<String>) -> DagMutation {
    DagMutation::ReorderNodes(ReorderNodes { order })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReorderNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Reorder nodes".into()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
