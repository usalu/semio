//! 🧬️ DAG artifact — document mutation dispatch.

use crate::artifacts::dag::schema::diff::text::{
    dag_edges_delta_from_collection_mutation, dag_nodes_delta_from_collection_mutation, DagDiff, DagFixtureEdgeList, DagNodeSpecList,
};
use crate::artifacts::dag::schema::DagArtifact;
use crate::artifacts::dag::{DagFixtureEdge, DagNodePatch, DagNodeSpec, DagSnapshot};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type DagEnvelope = store::DocumentEnvelope<DagSnapshot, DagMutation>;
pub type DagStore = store::DocumentStore<DagSnapshot, DagMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Typed DAG operation for the play app document store.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DagMutation {
    Nodes(CollectionMutation<String, DagNodeSpec, DagNodePatch>),
    Edges(CollectionMutation<String, DagFixtureEdge, infinite_board_port_directed_dag::DagEdgePatch>),
    SetNodes { nodes: Vec<DagNodeSpec> },
    SetEdges { edges: Vec<DagFixtureEdge> },
    SetSnapshot { snapshot: DagSnapshot },
}

pub fn apply_dag_mutation(snapshot: &mut DagSnapshot, mutation: &DagMutation) {
    *snapshot = <DagMutation as Mutation<DagSnapshot>>::diff(mutation, snapshot).apply(snapshot);
}

pub fn inverse_dag_mutation(snapshot: &DagSnapshot, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as Mutation<DagSnapshot>>::inverse(mutation, snapshot)
}

impl Mutation<DagSnapshot> for DagMutation {
    type Diff = DagDiff;

    fn diff(&self, snapshot: &DagSnapshot) -> DagDiff {
        match self {
            DagMutation::Nodes(operation) => DagDiff {
                nodes: Some(dag_nodes_delta_from_collection_mutation(&snapshot.nodes, operation)),
                ..Default::default()
            },
            DagMutation::Edges(operation) => DagDiff {
                edges: Some(dag_edges_delta_from_collection_mutation(&snapshot.edges, operation)),
                ..Default::default()
            },
            DagMutation::SetNodes { nodes } => DagDiff { set_nodes: Some(DagNodeSpecList { values: nodes.clone() }), ..Default::default() },
            DagMutation::SetEdges { edges } => DagDiff { set_edges: Some(DagFixtureEdgeList { values: edges.clone() }), ..Default::default() },
            DagMutation::SetSnapshot { snapshot } => DagDiff {
                artifact: Some(Box::new(DagArtifact::from_snapshot(snapshot.clone()))),
                ..Default::default()
            },
        }
    }

    fn inverse(&self, snapshot: &DagSnapshot) -> Vec<Self> {
        match self {
            DagMutation::Nodes(operation) => vec![DagMutation::Nodes(inverse_collection_mutation(&snapshot.nodes, operation))],
            DagMutation::Edges(operation) => vec![DagMutation::Edges(inverse_collection_mutation(&snapshot.edges, operation))],
            DagMutation::SetNodes { .. } => vec![DagMutation::SetNodes { nodes: snapshot.nodes.clone() }],
            DagMutation::SetEdges { .. } => vec![DagMutation::SetEdges { edges: snapshot.edges.clone() }],
            DagMutation::SetSnapshot { .. } => vec![DagMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
//#endregion 🔖️Mutations

//#region 🔖️WireBridge
fn kernel_mutation(operation: &DagMutation) -> infinite_board_port_directed_dag::DagMutation {
    use infinite_board_port_directed_dag::DagMutation as KernelMutation;
    match operation {
        DagMutation::Nodes(op) => KernelMutation::Nodes(op.clone()),
        DagMutation::Edges(op) => KernelMutation::Edges(op.clone()),
        DagMutation::SetNodes { nodes } => KernelMutation::SetNodes { nodes: nodes.clone() },
        DagMutation::SetEdges { edges } => KernelMutation::SetEdges { edges: edges.clone() },
        DagMutation::SetSnapshot { snapshot } => KernelMutation::SetSnapshot {
            snapshot: infinite_board_port_directed_dag::DagSnapshot::from(snapshot.clone()),
        },
    }
}

fn plugin_mutation(operation: infinite_board_port_directed_dag::DagMutation) -> DagMutation {
    use infinite_board_port_directed_dag::DagMutation as KernelMutation;
    match operation {
        KernelMutation::Nodes(op) => DagMutation::Nodes(op),
        KernelMutation::Edges(op) => DagMutation::Edges(op),
        KernelMutation::SetNodes { nodes } => DagMutation::SetNodes { nodes },
        KernelMutation::SetEdges { edges } => DagMutation::SetEdges { edges },
        KernelMutation::SetSnapshot { snapshot } => DagMutation::SetSnapshot { snapshot: snapshot.into() },
    }
}

impl protocol::OpText for DagMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(plugin_mutation(infinite_board_port_directed_dag::DagMutation::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        kernel_mutation(self).print_op()
    }
}

impl protocol::OpBinary for DagMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        kernel_mutation(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(plugin_mutation(infinite_board_port_directed_dag::DagMutation::decode_op(bytes)?))
    }
}
//#endregion 🔖️WireBridge
