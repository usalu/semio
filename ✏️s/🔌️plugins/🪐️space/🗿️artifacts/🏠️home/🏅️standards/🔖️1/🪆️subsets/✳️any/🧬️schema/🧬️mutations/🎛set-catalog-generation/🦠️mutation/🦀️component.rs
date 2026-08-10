//! 🎛 SHome mutation — `SetCatalogGeneration` apply delegate.
use crate::artifacts::home::SHomeSnapshot;
use crate::artifacts::home::mutations::SHomeMutation;

pub fn apply(projection: &mut SHomeSnapshot, mutation: &SHomeMutation) {
    crate::artifacts::home::mutations::apply_shome_mutation(projection, mutation);
}
