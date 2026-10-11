//! 🪜️ The live-maintenance rotation of one hosted app: priority and pressure units first, then a fair fixed round robin
//! in which an empty stage never consumes the call. Every unit quotes its next demand on all axes and spends the caller's
//! unchanged grant on exactly one owner; the same module drives the envelope-ingress, store-replacement and archive-load
//! frontiers for both the live rotation and the close ladder.
use super::*;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaintenanceUnit {
    CommandPrune,
    PrivateChildGroup(u64),
    DirectIngress,
    GrantedMounted,
    ToolOverlay,
    ToolRuns,
    TimeTravel,
    Pressure(u8),
    Stage(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaintenanceTool {
    Worker(usize, u64),
    Awaiting,
    Idle,
}

const ROTATION_START: usize = 11;
const MAINTENANCE_POSITIONS: usize = ROTATION_START + MAINTENANCE_STAGES as usize;

fn item() -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, ..Default::default() }
}

fn item_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..Default::default() }
}

fn nonzero(demand: RetirementDemand) -> RetirementDemand {
    if demand == RetirementDemand::default() { item_demand() } else { demand }
}

fn idle() -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress::default())
}

fn progressed(step: RetainedCloneStep) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(step.progress())
}

fn removed(progress: RetainedCloneProgress) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress })
}

fn maintenance_fault(code: &'static str, message: &'static str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

/// 🧯️ Turns a refused owner-catalog admission into its fault, keeping the receipt the refused build actually spent.
fn owners_admission_fault<P, Mutation>(refusal: store::DocumentStoreOwnersAdmissionError<P, Mutation>) -> Fault
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    let store::DocumentStoreOwnersAdmissionError { error, owners, progress } = refusal;
    std::mem::forget(owners);
    plugin_retirement_fault(error).with_retained_progress(progress)
}

fn worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)))
}

