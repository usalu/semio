use crate::artifacts::block3d::Block3dDefinition;
use crate::artifacts::block3d::mutations::Block3dMutation;

pub fn inverse(base: &Block3dDefinition, mutation: &Block3dMutation) -> Vec<Block3dMutation> {
    <Block3dMutation as protocol::Mutation<Block3dDefinition>>::inverse(mutation, base)
}
