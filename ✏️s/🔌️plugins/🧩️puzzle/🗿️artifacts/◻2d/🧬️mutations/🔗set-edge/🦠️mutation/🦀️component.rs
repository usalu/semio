//! 🔗 Puzzle2d mutation — `SetEdge` apply delegate.
use crate::artifacts::puzzle2d::Puzzle2dProjection;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;

pub fn apply(projection: &mut Puzzle2dProjection, mutation: &Puzzle2dMutation) {
    crate::artifacts::puzzle2d::mutations::apply_puzzle2d_mutation(projection, mutation);
}
