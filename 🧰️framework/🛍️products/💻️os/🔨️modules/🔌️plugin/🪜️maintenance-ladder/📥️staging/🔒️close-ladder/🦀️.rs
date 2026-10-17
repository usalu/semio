//! 🛶️ Staged slice of the `close-ladder` band of the mounted artifact instance: its `instance-ladders` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep, plugin_retirement_fault};
use semio_framework_plugin_instance_stores::stores_components::{ActionIngress, PeerRoster};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::{ArtifactOwnedDisposer, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use crate::ladder_bundle::{LadderCtx, LadderRef};
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_framework_plugin_decl_views::PeerPresenceRoot;
use semio_framework_plugin_peer::{PeerPresenceRootRetirement, PeerRosterPublication};
use semio_framework_plugin_instance_jobs::jobs_components::PendingReserved;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_operation::TOOL_CANCELLATION_SLOTS;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::idle;
use semio_framework_plugin_instance_jobs::jobs_close_ladder::{handoff, handoff_demand, release_demand};
use crate::ladders_envelope_ladder::{erased_demand, erased_step};
use crate::ladders_lifecycle_stores::ORIGINAL_STORE_LANES;

#[doc(hidden)]
pub fn released(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }
}

#[doc(hidden)]
pub fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained field depth overflow"))?;
    Ok(demand)
}

