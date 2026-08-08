use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dDefinition;
use crate::artifacts::block2d::mutations::Block2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Block2dMutation, base: &Block2dDefinition) -> Block2dDiff {
    <Block2dMutation as protocol::Mutation<Block2dDefinition>>::diff(mutation, base)
}
