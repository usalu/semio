//! 🎛 SHome mutation — `SetCatalogGeneration` apply delegate.
use crate::artifacts::home::SHomeDocument;
use crate::artifacts::home::mutations::SHomeMutation;

pub fn apply(projection: &mut SHomeDocument, mutation: &SHomeMutation) {
    crate::artifacts::home::mutations::apply_shome_mutation(projection, mutation);
}
