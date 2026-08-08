use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dProjection;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Puzzle3dMutation, base: &Puzzle3dProjection) -> Puzzle3dDiff {
    <Puzzle3dMutation as protocol::Mutation<Puzzle3dProjection>>::diff(mutation, base)
}
