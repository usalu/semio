use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

pub fn inverse(base: &Block2dSnapshot, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    <Block2dMutation as protocol::Mutation<Block2dSnapshot>>::inverse(mutation, base)
}