#[doc(hidden)]
pub fn child_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
    /// ♻️ Releases exactly one retained field owner under the caller's unchanged grant; a grant below the quote yields.
    pub fn retained_fields_close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let value_fault = |error: ValueError| plugin_sdk_fault(error.into_message());
        let idle = RetainedCloneProgress::default();
        if self.shared().retained_fields_own_terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let demand = self.shared().retained_fields_close_demand(grant.maximum_copy_bytes).map_err(value_fault)?;
        if grant.maximum_depth < demand.depth {
            return Err(plugin_sdk_fault("retained app fields exceed their admitted close depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let child = child_grant(grant);
        let done = |progress: RetainedCloneProgress| Ok(RetainedCloneStep::Progress(progress));
        if self.history.history_view.take().is_some() {
            return done(handoff());
        }
        if let Some(cache) = self.history.cache.take() {
            drop(cache);
            return done(handoff());
        }
        if !self.history.command_log.terminal_is_empty() {
            return self.history.command_log.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(value_fault);
        }
        if let Some(sequence) = self.history.history_dirty_sequences.iter().next().copied() {
            self.history.history_dirty_sequences.remove(&sequence);
            return done(handoff());
        }
        if let Some(pin) = self.child_composition.pending_child_pins.pop() {
            drop(pin);
            return done(handoff());
        }
        if !self.child_composition.composition.terminal_is_empty() {
            return self.child_composition.composition.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(value_fault);
        }
        if let Some((domain, hover)) = self.interaction.interaction_hover.pop_first() {
            let bytes = domain.capacity();
            drop((domain, hover));
            return done(released(bytes));
        }
        if let Some(domain) = self.interaction.interaction_ui_topology.keys().next().cloned() {
            let bytes = domain.capacity();
            drop(self.interaction.interaction_ui_topology.remove(&domain));
            drop(domain);
            return done(released(bytes));
        }
        if let Some(transaction) = self.txn.pending_transaction.as_mut() {
            if let Some(operation) = transaction.ops.pop() {
                drop(operation);
                return done(handoff());
            }
            if let Some(child_emit) = transaction.children.last_mut() {
                return Ok(match child_emit.close_one(child).map_err(value_fault)? {
                    RetainedCloneStep::Complete(progress) => {
                        transaction.children.pop();
                        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress })
                    }
                    step => step,
                });
            }
        }
        if let Some(transaction) = self.txn.pending_transaction.take() {
            drop(transaction);
            return done(handoff());
        }
        if let Some(proposal) = self.txn.pending_transaction_proposal.as_mut() {
            if let Some(operation) = proposal.local_ops.pop() {
                let bytes = operation.capacity();
                drop(operation);
                return done(released(bytes));
            }
            if let Some(foreign) = proposal.foreign.pop() {
                drop(foreign);
                return done(handoff());
            }
        }
        if let Some(proposal) = self.txn.pending_transaction_proposal.take() {
            drop(proposal);
            return done(handoff());
        }
        if let Some(presence) = self.roster.pending_presence.pop() {
            drop(presence);
            return done(handoff());
        }
        if let Some(wire) = self.pending.last_emit_wire.as_mut() {
            for bytes in [&mut wire.document, &mut wire.config, &mut wire.draft, &mut wire.children] {
                if bytes.capacity() != 0 {
                    let capacity = bytes.capacity();
                    drop(std::mem::take(bytes));
                    return done(released(capacity));
                }
            }
        }
        if self.pending.last_emit_wire.take().is_some() {
            return done(handoff());
        }
        if let Some(effect) = self.tools.typed_effect_outbox.pop() {
            drop(effect);
            return done(handoff());
        }
        if let Some(event) = self.tools.typed_event_outbox.pop() {
            drop(event);
            return done(handoff());
        }
        if let Some(scope) = self.tools.typed_ui_outbox.pop() {
            drop(scope);
            return done(handoff());
        }
        if std::mem::take(&mut self.tools.operation_progress_retired) {
            return done(handoff());
        }
        if let Some(completion) = self.tools.typed_completion_outbox.pop() {
            drop(completion);
            return done(handoff());
        }
        if let Some(composed) = self.tools.typed_composed_outbox.pop() {
            drop(composed);
            return done(handoff());
        }
        if !self.tools.registry.terminal_is_empty() {
            return self.tools.registry.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|fault| fault);
        }
        if self.tools.bounded_tool_proofs.pop_first().is_some() {
            return done(handoff());
        }
        if let Some(contract) = self.tools.bounded_tool_contracts.pop() {
            drop(contract);
            return done(handoff());
        }
        if let Some((_, registration)) = self.tools.framework_tool_registrations.pop_first() {
            drop(registration);
            return done(handoff());
        }
        if let Some((_, registration)) = self.tools.app_tool_registrations.pop_first() {
            drop(registration);
            return done(handoff());
        }
        if self.tools.tool_job_controller_id.capacity() != 0 {
            let capacity = self.tools.tool_job_controller_id.capacity();
            drop(std::mem::take(&mut self.tools.tool_job_controller_id));
            return done(released(capacity));
        }
        Ok(RetainedCloneStep::Complete(idle))
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    /// 📏️ Quotes the next retained-field turn on every independent axis; nothing is owed once every field is empty.
    pub fn retained_fields_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.history.history_view.is_some() || self.history.cache.is_some() {
            return Ok(handoff_demand());
        }
        if !self.history.command_log.terminal_is_empty() {
            return nested(self.history.command_log.retirement_demands()?);
        }
        if !self.history.history_dirty_sequences.is_empty() || !self.child_composition.pending_child_pins.is_empty() {
            return Ok(handoff_demand());
        }
        if !self.child_composition.composition.terminal_is_empty() {
            return nested(RetirementDemand {
                copy_bytes: self.child_composition.composition.next_close_copy_byte_demand()?,
                capacity_bytes: self.child_composition.composition.next_close_capacity_byte_demand(body)?,
                release_bytes: self.child_composition.composition.next_close_release_byte_demand()?,
                depth: self.child_composition.composition.next_close_depth_demand()?,
            })
            ;
        }
        if let Some((domain, _)) = self.interaction.interaction_hover.first_key_value() {
            return Ok(release_demand(domain.capacity()));
        }
        if let Some(domain) = self.interaction.interaction_ui_topology.keys().next() {
            return Ok(release_demand(domain.capacity()));
        }
        if let Some(transaction) = self.txn.pending_transaction.as_ref() {
            if !transaction.ops.is_empty() {
                return Ok(handoff_demand());
            }
            if let Some(child) = transaction.children.last() {
                return child.retirement_demands();
            }
            return Ok(handoff_demand());
        }
        if let Some(proposal) = self.txn.pending_transaction_proposal.as_ref() {
            if let Some(operation) = proposal.local_ops.last() {
                return Ok(release_demand(operation.capacity()));
            }
            return Ok(handoff_demand());
        }
        if !self.roster.pending_presence.is_empty() || self.pending.last_emit_wire.is_some() {
            if let Some(wire) = self.pending.last_emit_wire.as_ref() {
                if let Some(bytes) = [&wire.document, &wire.config, &wire.draft, &wire.children].into_iter().map(Vec::capacity).find(|capacity| *capacity != 0) {
                    return Ok(release_demand(bytes));
                }
            }
            return Ok(handoff_demand());
        }
        if self.tools.typed_effect_outbox.len() != 0 || self.tools.typed_event_outbox.len() != 0 || self.tools.typed_ui_outbox.len() != 0 || self.tools.operation_progress_retired || self.tools.typed_completion_outbox.len() != 0 || self.tools.typed_composed_outbox.len() != 0 {
            return Ok(handoff_demand());
        }
        if !self.tools.registry.terminal_is_empty() {
            return nested(self.tools.registry.retirement_demands());
        }
        if !self.tools.bounded_tool_proofs.is_empty() || !self.tools.bounded_tool_contracts.is_empty() || !self.tools.framework_tool_registrations.is_empty() || !self.tools.app_tool_registrations.is_empty() {
            return Ok(handoff_demand());
        }
        if self.tools.tool_job_controller_id.capacity() != 0 {
            return Ok(release_demand(self.tools.tool_job_controller_id.capacity()));
        }
        Ok(Default::default())
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    /// 🪹️ Every field owner the retained-field rung releases is empty.
    pub fn retained_fields_own_terminal_is_empty(&self) -> bool {
        self.history.cache.is_none()
            && self.history.history_view.is_none()
            && self.history.command_log.terminal_is_empty()
            && self.history.history_dirty_sequences.is_empty()
            && self.child_composition.pending_child_pins.is_empty()
            && self.child_composition.composition.terminal_is_empty()
            && self.interaction.interaction_hover.is_empty()
            && self.interaction.interaction_ui_topology.is_empty()
            && self.txn.pending_transaction.is_none()
            && self.txn.pending_transaction_proposal.is_none()
            && self.roster.pending_presence.is_empty()
            && self.pending.last_emit_wire.is_none()
            && self.tools.typed_effect_outbox.len() == 0
            && self.tools.typed_event_outbox.len() == 0
            && self.tools.typed_ui_outbox.len() == 0
            && !self.tools.operation_progress_retired
            && self.tools.typed_completion_outbox.len() == 0
            && self.tools.typed_composed_outbox.len() == 0
            && self.tools.registry.terminal_is_empty()
            && self.tools.bounded_tool_proofs.is_empty()
            && self.tools.bounded_tool_contracts.is_empty()
            && self.tools.framework_tool_registrations.is_empty()
            && self.tools.app_tool_registrations.is_empty()
            && self.tools.tool_job_controller_id.is_empty()
    }
}

