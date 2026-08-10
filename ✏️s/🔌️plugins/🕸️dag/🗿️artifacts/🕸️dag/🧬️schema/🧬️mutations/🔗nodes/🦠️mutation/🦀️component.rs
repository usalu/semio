use crate::artifacts::dag::DagSnapshot;
use crate::artifacts::dag::schema::mutations::DagMutation;

pub fn apply(projection: &mut DagSnapshot, mutation: &DagMutation) {
    crate::artifacts::dag::schema::mutations::apply_dag_mutation(projection, mutation);
}
