use crate::artifacts::puzzle5d::Puzzle5dPlayProjection;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;

pub fn inverse(base: &Puzzle5dPlayProjection, mutation: &Puzzle5dMutation) -> Vec<Puzzle5dMutation> {
    <Puzzle5dMutation as protocol::Mutation<Puzzle5dPlayProjection>>::inverse(mutation, base)
}
