//! 🚚️ DAG mutation — `MoveNodes`: one node-graph drag as intent (the node-graph gesture record of design §13.3): the
//! canvas offset every addressed node moves by from its BASE position, so editing the drag in history replays it on any
//! base.
use crate::diff::DagDiff;
use crate::mutations::{dag_label_number, DagMutation};
use crate::DagSnapshot;

//#region 🔖️Mutation
/// 🚚️ `move-nodes` payload — RELATIVE `(dx, dy)` over the addressed nodes.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_nodes(ids: Vec<String>, dx: f64, dy: f64) -> DagMutation {
    DagMutation::MoveNodes(MoveNodes { ids, dx, dy })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for MoveNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(dag_label_number);
        protocol::LocalizedLabel::native(&format!("Move {} node(s) by ({x_en}, {y_en})", self.ids.len()), &format!("{} Knoten um ({x_de}; {y_de}) verschieben", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Mutation
