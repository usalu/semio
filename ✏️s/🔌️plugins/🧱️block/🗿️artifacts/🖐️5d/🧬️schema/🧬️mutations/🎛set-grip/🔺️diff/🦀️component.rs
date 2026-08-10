use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Block5dMutation, base: &Block5dSnapshot) -> Block5dDiff {
    <Block5dMutation as protocol::Mutation<Block5dSnapshot>>::diff(mutation, base)
}
