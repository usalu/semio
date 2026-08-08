use crate::artifacts::puzzle2d::Puzzle2dProjection;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;

pub fn inverse(base: &Puzzle2dProjection, mutation: &Puzzle2dMutation) -> Vec<Puzzle2dMutation> {
    <Puzzle2dMutation as protocol::Mutation<Puzzle2dProjection>>::inverse(mutation, base)
}
