//! 🎛 Puzzle3d mutation — `SetObject` apply delegate.
use crate::artifacts::puzzle3d::Puzzle3dProjection;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;

pub fn apply(projection: &mut Puzzle3dProjection, mutation: &Puzzle3dMutation) {
    crate::artifacts::puzzle3d::mutations::apply_puzzle3d_mutation(projection, mutation);
}
