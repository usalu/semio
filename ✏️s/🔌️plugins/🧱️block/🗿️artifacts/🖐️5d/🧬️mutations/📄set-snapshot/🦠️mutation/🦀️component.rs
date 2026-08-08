//! 📄 Block5d mutation — `SetSnapshot` apply delegate.
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

pub fn apply(projection: &mut Block5dSnapshot, mutation: &Block5dMutation) {
    crate::artifacts::block5d::mutations::apply_block5d_mutation(projection, mutation);
}
