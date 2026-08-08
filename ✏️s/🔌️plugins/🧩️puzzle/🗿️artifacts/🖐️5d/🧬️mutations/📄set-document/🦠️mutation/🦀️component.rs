//! 📄 Puzzle5d mutation — `SetDocument` apply delegate.
use crate::artifacts::puzzle5d::Puzzle5dPlayProjection;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;

pub fn apply(projection: &mut Puzzle5dPlayProjection, mutation: &Puzzle5dMutation) {
    crate::artifacts::puzzle5d::mutations::apply_puzzle5d_mutation(projection, mutation);
}
