use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Block2dMutation, base: &Block2dSnapshot) -> Block2dDiff {
    <Block2dMutation as protocol::Mutation<Block2dSnapshot>>::diff(mutation, base)
}