impl MaintenanceUnit {
    fn trace(self) -> u64 {
        match self {
            Self::Pressure(stage) | Self::Stage(stage) => u64::from(stage),
            Self::CommandPrune => 100,
            Self::PrivateChildGroup(_) => 101,
            Self::DirectIngress => 102,
            Self::GrantedMounted => 103,
            Self::ToolOverlay => 104,
            Self::ToolRuns => 105,
            Self::TimeTravel => 106,
        }
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    fn config_lane_under_pressure(&self) -> bool {
        self.config_store.maintenance_retirements_under_pressure() || self.draft_store.maintenance_retirements_under_pressure() || self.interaction_store.maintenance_retirements_under_pressure() || self.window_config_store.maintenance_retirements_under_pressure()
    }

    /// 🌡️ Pressure beats fairness: a displaced-owner or child retirement queue a quarter full is drained out of turn.
    pub(crate) fn maintenance_ladder_under_pressure(&self) -> bool {
        self.command_prune_requested() || self.pending_command_prune.is_some() || !self.command_log.pruned_terminal_is_empty() || self.next_closing_private_child_group().is_some() || self.store.maintenance_retirements_under_pressure() || self.config_lane_under_pressure() || self.child_retirements_under_pressure()
    }

    /// 🪹️ Every live retirement frontier and its retained directory backing is empty.
    pub(crate) fn maintenance_ladder_terminal_is_empty(&self) -> bool {
        self.window_config_store.direct_ingress_terminal_is_empty()
            && !self.tool_overlay_retirement_pending()
            && self.tool_run_retirement_demand(0).is_ok_and(|demand| demand == RetirementDemand::default())
            && self.time_travel_retirement_demands(0).is_ok_and(|demand| demand == RetirementDemand::default())
            && self.pending_command_prune.is_none()
            && !self.command_prune_requested()
            && self.command_log.pruned_terminal_is_empty()
            && self.private_child_groups_terminal_is_empty()
            && self.tool_operations.is_empty()
            && self.tool_cancellations.active_operation_count() == 0
            && self.latest_wins_commands.is_empty()
            && self.latest_wins_keys.map.as_ref().is_some_and(|map| map.is_empty())
            && self.latest_wins_keys.can_begin()
            && self.media_closures.is_empty()
            && self.segmented_closures.is_empty()
            && self.snapshot_retirements.is_empty()
            && self.child_content_retirements.is_empty()
            && self.child_member_retirements.is_empty()
            && self.peer_roster_publications.is_empty()
            && self.peer_presence_retirements.is_empty()
            && self.presence_peer_retirements.is_empty()
            && self.envelope_ingress.is_empty()
            && self.envelope_decode_jobs.is_empty()
            && self.store_replacement_jobs.is_empty()
            && self.document_archive_loads.is_empty()
            && self.envelope_field_decoder_retirements.is_empty()
            && self.envelope_field_decoders.next_returned_ticket().is_none()
            && self.envelope_completed_record_retirements.is_empty()
            && self.envelope_completed_records.terminal_is_empty()
            && self.store.maintenance_retirements_terminal_is_empty()
            && self.config_store.maintenance_retirements_terminal_is_empty()
            && self.draft_store.maintenance_retirements_terminal_is_empty()
            && self.interaction_store.maintenance_retirements_terminal_is_empty()
            && self.window_config_store.maintenance_retirements_terminal_is_empty()
            && self.window_transient_store.maintenance_terminal_is_empty()
            && self.retired_window_transient_stores.is_empty()
            && self.presence_store.local_read_maintenance_is_idle()
            && self.snapshot_read_returns_terminal_is_empty()
            && !semio_framework_job::worker_job_retirements_are_parked()
            && self.live_runtime_instance_id.is_none_or(|instance_id| A::mounted_jobs_terminal_is_empty(instance_id))
    }

    fn maintenance_unit_at(&self, position: usize, cursor: u8) -> Option<MaintenanceUnit> {
        match position {
            0 => (self.command_prune_requested() || self.pending_command_prune.is_some() || !self.command_log.pruned_terminal_is_empty()).then_some(MaintenanceUnit::CommandPrune),
            1 => self.next_closing_private_child_group().map(MaintenanceUnit::PrivateChildGroup),
            2 => (!self.window_config_store.direct_ingress_terminal_is_empty()).then_some(MaintenanceUnit::DirectIngress),
            3 => self.next_granted_mounted_retirement().is_some().then_some(MaintenanceUnit::GrantedMounted),
            4 => self.tool_overlay_retirement_pending().then_some(MaintenanceUnit::ToolOverlay),
            5 => Some(MaintenanceUnit::ToolRuns),
            6 => Some(MaintenanceUnit::TimeTravel),
            7 => self.store.maintenance_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_DOCUMENT_DISPLACED_STAGE)),
            8 => self.config_lane_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE)),
            9 => self.child_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CHILD_ROOT_STAGE)),
            10 => self.child_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CHILD_MEMBER_STAGE)),
            _ => Some(MaintenanceUnit::Stage(((usize::from(cursor) + position - ROTATION_START) % usize::from(MAINTENANCE_STAGES)) as u8)),
        }
    }

    fn maintenance_unit_demand(&self, unit: MaintenanceUnit, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        let present = |demand: RetirementDemand| (demand != RetirementDemand::default()).then_some(demand);
        Ok(match unit {
            MaintenanceUnit::CommandPrune => Some(self.command_prune_demands(false, body)?),
            MaintenanceUnit::PrivateChildGroup(operation) => Some(self.private_child_group_operation_close_demands(operation, body)?),
            MaintenanceUnit::DirectIngress => Some(self.window_config_store.direct_ingress_demands(body)?),
            MaintenanceUnit::GrantedMounted => Some(self.granted_mounted_retirement_demands(body)?),
            MaintenanceUnit::ToolOverlay => Some(self.tool_overlay_retirement_demand(body)?),
            MaintenanceUnit::ToolRuns => present(self.tool_run_retirement_demand(body)?),
            MaintenanceUnit::TimeTravel => present(self.time_travel_retirement_demands(body)?),
            MaintenanceUnit::Pressure(stage) | MaintenanceUnit::Stage(stage) => self.maintenance_stage_demand(stage, body)?,
        })
    }

    fn maintenance_unit_step(&mut self, unit: MaintenanceUnit, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        match unit {
            MaintenanceUnit::CommandPrune => self.advance_command_prune_step(grant, false),
            MaintenanceUnit::PrivateChildGroup(operation) => match self.close_private_child_group_operation_step(operation, grant)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => Ok(PluginLifecycleStep::Progress(progress)),
                semio_framework_job::InteractiveJobCloseStep::Blocked => Ok(PluginLifecycleStep::Blocked { reason: "private child group close awaits its owner" }),
                semio_framework_job::InteractiveJobCloseStep::Refused { kind, .. } => Err(plugin_sdk_fault(format!("private child group close was refused: {kind:?}"))),
            },
            MaintenanceUnit::DirectIngress => self.window_config_store.close_direct_ingress(grant).map(progressed).map_err(plugin_retirement_fault),
            MaintenanceUnit::GrantedMounted => self.granted_mounted_retirement_step(grant).map(progressed).map_err(plugin_retirement_fault),
            MaintenanceUnit::ToolOverlay => self.tool_overlay_retirement_step(grant).map(progressed).map_err(plugin_retirement_fault),
            MaintenanceUnit::ToolRuns => Ok(self.tool_run_retire_step(grant)?.map_or_else(idle, progressed)),
            MaintenanceUnit::TimeTravel => Ok(self.time_travel_retire_step(grant)?.map_or_else(idle, progressed)),
            MaintenanceUnit::Pressure(stage) | MaintenanceUnit::Stage(stage) => self.maintenance_stage_step(stage, grant),
        }
    }

    /// 📏️ Quotes the first unit with work, in the exact order [`Self::maintenance_ladder_step`] visits them.
    pub(crate) fn maintenance_ladder_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        for position in 0..MAINTENANCE_POSITIONS {
            let Some(unit) = self.maintenance_unit_at(position, self.maintenance_stage) else { continue };
            if let Some(demand) = self.maintenance_unit_demand(unit, body)? {
                return Ok(demand);
            }
        }
        Ok(RetirementDemand::default())
    }

    /// 🎡️ Visits units in fair order and spends the grant on the first one that releases something. An empty or unfunded
    /// unit never consumes the call; a blocked or input-starved unit is remembered and reported only when nothing else moved.
    pub(crate) fn maintenance_ladder_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        LAST_MAINTENANCE_STAGE.store(u64::MAX, std::sync::atomic::Ordering::Relaxed);
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = self.maintenance_stage;
        let mut awaiting = None;
        let mut blocked = None;
        for position in 0..MAINTENANCE_POSITIONS {
            let Some(unit) = self.maintenance_unit_at(position, cursor) else { continue };
            if let MaintenanceUnit::Stage(stage) = unit {
                self.maintenance_stage = (stage + 1) % MAINTENANCE_STAGES;
            }
            let Some(demand) = self.maintenance_unit_demand(unit, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)? else { continue };
            if !plugin_turn_admitted(demand, grant)? {
                continue;
            }
            LAST_MAINTENANCE_STAGE.store(unit.trace(), std::sync::atomic::Ordering::Relaxed);
            match self.maintenance_unit_step(unit, grant)? {
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) if progress != empty => return Ok(PluginLifecycleStep::Progress(progress)),
                step @ PluginLifecycleStep::AwaitingInput { .. } => awaiting = awaiting.or(Some(step)),
                step @ PluginLifecycleStep::Blocked { .. } => blocked = blocked.or(Some(step)),
                PluginLifecycleStep::Progress(_) | PluginLifecycleStep::Complete(_) => {}
            }
        }
        if let Some(step) = awaiting.or(blocked) {
            return Ok(step);
        }
        Ok(if self.maintenance_terminal_is_empty() { PluginLifecycleStep::Complete(empty) } else { PluginLifecycleStep::Progress(empty) })
    }

    fn maintenance_tool_selection(&self) -> MaintenanceTool {
        let mut occupied = false;
        let mut every_operation_awaits_presented_ack = true;
        for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let index = (self.maintenance_tool_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            let Some((operation_id, operation)) = self.tool_operations.entry(index).map(|entry| (entry.0, &entry.1)) else { continue };
            occupied = true;
            match operation.stage {
                MountedTypedCommandFullOperationStage::Worker => return MaintenanceTool::Worker(index, operation_id),
                MountedTypedCommandFullOperationStage::AwaitingAck if operation.result_page_presented => {}
                MountedTypedCommandFullOperationStage::Retiring | MountedTypedCommandFullOperationStage::Publishing | MountedTypedCommandFullOperationStage::AwaitingAck => every_operation_awaits_presented_ack = false,
            }
        }
        if occupied && every_operation_awaits_presented_ack { MaintenanceTool::Awaiting } else { MaintenanceTool::Idle }
    }

    fn maintenance_peer_roster_generation(&self) -> Option<u64> {
        let generation = self.peer_roster_processed_generation.checked_add(1)?;
        if self.peer_roster_publications.get(generation).is_some() { Some(generation) } else { self.orphaned_peer_roster_generation() }
    }

    fn maintenance_stage_demand(&self, stage: u8, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        Ok(match stage {
            0 => match self.maintenance_tool_selection() {
                MaintenanceTool::Worker(_, operation_id) => self.tool_operations.get(operation_id).map(|operation| operation.worker_step_demands(body)).transpose()?,
                MaintenanceTool::Awaiting => Some(RetirementDemand::default()),
                MaintenanceTool::Idle => None,
            },
            1 => {
                let Some((_, operation_id)) = self.media_closures.next_id_from(self.maintenance_media_cursor) else { return Ok(None) };
                self.media_closures.get(operation_id).map(|active| active.retirement_demands(body)).transpose()?
            }
            2 => {
                let Some((_, operation_id)) = self.segmented_closures.next_id_from(self.maintenance_segment_cursor) else { return Ok(None) };
                let remaining = self.segmented_closures.get(operation_id).map_or(0, |output| output.chunks.chunks_remaining());
                Some(RetirementDemand { release_bytes: if remaining == 0 { 0 } else { ARTIFACT_OUTPUT_CHUNK_BYTES }, depth: 1, ..Default::default() })
            }
            3 => {
                let Some((_, operation_id)) = self.snapshot_retirements.next_id_from(self.maintenance_snapshot_cursor) else { return Ok(None) };
                if self.media_exports.get(operation_id).is_some() || self.media_closures.get(operation_id).is_some() {
                    return Ok(Some(RetirementDemand::default()));
                }
                self.snapshot_retirements.get(operation_id).map(|retention| retention.retirement_demands(body)).transpose()?
            }
            4 => (!self.child_content_retirements.is_empty()).then(|| self.child_root_retirement_demands(body)).transpose()?,
            5 => {
                let Some((_, generation)) = self.peer_presence_retirements.next_id_from(self.maintenance_peer_presence_cursor) else { return Ok(None) };
                self.peer_presence_retirements.get(generation).map(|retirement| retirement.retirement_demands(body)).transpose()?
            }
            6 => {
                let Some((_, generation)) = self.presence_peer_retirements.next_id_from(self.maintenance_presence_peer_cursor) else { return Ok(None) };
                self.presence_peer_retirements.get(generation).map(|retirement| retirement.retirement_demands(body)).transpose()?
            }
            7 => {
                let Some(generation) = self.maintenance_peer_roster_generation() else { return Ok(None) };
                let Some(publication) = self.peer_roster_publications.get(generation) else { return Ok(None) };
                Some(if publication.is_faulted() {
                    publication.close_demands(body)?
                } else if publication.stage == PeerRosterPublicationStage::Ready {
                    RetirementDemand { capacity_bytes: child_content_arc_bytes::<PeerPresenceRoot>(), depth: 1, ..Default::default() }
                } else {
                    publication.step_demands()?
                })
            }
            8 => (!self.snapshot_read_returns_terminal_is_empty()).then(|| self.snapshot_read_returns_demands(body)).transpose()?,
            MAINTENANCE_DOCUMENT_DISPLACED_STAGE => (!self.store.maintenance_retirements_terminal_is_empty()).then(|| self.store.maintenance_retirements_demands(body)).transpose()?,
            10 => self.envelope_ingress_target(false).map(|_| self.envelope_ingress_demands(false, body)).transpose()?,
            11 => (!self.envelope_decode_jobs.is_empty()).then(|| self.envelope_decode_jobs_demands(false, body)).transpose()?,
            12 => (!self.envelope_field_decoder_retirements.is_empty() || self.envelope_field_decoders.next_returned_ticket().is_some()).then(|| self.envelope_field_decoder_returns_demands(false, body)).transpose()?,
            13 => (!self.envelope_completed_record_retirements.is_empty() || !self.envelope_completed_records.terminal_is_empty()).then(|| self.envelope_completed_record_returns_demands(false, body)).transpose()?,
            14 => self.store_replacement_target(false).map(|_| self.store_replacement_demands(false, body)).transpose()?,
            15 => match self.live_runtime_instance_id {
                Some(instance_id) if !A::mounted_jobs_terminal_is_empty(instance_id) => Some(A::mounted_job_maintenance_demands(instance_id, body)?),
                _ => None,
            },
            16 => (!self.instance_operation_owner.terminal_is_empty().unwrap_or(false)).then(|| self.instance_operation_owner.retirement_demands(body)).transpose()?,
            17 => Some(self.latest_wins_keys.advance_demands()?).filter(|demand| *demand != RetirementDemand::default()),
            18 => Some(item_demand()),
            19 => (!self.presence_store.local_read_maintenance_is_idle()).then(|| self.presence_store.maintenance_local_reads_demands(body)).transpose()?,
            MAINTENANCE_CHILD_MEMBER_STAGE => (!self.child_member_retirements.is_empty()).then(|| self.child_member_retirement_demands(body)).transpose()?,
            21 => (!self.retired_window_transient_stores.is_empty()).then(|| self.document_windows_retirement_demands(body, false)).transpose()?,
            22 => Some(self.window_transient_store.maintenance_demands(body)?).filter(|demand| *demand != RetirementDemand::default()),
            23 => semio_framework_job::worker_job_retirements_are_parked().then(item_demand),
            24 => self.document_archive_target(false).map(|_| self.document_archive_demands(false, body)).transpose()?,
            MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE => {
                if !self.config_store.maintenance_retirements_terminal_is_empty() {
                    Some(self.config_store.maintenance_retirements_demands(body)?)
                } else if !self.draft_store.maintenance_retirements_terminal_is_empty() {
                    Some(self.draft_store.maintenance_retirements_demands(body)?)
                } else if !self.interaction_store.maintenance_retirements_terminal_is_empty() {
                    Some(self.interaction_store.maintenance_retirements_demands(body)?)
                } else if !self.window_config_store.maintenance_retirements_terminal_is_empty() {
                    Some(self.window_config_store.maintenance_retirements_demands(body)?)
                } else {
                    None
                }
            }
            _ => unreachable!("fixed maintenance stage"),
        })
    }

    fn maintenance_stage_step(&mut self, stage: u8, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let value_fault = plugin_retirement_fault;
        match stage {
            0 => match self.maintenance_tool_selection() {
                MaintenanceTool::Awaiting => Ok(PluginLifecycleStep::AwaitingInput { reason: "every typed operation awaits its exact presented host result ACK" }),
                MaintenanceTool::Idle => Ok(idle()),
                MaintenanceTool::Worker(index, operation_id) => {
                    self.maintenance_tool_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                    let pool = worker_pool();
                    let operation = self.tool_operations.get_mut(operation_id).ok_or_else(|| maintenance_fault("interactive-job.maintenance-tool-authority", "typed operation authority changed during one fixed maintenance step"))?;
                    let stepped = operation.drive_worker_step(grant);
                    #[cfg(target_arch = "wasm32")]
                    if let Some(now_ms) = semio_framework_job::default_now_ms() {
                        pool.pump(now_ms);
                    }
                    match stepped {
                        Ok(step) => Ok(step),
                        Err(fault) => {
                            self.fault_typed_operation_worker(operation_id, &fault)?;
                            Ok(PluginLifecycleStep::Progress(item()))
                        }
                    }
                }
            },
            1 => {
                let Some((index, operation_id)) = self.media_closures.next_id_from(self.maintenance_media_cursor) else { return Ok(idle()) };
                self.maintenance_media_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                let authority = || maintenance_fault("interactive-job.maintenance-media-authority", "live media cleanup authority changed during one fixed step");
                let step = self.media_closures.get_mut(operation_id).ok_or_else(authority)?.close_step(grant)?;
                let PluginLifecycleStep::Complete(final_progress) = step else { return Ok(step) };
                let active = self.media_closures.get(operation_id).ok_or_else(authority)?;
                if !active.terminal_is_empty()? {
                    return Err(maintenance_fault("interactive-job.maintenance-media-terminal-not-empty", "live media cleanup reported Complete without an exact terminal-empty witness"));
                }
                self.quarantine_media_snapshot(operation_id, active).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-snapshot-quarantine"), error.to_string()))?;
                drop(self.media_closures.remove(operation_id).ok_or_else(authority)?);
                Ok(removed(final_progress))
            }
            2 => {
                let Some((index, operation_id)) = self.segmented_closures.next_id_from(self.maintenance_segment_cursor) else { return Ok(idle()) };
                self.maintenance_segment_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                let authority = || maintenance_fault("interactive-job.maintenance-segment-authority", "live segmented cleanup authority changed during one fixed step");
                match self.segmented_closures.get(operation_id).ok_or_else(authority)?.chunks.close_take_chunk()? {
                    Some(chunk) => Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { released_bytes: chunk.len(), ..item() })),
                    None => {
                        let output = self.segmented_closures.remove(operation_id).ok_or_else(authority)?;
                        if !output.terminal_is_empty() {
                            self.segmented_closures.insert_admitted(operation_id, output);
                            return Err(maintenance_fault("interactive-job.maintenance-segment-terminal-not-empty", "live segmented cleanup reached terminal without an empty exact chunk queue"));
                        }
                        drop(output);
                        Ok(PluginLifecycleStep::Progress(item()))
                    }
                }
            }
            3 => {
                let Some((index, operation_id)) = self.snapshot_retirements.next_id_from(self.maintenance_snapshot_cursor) else { return Ok(idle()) };
                self.maintenance_snapshot_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                if self.media_exports.get(operation_id).is_some() || self.media_closures.get(operation_id).is_some() {
                    return Ok(PluginLifecycleStep::Blocked { reason: "snapshot retirement waits for its exact live media owner" });
                }
                let authority = || maintenance_fault("interactive-job.maintenance-snapshot-authority", "live snapshot retirement authority changed during one fixed step");
                let step = self.snapshot_retirements.get_mut(operation_id).ok_or_else(authority)?.close_step(grant)?;
                let PluginLifecycleStep::Complete(final_progress) = step else { return Ok(step) };
                if !self.snapshot_retirements.get(operation_id).is_some_and(ArtifactSnapshotCloseRetention::terminal_is_empty) {
                    return Err(maintenance_fault("interactive-job.maintenance-snapshot-terminal-not-empty", "live snapshot retirement reported Complete without its exact terminal-empty witness"));
                }
                drop(self.snapshot_retirements.remove(operation_id).ok_or_else(authority)?);
                Ok(removed(final_progress))
            }
            MAINTENANCE_CHILD_ROOT_STAGE => self.child_root_retirement_step(grant).map(PluginLifecycleStep::Progress),
            5 => {
                let Some((index, generation)) = self.peer_presence_retirements.next_id_from(self.maintenance_peer_presence_cursor) else { return Ok(idle()) };
                let authority = || maintenance_fault("interactive-job.maintenance-peer-presence-authority", "live peer-presence retirement authority changed during one fixed step");
                match self.peer_presence_retirements.get_mut(generation).ok_or_else(authority)?.close_step(grant).map_err(value_fault)? {
                    PluginLifecycleStep::Progress(progress) => {
                        self.maintenance_peer_presence_cursor = if progress == RetainedCloneProgress::default() { (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS } else { index };
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                    PluginLifecycleStep::Complete(progress) => {
                        if !self.peer_presence_retirements.get(generation).is_some_and(PeerPresenceRootRetirement::terminal_is_empty) {
                            return Err(maintenance_fault("interactive-job.maintenance-peer-presence-terminal-not-empty", "live peer-presence retirement reported Complete without its exact terminal-empty witness"));
                        }
                        drop(self.peer_presence_retirements.remove(generation).ok_or_else(authority)?);
                        self.maintenance_peer_presence_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(removed(progress))
                    }
                    blocked => {
                        self.maintenance_peer_presence_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(blocked)
                    }
                }
            }
            6 => {
                let Some((index, generation)) = self.presence_peer_retirements.next_id_from(self.maintenance_presence_peer_cursor) else { return Ok(idle()) };
                let authority = || maintenance_fault("interactive-job.maintenance-presence-peer-authority", "live app-typed presence retirement authority changed during one fixed step");
                match self.presence_peer_retirements.get_mut(generation).ok_or_else(authority)?.close_step(grant).map_err(value_fault)? {
                    RetainedCloneStep::Progress(progress) => {
                        self.maintenance_presence_peer_cursor = if progress == RetainedCloneProgress::default() { (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS } else { index };
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                    RetainedCloneStep::Complete(progress) => {
                        if !self.presence_peer_retirements.get(generation).is_some_and(store::PresencePeersRetirement::terminal_is_empty) {
                            return Err(maintenance_fault("interactive-job.maintenance-presence-peer-terminal-not-empty", "live app-typed presence retirement reported Complete without its exact terminal-empty witness"));
                        }
                        drop(self.presence_peer_retirements.remove(generation).ok_or_else(authority)?);
                        self.maintenance_presence_peer_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(removed(progress))
                    }
                }
            }
            7 => self.maintenance_peer_roster_step(grant),
            8 => {
                for _ in 0..3 {
                    let demand = self.snapshot_read_returns_demands(grant.maximum_copy_bytes).map_err(value_fault)?;
                    if !plugin_turn_admitted(demand, grant)? {
                        break;
                    }
                    let step = self.advance_snapshot_read_returns_one(grant)?;
                    if step.progress() != RetainedCloneProgress::default() {
                        return Ok(progressed(step));
                    }
                }
                Ok(idle())
            }
            MAINTENANCE_DOCUMENT_DISPLACED_STAGE => self.store.maintenance_retirements_step(grant).map(progressed).map_err(value_fault),
            10 => self.drive_envelope_ingress(grant, false),
            11 => self.drive_envelope_decode_jobs(grant, false),
            12 => self.drive_envelope_field_decoder_returns(grant, false),
            13 => self.drive_envelope_completed_record_returns(grant, false),
            14 => self.drive_store_replacement_jobs(grant, false),
            15 => match self.live_runtime_instance_id {
                Some(instance_id) => A::mounted_job_maintenance_step(instance_id, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant }),
                None => Ok(idle()),
            },
            16 => self.instance_operation_owner.maintenance_step(grant),
            17 => self.latest_wins_keys.advance(grant).map(progressed),
            18 => {
                if self.tool_cancellations.cleanup_finished_slot(self.maintenance_cancellation_cursor)?.is_none() {
                    return Ok(idle());
                }
                self.maintenance_cancellation_cursor = (self.maintenance_cancellation_cursor + 1) % (TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS);
                Ok(PluginLifecycleStep::Progress(item()))
            }
            19 => self.presence_store.maintenance_local_reads_step(grant).map(progressed).map_err(value_fault),
            MAINTENANCE_CHILD_MEMBER_STAGE => self.child_member_retirement_step(grant).map(PluginLifecycleStep::Progress),
            21 => self.retire_document_windows_step(grant, false),
            22 => self.window_transient_store.maintenance_step(grant),
            23 => {
                let step = semio_framework_job::pump_worker_job_retirements(1, grant);
                Ok(plugin_worker_close_progress(step, grant, !semio_framework_job::worker_job_retirements_are_parked())?.map_or(PluginLifecycleStep::Blocked { reason: "a parked worker-job session awaits its genuine handback" }, PluginLifecycleStep::Progress))
            }
            24 => self.drive_document_archive_load_retirements(grant, false),
            MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE => {
                let step = if !self.config_store.maintenance_retirements_terminal_is_empty() {
                    self.config_store.maintenance_retirements_step(grant).map_err(value_fault)
                } else if !self.draft_store.maintenance_retirements_terminal_is_empty() {
                    self.draft_store.maintenance_retirements_step(grant).map_err(value_fault)
                } else if !self.interaction_store.maintenance_retirements_terminal_is_empty() {
                    self.interaction_store.maintenance_retirements_step(grant).map_err(value_fault)
                } else {
                    self.window_config_store.maintenance_retirements_step(grant)
                };
                step.map(progressed)
            }
            _ => unreachable!("fixed maintenance stage"),
        }
    }

    /// 🧭️ Walks one mounted peer roster through decode, validation, atomic publication and outcome hand-off, or through its cleanup once faulted.
    fn maintenance_peer_roster_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let Some(generation) = self.maintenance_peer_roster_generation() else { return Ok(idle()) };
        let index = Self::peer_roster_slot(generation);
        let authority = || maintenance_fault("interactive-job.maintenance-peer-roster-authority", "peer roster publication changed during one fixed step");
        let publication = self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?;
        let faulted = publication.is_faulted();
        let ready = !faulted && publication.stage == PeerRosterPublicationStage::Ready;
        let step = if faulted {
            publication.close_step(grant)?
        } else if ready {
            PluginLifecycleStep::Complete(RetainedCloneProgress::default())
        } else {
            match publication.step(&self.presence_store, grant) {
                Ok(step) => step,
                Err(fault) => {
                    publication.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            }
        };
        let PluginLifecycleStep::Complete(mut progress) = step else {
            self.maintenance_peer_roster_cursor = index;
            return Ok(step);
        };
        if faulted {
            if !self.peer_roster_publications.get(generation).is_some_and(PeerRosterPublication::terminal_is_empty) {
                return Err(maintenance_fault("interactive-job.maintenance-peer-roster-terminal-not-empty", "faulted peer roster publication reported Complete without its exact terminal-empty witness"));
            }
        } else {
            let (seq, cancel) = {
                let publication = self.peer_roster_publications.get(generation).ok_or_else(authority)?;
                (publication.seq, publication.cancel.clone())
            };
            let admission = match self.validate_peer_roster_publication(seq, generation, &cancel) {
                Ok(admission) => admission,
                Err(fault) => {
                    self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            };
            let candidate = match self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.take_candidate() {
                Ok(candidate) => candidate,
                Err(fault) => {
                    self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            };
            let capacity = child_content_arc_bytes::<PeerPresenceRoot>();
            if let Err(candidate) = self.publish_peer_roster_candidate_admitted(admission, candidate) {
                self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.retain_rejected_candidate(candidate);
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { retained_capacity_bytes: capacity, ..item() }));
            }
            progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(capacity);
        }
        if !self.peer_roster_publications.get(generation).is_some_and(PeerRosterPublication::terminal_is_empty) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-terminal-not-empty", "peer roster publication reached outcome handoff without an exact empty terminal witness"));
        }
        let (seq, fault) = {
            let publication = self.peer_roster_publications.get_mut(generation).ok_or_else(authority)?;
            (publication.seq, publication.fault.take())
        };
        if !self.peer_roster_outcomes.can_insert(generation) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-outcome", "pre-admitted peer roster outcome slot changed before exact handoff"));
        }
        self.peer_roster_outcomes.insert_admitted(generation, PresenceRosterOutcome { seq, fault });
        drop(self.peer_roster_publications.remove(generation).ok_or_else(authority)?);
        let reservation_slot = Self::peer_roster_slot(generation);
        if self.peer_roster_reservations[reservation_slot] != Some((generation, seq)) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-reservation", "terminal peer roster lost its exact ingress reservation"));
        }
        self.peer_roster_reservations[reservation_slot] = None;
        self.peer_roster_processed_generation = self.peer_roster_processed_generation.max(generation);
        self.maintenance_peer_roster_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        Ok(removed(progress))
    }

    fn envelope_ingress_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close_envelope_ingress_cursor } else { self.maintenance_envelope_ingress_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.envelope_ingress.entry(index).filter(|entry| closing || entry.1.closing).map(|entry| (index, entry.0))
        })
    }

    /// 📏️ Quotes the next envelope-ingress page release: the page payload, then the slot backing's whole physical release.
    pub(crate) fn envelope_ingress_demands(&self, closing: bool, _body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation_id)) = self.envelope_ingress_target(closing) else { return Ok(RetirementDemand::default()) };
        Ok(self.envelope_ingress.get(operation_id).map_or_else(RetirementDemand::default, ActiveArtifactEnvelopeIngress::retirement_demands))
    }

    /// 🎟️ One granted release turn of the next closing envelope ingress; a closing app closes every ingress.
    pub(crate) fn drive_envelope_ingress(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let Some((index, operation_id)) = self.envelope_ingress_target(closing) else { return Ok(PluginLifecycleStep::Complete(empty)) };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close_envelope_ingress_cursor } else { &mut self.maintenance_envelope_ingress_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let ingress = self.envelope_ingress.get_mut(operation_id).ok_or_else(|| maintenance_fault("artifact-envelope.ingress-owner", "artifact envelope ingress changed during one bounded maintenance step"))?;
        if closing {
            ingress.closing = true;
        }
        match ingress.close_step(grant)? {
            RetainedCloneStep::Progress(progress) => Ok(PluginLifecycleStep::Progress(progress)),
            RetainedCloneStep::Complete(progress) => {
                let ingress = self.envelope_ingress.remove(operation_id).ok_or_else(|| maintenance_fault("artifact-envelope.ingress-owner", "terminal artifact envelope ingress changed before exact removal"))?;
                if !ingress.terminal_is_empty() {
                    return Err(maintenance_fault("artifact-envelope.ingress-false-terminal", "artifact envelope ingress reported Complete before every page was released"));
                }
                drop(ingress);
                Ok(removed(progress))
            }
        }
    }

    fn store_replacement_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close_store_replacement_cursor } else { self.maintenance_store_replacement_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.store_replacement_jobs
                .entry(index)
                .filter(|entry| closing || (entry.1.state != ActiveArtifactStoreReplacementState::Complete && !(entry.1.state == ActiveArtifactStoreReplacementState::AwaitingMembers && entry.1.member_ingress.is_some())))
                .map(|entry| (index, entry.0))
        })
    }

    fn store_replacement_commit_capacity() -> usize {
        ARTIFACT_LIVE_OUTPUT_SLOTS * size_of::<(u64, ChildContentRetirement)>()
    }

    fn store_replacement_unit_demands(active: &ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        use ActiveArtifactStoreReplacementState as State;
        let live = matches!(active.state, State::AwaitingMembers | State::OpeningMembers | State::ClosingRejectedMember | State::ValidatingClosure | State::PreparingCandidateViews);
        Ok(match active.state {
            State::Initializing => active.initializer_demands(body)?,
            _ if live && (closing || active.cancelled()) => item_demand(),
            State::AwaitingMembers | State::ClosingRejectedMember => item_demand(),
            State::OpeningMembers => active.member_open_demands(body)?,
            State::ValidatingClosure => active.closure_demands(),
            State::PreparingCandidateViews => active.candidate_views_demands(),
            State::RetiringRejectedMembers | State::RetiringCommittedMembers | State::RetiringCommittedStore | State::RetiringRejectedCandidate => active.retirement_demands(body)?,
            State::CandidateReady => RetirementDemand { capacity_bytes: Self::store_replacement_commit_capacity(), depth: 1, ..Default::default() },
            State::Complete => if closing { item_demand() } else { RetirementDemand::default() },
        })
    }

    /// 📏️ Quotes the next store-replacement turn selected by the target replacement's own state.
    pub(crate) fn store_replacement_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation_id)) = self.store_replacement_target(closing) else { return Ok(RetirementDemand::default()) };
        self.store_replacement_jobs.get(operation_id).map_or(Ok(RetirementDemand::default()), |active| Self::store_replacement_unit_demands(active, closing, body))
    }

    /// 🎟️ One granted turn of the next store replacement; a closing app cancels it first and removes it once terminal.
    pub(crate) fn drive_store_replacement_jobs(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        use ActiveArtifactStoreReplacementState as State;
        let empty = RetainedCloneProgress::default();
        let item = item();
        let Some((index, operation_id)) = self.store_replacement_target(closing) else { return Ok(PluginLifecycleStep::Complete(empty)) };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close_store_replacement_cursor } else { &mut self.maintenance_store_replacement_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let owner = || maintenance_fault("artifact-store.replacement-owner", "store replacement changed during one fixed maintenance step");
        let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(owner)?;
        if closing {
            active.request_cancel();
        }
        let state = active.state;
        let demand = Self::store_replacement_unit_demands(active, closing, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if !plugin_turn_admitted(demand, grant)? {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        if state == State::Initializing {
            let pool = worker_pool();
            let step = active.drive_initializer(&pool, grant);
            #[cfg(target_arch = "wasm32")]
            if let Some(now_ms) = semio_framework_job::default_now_ms() {
                pool.pump(now_ms);
            }
            return step;
        }
        if matches!(state, State::AwaitingMembers | State::OpeningMembers | State::ClosingRejectedMember | State::ValidatingClosure | State::PreparingCandidateViews) && active.cancelled() {
            active.refuse(ArtifactStoreReplacementRefusal::Cancelled);
            return Ok(PluginLifecycleStep::Progress(item));
        }
        match state {
            State::AwaitingMembers => {
                if active.member_ingress.is_some() {
                    return Ok(PluginLifecycleStep::Progress(empty));
                }
                let candidate = active.retained_store.as_ref().ok_or_else(|| plugin_sdk_fault("awaiting owned document replacement lost its parent candidate"))?;
                let projection = A::child_restore_projection(candidate.snapshot_ref())?;
                if projection.get(0).is_none() {
                    active.begin_members(0, u64::MAX)?;
                    active.seal_members()?;
                    return Ok(PluginLifecycleStep::Progress(item));
                }
                self.complete_store_replacement_genesis(operation_id)?;
                Ok(PluginLifecycleStep::Progress(item))
            }
            State::OpeningMembers => active.drive_member_open(grant),
            State::ClosingRejectedMember => {
                active.state = State::RetiringRejectedMembers;
                Ok(PluginLifecycleStep::Progress(item))
            }
            State::ValidatingClosure => active.drive_closure(grant),
            State::PreparingCandidateViews => active.drive_candidate_views(grant),
            State::RetiringRejectedMembers | State::RetiringCommittedMembers => {
                if active.retained_store.is_some() && active.retained_disposer.is_none() {
                    *active.retained_disposer = Some(A::build_document_store_disposer().ok_or_else(|| plugin_sdk_fault("app did not supply the required rejected-candidate disposer"))?);
                }
                active.drive_rejected_members(&self.child_content_root, grant)
            }
            State::CandidateReady => self.commit_store_replacement_candidate(operation_id, closing),
            State::RetiringCommittedStore | State::RetiringRejectedCandidate => active.drive_retained_store(grant),
            State::Complete | State::Initializing => {
                if !closing {
                    return Ok(PluginLifecycleStep::Complete(empty));
                }
                if !active.terminal_is_empty() {
                    return Err(maintenance_fault("artifact-store.replacement-terminal-not-empty", "store replacement reported terminal before every exact owner was empty"));
                }
                let active = self.store_replacement_jobs.remove(operation_id).ok_or_else(|| maintenance_fault("artifact-store.replacement-owner", "terminal store replacement changed before exact close removal"))?;
                drop(active);
                Ok(PluginLifecycleStep::Progress(item))
            }
        }
    }

    /// 🔐️ Publishes a ready replacement candidate or retains the exact guard evidence that refused it; the commit is one non-suspending boundary.
    fn commit_store_replacement_candidate(&mut self, operation_id: u64, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        use ActiveArtifactStoreReplacementState as State;
        let item = item();
        let replacement_content_retirements = ArtifactFixedRegistry::new();
        let live_generation = semio_framework_job::Generation(self.store.generation_now());
        let next_content_generation = self.child_content_generation.checked_add(1);
        let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| maintenance_fault("artifact-store.replacement-owner", "ready store candidate changed before exact commit validation"))?;
        let disposer = A::build_document_store_disposer().ok_or_else(|| maintenance_fault("artifact-store.replacement-disposer-missing", "app did not supply the required displaced-store disposer"))?;
        let complete_candidate = active.candidate_children.as_ref().is_some_and(|children| active.candidate_source_generation == children.len() as u64)
            && active.candidate_content.is_some()
            && active.candidate_composition.is_some()
            && active.view_member_ordinal == active.candidate_children.as_ref().map_or(usize::MAX, ChildMemberRegistry::len);
        if closing || active.cancelled() || active.generation != live_generation || active.base_child_content_generation != self.child_content_generation || !complete_candidate || !replacement_content_retirements.allocation_admitted || next_content_generation.is_none() {
            let guard = ArtifactStoreReplacementPublicationGuard {
                closing,
                cancelled: active.cancelled(),
                parent_generation: active.generation.0,
                live_generation: live_generation.0,
                base_child_generation: active.base_child_content_generation,
                child_generation: self.child_content_generation,
                complete_candidate,
                retirement_admitted: replacement_content_retirements.allocation_admitted,
                next_generation: next_content_generation.is_some(),
            };
            *active.retained_disposer = Some(disposer);
            active.refuse(ArtifactStoreReplacementRefusal::PublicationGuard(guard));
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if A::validate_document_store_publication(active.operation, active.generation, live_generation).is_err() {
            *active.retained_disposer = Some(disposer);
            active.refuse(ArtifactStoreReplacementRefusal::PublicationAuthorityRejected);
            return Ok(PluginLifecycleStep::Progress(item));
        }
        let window_reset = self.prepare_document_window_reset()?;
        let active = self.store_replacement_jobs.get_mut(operation_id).expect("validated replacement remains exclusively owned across non-suspending window preparation");
        let candidate = active.retained_store.take().expect("complete replacement retains its exact parent candidate");
        let candidate_children = active.candidate_children.take().expect("complete replacement retains its exact member registry");
        let candidate_content = active.candidate_content.take().expect("complete replacement retains its exact content view");
        let candidate_composition = active.candidate_composition.take().expect("complete replacement retains its exact coordinator");
        let local_actor = self.store.local_actor_id().clone();
        let displaced = match publish_boxed_document_store_candidate_if_authoritative(&mut self.store, candidate, || Ok(())) {
            Ok(displaced) => displaced,
            Err(_) => unreachable!("prevalidated publication cannot reject inside its non-suspending commit boundary"),
        };
        assert_eq!(self.store.local_actor_id(), &local_actor, "document replacement retains its opened actor");
        self.store.enable_convergence_early_exit();
        self.store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));
        self.store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));
        let displaced_children = std::mem::replace(&mut self.children, candidate_children);
        let displaced_content = std::mem::replace(&mut *self.child_content_root, candidate_content);
        let displaced_content_retirements = std::mem::replace(&mut self.child_content_retirements, replacement_content_retirements);
        let displaced_composition = std::mem::replace(&mut self.composition, candidate_composition);
        let displaced_pins = std::mem::take(&mut self.pending_child_pins);
        *active.retained_store = Some(displaced);
        *active.candidate_children = Some(displaced_children);
        *active.candidate_content = Some(displaced_content);
        if displaced_content_retirements.is_empty() {
            drop(displaced_content_retirements);
        } else {
            *active.displaced_content_retirements = Some(displaced_content_retirements);
        }
        *active.candidate_composition = Some(displaced_composition);
        if !displaced_pins.is_empty() {
            *active.displaced_pins = Some(CompositionPinsRetirement::new(displaced_pins));
        }
        *active.retained_disposer = Some(disposer);
        active.committed = true;
        active.state = State::RetiringCommittedMembers;
        self.child_content_generation = next_content_generation.expect("candidate generation exhaustion was rejected before commit");
        self.commit_document_window_reset(window_reset);
        self.cache = None;
        self.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(false);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { retained_capacity_bytes: Self::store_replacement_commit_capacity(), ..item }))
    }

    fn document_archive_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close_document_archive_cursor } else { self.maintenance_document_archive_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.document_archive_loads.entry(index).filter(|entry| closing || (!(entry.1.terminal() && entry.1.terminal_is_empty()) && !entry.1.awaits_merge())).map(|entry| (index, entry.0))
        })
    }

    fn document_archive_load_demands(&self, active: &ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = item_demand();
        if active.terminal_target.is_some() || (closing && !active.terminal()) {
            let replacement_live = active.replacement.is_some_and(|handle| self.store_replacement_jobs.get(handle.operation.0).is_some());
            return Ok(if replacement_live || active.terminal_is_empty() { item } else { nonzero(active.retirement_demands(body)?) });
        }
        if active.state == ActiveDocumentArchiveLoadState::Pending {
            return Ok(item);
        }
        Ok(match active.phase {
            ActiveDocumentArchiveLoadPhase::DecodeParent => RetirementDemand { copy_bytes: 1, ..item },
            ActiveDocumentArchiveLoadPhase::RetireParentHistoryAuxiliary => {
                if active.retained_pending.is_some() {
                    store::artifact_retirement_owned_birth_demands(&active.retained_pending)?
                } else if let Some(retained) = active.retained.as_ref() {
                    store::artifact_retirement_box_demands(retained, body)?
                } else {
                    item
                }
            }
            ActiveDocumentArchiveLoadPhase::MergeParent | ActiveDocumentArchiveLoadPhase::Terminal => RetirementDemand::default(),
            ActiveDocumentArchiveLoadPhase::HydrateParent => {
                if active.hydration.is_none() {
                    item
                } else {
                    RetirementDemand { capacity_bytes: store::retire_document_envelope_birth_bytes::<A::Snapshot, A::Mutation>() + size_of::<store::DocumentStoreOwners<A::Snapshot, A::Mutation>>(), depth: 4, ..Default::default() }
                }
            }
            ActiveDocumentArchiveLoadPhase::FillMember => {
                let length = active.member.as_ref().and_then(|member| member.entry.as_ref().map(|entry| entry.envelope_pack.len().saturating_sub(member.copied).min(store::OWNED_SCHEMA_DECODE_PAGE_BYTES))).unwrap_or(0);
                RetirementDemand { copy_bytes: length, capacity_bytes: length, ..item }
            }
            ActiveDocumentArchiveLoadPhase::RetireMemberSource => active.member.as_ref().map_or(item, |member| nonzero(member.source_retirement_demands())),
            ActiveDocumentArchiveLoadPhase::AwaitingMembers | ActiveDocumentArchiveLoadPhase::BeginMember | ActiveDocumentArchiveLoadPhase::AdmitMember | ActiveDocumentArchiveLoadPhase::SealMembers | ActiveDocumentArchiveLoadPhase::AwaitReplacement => item,
        })
    }

    /// 📏️ Quotes the next archive-load turn of the target load, in the phase its state machine is about to run.
    pub(crate) fn document_archive_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation)) = self.document_archive_target(closing) else {
            let backing = if closing { self.document_archive_loads.empty_backing_byte_demand().unwrap_or(0) } else { 0 };
            return Ok(if backing == 0 { RetirementDemand::default() } else { RetirementDemand { release_bytes: backing, depth: 1, ..Default::default() } });
        };
        self.document_archive_loads.get(operation).map_or(Ok(RetirementDemand::default()), |active| self.document_archive_load_demands(active, closing, body))
    }

    /// 🧺️ Builds the app's exact owner catalog and hands a rejected envelope to its original retirement; the answer carries both receipts.
    fn retire_document_archive_envelope(envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), Fault> {
        let (owners, built) = match A::build_document_store_owners(grant) {
            Some(Ok(built)) => built,
            Some(Err(refusal)) => {
                std::mem::forget(envelope);
                return Err(owners_admission_fault(refusal));
            }
            None => {
                std::mem::forget(envelope);
                return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive hydration lost its exact owner catalog while retiring a rejected envelope"));
            }
        };
        let remaining = RetainedCloneGrant {
            maximum_items: grant.maximum_items.saturating_sub(built.copied_items),
            maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(built.copied_bytes),
            maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(built.retained_capacity_bytes),
            maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(built.released_bytes),
            maximum_depth: grant.maximum_depth,
        };
        match owners.retire_envelope_uninstalled(envelope, remaining) {
            Ok((retirement, progress)) => Ok((retirement, built.checked_add(progress).map_err(plugin_retirement_fault)?)),
            Err((error, owners, envelope)) => {
                std::mem::forget((owners, envelope));
                Err(plugin_retirement_fault(error).with_retained_progress(built))
            }
        }
    }

    fn drive_document_archive_terminal(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let target = active.terminal_target.ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive terminal cleanup has no target"))?;
        if let Some(handle) = active.replacement {
            if self.store_replacement_jobs.get(handle.operation.0).is_some() {
                if target != ActiveDocumentArchiveLoadState::Ready {
                    self.cancel_artifact_store_replacement(handle)?;
                }
                let poll = self.poll_artifact_store_replacement(handle);
                if matches!(poll, ArtifactEnvelopeDecodeOperationPoll::Ready | ArtifactEnvelopeDecodeOperationPoll::Cancelled | ArtifactEnvelopeDecodeOperationPoll::Fault) && self.acknowledge_artifact_store_replacement(handle)? {
                    active.replacement = None;
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
                return Ok(idle());
            }
            active.replacement = None;
            if target == ActiveDocumentArchiveLoadState::Ready {
                return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "ready recursive document archive lost its replacement acknowledgement authority"));
            }
        }
        match active.close_step(grant).map_err(plugin_retirement_fault)? {
            RetainedCloneStep::Complete(progress) => {
                if !active.terminal_is_empty() {
                    return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive cleanup returned Complete with a live owner"));
                }
                active.state = target;
                active.phase = ActiveDocumentArchiveLoadPhase::Terminal;
                Ok(removed(progress))
            }
            step => Ok(progressed(step)),
        }
    }

    /// 🧵️ One granted unit of a whole-document archive load: decode, hydrate, admit every member, seal and await the replacement.
    ///
    /// Genesis and `try_begin_owned_document_members` happen in ONE step: yielding between them would let the replacement
    /// lane observe `AwaitingMembers` with no ingress and seal the roster before the archived members were admitted. A
    /// childless candidate is sealed by the replacement lane itself, so this load then only awaits its outcome.
    fn advance_document_archive_load(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let item = item();
        let missing = |message: &'static str| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, message);
        if active.terminal_target.is_some() {
            return self.drive_document_archive_terminal(active, grant);
        }
        if active.state == ActiveDocumentArchiveLoadState::Pending {
            active.state = ActiveDocumentArchiveLoadState::Running;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        match active.phase {
            ActiveDocumentArchiveLoadPhase::DecodeParent => {
                let archive = active.archive.as_ref().ok_or_else(|| missing("recursive document archive parent input owner is absent"))?;
                let decoder = active.history.as_mut().ok_or_else(|| missing("recursive document archive parent history decoder is absent"))?;
                match decoder.step(&archive.parent_spr, grant.maximum_copy_bytes, 1).map_err(|error| document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent SPR was rejected: {error}")))? {
                    protocol::RetainedHistoryDecodeStep::Pending { decoded_records, .. } => {
                        let copied_bytes = grant.maximum_copy_bytes.min(archive.parent_spr.len());
                        let discovered = if active.merge { 0 } else { decoded_records.saturating_sub(active.decoded) };
                        active.decoded = decoded_records;
                        active.completed = active.completed.saturating_add(discovered);
                        active.total = active.total.saturating_add(discovered);
                        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes, ..item }))
                    }
                    protocol::RetainedHistoryDecodeStep::Ready => {
                        active.decoded_history = decoder.take_ready();
                        if active.decoded_history.is_none() {
                            return Err(missing("ready recursive document archive history retained no decoded owner"));
                        }
                        let auxiliary = decoder.take_auxiliary_owners();
                        active.retained_pending = Some((None, Some(auxiliary)));
                        let decoder = active.history.take().ok_or_else(|| missing("ready recursive document archive history decoder changed before terminal transfer"))?;
                        if !decoder.terminal_is_empty() {
                            active.history = Some(decoder);
                            return Err(missing("ready recursive document archive history decoder retained an untransferred owner"));
                        }
                        drop(decoder);
                        active.phase = ActiveDocumentArchiveLoadPhase::RetireParentHistoryAuxiliary;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::RetireParentHistoryAuxiliary => {
                let step = if active.retained_pending.is_some() {
                    store::artifact_retirement_admit_owned(&mut active.retained_pending, &mut active.retained, grant).map_err(plugin_retirement_fault)?
                } else if active.retained.is_some() {
                    store::artifact_retirement_box_close_step(&mut active.retained, grant).map_err(plugin_retirement_fault)?
                } else {
                    RetainedCloneStep::Progress(item)
                };
                if active.retained_pending.is_none() && active.retained.is_none() {
                    active.phase = if active.merge { ActiveDocumentArchiveLoadPhase::MergeParent } else { ActiveDocumentArchiveLoadPhase::HydrateParent };
                }
                Ok(progressed(step))
            }
            ActiveDocumentArchiveLoadPhase::MergeParent => Ok(PluginLifecycleStep::Progress(empty)),
            ActiveDocumentArchiveLoadPhase::HydrateParent => {
                if active.hydration.is_none() {
                    let (owners, built) = A::build_document_store_owners(grant).ok_or_else(|| missing("document archive parent requires the app's exact document owner catalog"))?.map_err(owners_admission_fault)?;
                    let archive = active.archive.as_mut().ok_or_else(|| missing("recursive document archive parent input owner is absent"))?;
                    let pack = std::mem::take(&mut archive.parent_pack);
                    let history = active.decoded_history.take().ok_or_else(|| missing("recursive document archive decoded history owner is absent"))?;
                    let expected = ArtifactRef { artifact_id: history.doc_id.clone(), dialect: A::DIALECT.into() };
                    let owner = self.store.envelope().owner.clone();
                    active.hydration = Some(store::RetainedPersistedDocumentHydration::from_pack(
                        pack,
                        history,
                        expected,
                        owner,
                        A::DOCUMENT_SCHEMA.to_string(),
                        owners,
                        semio_framework_job::OperationId(active.operation),
                        semio_framework_job::Generation(self.store.generation_now()),
                        u64::MAX,
                        store::PersistedDocumentHydrationTarget::Envelope,
                        self.store.local_actor_id().clone(),
                    ));
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: built.copied_items.max(1), ..built }));
                }
                let mut sequence = active.hydration_sequence;
                let mut spent = RetainedCloneProgress::default();
                let fuel = grant.maximum_items.max(grant.maximum_copy_bytes.min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)) as u64;
                let mut cx = semio_framework_job::StepContext::new(
                    semio_framework_job::OperationId(active.operation),
                    semio_framework_job::Generation(self.store.generation_now()),
                    semio_framework_job::StepBudget::new(fuel, u64::MAX, grant),
                    semio_framework_job::CancelToken::root_now(),
                    semio_framework_job::default_now_us,
                    &mut sequence,
                    &mut spent,
                );
                let step = active.hydration.as_mut().expect("recursive document parent hydration remains retained").step(&mut cx, RetainedCloneGrant { maximum_items: 1, ..grant });
                drop(cx);
                active.hydration_sequence = sequence;
                let item = RetainedCloneProgress { copied_items: spent.copied_items.max(1), ..spent };
                let envelope = match step {
                    store::PersistedDocumentHydrationStep::Pending(_) => return Ok(PluginLifecycleStep::Progress(item)),
                    store::PersistedDocumentHydrationStep::Rejected(diagnostic) => {
                        return Err(document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent Pack and SPR hydration was rejected: {diagnostic:?}")).with_retained_progress(spent));
                    }
                    store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Envelope(envelope)) => envelope,
                    store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Store(_)) => {
                        return Err(missing("document archive parent hydration returned a store outside its requested envelope boundary"));
                    }
                };
                let hydration = active.hydration.take().ok_or_else(|| plugin_sdk_fault("ready recursive document parent hydration owner changed before handoff"))?;
                if !store::ErasedSnapshotRetirement::terminal_is_empty(&hydration) {
                    active.hydration = Some(hydration);
                    let (retained, retired) = Self::retire_document_archive_envelope(envelope, grant)?;
                    active.retained = Some(retained);
                    return Err(plugin_sdk_fault("ready recursive document parent hydration retained nonterminal ownership").with_retained_progress(spent.checked_add(retired).map_err(plugin_retirement_fault)?));
                }
                drop(hydration);
                match self.begin_persisted_document_store_replacement(envelope) {
                    Ok(handle) => {
                        active.replacement = Some(handle);
                        active.completed = active.completed.saturating_add(1);
                        active.phase = ActiveDocumentArchiveLoadPhase::AwaitingMembers;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                    Err((fault, envelope)) => {
                        let (retained, retired) = Self::retire_document_archive_envelope(envelope, grant)?;
                        active.retained = Some(retained);
                        Err(fault.with_retained_progress(spent.checked_add(retired).map_err(plugin_retirement_fault)?))
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::AwaitingMembers => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let state = self.store_replacement_jobs.get(handle.operation.0).map(|replacement| replacement.state);
                if let Some(fold) = self.store_replacement_jobs.get(handle.operation.0).map(|replacement| replacement.fold_progress) {
                    active.fold = fold;
                }
                match state {
                    Some(ActiveArtifactStoreReplacementState::AwaitingMembers) => {
                        if !active.genesis_complete {
                            self.complete_document_archive_genesis(active, handle)?;
                            active.genesis_complete = true;
                        }
                        let expected = active.archive.as_ref().ok_or_else(|| missing("recursive document archive member roster owner is absent"))?.members.len();
                        if self.try_begin_owned_document_members(handle, expected, u64::MAX)? {
                            active.phase = ActiveDocumentArchiveLoadPhase::BeginMember;
                            Ok(PluginLifecycleStep::Progress(item))
                        } else {
                            Ok(PluginLifecycleStep::Progress(empty))
                        }
                    }
                    Some(ActiveArtifactStoreReplacementState::Complete) | None => Err(missing("document archive parent initialization failed before retained member admission")),
                    Some(ActiveArtifactStoreReplacementState::Initializing) => Ok(PluginLifecycleStep::Progress(empty)),
                    Some(_) => {
                        if active.archive.as_ref().map_or(0, |archive| archive.members.len()) != 0 {
                            return Err(missing("document archive replacement sealed its member roster before the archived members were admitted"));
                        }
                        active.genesis_complete = true;
                        active.phase = ActiveDocumentArchiveLoadPhase::AwaitReplacement;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::BeginMember => {
                let archive = active.archive.as_mut().ok_or_else(|| missing("recursive document archive member roster owner is absent"))?;
                let Some(entry) = archive.members.pop() else {
                    active.phase = ActiveDocumentArchiveLoadPhase::SealMembers;
                    return Ok(PluginLifecycleStep::Progress(item));
                };
                match PendingDocumentArchiveMember::new(entry) {
                    Ok(member) => {
                        active.member = Some(member);
                        active.phase = ActiveDocumentArchiveLoadPhase::FillMember;
                    }
                    Err((fault, entry)) => {
                        archive.members.push(entry);
                        return Err(fault);
                    }
                }
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::FillMember => {
                let member = active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?;
                let before = member.copied;
                let filled = member.fill_one_page(grant.maximum_copy_bytes.min(grant.maximum_capacity_bytes))?;
                let bytes = member.copied - before;
                if filled {
                    active.phase = ActiveDocumentArchiveLoadPhase::RetireMemberSource;
                }
                Ok(PluginLifecycleStep::Progress(if filled || bytes != 0 { RetainedCloneProgress { copied_bytes: bytes, retained_capacity_bytes: bytes, ..item } } else { empty }))
            }
            ActiveDocumentArchiveLoadPhase::RetireMemberSource => {
                let member = active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?;
                match member.retire_source_step(grant) {
                    RetainedCloneStep::Complete(_) => {
                        active.phase = ActiveDocumentArchiveLoadPhase::AdmitMember;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                    RetainedCloneStep::Progress(progress) => Ok(PluginLifecycleStep::Progress(progress)),
                }
            }
            ActiveDocumentArchiveLoadPhase::AdmitMember => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let ingress = match active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?.take_ingress(handle, self.store.local_actor_id().clone()) {
                    Ok(ingress) => ingress,
                    Err((fault, ingress)) => {
                        if !ingress.terminal_is_empty() {
                            active.rejected_ingress = Some(ingress);
                        }
                        return Err(fault);
                    }
                };
                if let Err((fault, ingress)) = self.admit_owned_document_member(handle, ingress) {
                    active.rejected_ingress = Some(ingress);
                    return Err(fault);
                }
                let member = active.member.take().ok_or_else(|| missing("recursive document archive transferred member owner changed before exact removal"))?;
                if !member.terminal_is_empty() {
                    active.member = Some(member);
                    return Err(missing("recursive document archive admitted member retained a local owner"));
                }
                drop(member);
                active.completed = active.completed.saturating_add(1);
                active.phase = ActiveDocumentArchiveLoadPhase::BeginMember;
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::SealMembers => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                self.seal_owned_document_members(handle)?;
                active.phase = ActiveDocumentArchiveLoadPhase::AwaitReplacement;
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::AwaitReplacement => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let target = match self.poll_artifact_store_replacement(handle) {
                    ArtifactEnvelopeDecodeOperationPoll::Pending | ArtifactEnvelopeDecodeOperationPoll::Progress => return Ok(PluginLifecycleStep::Progress(empty)),
                    ArtifactEnvelopeDecodeOperationPoll::Ready => ActiveDocumentArchiveLoadState::Ready,
                    ArtifactEnvelopeDecodeOperationPoll::Cancelled => ActiveDocumentArchiveLoadState::Cancelled,
                    ArtifactEnvelopeDecodeOperationPoll::Fault => ActiveDocumentArchiveLoadState::Fault,
                };
                let refusal = self.artifact_store_replacement_refusal(handle);
                if !self.acknowledge_artifact_store_replacement(handle)? {
                    return Ok(PluginLifecycleStep::Progress(empty));
                }
                active.replacement = None;
                if target == ActiveDocumentArchiveLoadState::Fault {
                    active.fault = semio_framework_diagnostic::encode_fault_bytes(&refusal.unwrap_or_else(|| missing("document archive replacement failed closure, authority, or retained publication validation before it recorded a leg")));
                }
                active.request_terminal(target);
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::Terminal => Ok(PluginLifecycleStep::Complete(empty)),
        }
    }

    /// 🎟️ One granted turn of the next archive load; a closing app cancels every load and drops it once terminal-empty.
    pub(crate) fn drive_document_archive_load_retirements(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let Some((index, operation)) = self.document_archive_target(closing) else {
            if closing && self.document_archive_loads.empty_backing_byte_demand().is_some_and(|bytes| bytes != 0) {
                return self.document_archive_loads.close_empty_backing_step(grant).map(|step| PluginLifecycleStep::Progress(step.progress()));
            }
            return Ok(PluginLifecycleStep::Complete(empty));
        };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close_document_archive_cursor } else { &mut self.maintenance_document_archive_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let mut active = self.document_archive_loads.remove(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive retirement authority changed during one bounded step"))?;
        if closing && !active.terminal() {
            active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
        }
        let funded = self.document_archive_load_demands(&active, closing, grant.maximum_copy_bytes).map_err(plugin_retirement_fault).and_then(|demand| plugin_turn_admitted(demand, grant));
        if !matches!(funded, Ok(true)) {
            self.document_archive_loads.insert_admitted(operation, active);
            return funded.map(|_| PluginLifecycleStep::Progress(empty));
        }
        let step = match self.advance_document_archive_load(&mut active, grant) {
            Ok(step) => Ok(step),
            Err(primary) => {
                active.request_fault(&primary);
                self.drive_document_archive_terminal(&mut active, grant).map_err(|cleanup| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("recursive document archive load failed before retained cleanup: {primary:?}; retained cleanup also failed: {cleanup:?}")))
            }
        };
        if closing && active.terminal() && active.terminal_is_empty() {
            drop(active);
            return Ok(PluginLifecycleStep::Progress(item()));
        }
        self.document_archive_loads.insert_admitted(operation, active);
        step
    }
}
