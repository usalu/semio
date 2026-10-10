//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use super::*;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

fn handoff() -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, ..Default::default() }
}

fn released(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }
}

fn handoff_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..Default::default() }
}

fn release_demand(bytes: usize) -> RetirementDemand {
    RetirementDemand { release_bytes: bytes, depth: 1, ..Default::default() }
}

fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained field depth overflow"))?;
    Ok(demand)
}

fn child_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 📏️ Quotes the next retained-field turn on every independent axis; nothing is owed once every field is empty.
    pub(crate) fn retained_fields_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.history_view.is_some() || self.cache.is_some() {
            return Ok(handoff_demand());
        }
        if !self.command_log.terminal_is_empty() {
            return nested(self.command_log.retirement_demands()?);
        }
        if !self.history_dirty_sequences.is_empty() || !self.pending_child_pins.is_empty() {
            return Ok(handoff_demand());
        }
        if !self.composition.terminal_is_empty() {
            return nested(RetirementDemand {
                copy_bytes: self.composition.next_close_copy_byte_demand()?,
                capacity_bytes: self.composition.next_close_capacity_byte_demand(body)?,
                release_bytes: self.composition.next_close_release_byte_demand()?,
                depth: self.composition.next_close_depth_demand()?,
            })
            ;
        }
        if let Some((domain, _)) = self.interaction_hover.first_key_value() {
            return Ok(release_demand(domain.capacity()));
        }
        if let Some(domain) = self.interaction_ui_topology.keys().next() {
            return Ok(release_demand(domain.capacity()));
        }
        if let Some(transaction) = self.pending_transaction.as_ref() {
            if !transaction.ops.is_empty() {
                return Ok(handoff_demand());
            }
            if let Some(child) = transaction.children.last() {
                return Ok(release_demand(child.next_close_byte_demand()));
            }
            return Ok(handoff_demand());
        }
        if let Some(proposal) = self.pending_transaction_proposal.as_ref() {
            if let Some(operation) = proposal.local_ops.last() {
                return Ok(release_demand(operation.capacity()));
            }
            return Ok(handoff_demand());
        }
        if !self.pending_presence.is_empty() || self.last_emit_wire.is_some() {
            if let Some(wire) = self.last_emit_wire.as_ref() {
                if let Some(bytes) = [&wire.document, &wire.config, &wire.draft, &wire.children].into_iter().map(Vec::capacity).find(|capacity| *capacity != 0) {
                    return Ok(release_demand(bytes));
                }
            }
            return Ok(handoff_demand());
        }
        if self.typed_effect_outbox.len() != 0 || self.typed_event_outbox.len() != 0 || self.typed_ui_outbox.len() != 0 || self.operation_progress_retired || self.typed_completion_outbox.len() != 0 || self.typed_composed_outbox.len() != 0 {
            return Ok(handoff_demand());
        }
        if !self.registry.terminal_is_empty() {
            return nested(self.registry.retirement_demands());
        }
        if !self.bounded_tool_proofs.is_empty() || !self.bounded_tool_contracts.is_empty() || !self.framework_tool_registrations.is_empty() || !self.app_tool_registrations.is_empty() {
            return Ok(handoff_demand());
        }
        if self.tool_job_controller_id.capacity() != 0 {
            return Ok(release_demand(self.tool_job_controller_id.capacity()));
        }
        Ok(Default::default())
    }

    /// ♻️ Releases exactly one retained field owner under the caller's unchanged grant; a grant below the quote yields.
    pub(crate) fn retained_fields_close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let value_fault = |error: ValueError| plugin_sdk_fault(error.into_message());
        let idle = RetainedCloneProgress::default();
        if self.retained_fields_own_terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let demand = self.retained_fields_close_demand(grant.maximum_copy_bytes).map_err(value_fault)?;
        if grant.maximum_depth < demand.depth {
            return Err(plugin_sdk_fault("retained app fields exceed their admitted close depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let child = child_grant(grant);
        let done = |progress: RetainedCloneProgress| Ok(RetainedCloneStep::Progress(progress));
        if self.history_view.take().is_some() {
            return done(handoff());
        }
        if let Some(cache) = self.cache.take() {
            drop(cache);
            return done(handoff());
        }
        if !self.command_log.terminal_is_empty() {
            return self.command_log.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(value_fault);
        }
        if let Some(sequence) = self.history_dirty_sequences.iter().next().copied() {
            self.history_dirty_sequences.remove(&sequence);
            return done(handoff());
        }
        if let Some(pin) = self.pending_child_pins.pop() {
            drop(pin);
            return done(handoff());
        }
        if !self.composition.terminal_is_empty() {
            return self.composition.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(value_fault);
        }
        if let Some((domain, hover)) = self.interaction_hover.pop_first() {
            let bytes = domain.capacity();
            drop((domain, hover));
            return done(released(bytes));
        }
        if let Some(domain) = self.interaction_ui_topology.keys().next().cloned() {
            let bytes = domain.capacity();
            drop(self.interaction_ui_topology.remove(&domain));
            drop(domain);
            return done(released(bytes));
        }
        if let Some(transaction) = self.pending_transaction.as_mut() {
            if let Some(operation) = transaction.ops.pop() {
                drop(operation);
                return done(handoff());
            }
            if let Some(child_emit) = transaction.children.last_mut() {
                return Ok(match child_emit.close_one(child) {
                    RetainedCloneStep::Complete(_) => {
                        transaction.children.pop();
                        RetainedCloneStep::Progress(handoff())
                    }
                    step => step,
                });
            }
        }
        if let Some(transaction) = self.pending_transaction.take() {
            drop(transaction);
            return done(handoff());
        }
        if let Some(proposal) = self.pending_transaction_proposal.as_mut() {
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
        if let Some(proposal) = self.pending_transaction_proposal.take() {
            drop(proposal);
            return done(handoff());
        }
        if let Some(presence) = self.pending_presence.pop() {
            drop(presence);
            return done(handoff());
        }
        if let Some(wire) = self.last_emit_wire.as_mut() {
            for bytes in [&mut wire.document, &mut wire.config, &mut wire.draft, &mut wire.children] {
                if bytes.capacity() != 0 {
                    let capacity = bytes.capacity();
                    drop(std::mem::take(bytes));
                    return done(released(capacity));
                }
            }
        }
        if self.last_emit_wire.take().is_some() {
            return done(handoff());
        }
        if let Some(effect) = self.typed_effect_outbox.pop() {
            drop(effect);
            return done(handoff());
        }
        if let Some(event) = self.typed_event_outbox.pop() {
            drop(event);
            return done(handoff());
        }
        if let Some(scope) = self.typed_ui_outbox.pop() {
            drop(scope);
            return done(handoff());
        }
        if std::mem::take(&mut self.operation_progress_retired) {
            return done(handoff());
        }
        if let Some(completion) = self.typed_completion_outbox.pop() {
            drop(completion);
            return done(handoff());
        }
        if let Some(composed) = self.typed_composed_outbox.pop() {
            drop(composed);
            return done(handoff());
        }
        if !self.registry.terminal_is_empty() {
            return self.registry.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|fault| fault);
        }
        if self.bounded_tool_proofs.pop_first().is_some() {
            return done(handoff());
        }
        if let Some(contract) = self.bounded_tool_contracts.pop() {
            drop(contract);
            return done(handoff());
        }
        if let Some((_, registration)) = self.framework_tool_registrations.pop_first() {
            drop(registration);
            return done(handoff());
        }
        if let Some((_, registration)) = self.app_tool_registrations.pop_first() {
            drop(registration);
            return done(handoff());
        }
        if self.tool_job_controller_id.capacity() != 0 {
            let capacity = self.tool_job_controller_id.capacity();
            drop(std::mem::take(&mut self.tool_job_controller_id));
            return done(released(capacity));
        }
        Ok(RetainedCloneStep::Complete(idle))
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🪹️ Every field owner the retained-field rung releases is empty.
    pub(crate) fn retained_fields_own_terminal_is_empty(&self) -> bool {
        self.cache.is_none()
            && self.history_view.is_none()
            && self.command_log.terminal_is_empty()
            && self.history_dirty_sequences.is_empty()
            && self.pending_child_pins.is_empty()
            && self.composition.terminal_is_empty()
            && self.interaction_hover.is_empty()
            && self.interaction_ui_topology.is_empty()
            && self.pending_transaction.is_none()
            && self.pending_transaction_proposal.is_none()
            && self.pending_presence.is_empty()
            && self.last_emit_wire.is_none()
            && self.typed_effect_outbox.len() == 0
            && self.typed_event_outbox.len() == 0
            && self.typed_ui_outbox.len() == 0
            && !self.operation_progress_retired
            && self.typed_completion_outbox.len() == 0
            && self.typed_composed_outbox.len() == 0
            && self.registry.terminal_is_empty()
            && self.bounded_tool_proofs.is_empty()
            && self.bounded_tool_contracts.is_empty()
            && self.framework_tool_registrations.is_empty()
            && self.app_tool_registrations.is_empty()
            && self.tool_job_controller_id.is_empty()
    }
}

/// 🪜️ One frontier of the close ladder, in the order the ladder drains them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CloseRung {
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
    Uncovered,
}

const CANCELLATION_SLOT_SPAN: usize = TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS;

/// 🧺️ One owner of a fixed registry that the close ladder drains in place: its next turn quote, its granted turn and its terminal witness.
trait ClosingOwner {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn closing_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn closing_terminal_is_empty(&self) -> bool;
}

impl ClosingOwner for ArtifactSnapshotCloseRetention {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.retirement_demands(body) }
    fn closing_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> { self.close_step(grant) }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

impl<A: ArtifactApp> ClosingOwner for PeerRosterPublication<A> {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close_demands(body) }
    fn closing_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> { self.close_step(grant) }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

impl ClosingOwner for PeerPresenceRootRetirement {
    fn closing_demands(&self, _body: usize) -> Result<RetirementDemand, ValueError> { self.retirement_demands() }
    fn closing_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> { self.close_step(grant).map(|step| PluginLifecycleStep::Progress(step.progress())) }
    fn closing_terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
}

impl<P: Send + Sync + 'static> ClosingOwner for store::PresencePeersRetirement<P> {
    fn closing_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { super::envelope_ladder::erased_demand(self, body) }
    fn closing_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> { super::envelope_ladder::erased_step(self, grant).map(|step| PluginLifecycleStep::Progress(step.progress())) }
    fn closing_terminal_is_empty(&self) -> bool { store::PresencePeersRetirement::terminal_is_empty(self) }
}

