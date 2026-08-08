//! 🎛 TrinityGraph mutation — `SetDataProperty` apply delegate.
use crate::artifacts::jack::TrinityGraphDocument;
use crate::artifacts::jack::mutations::TrinityGraphMutation;

pub fn apply(projection: &mut TrinityGraphDocument, mutation: &TrinityGraphMutation) {
    crate::artifacts::jack::mutations::apply_trinity_graph_mutation(projection, mutation);
}
