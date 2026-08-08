use crate::artifacts::block2d::Block2dDefinition;
use crate::artifacts::block2d::mutations::Block2dMutation;

pub fn inverse(base: &Block2dDefinition, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    <Block2dMutation as protocol::Mutation<Block2dDefinition>>::inverse(mutation, base)
}
