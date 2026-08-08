use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dDefinition;
use crate::artifacts::block5d::mutations::Block5dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Block5dMutation, base: &Block5dDefinition) -> Block5dDiff {
    <Block5dMutation as protocol::Mutation<Block5dDefinition>>::diff(mutation, base)
}
