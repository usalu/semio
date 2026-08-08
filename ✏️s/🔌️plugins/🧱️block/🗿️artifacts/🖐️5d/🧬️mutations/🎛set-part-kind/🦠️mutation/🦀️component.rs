//! 🎛 Block5d mutation — `SetPartKind` apply delegate.
use crate::artifacts::block5d::Block5dDefinition;
use crate::artifacts::block5d::mutations::Block5dMutation;

pub fn apply(projection: &mut Block5dDefinition, mutation: &Block5dMutation) {
    crate::artifacts::block5d::mutations::apply_block5d_mutation(projection, mutation);
}
