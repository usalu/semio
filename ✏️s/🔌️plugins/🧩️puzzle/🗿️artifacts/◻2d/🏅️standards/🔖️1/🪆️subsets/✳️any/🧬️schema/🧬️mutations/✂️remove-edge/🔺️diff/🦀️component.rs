use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Puzzle2dMutation, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(mutation, base)
}