/// 🪜️ One frontier of the close ladder, in the order the ladder drains them.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloseRung {
    Begin,
    DirectIngress,
    CommandPrune,
    GrantedMounted,
    PrivateChildGroups,
    PendingReserved,
    ReservedCommit,
    ReservedOutcome,
    ToolRuns,
    TimeTravel,
    ToolOverlay,
    LocalQuery,
    SnapshotReturns,
    PresenceLocalReads,
    MountedJobs,
    CancellationSlots,
    LatestWinsKeys,
    LatestWinsCommands,
    TypedOperations,
    InstanceOwner,
    AgentPreview,
    ConfigurationFailure,
    RenderFailure,
    MediaRefusal,
    ActionIngress,
    MediaExports,
    MediaClosures,
    Segments,
    SegmentClosures,
    SnapshotRetentions,
    ChildMemberRetirements,
    ChildRootRetirements,
    DetachChildRoot,
    ClosingChildMember,
    DetachChildren,
    PeerRoster,
    PeerRosterOutcomes,
    PeerPresenceRetirements,
    PresencePeerRetirements,
    DetachPeerPresence,
    EnvelopeIngress,
    EnvelopeDecode,
    StoreReplacement,
    DocumentArchive,
    WindowRetirements,
    OwnedStores,
    RetainedFields,
    CancellationIssuer,
    DocumentWindowAdmission,
    ToolLeaseAdmission,
    Uncovered,
}

