//! ➖ Block3d mutation — `RemoveCompatibilityRule` apply delegate.
use crate::artifacts::block3d::Block3dDefinition;
use crate::artifacts::block3d::mutations::Block3dMutation;

pub fn apply(projection: &mut Block3dDefinition, mutation: &Block3dMutation) {
    crate::artifacts::block3d::mutations::apply_block3d_mutation(projection, mutation);
}
