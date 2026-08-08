use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;

pub fn inverse(base: &Puzzle3dSnapshot, mutation: &Puzzle3dMutation) -> Vec<Puzzle3dMutation> {
    <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::inverse(mutation, base)
}
