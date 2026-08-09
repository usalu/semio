use crate::artifacts::dag::DagSnapshot;
use crate::artifacts::dag::mutations::DagMutation;

pub fn apply(snapshot: &mut DagSnapshot, mutation: &DagMutation) {
    crate::artifacts::dag::mutations::apply_dag_mutation(snapshot, mutation);
}
