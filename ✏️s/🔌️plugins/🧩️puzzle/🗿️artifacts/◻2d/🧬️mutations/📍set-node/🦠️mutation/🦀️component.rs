//! 📍 Puzzle2d mutation — `SetNode` apply delegate.
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;

pub fn apply(projection: &mut Puzzle2dSnapshot, mutation: &Puzzle2dMutation) {
    crate::artifacts::puzzle2d::mutations::apply_puzzle2d_mutation(projection, mutation);
}