const CANCELLATION_SLOT_SPAN: usize = TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS;

/// 🧺️ One owner of a fixed registry that the close ladder drains in place: its next turn quote, its granted turn and its terminal witness.
#[doc(hidden)]
pub trait ClosingOwner {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn closing_step(&mut self, _cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,PluginLifecycleFailure>;
    fn closing_terminal_is_empty(&self) -> bool;
}

impl ClosingOwner for ArtifactSnapshotCloseRetention {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.retirement_demands(body) }
    fn closing_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,PluginLifecycleFailure> { self.close_step(cx,grant).map_err(Into::into) }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

impl<A: ArtifactApp> ClosingOwner for PeerRosterPublication<A> {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let demand = self.close_demands(body)?;
        Ok(if demand == RetirementDemand::default() && !self.fault.terminal_is_empty() { RetirementDemand { copy_bytes: size_of::<Option<Fault>>(), depth: 1, ..Default::default() } } else { demand })
    }
    fn closing_step(&mut self, _cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,PluginLifecycleFailure> {
        match self.close_step(grant)? {
            PluginLifecycleStep::Complete(_) if !self.fault.terminal_is_empty() => {
                let (fault, progress) = self.take_original_fault(grant).map_err(plugin_retirement_fault)?;
                drop(fault);
                Ok(PluginLifecycleStep::Progress(progress))
            }
            step => Ok(step),
        }
    }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() && self.fault.terminal_is_empty() }
}

