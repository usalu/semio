//! 📸 Sourcing mutation — `SetSnapshot` apply delegate.
use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::mutations::SourcingMutation;

pub fn apply(snapshot: &mut CurateSnapshot, mutation: &SourcingMutation) {
    crate::artifacts::curate::mutations::apply_sourcing_mutation(snapshot, mutation);
}
