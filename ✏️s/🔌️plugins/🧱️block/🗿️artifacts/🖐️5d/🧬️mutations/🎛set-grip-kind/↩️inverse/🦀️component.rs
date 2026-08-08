use crate::artifacts::block5d::Block5dDefinition;
use crate::artifacts::block5d::mutations::Block5dMutation;

pub fn inverse(base: &Block5dDefinition, mutation: &Block5dMutation) -> Vec<Block5dMutation> {
    <Block5dMutation as protocol::Mutation<Block5dDefinition>>::inverse(mutation, base)
}