/// 📏️ Quotes the registry owner at `cursor`, or one hand-off when it only awaits removal.
fn registry_demands<T: ClosingOwner>(registry: &ArtifactFixedRegistry<T>, cursor: usize, body: usize) -> Result<RetirementDemand, ValueError> {
    let Some((_, id)) = registry.next_id_from(cursor) else { return Ok(RetirementDemand::default()) };
    let Some(owner) = registry.get(id) else { return Ok(handoff_demand()) };
    if owner.closing_terminal_is_empty() { return Ok(handoff_demand()); }
    owner.closing_demands(body)
}

/// 🎟️ Advances the registry owner at `cursor` by one granted turn and removes it at its terminal-empty witness; answers the removed id.
fn registry_step<T: ClosingOwner>(registry: &mut ArtifactFixedRegistry<T>, cursor: &mut usize, grant: RetainedCloneGrant) -> Result<(PluginLifecycleStep, Option<u64>), Fault> {
    let idle = RetainedCloneProgress::default();
    let Some((index, id)) = registry.next_id_from(*cursor) else { return Ok((PluginLifecycleStep::Complete(idle), None)) };
    let owner = registry.get_mut(id).ok_or_else(|| plugin_sdk_fault("closing registry owner changed during one fixed step"))?;
    let progress = if owner.closing_terminal_is_empty() {
        idle
    } else {
        match owner.closing_step(grant)? {
            PluginLifecycleStep::Progress(progress) => {
                if !owner.closing_terminal_is_empty() {
                    *cursor = index;
                    return Ok((PluginLifecycleStep::Progress(progress), None));
                }
                progress
            }
            PluginLifecycleStep::Complete(progress) => {
                if !owner.closing_terminal_is_empty() { return Err(plugin_sdk_fault("closing registry owner reported Complete without its exact terminal-empty witness")); }
                progress
            }
            blocked => {
                *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok((blocked, None));
            }
        }
    };
    drop(registry.remove(id));
    *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
    Ok((PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }), Some(id)))
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    fn close_rung(&self) -> CloseRung {
        if !self.close_started { return CloseRung::Begin; }
        if !self.window_config_store.direct_ingress_terminal_is_empty() { return CloseRung::DirectIngress; }
        if self.pending_command_prune.is_some() || self.command_prune_requested() || !self.command_log.pruned_terminal_is_empty() { return CloseRung::CommandPrune; }
        if self.next_granted_mounted_retirement().is_some() { return CloseRung::GrantedMounted; }
        if !self.private_child_groups_terminal_is_empty() { return CloseRung::PrivateChildGroups; }
        if self.pending_reserved.next_id_from(0).is_some() { return CloseRung::PendingReserved; }
        if !self.reserved_commits.is_empty() { return CloseRung::ReservedCommit; }
        if self.reserved_commit_outcome.is_some() { return CloseRung::ReservedOutcome; }
        if !self.tool_runs.terminal_is_empty() { return CloseRung::ToolRuns; }
        if !self.time_travel.terminal_is_empty() { return CloseRung::TimeTravel; }
        if self.tool_overlay_retirement_pending() { return CloseRung::ToolOverlay; }
        if self.local_interaction_query.is_some() { return CloseRung::LocalQuery; }
        if self.next_snapshot_read_return_stage().is_some() { return CloseRung::SnapshotReturns; }
        if !self.presence_store.retirement_started() && !self.presence_store.local_read_maintenance_is_idle() { return CloseRung::PresenceLocalReads; }
        if self.live_runtime_instance_id.is_some_and(|instance_id| !A::mounted_jobs_terminal_is_empty(instance_id)) { return CloseRung::MountedJobs; }
        if self.close_cancellation_cursor < CANCELLATION_SLOT_SPAN { return CloseRung::CancellationSlots; }
        if !self.latest_wins_keys.terminal_is_empty() { return CloseRung::LatestWinsKeys; }
        if self.latest_wins_order.items.front().is_some() { return CloseRung::LatestWinsCommands; }
        if !self.tool_operations.is_empty() { return CloseRung::TypedOperations; }
        if !self.close_instance_operation_owner_drained { return CloseRung::InstanceOwner; }
        if !self.media_exports.is_empty() { return CloseRung::MediaExports; }
        if !self.media_closures.is_empty() { return CloseRung::MediaClosures; }
        if !self.segmented_downloads.is_empty() { return CloseRung::Segments; }
        if !self.segmented_closures.is_empty() { return CloseRung::SegmentClosures; }
        if !self.snapshot_retirements.is_empty() { return CloseRung::SnapshotRetentions; }
        if !self.child_member_retirements.is_empty() { return CloseRung::ChildMemberRetirements; }
        if !self.child_content_retirements.is_empty() { return CloseRung::ChildRootRetirements; }
        if !self.close_child_root_detached { return CloseRung::DetachChildRoot; }
        if self.close_child_member.is_some() { return CloseRung::ClosingChildMember; }
        if !self.children.is_empty() { return CloseRung::DetachChildren; }
        if !self.peer_roster_publications.is_empty() { return CloseRung::PeerRoster; }
        if !self.peer_roster_outcomes.is_empty() { return CloseRung::PeerRosterOutcomes; }
        if !self.peer_presence_retirements.is_empty() { return CloseRung::PeerPresenceRetirements; }
        if !self.presence_peer_retirements.is_empty() { return CloseRung::PresencePeerRetirements; }
        if !self.close_peer_presence_detached { return CloseRung::DetachPeerPresence; }
        if !self.envelope_ingress.is_empty() { return CloseRung::EnvelopeIngress; }
        if !self.envelope_decode_terminal_is_empty() { return CloseRung::EnvelopeDecode; }
        if !self.store_replacement_jobs.is_empty() { return CloseRung::StoreReplacement; }
        if !self.document_archive_loads.is_empty() { return CloseRung::DocumentArchive; }
        if !self.retired_window_transient_stores.is_empty() { return CloseRung::WindowRetirements; }
        if self.close_owned_stage < ORIGINAL_STORE_LANES.len() as u8 || self.close_owned_advance_ready { return CloseRung::OwnedStores; }
        if !self.retained_fields_own_terminal_is_empty() { return CloseRung::RetainedFields; }
        CloseRung::Uncovered
    }

    fn lifecycle_retained(step: PluginLifecycleStep) -> RetainedCloneStep {
        match step {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => RetainedCloneStep::Progress(progress),
            PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => RetainedCloneStep::Progress(RetainedCloneProgress::default()),
        }
    }

    fn uncovered_close_frontier() -> ValueError {
        ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plugin close retains an original frontier without its granted owner authority")
    }

    fn close_media_demands(registry: &ArtifactFixedRegistry<ActiveMediaExport>, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, id)) = registry.next_id_from(0) else { return Ok(RetirementDemand::default()) };
        let Some(active) = registry.get(id) else { return Ok(handoff_demand()) };
        active.retirement_demands(body)
    }

    fn close_segment_demands(registry: &ArtifactFixedRegistry<ArtifactDownloadOutput>) -> RetirementDemand {
        let Some((_, id)) = registry.next_id_from(0) else { return RetirementDemand::default() };
        if registry.get(id).is_some_and(|output| output.chunks.chunks_remaining() != 0) { release_demand(ARTIFACT_OUTPUT_CHUNK_BYTES) } else { handoff_demand() }
    }

    fn close_segment_step(registry: &mut ArtifactFixedRegistry<ArtifactDownloadOutput>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let idle = RetainedCloneProgress::default();
        let Some((_, id)) = registry.next_id_from(0) else { return Ok(RetainedCloneStep::Progress(idle)) };
        let output = registry.get(id).ok_or_else(|| plugin_sdk_fault("segmented close authority changed during one fixed step"))?;
        if output.chunks.chunks_remaining() != 0 {
            if !plugin_turn_admitted(release_demand(ARTIFACT_OUTPUT_CHUNK_BYTES), grant)? { return Ok(RetainedCloneStep::Progress(idle)); }
            return Ok(RetainedCloneStep::Progress(match output.chunks.close_take_chunk()? {
                Some(chunk) => released(chunk.len()),
                None => idle,
            }));
        }
        if !plugin_turn_admitted(handoff_demand(), grant)? { return Ok(RetainedCloneStep::Progress(idle)); }
        let output = registry.remove(id).ok_or_else(|| plugin_sdk_fault("terminal segmented owner changed before exact removal"))?;
        if !output.terminal_is_empty() {
            registry.insert_admitted(id, output);
            return Err(plugin_sdk_fault("segmented owner reached terminal without an empty exact chunk witness"));
        }
        drop(output);
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    fn close_media_step(&mut self, closures: bool, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let registry = if closures { &mut self.media_closures } else { &mut self.media_exports };
        let Some((_, id)) = registry.next_id_from(0) else { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())) };
        let active = registry.get_mut(id).ok_or_else(|| plugin_sdk_fault("media close authority changed during one fixed step"))?;
        let progress = match active.close_step(grant)? {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => progress,
            PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),
        };
        if !active.terminal_is_empty()? { return Ok(RetainedCloneStep::Progress(progress)); }
        let registry = if closures { &self.media_closures } else { &self.media_exports };
        let active = registry.get(id).ok_or_else(|| plugin_sdk_fault("completed media close lost its owner"))?;
        self.quarantine_media_snapshot(id, active).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let registry = if closures { &mut self.media_closures } else { &mut self.media_exports };
        drop(registry.remove(id).ok_or_else(|| plugin_sdk_fault("terminal media owner changed before exact removal"))?);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    fn close_latest_wins_command_demands(&self, operation_id: u64, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some(pending) = self.latest_wins_commands.get(operation_id) else { return Ok(handoff_demand()) };
        if !pending.command_owners_are_empty() { return pending.retirement_demands(body); }
        Ok(if pending.meta.actor.capacity() != 0 { plugin_text_retirement_demand(&pending.meta.actor) } else { handoff_demand() })
    }

    fn close_latest_wins_command_step(&mut self, operation_id: u64, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let idle = RetainedCloneProgress::default();
        let Some(pending) = self.latest_wins_commands.get_mut(operation_id) else {
            self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
            return Ok(RetainedCloneStep::Progress(handoff()));
        };
        if !pending.command_owners_are_empty() { return pending.close_step(grant); }
        if pending.meta.actor.capacity() != 0 {
            return Ok(RetainedCloneStep::Progress(plugin_text_close(&mut pending.meta.actor, grant).unwrap_or(idle)));
        }
        if !plugin_turn_admitted(handoff_demand(), grant)? { return Ok(RetainedCloneStep::Progress(idle)); }
        if let Some(lease) = pending.lease.take() { lease.finish(); }
        drop(self.latest_wins_commands.remove(operation_id));
        self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    fn close_typed_operation_demands(&self, body: usize) -> Result<RetirementDemand, Fault> {
        let Some((_, operation_id)) = self.tool_operations.next_id_from(0) else { return Ok(RetirementDemand::default()) };
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| plugin_sdk_fault("typed operation owner changed during bounded close"))?;
        if operation.stage == MountedTypedCommandFullOperationStage::Retiring { return self.typed_operation_retirement_demands(operation_id, body); }
        operation.close_demands(body).map_err(plugin_retirement_fault)
    }

    fn close_typed_operation_unit(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let Some((_, operation_id)) = self.tool_operations.next_id_from(0) else { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())) };
        let operation = self.tool_operations.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("typed operation owner changed during bounded close"))?;
        if operation.stage == MountedTypedCommandFullOperationStage::Retiring { return self.retire_typed_operation_unit(operation_id, grant); }
        let step = operation.close_step(grant)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !operation.terminal_is_empty() { return Err(plugin_sdk_fault("typed operation close reported Complete without exact terminal emptiness")); }
        let operation = self.tool_operations.remove(operation_id).ok_or_else(|| plugin_sdk_fault("terminal typed operation changed before exact removal"))?;
        self.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    fn detach_child_root_step(&mut self) -> Result<RetainedCloneStep, Fault> {
        let generation = if self.child_content_root.root.as_ref().is_some_and(|root| root.len != 0) {
            let generation = self.child_content_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child root close generation exhausted before exact ownership transfer"))?;
            if !self.child_content_retirements.can_insert(generation) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            Some(generation)
        } else {
            None
        };
        let previous = std::mem::replace(&mut *self.child_content_root, ChildContentView::EMPTY);
        if let Some(generation) = generation {
            self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, true));
            self.child_content_generation = generation;
        } else {
            drop(previous);
        }
        self.close_child_root_detached = true;
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    fn detach_peer_presence_step(&mut self) -> Result<RetainedCloneStep, Fault> {
        let generation = if self.peer_presence.is_empty() {
            None
        } else {
            let generation = self.peer_presence_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("peer-presence close generation exhausted before exact ownership transfer"))?;
            if !self.peer_presence_retirements.can_insert(generation) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            Some(generation)
        };
        let previous = std::mem::replace(&mut *self.peer_presence, PeerPresenceRoot::empty_shared());
        if let Some(generation) = generation {
            self.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
            self.peer_presence_generation = generation;
        } else {
            drop(previous);
        }
        self.close_peer_presence_detached = true;
        Ok(RetainedCloneStep::Progress(handoff()))
    }

    /// 📏️ Quotes the next close turn on every independent axis; a closed app owes nothing.
    pub(crate) fn close_ladder_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let fault = |fault: Fault| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, fault.message);
        match self.close_rung() {
            CloseRung::Begin | CloseRung::PendingReserved | CloseRung::ReservedCommit | CloseRung::ReservedOutcome | CloseRung::CancellationSlots | CloseRung::DetachChildRoot | CloseRung::DetachChildren | CloseRung::PeerRosterOutcomes | CloseRung::DetachPeerPresence => Ok(handoff_demand()),
            CloseRung::DirectIngress => self.window_config_store.direct_ingress_demands(body),
            CloseRung::CommandPrune => self.command_prune_demands(true, body),
            CloseRung::GrantedMounted => self.granted_mounted_retirement_demands(body),
            CloseRung::PrivateChildGroups => self.private_child_group_close_demands(body),
            CloseRung::ToolRuns => self.tool_run_retirement_demand(body),
            CloseRung::TimeTravel => self.time_travel_retirement_demands(body),
            CloseRung::ToolOverlay => self.tool_overlay_retirement_demand(body),
            CloseRung::LocalQuery => self.local_interaction_query.as_ref().map_or(Ok(Default::default()), |query| if query.is_closing() { query.retirement_demands(body) } else { Ok(handoff_demand()) }),
            CloseRung::SnapshotReturns => self.snapshot_read_returns_demands(body),
            CloseRung::PresenceLocalReads => self.presence_store.maintenance_local_reads_demands(body),
            CloseRung::MountedJobs => self.live_runtime_instance_id.map_or(Ok(Default::default()), |instance_id| A::mounted_job_close_demands(instance_id, body)),
            CloseRung::LatestWinsKeys => self.latest_wins_keys.advance_demands(),
            CloseRung::LatestWinsCommands => self.latest_wins_order.items.front().copied().map_or(Ok(Default::default()), |operation_id| self.close_latest_wins_command_demands(operation_id, body)),
            CloseRung::TypedOperations => self.close_typed_operation_demands(body).map_err(fault),
            CloseRung::InstanceOwner => self.instance_operation_owner.retirement_demands(body),
            CloseRung::MediaExports => Self::close_media_demands(&self.media_exports, body),
            CloseRung::MediaClosures => Self::close_media_demands(&self.media_closures, body),
            CloseRung::Segments => Ok(Self::close_segment_demands(&self.segmented_downloads)),
            CloseRung::SegmentClosures => Ok(Self::close_segment_demands(&self.segmented_closures)),
            CloseRung::SnapshotRetentions => registry_demands(&self.snapshot_retirements, self.close_snapshot_cursor, body),
            CloseRung::ChildMemberRetirements => {
                let member = self.child_member_retirement_demands(body)?;
                if self.child_content_retirements.is_empty() { Ok(member) } else { Ok(widest_demand(member, self.child_root_retirement_demands(body)?)) }
            }
            CloseRung::ChildRootRetirements => self.child_root_retirement_demands(body),
            CloseRung::ClosingChildMember => self.close_child_member.as_ref().map_or(Ok(Default::default()), |retirement| retirement.retirement_demands(body)),
            CloseRung::PeerRoster => registry_demands(&self.peer_roster_publications, self.close_peer_roster_cursor, body),
            CloseRung::PeerPresenceRetirements => registry_demands(&self.peer_presence_retirements, self.close_peer_presence_cursor, body),
            CloseRung::PresencePeerRetirements => registry_demands(&self.presence_peer_retirements, self.close_presence_peer_cursor, body),
            CloseRung::EnvelopeIngress => self.envelope_ingress_demands(true, body),
            CloseRung::EnvelopeDecode => self.envelope_decode_close_demands(body),
            CloseRung::StoreReplacement => self.store_replacement_demands(true, body),
            CloseRung::DocumentArchive => self.document_archive_demands(true, body),
            CloseRung::WindowRetirements => self.document_windows_retirement_demands(body, true),
            CloseRung::OwnedStores => self.close_owned_stores_demands(body),
            CloseRung::RetainedFields => self.retained_fields_close_demand(body),
            CloseRung::Uncovered => if self.close_terminal_is_empty() { Ok(Default::default()) } else { Err(Self::uncovered_close_frontier()) },
        }
    }

    /// 🎟️ Advances exactly one frontier under the caller's unchanged grant; a closed app answers `Complete`.
    pub(crate) fn close_ladder_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let value_fault = |error: ValueError| plugin_sdk_fault(error.into_message());
        let idle = RetainedCloneProgress::default();
        let child = child_grant(grant);
        match self.close_rung() {
            CloseRung::Begin => {
                self.tool_cancellations.cancel_scope_generation();
                self.peer_roster_scope.cancel_now();
                self.local_interaction_authority.close();
                if let Some(query) = self.local_interaction_query.as_mut() { query.begin_close(); }
                self.tool_runs.begin_close();
                self.latest_wins_keys.begin_close();
                self.close_started = true;
                self.begin_gesture_retirement();
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::DirectIngress => self.window_config_store.close_direct_ingress(grant).map_err(value_fault),
            CloseRung::CommandPrune => self.advance_command_prune_step(grant, true).map(Self::lifecycle_retained),
            CloseRung::GrantedMounted => self.granted_mounted_retirement_step(grant).map_err(value_fault),
            CloseRung::PrivateChildGroups => Ok(match self.close_private_child_group_step(grant)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Progress(progress),
                semio_framework_job::InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(idle),
                semio_framework_job::InteractiveJobCloseStep::Refused(kind) => return Err(plugin_sdk_fault(format!("private child group close was refused: {kind:?}"))),
            }),
            CloseRung::PendingReserved => {
                if let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                    if let Some(pending) = self.pending_reserved.remove(id) { pending.permit.finish(); }
                }
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::ReservedCommit => {
                if let Some(commit) = self.reserved_commits.pop_front() { commit.permit.finish(); }
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::ReservedOutcome => {
                drop(self.reserved_commit_outcome.take());
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::ToolRuns => self.tool_run_close_step(grant),
            CloseRung::TimeTravel => self.time_travel_close_step(grant),
            CloseRung::ToolOverlay => self.tool_overlay_retirement_step(grant).map_err(value_fault),
            CloseRung::LocalQuery => {
                let Some(query) = self.local_interaction_query.as_mut() else { return Ok(RetainedCloneStep::Progress(idle)) };
                if !query.is_closing() {
                    query.begin_close();
                    return Ok(RetainedCloneStep::Progress(handoff()));
                }
                let step = query.close_step(child).map_err(value_fault)?;
                if query.terminal_is_empty() {
                    drop(self.take_local_interaction_query_reply());
                    self.local_interaction_query = None;
                }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            CloseRung::SnapshotReturns => {
                if let Some(stage) = self.next_snapshot_read_return_stage() { self.snapshot_read_return_stage = stage; }
                self.advance_snapshot_read_returns_one(grant).map(|step| RetainedCloneStep::Progress(step.progress()))
            }
            CloseRung::PresenceLocalReads => self.presence_store.maintenance_local_reads_step(grant).map_err(value_fault),
            CloseRung::MountedJobs => {
                let Some(instance_id) = self.live_runtime_instance_id else { return Ok(RetainedCloneStep::Progress(idle)) };
                A::mounted_job_close_step(instance_id, grant).map(Self::lifecycle_retained)
            }
            CloseRung::CancellationSlots => {
                let ceiling = self.close_cancellation_cursor.saturating_add(grant.maximum_items).min(CANCELLATION_SLOT_SPAN);
                let mut slots = 0;
                while self.close_cancellation_cursor < ceiling {
                    let _ = self.tool_cancellations.cleanup_slot(self.close_cancellation_cursor)?;
                    self.close_cancellation_cursor += 1;
                    slots += 1;
                }
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: slots, ..idle }))
            }
            CloseRung::LatestWinsKeys => Ok(RetainedCloneStep::Progress(self.latest_wins_keys.advance(grant)?.progress())),
            CloseRung::LatestWinsCommands => {
                let Some(operation_id) = self.latest_wins_order.items.front().copied() else { return Ok(RetainedCloneStep::Progress(idle)) };
                self.close_latest_wins_command_step(operation_id, grant).map(|step| RetainedCloneStep::Progress(step.progress()))
            }
            CloseRung::TypedOperations => self.close_typed_operation_unit(grant).map(Self::lifecycle_retained),
            CloseRung::InstanceOwner => {
                let step = self.instance_operation_owner.close_step(grant)?;
                if matches!(step, PluginLifecycleStep::Complete(_)) {
                    self.instance_operation_owner.close_inference();
                    if !self.instance_operation_owner.close_terminal_is_empty()? { return Err(plugin_sdk_fault("app-instance operation owner reported Complete without an exact terminal-empty witness")); }
                    self.close_instance_operation_owner_drained = true;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..step.progress().unwrap_or_default() }));
                }
                Ok(Self::lifecycle_retained(step))
            }
            CloseRung::MediaExports => self.close_media_step(false, grant),
            CloseRung::MediaClosures => self.close_media_step(true, grant),
            CloseRung::Segments => Self::close_segment_step(&mut self.segmented_downloads, grant),
            CloseRung::SegmentClosures => Self::close_segment_step(&mut self.segmented_closures, grant),
            CloseRung::SnapshotRetentions => registry_step(&mut self.snapshot_retirements, &mut self.close_snapshot_cursor, grant).map(|(step, _)| Self::lifecycle_retained(step)),
            CloseRung::ChildMemberRetirements => {
                let progress = self.child_member_retirement_step(grant)?;
                if progress == idle && !self.child_content_retirements.is_empty() {
                    return Ok(RetainedCloneStep::Progress(self.child_root_retirement_step(grant)?));
                }
                Ok(RetainedCloneStep::Progress(progress))
            }
            CloseRung::ChildRootRetirements => Ok(RetainedCloneStep::Progress(self.child_root_retirement_step(grant)?)),
            CloseRung::DetachChildRoot => self.detach_child_root_step(),
            CloseRung::ClosingChildMember => {
                let Some(retirement) = self.close_child_member.as_mut() else { return Ok(RetainedCloneStep::Progress(idle)) };
                let step = retirement.close_step(grant)?;
                if let RetainedCloneStep::Complete(progress) = step {
                    if !retirement.terminal_is_empty() { return Err(plugin_sdk_fault("child member retirement reported Complete without its exact terminal-empty witness")); }
                    drop(self.close_child_member.take());
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }));
                }
                Ok(step)
            }
            CloseRung::DetachChildren => {
                if self.close_child_member_cursor >= CHILD_CONTENT_SLOTS { return Err(plugin_sdk_fault("fixed child-member close cursor exhausted before every exact owner was detached")); }
                let index = self.close_child_member_cursor;
                self.close_child_member_cursor += 1;
                if let Some(entry) = self.children.take_at(index) { *self.close_child_member = Some(ChildMemberRetirement::new(entry)); }
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::PeerRoster => {
                let (step, removed) = registry_step(&mut self.peer_roster_publications, &mut self.close_peer_roster_cursor, grant)?;
                if let Some(generation) = removed {
                    let slot = Self::peer_roster_slot(generation);
                    if self.peer_roster_reservations[slot].is_some_and(|(reserved, _)| reserved == generation) { self.peer_roster_reservations[slot] = None; }
                }
                Ok(Self::lifecycle_retained(step))
            }
            CloseRung::PeerRosterOutcomes => {
                if let Some((index, generation)) = self.peer_roster_outcomes.next_id_from(self.close_peer_roster_cursor) {
                    drop(self.peer_roster_outcomes.remove(generation).ok_or_else(|| plugin_sdk_fault("peer roster outcome changed before exact removal"))?);
                    self.close_peer_roster_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                }
                Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::PeerPresenceRetirements => registry_step(&mut self.peer_presence_retirements, &mut self.close_peer_presence_cursor, grant).map(|(step, _)| Self::lifecycle_retained(step)),
            CloseRung::PresencePeerRetirements => registry_step(&mut self.presence_peer_retirements, &mut self.close_presence_peer_cursor, grant).map(|(step, _)| Self::lifecycle_retained(step)),
            CloseRung::DetachPeerPresence => self.detach_peer_presence_step(),
            CloseRung::EnvelopeIngress => self.drive_envelope_ingress(grant, true).map(Self::lifecycle_retained),
            CloseRung::EnvelopeDecode => self.envelope_decode_close_step(grant).map(Self::lifecycle_retained),
            CloseRung::StoreReplacement => self.drive_store_replacement_jobs(grant, true).map(Self::lifecycle_retained),
            CloseRung::DocumentArchive => self.drive_document_archive_load_retirements(grant, true).map(Self::lifecycle_retained),
            CloseRung::WindowRetirements => self.retire_document_windows_step(grant, true).map(Self::lifecycle_retained),
            CloseRung::OwnedStores => self.close_owned_stores_step(grant).map(Self::lifecycle_retained),
            CloseRung::RetainedFields => self.retained_fields_close_step(grant),
            CloseRung::Uncovered => if self.close_terminal_is_empty() { Ok(RetainedCloneStep::Complete(idle)) } else { Err(value_fault(Self::uncovered_close_frontier())) },
        }
    }
}

fn widest_demand(left: RetirementDemand, right: RetirementDemand) -> RetirementDemand {
    RetirementDemand { copy_bytes: left.copy_bytes.max(right.copy_bytes), capacity_bytes: left.capacity_bytes.max(right.capacity_bytes), release_bytes: left.release_bytes.max(right.release_bytes), depth: left.depth.max(right.depth) }
}
