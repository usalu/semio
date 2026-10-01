//! 📍️ DAG mutation — `SetNodePositions`: puts a set of nodes at absolute canvas positions in ONE row. It is the exact
//! undo of a `move-nodes` drag (every moved node back at its base position) and of itself, so a multi-node gesture stays
//! one point-invertible row however many nodes it moved.
use crate::diff::DagDiff;
use crate::mutations::DagMutation;
use crate::DagSnapshot;

//#region 🔖️Mutation
/// 📌️ One node's absolute canvas position.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagNodePosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 📍️ `set-node-positions` payload — FINAL-state absolute positions, each node once.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetNodePositions {
    pub positions: Vec<DagNodePosition>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_node_positions(positions: Vec<DagNodePosition>) -> DagMutation {
    DagMutation::SetNodePositions(SetNodePositions { positions })
}

impl SetNodePositions {
    /// 🆔️ The positioned node ids, in payload order.
    pub fn ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.id.clone()).collect()
    }
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for SetNodePositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-positions", kind: "set-node-positions", record: "SetNodePositions" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set the positions of {} node(s)", self.positions.len()), &format!("Positionen von {} Knoten setzen", self.positions.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids()
    }
}
//#endregion 🔖️Mutation
