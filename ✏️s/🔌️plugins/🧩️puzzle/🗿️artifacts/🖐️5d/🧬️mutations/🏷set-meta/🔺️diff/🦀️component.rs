use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dPlaySnapshot;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Puzzle5dMutation, base: &Puzzle5dPlaySnapshot) -> Puzzle5dDiff {
    <Puzzle5dMutation as protocol::Mutation<Puzzle5dPlaySnapshot>>::diff(mutation, base)
}
