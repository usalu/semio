use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

pub fn inverse(base: &Block5dSnapshot, mutation: &Block5dMutation) -> Vec<Block5dMutation> {
    <Block5dMutation as protocol::Mutation<Block5dSnapshot>>::inverse(mutation, base)
}
