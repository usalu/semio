//! 🎛 TrinityGraph mutation — `SetFixture` apply delegate.
use crate::artifacts::jack::JackSnapshot;
use crate::artifacts::jack::mutations::TrinityGraphMutation;

pub fn apply(projection: &mut JackSnapshot, mutation: &TrinityGraphMutation) {
    crate::artifacts::jack::mutations::apply_trinity_graph_mutation(projection, mutation);
}
