use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Puzzle3dMutation, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(mutation, base)
}
