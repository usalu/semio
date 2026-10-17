//! 🏕️ Staged slice of the `close-ladder` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedRegistry, PluginLifecycleStep, plugin_retirement_fault, plugin_turn_admitted};
use semio_framework_plugin_document_media::media_export::ActiveMediaExport;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::jobs_components::ToolOperations;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

#[doc(hidden)]
pub fn handoff() -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, ..Default::default() }
}

#[doc(hidden)]
pub fn handoff_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..Default::default() }
}

#[doc(hidden)]
pub fn release_demand(bytes: usize) -> RetirementDemand {
    RetirementDemand { release_bytes: bytes, depth: 1, ..Default::default() }
}

impl<A: ArtifactApp> ToolOperations<A> {
    pub fn lifecycle_retained(step: PluginLifecycleStep) -> RetainedCloneStep {
        match step {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => RetainedCloneStep::Progress(progress),
            PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => RetainedCloneStep::Progress(RetainedCloneProgress::default()),
        }
    }

    pub fn close_media_demands(&self,registry: &ArtifactFixedRegistry<ActiveMediaExport>, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, id)) = registry.next_id_from(0) else { return Ok(RetirementDemand::default()) };
        let Some(active) = registry.get(id) else { return Ok(handoff_demand()) };
        active.retirement_demands(body,&self.tool_cancellations)
    }

    pub fn close_latest_wins_command_demands(&self, operation_id: u64, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some(pending) = self.latest_wins_commands.get(operation_id) else { return Ok(handoff_demand()) };
        if !pending.command_owners_are_empty() { return pending.retirement_demands(body); }
        Ok(if pending.meta.actor.has_owner() { release_demand(pending.meta.actor.original_allocation_bytes()) } else { handoff_demand() })
    }

    pub fn close_latest_wins_command_step(&mut self, operation_id: u64, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, PluginLifecycleFailure> {
        let idle = RetainedCloneProgress::default();
        let Some(pending) = self.latest_wins_commands.get_mut(operation_id) else {
            self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
            return Ok(RetainedCloneStep::Progress(handoff()));
        };
        if !pending.command_owners_are_empty() { return pending.close_step(grant).map(ToolOperations::<A>::lifecycle_retained); }
        if pending.meta.actor.has_owner() {
            if !plugin_turn_admitted(release_demand(pending.meta.actor.original_allocation_bytes()), grant)? { return Ok(RetainedCloneStep::Progress(idle)); }
            return match std::mem::take(&mut pending.meta.actor).close_original_lease(grant) {
                Ok((original, mut progress)) => {
                    progress.released_bytes += original.as_ref().map_or(0, String::capacity);
                    drop(original);
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
                }
                Err((error, actor)) => {
                    pending.meta.actor = actor;
                    Err(error.into())
                }
            };
        }
        if !plugin_turn_admitted(handoff_demand(), grant)? { return Ok(RetainedCloneStep::Progress(idle)); }
        if let Some(mut lease) = pending.lease.take() { lease.finish(); }
        drop(self.latest_wins_commands.remove(operation_id));
        self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    pub fn close_typed_operation_demands<M: SpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>, body: usize) -> Result<RetirementDemand, Fault> {
        let Some((_, operation_id)) = self.tool_operations.next_id_from(0) else { return Ok(RetirementDemand::default()) };
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| plugin_sdk_fault("typed operation owner changed during bounded close"))?;
        if operation.stage == MountedTypedCommandFullOperationStage::Retiring { return self.typed_operation_retirement_demands(child_composition, operation_id, body); }
        operation.close_demands(body).map_err(plugin_retirement_fault)
    }
}
