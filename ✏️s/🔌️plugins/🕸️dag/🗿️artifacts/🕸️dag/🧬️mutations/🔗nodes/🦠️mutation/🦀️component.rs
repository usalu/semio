use crate::artifacts::dag::DagDocument;
use crate::artifacts::dag::mutations::DagMutation;

pub fn apply(projection: &mut DagDocument, mutation: &DagMutation) {
    crate::artifacts::dag::mutations::apply_dag_mutation(projection, mutation);
}
