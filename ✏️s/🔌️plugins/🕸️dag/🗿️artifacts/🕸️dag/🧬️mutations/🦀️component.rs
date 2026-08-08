//! 🧬️ DAG artifact — kernel `DagMutation` facet.
pub use infinite_board_port_directed_dag::DagMutation;

use infinite_board_port_directed_dag::DagDocument;
use protocol::Mutation;

pub fn apply_dag_mutation(projection: &mut DagDocument, mutation: &DagMutation) {
    *projection = <DagMutation as Mutation<DagDocument>>::diff(mutation, projection).apply(projection);
}

pub fn inverse_dag_mutation(projection: &DagDocument, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as Mutation<DagDocument>>::inverse(mutation, projection)
}
