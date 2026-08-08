use crate::artifacts::puzzle3d::Puzzle3dProjection;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;

pub fn inverse(base: &Puzzle3dProjection, mutation: &Puzzle3dMutation) -> Vec<Puzzle3dMutation> {
    <Puzzle3dMutation as protocol::Mutation<Puzzle3dProjection>>::inverse(mutation, base)
}
