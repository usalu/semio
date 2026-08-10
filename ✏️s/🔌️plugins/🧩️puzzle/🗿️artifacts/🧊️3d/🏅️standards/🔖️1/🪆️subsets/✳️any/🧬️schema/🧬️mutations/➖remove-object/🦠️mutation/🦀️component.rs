//! ➖ Puzzle3d mutation — `RemoveObject` apply delegate.
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;

pub fn apply(projection: &mut Puzzle3dSnapshot, mutation: &Puzzle3dMutation) {
    crate::artifacts::puzzle3d::mutations::apply_puzzle3d_mutation(projection, mutation);
}