impl ClosingOwner for PeerPresenceRootRetirement {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.retirement_demands(body) }
    fn closing_step(&mut self, _cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,PluginLifecycleFailure> { self.close_step(grant).map_err(Into::into) }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

impl<P: Send + Sync + 'static> ClosingOwner for store::os_store::presence::PresencePeersRetirement<P> {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { erased_demand(self, body) }
    fn closing_step(&mut self, _cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,PluginLifecycleFailure> { erased_step(self, grant).map(|step| PluginLifecycleStep::Progress(step.progress())) }
    fn closing_terminal_is_empty(&self) -> bool { store::os_store::presence::PresencePeersRetirement::terminal_is_empty(self) }
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
    pub fn close_typed_operation_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=self.tools.close_typed_operation_demands(&*self.child_composition, grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|self.close_typed_operation_granted(same,actual).map(|step|(step,false)))
    }

    fn close_typed_operation_granted(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let Some((_, operation_id)) = self.tools.tool_operations.next_id_from(0) else { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())) };
        let operation = self.tools.tool_operations.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("typed operation owner changed during bounded close"))?;
        if operation.stage == MountedTypedCommandFullOperationStage::Retiring { return self.as_typed().retire_typed_operation_unit(operation_id,cx); }
        let step = operation.close_step(cx)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !operation.terminal_is_empty() { return Err(plugin_sdk_fault("typed operation close reported Complete without exact terminal emptiness").into()); }
        let operation = self.tools.tool_operations.remove(operation_id).ok_or_else(|| plugin_sdk_fault("terminal typed operation changed before exact removal"))?;
        self.tools.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    pub fn detach_child_root_step(&mut self) -> Result<RetainedCloneStep, Fault> {
        let generation = if self.child_composition.child_content_root.root.as_ref().is_some_and(|root| root.len != 0) {
            let generation = self.child_composition.child_content_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child root close generation exhausted before exact ownership transfer"))?;
            if !self.child_composition.child_content_retirements.can_insert(generation) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            Some(generation)
        } else {
            None
        };
        let previous = std::mem::replace(&mut *self.child_composition.child_content_root, ChildContentView::EMPTY);
        if let Some(generation) = generation {
            self.child_composition.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, true));
            self.child_composition.child_content_generation = generation;
        } else {
            drop(previous);
        }
        self.close.close_child_root_detached = true;
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    pub fn detach_peer_presence_step(&mut self) -> Result<RetainedCloneStep, Fault> {
        let generation = if self.roster.peer_presence.is_empty() {
            None
        } else {
            let generation = self.roster.peer_presence_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("peer-presence close generation exhausted before exact ownership transfer"))?;
            if !self.roster.peer_presence_retirements.can_insert(generation) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            Some(generation)
        };
        let previous = std::mem::replace(&mut *self.roster.peer_presence, PeerPresenceRoot::empty_shared());
        if let Some(generation) = generation {
            self.roster.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
            self.roster.peer_presence_generation = generation;
        } else {
            drop(previous);
        }
        self.close.close_peer_presence_detached = true;
        Ok(RetainedCloneStep::Progress(handoff()))
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    /// 🩺️ The ladder frontier a stalled close is parked on, named for deadline reports.
    pub fn close_rung_name(&self) -> String {
        format!("{:?}", self.close_rung())
    }

    pub fn close_rung(&self) -> CloseRung {
        if !self.close.close_started { return CloseRung::Begin; }
        if !self.stores.window_config_store.direct_ingress_terminal_is_empty() { return CloseRung::DirectIngress; }
        if self.history.pending_command_prune.is_some() || self.history.command_prune_requested() || !self.history.command_log.pruned_terminal_is_empty() { return CloseRung::CommandPrune; }
        if self.next_granted_mounted_retirement().is_some() { return CloseRung::GrantedMounted; }
        if !self.child_composition.private_child_groups_terminal_is_empty() { return CloseRung::PrivateChildGroups; }
        if self.pending.pending_reserved.next_id_from(0).is_some() { return CloseRung::PendingReserved; }
        if !self.pending.reserved_commits.is_empty() { return CloseRung::ReservedCommit; }
        if self.pending.reserved_commit_outcome.is_some() { return CloseRung::ReservedOutcome; }
        if !self.tool_runs.terminal_is_empty() { return CloseRung::ToolRuns; }
        if !self.time_travel.terminal_is_empty() { return CloseRung::TimeTravel; }
        if self.render_ref().tool_overlay_retirement_pending() { return CloseRung::ToolOverlay; }
        if self.interaction.local_interaction_query.is_some() { return CloseRung::LocalQuery; }
        if self.ingest.next_snapshot_read_return_stage(&*self.stores, &*self.interaction).is_some() { return CloseRung::SnapshotReturns; }
        if !self.stores.presence_store.retirement_started() && !self.stores.presence_store.local_read_maintenance_is_idle() { return CloseRung::PresenceLocalReads; }
        if self.tools.live_runtime_instance_id.is_some_and(|instance_id| !A::mounted_jobs_terminal_is_empty(instance_id)) { return CloseRung::MountedJobs; }
        if !self.tools.latest_wins_keys.terminal_is_empty() { return CloseRung::LatestWinsKeys; }
        if self.tools.latest_wins_order.items.front().is_some() { return CloseRung::LatestWinsCommands; }
        if !self.tools.tool_operations.is_empty() { return CloseRung::TypedOperations; }
        if !self.close.close_instance_operation_owner_drained||!self.tools.instance_operation_owner.inference_is_empty(){return CloseRung::InstanceOwner;}
        if self.tools.agent_lane_preview.is_some(){return CloseRung::AgentPreview;}
        if !self.pending.configuration_failure_terminal_is_empty(){return CloseRung::ConfigurationFailure;}
        if !self.pending.render_failure_terminal_is_empty(){return CloseRung::RenderFailure;}
        if !self.media.media_submission_refusal_terminal_is_empty(){return CloseRung::MediaRefusal;}
        if !self.ingress.action_ingress_sources_closed(){return CloseRung::ActionIngress;}
        if !self.media.media_exports.is_empty() { return CloseRung::MediaExports; }
        if !self.media.media_closures.is_empty() { return CloseRung::MediaClosures; }
        if !self.media.segmented_downloads.is_empty() { return CloseRung::Segments; }
        if !self.media.segmented_closures.is_empty() { return CloseRung::SegmentClosures; }
        if !self.media.snapshot_retirements.is_empty() { return CloseRung::SnapshotRetentions; }
        let child_root_detached = self.close.close_child_root_detached;
        if self.child_composition.child_member_retirements.empty_backing_byte_demand() != Some(0) && (child_root_detached || !self.child_composition.child_member_retirements.is_empty()) { return CloseRung::ChildMemberRetirements; }
        if self.child_composition.child_content_retirements.empty_backing_byte_demand() != Some(0) && (child_root_detached || !self.child_composition.child_content_retirements.is_empty()) { return CloseRung::ChildRootRetirements; }
        if !child_root_detached { return CloseRung::DetachChildRoot; }
        if self.close.close_child_member.is_some() { return CloseRung::ClosingChildMember; }
        if !self.child_composition.children.is_empty() { return CloseRung::DetachChildren; }
        if !self.roster.peer_roster_publications.is_empty() { return CloseRung::PeerRoster; }
        if !self.roster.peer_roster_outcomes.is_empty() { return CloseRung::PeerRosterOutcomes; }
        if !self.roster.peer_presence_retirements.is_empty() { return CloseRung::PeerPresenceRetirements; }
        if !self.roster.presence_peer_retirements.is_empty() { return CloseRung::PresencePeerRetirements; }
        if !self.close.close_peer_presence_detached { return CloseRung::DetachPeerPresence; }
        if !self.ingest.envelope_ingress.is_empty() { return CloseRung::EnvelopeIngress; }
        if !self.ingest.envelope_decode_terminal_is_empty() { return CloseRung::EnvelopeDecode; }
        if !self.ingest.store_replacement_jobs.is_empty() { return CloseRung::StoreReplacement; }
        if self.ingest.document_archive_loads.empty_backing_byte_demand() != Some(0) { return CloseRung::DocumentArchive; }
        if self.stores.retired_window_transient_stores.empty_backing_byte_demand() != Some(0) { return CloseRung::WindowRetirements; }
        if self.close.close_owned_stage < ORIGINAL_STORE_LANES.len() as u8 || self.close.close_owned_advance_ready { return CloseRung::OwnedStores; }
        if !self.retained_fields_own_terminal_is_empty() { return CloseRung::RetainedFields; }
        if self.tools.pending_tool_lease_admission.is_some()||self.media.pending_media_import_permit.is_some(){return CloseRung::ToolLeaseAdmission;}
        if self.pending.pending_document_window_admission.is_some(){return CloseRung::DocumentWindowAdmission;}
        if self.tools.tool_cancellations.ready() && self.close.close_cancellation_cursor < CANCELLATION_SLOT_SPAN { return CloseRung::CancellationSlots; }
        if !self.tools.tool_cancellations.terminal_is_empty() { return CloseRung::CancellationIssuer; }
        CloseRung::Uncovered
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    pub fn retained_fields_terminal_is_empty(&self) -> bool {
        self.retained_fields_own_terminal_is_empty() && self.child_composition.children.is_empty() && self.interaction.local_interaction_query.is_none() && self.ingest.snapshot_read_returns_terminal_is_empty(&*self.stores, &*self.interaction)
    }
}
