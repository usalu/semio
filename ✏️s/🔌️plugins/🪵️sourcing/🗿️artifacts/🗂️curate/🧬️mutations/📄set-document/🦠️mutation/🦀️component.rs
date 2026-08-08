//! 📄 Sourcing mutation — `SetDocument` apply delegate.
use crate::artifacts::curate::SourcingDocument;
use crate::artifacts::curate::mutations::SourcingMutation;

pub fn apply(projection: &mut SourcingDocument, mutation: &SourcingMutation) {
    crate::artifacts::curate::mutations::apply_sourcing_mutation(projection, mutation);
}
