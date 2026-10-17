//! 🎻️ Staged slice of the `group` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🤝️ The instance's private child group publication and mounted command log turns.
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::Emit;
use store::MemberFactory;
use store::SpaceMember;
use semio_framework::FaultFrom;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use std::future::Future;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🧪️ Borrows original completed-output ownership and refusal observations without widening production visibility.
    #[cfg(test)]
    pub fn test_original_owned_emit_observation(&self,operation:u64)->Option<(usize,usize,usize,bool,bool)>{
        let mounted=self.tool_operations.get(operation)?;
        let ArtifactToolCompletionValue::Emit(Ok(emit),_)=mounted.publication.as_ref()? else{return None;};
        Some((emit.artifact_mutations.len(),emit.owned_child_emits.len(),emit.owned_child_emits.as_ptr() as usize,mounted.terminal_fault.is_some(),mounted.cancellation_lease.as_ref()?.token.is_cancelled_now()))
    }

    pub fn next_closing_private_child_group<M: SpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>)->Option<u64>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|child_composition.private_child_groups.entry(index).and_then(|(operation,owner)|(owner.closing&&self.tool_operations.get(*operation).is_none_or(|mounted|!mounted.owned_child_result_pending)).then_some(*operation)))}
}
