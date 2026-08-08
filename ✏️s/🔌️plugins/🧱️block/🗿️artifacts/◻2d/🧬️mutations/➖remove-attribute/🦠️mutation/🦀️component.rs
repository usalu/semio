//! ➖ Block2d mutation — `RemoveAttribute` apply delegate.
use crate::artifacts::block2d::Block2dDefinition;
use crate::artifacts::block2d::mutations::Block2dMutation;

pub fn apply(projection: &mut Block2dDefinition, mutation: &Block2dMutation) {
    crate::artifacts::block2d::mutations::apply_block2d_mutation(projection, mutation);
}
