//! 🎛 Block3d mutation — `SetObjectKind` apply delegate.
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

pub fn apply(projection: &mut Block3dSnapshot, mutation: &Block3dMutation) {
    crate::artifacts::block3d::mutations::apply_block3d_mutation(projection, mutation);
}
