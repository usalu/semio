//! 🏕️ Staged slice of the `close-ladder` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedRegistry, PluginLifecycleStep, plugin_retirement_fault, plugin_turn_admitted};
use semio_framework_plugin_operation::ToolCancellationLease;
use semio_framework_os_kernel::SpaceMember;
use semio_framework_plugin_document_media::media_export::ActiveMediaExport;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
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
        if !pending.metadata_is_empty(){return pending.metadata_close_demands()}
        if let Some(owner)=pending.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(self.tool_cancellations.handle().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"active cancellation retirement requires its original canonical registry"))?,pending.original_body)}}
        if pending.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
        Ok(handoff_demand())
    }

    pub fn close_latest_wins_command_step(&mut self,operation_id:u64,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,PluginLifecycleFailure>{
        let idle=RetainedCloneProgress::default();
        let Some(pending)=self.latest_wins_commands.get_mut(operation_id)else{self.typed_operation_reservations[operation_id as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]=None;assert_eq!(self.latest_wins_order.pop(),Some(operation_id));return Ok(RetainedCloneStep::Progress(handoff()))};
        pending.validate_original(cx)?;
        if !pending.command_owners_are_empty(){return pending.close_step(grant).map(ToolOperations::<A>::lifecycle_retained)}
        if !pending.metadata_is_empty(){let before=cx.retained_progress();pending.close_metadata(cx)?;return Ok(RetainedCloneStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))}
        if pending.lease.is_some()||pending.cancellation_retirement.is_some(){let demand=if let Some(owner)=pending.cancellation_retirement.as_ref(){if owner.terminal_is_empty(){RetirementDemand{release_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()}}else{owner.retirement_demands(self.tool_cancellations.handle().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"active cancellation retirement requires its original canonical registry"))?,pending.original_body)?}}else{RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()}};if !plugin_turn_admitted(demand,grant)?{return Ok(RetainedCloneStep::Progress(idle))}let progress=if let Some(owner)=pending.cancellation_retirement.as_mut(){if owner.terminal_is_empty(){semio_framework_plugin_operation::original_lease_cursor_release(&mut pending.cancellation_retirement,grant)?}else{owner.close_step(self.tool_cancellations.handle().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"active cancellation retirement requires its original canonical registry"))?,grant)?.progress()}}else{let(owner,progress)=semio_framework_plugin_operation::original_lease_cursor_admit(&mut pending.lease,grant)?;pending.cancellation_retirement=owner;progress};return Ok(RetainedCloneStep::Progress(progress))}
        if !plugin_turn_admitted(handoff_demand(),grant)?{return Ok(RetainedCloneStep::Progress(idle))}
        drop(self.latest_wins_commands.remove(operation_id));self.typed_operation_reservations[operation_id as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]=None;assert_eq!(self.latest_wins_order.pop(),Some(operation_id));Ok(RetainedCloneStep::Progress(handoff()))
    }

    pub fn close_typed_operation_demands<M: SpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>, stores:&semio_framework_plugin_instance_stores::stores_components::InstanceStores<A>, body: usize) -> Result<RetirementDemand, Fault> {
        if let Some(source)=self.original_emission_admission.as_ref(){return if source.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Box<semio_framework_plugin_retained_command::typed_operation::original_emission::OriginalEmissionAdmission<A>>>(),release_bytes:std::mem::size_of_val(&**source),depth:1,..Default::default()})}else{source.close_demands(&self.tool_cancellations,body).map_err(plugin_retirement_fault)}}
        if self.original_emission_receipt.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_plugin_job::original_emission::OriginalEmissionReceipt>>(),depth:1,..Default::default()})}
        let Some((_, operation_id)) = self.tool_operations.next_id_from(0) else { return Ok(RetirementDemand::default()) };
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| plugin_sdk_fault("typed operation owner changed during bounded close"))?;
        if operation.pending_stream_abort.is_some(){return operation.stream_abort_close_demands(&stores.store,body).map_err(plugin_retirement_fault)}
        if operation.stage == MountedTypedCommandFullOperationStage::Retiring { return self.typed_operation_retirement_demands(child_composition, operation_id, stores, body); }
        operation.close_demands(body).map_err(plugin_retirement_fault)
    }
}
