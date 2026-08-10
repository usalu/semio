use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;

pub fn inverse(base: &Puzzle2dSnapshot, mutation: &Puzzle2dMutation) -> Vec<Puzzle2dMutation> {
    <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::inverse(mutation, base)
}
