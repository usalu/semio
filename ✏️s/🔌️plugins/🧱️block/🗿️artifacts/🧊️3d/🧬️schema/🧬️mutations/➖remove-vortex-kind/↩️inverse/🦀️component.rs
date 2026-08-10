use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

pub fn inverse(base: &Block3dSnapshot, mutation: &Block3dMutation) -> Vec<Block3dMutation> {
    <Block3dMutation as protocol::Mutation<Block3dSnapshot>>::inverse(mutation, base)
}
