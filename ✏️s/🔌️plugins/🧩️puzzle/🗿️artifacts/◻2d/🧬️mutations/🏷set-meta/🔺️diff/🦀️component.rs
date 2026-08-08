use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dProjection;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Puzzle2dMutation, base: &Puzzle2dProjection) -> Puzzle2dDiff {
    <Puzzle2dMutation as protocol::Mutation<Puzzle2dProjection>>::diff(mutation, base)
}
