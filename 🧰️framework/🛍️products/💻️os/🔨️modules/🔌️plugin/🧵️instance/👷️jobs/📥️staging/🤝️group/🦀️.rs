//! 🎻️ Staged slice of the `group` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🤝️ The instance's private child group publication and mounted command log turns.
use super::*;
use crate::child_content::MountedChildGroupIdentitySource;
use crate::child_content::*;
use crate::child_content::mounted_child_group_receipt_group::{MountedGroupMemberReceipts,MountedCommandEntry,MountedCommandEntrySource};

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🧪️ Borrows original completed-output ownership and refusal observations without widening production visibility.
    #[cfg(test)]
    pub(crate) fn test_original_owned_emit_observation(&self,operation:u64)->Option<(usize,usize,usize,bool,bool)>{
        let mounted=self.tool_operations.get(operation)?;
        let ArtifactToolCompletionValue::Emit(Ok(emit),_)=mounted.publication.as_ref()? else{return None;};
        Some((emit.artifact_mutations.len(),emit.owned_child_emits.len(),emit.owned_child_emits.as_ptr() as usize,mounted.terminal_fault.is_some(),mounted.cancellation_lease.as_ref()?.token.is_cancelled_now()))
    }

    pub(crate) fn next_closing_private_child_group<M: SpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>)->Option<u64>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|child_composition.private_child_groups.entry(index).and_then(|(operation,owner)|(owner.closing&&self.tool_operations.get(*operation).is_none_or(|mounted|!mounted.owned_child_result_pending)).then_some(*operation)))}
}
