//! 🎛 Block2d mutation — `SetNodeKind` apply delegate.
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

pub fn apply(projection: &mut Block2dSnapshot, mutation: &Block2dMutation) {
    crate::artifacts::block2d::mutations::apply_block2d_mutation(projection, mutation);
}
