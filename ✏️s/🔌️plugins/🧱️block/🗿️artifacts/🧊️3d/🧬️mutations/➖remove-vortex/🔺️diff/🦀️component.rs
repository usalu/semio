use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dDefinition;
use crate::artifacts::block3d::mutations::Block3dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Block3dMutation, base: &Block3dDefinition) -> Block3dDiff {
    <Block3dMutation as protocol::Mutation<Block3dDefinition>>::diff(mutation, base)
}
