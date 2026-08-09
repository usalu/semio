use crate::artifacts::puzzle5d::Puzzle5dPlaySnapshot;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;

pub fn inverse(base: &Puzzle5dPlaySnapshot, mutation: &Puzzle5dMutation) -> Vec<Puzzle5dMutation> {
    <Puzzle5dMutation as protocol::Mutation<Puzzle5dPlaySnapshot>>::inverse(mutation, base)
}
