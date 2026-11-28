//! 🪜️ The live-maintenance rotation of one hosted app: priority and pressure units first, then a fair fixed round robin
//! in which an empty stage never consumes the call. Every unit quotes its next demand on all axes and spends the caller's
//! unchanged grant on exactly one owner; the same module drives the envelope-ingress, store-replacement and archive-load
//! frontiers for both the live rotation and the close ladder.
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;
use semio_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedRegistry, PluginLifecycleStep, plugin_retirement_fault, plugin_turn_admitted};
use semio_plugin_instance_stores::stores_components::{ActionIngress, InstanceStores, LocalInteraction, PeerRoster};
use semio_plugin_document_media::{ActiveArtifactEnvelopeIngress, ActiveArtifactStoreReplacement, ActiveArtifactStoreReplacementState, ActiveDocumentArchiveLoadState, ArtifactStoreReplacementPublicationGuard, ArtifactStoreReplacementRefusal, CompositionPinsRetirement, publish_boxed_document_store_candidate_if_authoritative};
use semio_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, document_load_fault};
use semio_plugin_store_lifecycle::store_lifecycle::{ArtifactDisposal, ArtifactDocumentStoreDisposer, ArtifactOwnedDisposer, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, ReferenceSpaceMember};
use semio_plugin_instance_children::children_components::ChildComposition;
use semio_plugin_child::{ChildContentRetirement, ChildMemberRegistry, ChildMemberRetirement};
use semio_plugin_decl_views::child_view::{ChildContentView, child_content_arc_bytes};
use crate::ladders_components::{CloseCursors, MaintenanceCursors};
use semio_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use semio_plugin_decl_emission::Emit;
use semio_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_plugin_mounted_owner::MountedOwnerPhaseV1;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use semio_plugin_interaction::mutations::InteractionConfigMutation;
use crate::ladder_bundle::{LadderCtx, LadderRef};
use std::mem::ManuallyDrop;
use semio_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_plugin_decl_views::PeerPresenceRoot;
use semio_plugin_peer::{PeerPresenceRootRetirement, PeerRosterPublication, PeerRosterPublicationStage, PresenceRosterOutcome};
use semio_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_instance_render::render_bundle::{RenderCtx, RenderRef};
use semio_plugin_operation::TOOL_CANCELLATION_SLOTS;
use semio_plugin_time_travel_turns::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
use semio_plugin_time_travel::TimeTravelLedger;
use semio_plugin_tool_machine::ToolMachineRuntime;
use semio_plugin_tool_run_turns::tool_run_bundle::{ToolRunCtx, ToolRunRef};
use semio_plugin_tool_run::ToolRunLedger;
use semio_plugin_instance_typed_ops::typed_bundle::TypedCtx;
use semio_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_plugin_composition_emission::root::{advance_original_child_root_retirement, original_child_root_retirement_demands};
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use crate::ladders_document_replacement::{retire_document_window_registry_demands, retire_document_window_registry_step};
use semio_ui_contract::*;
use semio_plugin_time_travel as time_travel;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

use semio_plugin_instance_docs::docs_maintenance_ladder::{idle, item, item_demand, progressed, removed};
use crate::ladders_plugin_app_impl::{MAINTENANCE_CHILD_MEMBER_STAGE, MAINTENANCE_CHILD_ROOT_STAGE, MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE, MAINTENANCE_DOCUMENT_DISPLACED_STAGE, MAINTENANCE_STAGES};

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaintenanceUnit {
    CommandPrune,
    MediaRefusal,
    ActionIngress,
    AgentPreview,
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
    AwaitingPublication,
    Idle,
}

const ROTATION_START: usize = 14;

#[doc(hidden)]
pub const MAINTENANCE_POSITIONS: usize = 1 + ROTATION_START + MAINTENANCE_STAGES as usize;

/// 🌡️ The rotation stage that pumps returned snapshot reads; the ladder also visits it first while any issuer is under return pressure.
const MAINTENANCE_SNAPSHOT_RETURN_STAGE: u8 = 8;

fn maintenance_fault(code: &'static str, message: &'static str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

fn worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)))
}

impl MaintenanceUnit {
    pub fn trace(self) -> u64 {
        match self {
            Self::Pressure(stage) | Self::Stage(stage) => u64::from(stage),
            Self::AgentPreview=>108,
            Self::MediaRefusal => 107,
            Self::ActionIngress=>109,
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

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
    pub fn maintenance_unit_step(&mut self,unit:MaintenanceUnit,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        match unit{
            MaintenanceUnit::AgentPreview=>self.tools.close_agent_lane_preview(cx, grant),
            MaintenanceUnit::MediaRefusal=>self.media.close_media_submission_refusal(&mut *self.stores,&mut *self.tools, cx, grant),
            MaintenanceUnit::ActionIngress=>self.ingress.close_action_ingress_sources(cx, grant, false),
            MaintenanceUnit::CommandPrune=>{
                self.history.advance_command_prune_step(&mut *self.stores, grant, false).map_err(Into::into)
            },
            MaintenanceUnit::PrivateChildGroup(operation)=>Ok(match self.child_composition.close_private_child_group_operation_step(&mut *self.stores, operation, cx, grant)?{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::Blocked{reason:"original private child group awaits custody"}}),
            MaintenanceUnit::DirectIngress=>{
                let Some(((result,work),_))=cx.with_original_effect(grant,|_,effect|{let result=self.stores.window_config_store.close_direct_ingress(effect);(result,effect.performed_work())})? else{return Ok(PluginLifecycleStep::Progress(Default::default()))};
                semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work}).map(progressed).map_err(Into::into)
            }
            MaintenanceUnit::GrantedMounted=>self.granted_mounted_retirement_step(cx).map(progressed),
            MaintenanceUnit::ToolOverlay => {
                let mut render = self.as_render();
                if render.tool_overlay_waits_for_foreground() {
                    return Ok(PluginLifecycleStep::AwaitingInput { reason: "tool overlay waits for the original foreground source" });
                }
                render.tool_overlay_retirement_step(cx, grant, false).map(progressed).map_err(Into::into)
            },
            MaintenanceUnit::ToolRuns=>Ok(self.as_tool_run().tool_run_retire_step(cx)?.map_or_else(idle,progressed)),
            MaintenanceUnit::TimeTravel=>Ok(self.as_time_travel().time_travel_retire_step_context(grant,cx)?.map_or_else(idle,progressed)),
            MaintenanceUnit::Pressure(stage)|MaintenanceUnit::Stage(stage)=>self.maintenance_stage_step(stage,cx,grant),
        }
    }

    fn maintenance_stage_step(&mut self,stage:u8,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if stage==26{return self.pending.close_document_window_admission(cx, false)}
        if stage==17{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.tools.latest_wins_keys.advance(effect).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)).map(progressed)}
        if stage==18{
            if self.tools.tool_cancellations.live_cleanup_awaits_custody() {
                return Ok(PluginLifecycleStep::AwaitingInput { reason: "cancellation cleanup awaits its original lease custody" });
            }
            let before=cx.retained_progress();let complete=self.tools.tool_cancellations.cleanup_slot(self.maintenance.maintenance_cancellation_cursor,cx,true)?;
            let progress=semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;
            if complete&&progress==RetainedCloneProgress::default(){
                let bytes=std::mem::size_of::<usize>();if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_copy_bytes<bytes||cx.retained_grant().maximum_depth==0{return Ok(idle())}
                cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
                self.maintenance.maintenance_cancellation_cursor=(self.maintenance.maintenance_cancellation_cursor+1)%(TOOL_CANCELLATION_SLOTS+ARTIFACT_LIVE_OUTPUT_SLOTS);
            }
            return Ok(PluginLifecycleStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
        }
        if stage==13{return self.drive_envelope_completed_record_returns(cx,grant,false)}
        if stage==12{return self.drive_envelope_field_decoder_returns(cx,grant,false)}
        if stage==24{return self.drive_document_archive_load_retirements(cx,grant,false)}
        if stage==11{return self.drive_envelope_decode_jobs(cx,grant,false)}
        if stage==16{return self.tools.instance_operation_owner.maintenance_step(cx,grant)}
        if stage == 15 {
            let Some(instance_id) = self.tools.live_runtime_instance_id else { return Ok(idle()) };
            return if self.ingest.document_blocks_pending_effects() {
                A::mounted_job_close_step(instance_id, cx, grant)
            } else {
                A::mounted_job_maintenance_step(instance_id, cx, grant)
            };
        }
        if stage==14{return self.drive_store_replacement_jobs(cx,grant,false)}
        if stage==23{return Ok(match semio_framework_job_worker::pump_worker_job_retirements(1,cx)?{semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress}|semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>PluginLifecycleStep::Blocked{reason:"original worker retirement awaits custody"}})}
if stage==0{return match self.shared().maintenance_tool_selection(){MaintenanceTool::AwaitingPublication=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed command awaits its original dispatch or publication turn"}),MaintenanceTool::Awaiting=>Ok(PluginLifecycleStep::AwaitingInput{reason:"every typed operation awaits its exact presented host result ACK"}),MaintenanceTool::Idle=>Ok(idle()),MaintenanceTool::Worker(index,operation_id)=>{self.maintenance.maintenance_tool_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;let owner=self.tools.tool_operations.get_mut(operation_id).ok_or_else(||maintenance_fault("interactive-job.maintenance-tool-authority","typed operation authority changed during its original turn"))?;cx.with_retained_loan(grant,|same|owner.drive_worker_step(same)).map_err(Into::into)}}}
        if stage==1{
            let Some((index,operation_id))=self.media.media_closures.next_id_from(self.maintenance.maintenance_media_cursor)else{return Ok(idle())};self.maintenance.maintenance_media_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
            if self.media.media_closures.get(operation_id).is_some_and(|active|active.download_pending){return Ok(PluginLifecycleStep::AwaitingInput{reason:"terminal media source remains retained until its original output receiving"})}
            let authority=||maintenance_fault("interactive-job.maintenance-media-authority","live media cleanup authority changed during its original turn");
            let step=self.media.media_closures.get_mut(operation_id).ok_or_else(authority)?.close_step(cx,grant,&self.tools.tool_cancellations)?;
            let PluginLifecycleStep::Complete(progress)=step else{return Ok(step)};
            let active=self.media.media_closures.get(operation_id).ok_or_else(authority)?;
            if !active.terminal_is_empty()?{return Err(maintenance_fault("interactive-job.maintenance-media-terminal-not-empty","live media cleanup reported Complete without its exact terminal witness").into())}
            if self.media.snapshot_retirements.get(operation_id).is_none(){return Err(maintenance_fault("interactive-job.maintenance-snapshot-quarantine","media export lost its original snapshot retirement owner").into())}
            drop(self.media.media_closures.remove(operation_id).ok_or_else(authority)?);return Ok(removed(progress))
        }
        if stage==2{
                let Some((index,operation_id))=self.media.segmented_closures.next_id_from(self.maintenance.maintenance_segment_cursor)else{return Ok(idle())};self.maintenance.maintenance_segment_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
                if let Some(source)=self.media.snapshot_retirements.get(operation_id){source.validate_retirement(cx)?}
                let original=self.media.segmented_closures.get_mut(operation_id).ok_or_else(||maintenance_fault("interactive-job.segment-close","original segmented close owner changed"))?;
                if original.terminal_is_empty(){self.media.segmented_closures.remove(operation_id);return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<semio_plugin_document_media::media_export::MediaDownloadRetirement>(),..Default::default()}))}return original.close_step(cx,grant)
        }
        if stage==3{
                let Some((index, operation_id)) = self.media.snapshot_retirements.next_id_from(self.maintenance.maintenance_snapshot_cursor) else { return Ok(idle()) };
                self.maintenance.maintenance_snapshot_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                if self.media.media_exports.get(operation_id).is_some() || self.media.media_closures.get(operation_id).is_some() || self.media.segmented_downloads.get(operation_id).is_some() || self.media.segmented_closures.get(operation_id).is_some() || self.media.media_download_preparation.as_ref().is_some_and(|pending|pending.operation_id==operation_id) {
                    return Ok(PluginLifecycleStep::Blocked { reason: "snapshot retirement waits for its exact live media owner" });
                }
                let authority = || maintenance_fault("interactive-job.maintenance-snapshot-authority", "live snapshot retirement authority changed during one fixed step");
                let step = self.media.snapshot_retirements.get_mut(operation_id).ok_or_else(authority)?.close_step(cx,grant)?;
                let PluginLifecycleStep::Complete(final_progress) = step else { return Ok(step) };
                if !self.media.snapshot_retirements.get(operation_id).is_some_and(ArtifactSnapshotCloseRetention::terminal_is_empty) {
                    return Err(maintenance_fault("interactive-job.maintenance-snapshot-terminal-not-empty", "live snapshot retirement reported Complete without its exact terminal-empty witness").into());
                }
                drop(self.media.snapshot_retirements.remove(operation_id).ok_or_else(authority)?);
                return Ok(removed(final_progress))
        }
        if stage==8{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.ingest.advance_snapshot_read_returns_one(&mut *self.stores,&mut *self.interaction,effect).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)).map(progressed)}
        if stage==MAINTENANCE_CHILD_ROOT_STAGE{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.child_root_retirement_step(effect).map(RetainedCloneStep::Progress).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)).map(progressed)}
        if stage==MAINTENANCE_CHILD_MEMBER_STAGE{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.child_member_retirement_step(effect).map(RetainedCloneStep::Progress).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)).map(progressed)}
        if stage==MAINTENANCE_DOCUMENT_DISPLACED_STAGE {
            if !self.stores.store.has_pending_maintenance_retirements() { return Ok(idle()); }
            return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|semio_plugin_ownership::bounded_ownership::plugin_settle_poll(self.stores.store.maintenance_retirements_step(effect))).map(progressed);
        }
        if stage==MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE{
            return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|{
                use semio_plugin_ownership::bounded_ownership::{plugin_settle_poll,PluginLifecycleFailure};
                if self.stores.config_store.has_pending_maintenance_retirements(){plugin_settle_poll(self.stores.config_store.maintenance_retirements_step(effect))}
                else if self.stores.draft_store.has_pending_maintenance_retirements(){plugin_settle_poll(self.stores.draft_store.maintenance_retirements_step(effect))}
                else if self.stores.interaction_store.has_pending_maintenance_retirements(){plugin_settle_poll(self.stores.interaction_store.maintenance_retirements_step(effect))}
                else{self.stores.window_config_store.maintenance_retirements_step(effect).map_err(PluginLifecycleFailure::from)}
            }).map(progressed)
        }
        let result=(||->Result<PluginLifecycleStep,Fault>{
        let value_fault = plugin_retirement_fault;
        match stage {
            MAINTENANCE_CHILD_ROOT_STAGE => unreachable!("original child retirement handled before the fault-only ladder"),
            5 => {
                let Some((index, generation)) = self.roster.peer_presence_retirements.next_id_from(self.maintenance.maintenance_peer_presence_cursor) else { return Ok(idle()) };
                let authority = || maintenance_fault("interactive-job.maintenance-peer-presence-authority", "live peer-presence retirement authority changed during one fixed step");
                let Some(closed)=Self::effected(cx,grant,|effect|self.roster.peer_presence_retirements.get_mut(generation).map(|owner|owner.close_step(effect)))? else { return Ok(idle()) };
                match closed.ok_or_else(authority)?.map_err(value_fault)? {
                    PluginLifecycleStep::Progress(progress) => {
                        self.maintenance.maintenance_peer_presence_cursor = if progress == RetainedCloneProgress::default() { (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS } else { index };
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                    PluginLifecycleStep::Complete(progress) => {
                        if !self.roster.peer_presence_retirements.get(generation).is_some_and(PeerPresenceRootRetirement::terminal_is_empty) {
                            return Err(maintenance_fault("interactive-job.maintenance-peer-presence-terminal-not-empty", "live peer-presence retirement reported Complete without its exact terminal-empty witness"));
                        }
                        drop(self.roster.peer_presence_retirements.remove(generation).ok_or_else(authority)?);
                        self.maintenance.maintenance_peer_presence_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(removed(progress))
                    }
                    blocked => {
                        self.maintenance.maintenance_peer_presence_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(blocked)
                    }
                }
            }
            6 => {
                let Some((index, generation)) = self.roster.presence_peer_retirements.next_id_from(self.maintenance.maintenance_presence_peer_cursor) else { return Ok(idle()) };
                let authority = || maintenance_fault("interactive-job.maintenance-presence-peer-authority", "live app-typed presence retirement authority changed during one fixed step");
                let Some(closed)=Self::effected(cx,grant,|effect|self.roster.presence_peer_retirements.get_mut(generation).map(|owner|{let result=owner.close_step(effect);semio_plugin_ownership::bounded_ownership::plugin_settle_value(semio_framework_value::RetirementPollExit::settle(effect,result))}))? else { return Ok(idle()) };
                match closed.ok_or_else(authority)?.map_err(value_fault)? {
                    RetainedCloneStep::Progress(progress) => {
                        self.maintenance.maintenance_presence_peer_cursor = if progress == RetainedCloneProgress::default() { (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS } else { index };
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                    RetainedCloneStep::Complete(progress) => {
                        if !self.roster.presence_peer_retirements.get(generation).is_some_and(store::os_store::presence::PresencePeersRetirement::terminal_is_empty) {
                            return Err(maintenance_fault("interactive-job.maintenance-presence-peer-terminal-not-empty", "live app-typed presence retirement reported Complete without its exact terminal-empty witness"));
                        }
                        drop(self.roster.presence_peer_retirements.remove(generation).ok_or_else(authority)?);
                        self.maintenance.maintenance_presence_peer_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                        Ok(removed(progress))
                    }
                }
            }
            7 => self.maintenance_peer_roster_step(cx, grant),
            8 => unreachable!("original snapshot returns handled before the fault-only ladder"),
            MAINTENANCE_DOCUMENT_DISPLACED_STAGE => unreachable!("original displaced retirement handled before the fault-only ladder"),
            10 => self.drive_envelope_ingress(grant, false),
            11 => unreachable!("original envelope decode handled before the fault-only ladder"),
            12 => unreachable!("original decoder return handled before the fault-only ladder"),
            13 => unreachable!("original completed record return handled before fault-only ladder"),

            17 => unreachable!("original latest-wins key advance handled before the fault-only ladder"),
            18 => unreachable!("original cancellation cleanup requires its actual caller"),
            19 => match Self::effected(cx,grant,|effect|self.stores.presence_store.maintenance_local_reads_step(effect))? { Some(poll) => semio_plugin_ownership::bounded_ownership::plugin_settle_value(poll).map(progressed).map_err(value_fault), None => Ok(idle()) },
            MAINTENANCE_CHILD_MEMBER_STAGE => unreachable!("original child retirement handled before the fault-only ladder"),
            21 => match Self::effected(cx,grant,|effect|self.retire_document_windows_step(effect,false))? { Some(step) => step, None => Ok(idle()) },
            22 => match Self::effected(cx,grant,|effect|self.stores.window_transient_store.maintenance_step(effect))? { Some(step) => step, None => Ok(idle()) },

            24 => unreachable!("original archive handled before the fault-only ladder"),
            MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE => unreachable!("original lane displaced retirement handled before the fault-only ladder"),
            26 => unreachable!("original window admission source return requires its actual caller"),
            _ => unreachable!("fixed maintenance stage"),
        }
        })();result.map_err(Into::into)
    }

    /// 🪙️ Runs one effect-taking step inside the turn's own performed-work effect and posts its performed work; `None` means the Work quote was not admitted and nothing ran.
    fn effected<R>(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->Result<Option<R>,Fault>{
        let Some((value,performed))=cx.with_original_effect(grant,|_,effect|run(effect)).map_err(plugin_retirement_fault)? else { return Ok(None) };
        cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));Ok(Some(value))
    }

    /// 🧭️ Walks one mounted peer roster through decode, validation, atomic publication and outcome hand-off, or through its cleanup once faulted.
    fn maintenance_peer_roster_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let Some(generation) = self.roster.maintenance_peer_roster_generation() else { return Ok(idle()) };
        let index = InstanceStores::<A>::peer_roster_slot(generation);
        let authority = || maintenance_fault("interactive-job.maintenance-peer-roster-authority", "peer roster publication changed during one fixed step");
        let publication = self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?;
        let faulted = publication.is_faulted();
        let ready = !faulted && publication.stage == PeerRosterPublicationStage::Ready;
        let step = if faulted {
            match Self::effected(cx, grant, |effect| publication.close_step(effect))? { Some(result) => result?, None => PluginLifecycleStep::Progress(RetainedCloneProgress::default()) }
        } else if ready {
            PluginLifecycleStep::Complete(RetainedCloneProgress::default())
        } else {
            match publication.step(&self.stores.presence_store, cx, grant) {
                Ok(step) => step,
                Err(fault) => {
                    publication.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            }
        };
        let PluginLifecycleStep::Complete(mut progress) = step else {
            self.maintenance.maintenance_peer_roster_cursor = index;
            return Ok(step);
        };
        if faulted {
            if !self.roster.peer_roster_publications.get(generation).is_some_and(PeerRosterPublication::terminal_is_empty) {
                return Err(maintenance_fault("interactive-job.maintenance-peer-roster-terminal-not-empty", "faulted peer roster publication reported Complete without its exact terminal-empty witness"));
            }
        } else {
            let (seq, cancel) = {
                let publication = self.roster.peer_roster_publications.get(generation).ok_or_else(authority)?;
                (publication.seq, publication.cancel.clone())
            };
            let admission = match self.roster.validate_peer_roster_publication(&*self.stores, seq, generation, &cancel) {
                Ok(admission) => admission,
                Err(fault) => {
                    self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            };
            let candidate = match self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.take_candidate() {
                Ok(candidate) => candidate,
                Err(fault) => {
                    self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.fail(fault);
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
            };
            let capacity = child_content_arc_bytes::<PeerPresenceRoot>();
            if let Err(candidate) = self.roster.publish_peer_roster_candidate_admitted(&mut *self.stores, admission, candidate) {
                self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?.retain_rejected_candidate(candidate);
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { retained_capacity_bytes: capacity, ..item() }));
            }
            progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(capacity);
        }
        if !self.roster.peer_roster_publications.get(generation).is_some_and(PeerRosterPublication::terminal_is_empty) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-terminal-not-empty", "peer roster publication reached outcome handoff without an exact empty terminal witness"));
        }
        let (seq, fault) = {
            let publication = self.roster.peer_roster_publications.get_mut(generation).ok_or_else(authority)?;
            (publication.seq, publication.fault.take())
        };
        if !self.roster.peer_roster_outcomes.can_insert(generation) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-outcome", "pre-admitted peer roster outcome slot changed before exact handoff"));
        }
        self.roster.peer_roster_outcomes.insert_admitted(generation, PresenceRosterOutcome { seq, fault });
        drop(self.roster.peer_roster_publications.remove(generation).ok_or_else(authority)?);
        let reservation_slot = InstanceStores::<A>::peer_roster_slot(generation);
        if self.roster.peer_roster_reservations[reservation_slot] != Some((generation, seq)) {
            return Err(maintenance_fault("interactive-job.maintenance-peer-roster-reservation", "terminal peer roster lost its exact ingress reservation"));
        }
        self.roster.peer_roster_reservations[reservation_slot] = None;
        self.roster.peer_roster_processed_generation = self.roster.peer_roster_processed_generation.max(generation);
        self.maintenance.maintenance_peer_roster_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        Ok(removed(progress))
    }

    /// 🎟️ One granted release turn of the next closing envelope ingress; a closing app closes every ingress.
    pub fn drive_envelope_ingress(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        let Some((index, operation_id)) = self.shared().envelope_ingress_target(closing) else { return Ok(PluginLifecycleStep::Complete(empty)) };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close.close_envelope_ingress_cursor } else { &mut self.maintenance.maintenance_envelope_ingress_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let ingress = self.ingest.envelope_ingress.get_mut(operation_id).ok_or_else(|| maintenance_fault("artifact-envelope.ingress-owner", "artifact envelope ingress changed during one bounded maintenance step"))?;
        if closing {
            ingress.closing = true;
        }
        match ingress.close_step(grant)? {
            RetainedCloneStep::Progress(progress) => Ok(PluginLifecycleStep::Progress(progress)),
            RetainedCloneStep::Complete(progress) => {
                let ingress = self.ingest.envelope_ingress.remove(operation_id).ok_or_else(|| maintenance_fault("artifact-envelope.ingress-owner", "terminal artifact envelope ingress changed before exact removal"))?;
                if !ingress.terminal_is_empty() {
                    return Err(maintenance_fault("artifact-envelope.ingress-false-terminal", "artifact envelope ingress reported Complete before every page was released"));
                }
                drop(ingress);
                Ok(removed(progress))
            }
        }
    }

    /// 🎁️ One granted turn of the next store replacement; a closing app cancels it first and removes it once terminal.
    pub fn drive_store_replacement_jobs(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant,closing:bool)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let demand=self.shared().store_replacement_demands(closing,loan.maximum_copy_bytes)?;
        let advance=|same:&mut semio_framework_job::StepContext<'_>,grant|{
            let step=self.drive_store_replacement_original(same,grant,closing)?;
            let terminal=self.ingest.store_replacement_jobs.is_empty();
            let step=if !terminal{match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step}}else{step};
            Ok((step,terminal))
        };
        semio_plugin_document_media::original_composed_lifecycle_step(cx,loan,demand,advance)
    }

    /// 📈️ One store-replacement turn, which also adds the records the turn paid to the replacement's decode count: the load reports it as the progress of the long phase.
    fn drive_store_replacement_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let target = self.shared().store_replacement_target(closing).map(|(_, operation_id)| operation_id);
        let step = self.drive_store_replacement_unit(cx, grant, closing);
        thread_local! { static DEBUG_ZERO_RUN: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
        match &step {
            Ok(PluginLifecycleStep::Progress(progress)) if *progress == RetainedCloneProgress::default() => {
                let run = DEBUG_ZERO_RUN.with(|run| { run.set(run.get() + 1); run.get() });
                if run % 32 == 1 {
                    let state = target.and_then(|operation_id| self.ingest.store_replacement_jobs.get(operation_id).map(|active| active.state));
                    let quote = self.shared().store_replacement_demands(closing, grant.maximum_copy_bytes);
                    eprintln!("[DEBUG] store replacement zero reading run={run} state={state:?} grant={grant:?} quote={quote:?} fuel={} should_yield={} cancelled={}", cx.fuel_remaining(), cx.should_yield(), cx.is_cancelled());
                }
            }
            _ => DEBUG_ZERO_RUN.with(|run| run.set(0)),
        }
        if let (Some(operation_id), Ok(PluginLifecycleStep::Progress(progress))) = (target, &step) {
            if let Some(active) = self.ingest.store_replacement_jobs.get_mut(operation_id) {
                active.decoded = active.decoded.saturating_add(progress.copied_items as u64);
            }
        }
        step
    }

    fn drive_store_replacement_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        use ActiveArtifactStoreReplacementState as State;
        let empty = RetainedCloneProgress::default();
        let item = item();
        let Some((index, operation_id)) = self.shared().store_replacement_target(closing) else { return Ok(PluginLifecycleStep::Complete(empty)) };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close.close_store_replacement_cursor } else { &mut self.maintenance.maintenance_store_replacement_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let owner = || maintenance_fault("artifact-store.replacement-owner", "store replacement changed during one fixed maintenance step");
        let active = self.ingest.store_replacement_jobs.get_mut(operation_id).ok_or_else(owner)?;
        let mounted_cancelled=active.mounted_genesis.is_some_and(|(operation,_)|{
            let lease=self.tools.mounted_genesis_lease(operation,active.operation,active.generation);
            lease.is_none_or(|lease|lease.publication_is_cancelled()||lease.publication_claim().is_finished())
        });
        if closing || mounted_cancelled && !active.committed {
            active.request_cancel();
        }
        let state = active.state;
        let demand = store_replacement_unit_demands::<A, M>(active, closing, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if !plugin_turn_admitted(demand, grant)? {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        if state == State::Initializing {
            let step = active.drive_initializer(cx, grant);
            return step;
        }
        if matches!(state, State::AwaitingMembers | State::OpeningMembers | State::ClosingRejectedMember | State::ValidatingClosure | State::PreparingCandidateViews | State::CandidateReady) && active.cancelled() {
            active.refuse(ArtifactStoreReplacementRefusal::Cancelled);
            return Ok(PluginLifecycleStep::Progress(item));
        }
        match state {
            State::AwaitingMembers => {
                if active.mounted_genesis.is_some() && active.candidate_children.is_none() {
                    let children=ChildMemberRegistry::<M>::new();
                    let admitted=children.allocation_admitted;
                    *active.candidate_children=Some(children);
                    if !admitted {
                        active.refuse(ArtifactStoreReplacementRefusal::InitializerAdmissionRejected);
                    }
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{
                        copied_items:1,copied_bytes:demand.copy_bytes,
                        retained_capacity_bytes:if admitted{demand.capacity_bytes}else{0},..Default::default()
                    }));
                }
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
                self.ingest.complete_store_replacement_genesis(&mut *self.stores, operation_id)?;
                Ok(PluginLifecycleStep::Progress(item))
            }
            State::OpeningMembers => cx.with_replacement_current(active.replacement_identity(), |owner| active.drive_member_open(owner, grant)),
            State::ClosingRejectedMember => {
                active.state = State::RetiringRejectedMembers;
                Ok(PluginLifecycleStep::Progress(item))
            }
            State::ValidatingClosure => { let current = active.replacement_identity(); active.drive_closure(grant, current).map_err(Into::into) }
            State::PreparingCandidateViews => active.drive_candidate_views(grant).map_err(Into::into),
            State::RetiringRejectedMembers | State::RetiringCommittedMembers => {
                if active.retained_store.is_some() && active.retained_disposer.is_none() {
                    *active.retained_disposer = Some(A::build_document_store_disposer().ok_or_else(|| plugin_sdk_fault("app did not supply the required rejected-candidate disposer"))?);
                }
                cx.with_replacement_current(active.replacement_identity(), |owner| {
                    active.drive_rejected_members(&self.child_composition.child_content_root, owner, grant)
                })
            }
            State::CandidateReady => self.commit_store_replacement_candidate(operation_id, closing,cx).map_err(Into::into),
            State::RetiringCommittedStore | State::RetiringRejectedCandidate => {
                let Some((step, performed)) = cx.with_original_effect(grant, |_, effect| active.drive_retained_store(effect))? else { return Ok(PluginLifecycleStep::Progress(Default::default())) };
                cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
                step.map_err(Into::into)
            }
            State::Complete | State::Initializing => {
                if !closing {
                    return Ok(PluginLifecycleStep::Complete(empty));
                }
                if !active.terminal_is_empty() {
                    return Err(maintenance_fault("artifact-store.replacement-terminal-not-empty", "store replacement reported terminal before every exact owner was empty").into());
                }
                let active = self.ingest.store_replacement_jobs.remove(operation_id).ok_or_else(|| maintenance_fault("artifact-store.replacement-owner", "terminal store replacement changed before exact close removal"))?;
                drop(active);
                Ok(PluginLifecycleStep::Progress(item))
            }
        }
    }

    /// 🔐️ Publishes a ready replacement candidate or retains the exact guard evidence that refused it; the commit is one non-suspending boundary.
    fn commit_store_replacement_candidate(&mut self, operation_id: u64, closing: bool,cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure> {
        use ActiveArtifactStoreReplacementState as State;
        let item = item();
        let replacement_content_retirements = ArtifactFixedRegistry::new();
        let live_generation = semio_framework_job::Generation(self.stores.store.generation_now());
        let next_content_generation = self.child_composition.child_content_generation.checked_add(1);
        let active = self.ingest.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| maintenance_fault("artifact-store.replacement-owner", "ready store candidate changed before exact commit validation"))?;
        let mounted_permit=if let Some((operation,base_revision))=active.mounted_genesis {
            if base_revision!=self.stores.store.content_revision_now() {
                active.refuse(ArtifactStoreReplacementRefusal::PublicationAuthorityRejected);
                return Ok(PluginLifecycleStep::Progress(item));
            }
            let lease=self.tools.mounted_genesis_lease(operation,active.operation,active.generation);
            let Some(lease)=lease.filter(|lease|!lease.publication_is_cancelled()&&!lease.publication_claim().is_finished()) else {
                active.refuse(ArtifactStoreReplacementRefusal::Cancelled);
                return Ok(PluginLifecycleStep::Progress(item));
            };
            let Some(permit)=lease.try_claim_publication() else {return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default()))};
            Some(permit)
        }else{None};
        let disposer = A::build_document_store_disposer().ok_or_else(|| maintenance_fault("artifact-store.replacement-disposer-missing", "app did not supply the required displaced-store disposer"))?;
        let complete_candidate = active.candidate_children.as_ref().is_some_and(|children| active.candidate_source_generation == children.len() as u64)
            && active.candidate_content.is_some()
            && active.candidate_composition.is_some()
            && active.view_member_ordinal == active.candidate_children.as_ref().map_or(usize::MAX, ChildMemberRegistry::len);
        if closing || active.cancelled() || active.generation != live_generation || active.base_child_content_generation != self.child_composition.child_content_generation || !complete_candidate || !replacement_content_retirements.allocation_admitted || next_content_generation.is_none() {
            let guard = ArtifactStoreReplacementPublicationGuard {
                closing,
                cancelled: active.cancelled(),
                parent_generation: active.generation.0,
                live_generation: live_generation.0,
                base_child_generation: active.base_child_content_generation,
                child_generation: self.child_composition.child_content_generation,
                complete_candidate,
                retirement_admitted: replacement_content_retirements.allocation_admitted,
                next_generation: next_content_generation.is_some(),
            };
            *active.retained_disposer = Some(disposer);
            active.refuse(ArtifactStoreReplacementRefusal::PublicationGuard(guard));
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if A::validate_document_store_publication(&active.publication_token, semio_plugin_store_lifecycle::store_lifecycle::DocumentStoreLivePublication::new(semio_framework_job::OperationId(operation_id), live_generation, self.child_composition.child_content_generation)).is_err() {
            *active.retained_disposer = Some(disposer);
            active.refuse(ArtifactStoreReplacementRefusal::PublicationAuthorityRejected);
            return Ok(PluginLifecycleStep::Progress(item));
        }
        let before=cx.retained_progress();
        if mounted_permit.is_none() && !self.tools.admit_document_window_reset(&mut *self.pending, cx)?{
            return Ok(PluginLifecycleStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
        }
        let mut window_reset = Some(self.shared().prepare_document_window_reset()?);
        let active = self.ingest.store_replacement_jobs.get_mut(operation_id).expect("validated replacement remains exclusively owned across non-suspending window preparation");
        let candidate = active.retained_store.take().expect("complete replacement retains its exact parent candidate");
        let candidate_children = active.candidate_children.take().expect("complete replacement retains its exact member registry");
        let candidate_content = active.candidate_content.take().expect("complete replacement retains its exact content view");
        let candidate_composition = active.candidate_composition.take().expect("complete replacement retains its exact coordinator");
        let local_actor = self.stores.store.local_actor_id().clone();
        let displaced = match publish_boxed_document_store_candidate_if_authoritative(&mut self.stores.store, candidate, || Ok(())) {
            Ok(displaced) => displaced,
            Err(_) => unreachable!("prevalidated publication cannot reject inside its non-suspending commit boundary"),
        };
        assert_eq!(self.stores.store.local_actor_id(), &local_actor, "document replacement retains its opened actor");
        self.stores.store.enable_convergence_early_exit();
        self.stores.store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));
        self.stores.store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));
        let displaced_children = std::mem::replace(&mut self.child_composition.children, candidate_children);
        let displaced_content = std::mem::replace(&mut *self.child_composition.child_content_root, candidate_content);
        let displaced_content_retirements = std::mem::replace(&mut self.child_composition.child_content_retirements, replacement_content_retirements);
        let displaced_composition = std::mem::replace(&mut self.child_composition.composition, candidate_composition);
        let displaced_pins = std::mem::take(&mut self.child_composition.pending_child_pins);
        displaced.revoke_snapshot_read_authority();
        *active.retained_store = Some(displaced);
        *active.candidate_children = Some(displaced_children);
        *active.candidate_content = Some(displaced_content);
        if displaced_content_retirements.empty_backing_byte_demand()!=Some(0) {
            *active.displaced_content_retirements=Some(displaced_content_retirements);
        }
        *active.candidate_composition = Some(displaced_composition);
        if !displaced_pins.is_empty() {
            *active.displaced_pins = Some(CompositionPinsRetirement::new(displaced_pins));
        }
        *active.retained_disposer = Some(disposer);
        active.committed = true;
        active.committed_frontier = Some((self.stores.store.generation_now(), self.stores.store.content_revision_now()));
        active.state = State::RetiringCommittedMembers;
        self.child_composition.child_content_generation = next_content_generation.expect("candidate generation exhaustion was rejected before commit");
        if mounted_permit.is_some() {
            self.pending.install_document_window_reset(&mut *self.stores,&mut *self.history,&mut window_reset)?;
        } else {
            self.pending.commit_document_window_reset(&mut *self.stores, &mut *self.history, &mut window_reset, self.tools.tool_cancellations.scope_generation(), cx)?;
        }
        self.history.invalidate_cache();
        self.history.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(false);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { retained_capacity_bytes: store_replacement_commit_capacity(), ..item }))
    }

    /// 🔁️ One granted turn of the next archive load; a closing app cancels every load and drops it once terminal-empty.
    pub fn drive_document_archive_load_retirements(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,closing:bool)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let grant=semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(grant,cx.retained_grant());
        let demand=match self.shared().document_archive_target(closing) {
            Some((_,operation))=>self.ingest.document_archive_load_demands(self.ingest.document_archive_loads.get(operation).ok_or_else(||maintenance_fault("archive-load.owner","archive load original source changed before its quote"))?,closing,grant.maximum_copy_bytes)?,
            None=>self.ingest.document_archive_loads.empty_backing_byte_demand().map_or(Default::default(),|bytes|RetirementDemand{release_bytes:bytes,depth:1,..Default::default()})
        };
        if self.shared().document_archive_target(closing).is_none()&&(!closing||self.ingest.document_archive_loads.empty_backing_byte_demand().is_none_or(|bytes|bytes==0)){return Ok(PluginLifecycleStep::Complete(Default::default()))}
        if closing&&self.shared().document_archive_target(closing).is_none()&&self.ingest.document_archive_loads.empty_backing_byte_demand().is_some_and(|bytes|bytes!=0){let grant=semio_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,grant);if cx.retirement_should_yield()||!semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(PluginLifecycleStep::Progress(Default::default()))}let step=match self.ingest.document_archive_loads.close_empty_backing_step(grant){Ok(step)=>step,Err(error)=>return Err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(error).debit_original(cx))};let progress=step.progress();cx.consume_fuel(u64::from(progress!=Default::default()));cx.consume_retained(progress).map_err(|error|semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(error.with_retained_progress(progress)))?;return Ok(PluginLifecycleStep::Progress(progress))}
        semio_plugin_document_media::original_composed_lifecycle_step(cx, grant, demand, |same, grant| {
            let step = self.drive_document_archive_original(same, grant, closing)?;
            let terminal=self.shared().document_archive_target(closing).is_none()&&(!closing||self.ingest.document_archive_loads.empty_backing_byte_demand().is_none_or(|bytes|bytes==0));
            Ok((step, terminal))
        })
    }

    /// ⏳️ Ends every live whole-document load as a named `plugin.document-load.failed` fault once the ladder has read only host, worker or replacement waits for `waited_us`: a hung owner no longer waits silently. The fault names the phase, stage and member passes of each load; the load's own terminal turn then retires it.
    pub fn expire_stalled_document_archive_loads(&mut self, waited_us: u64) {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let Some(operation) = self.ingest.document_archive_loads.entry(index).map(|(operation, _)| *operation) else { continue };
            let Some(mut load) = self.ingest.document_archive_loads.remove(operation) else { continue };
            if !load.terminal() && load.terminal_target.is_none() {
                let fault = document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive load made no progress for {waited_us}us (ceiling {DOCUMENT_ARCHIVE_WAIT_CEILING_US}us): its host, worker or replacement owner never answered; phase={:?} stage={} members_done={} state={:?}", load.phase, load.stage, load.members_done, load.state));
                load.request_fault(&fault);
            }
            self.ingest.document_archive_loads.insert_admitted(operation, load);
        }
    }

    fn drive_document_archive_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,closing:bool)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let empty = RetainedCloneProgress::default();
        let Some((index, operation)) = self.shared().document_archive_target(closing) else {
            return Ok(PluginLifecycleStep::Complete(empty));
        };
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        let cursor = if closing { &mut self.close.close_document_archive_cursor } else { &mut self.maintenance.maintenance_document_archive_cursor };
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let mut active = self.ingest.document_archive_loads.remove(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive retirement authority changed during one bounded step"))?;
        if closing && !active.terminal() {
            active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
        }
        let funded = self.ingest.document_archive_load_demands(&active, closing, grant.maximum_copy_bytes).map_err(plugin_retirement_fault).and_then(|demand| plugin_turn_admitted(demand, grant));
        if !matches!(funded, Ok(true)) {
            self.ingest.document_archive_loads.insert_admitted(operation, active);
            return funded.map(|_| PluginLifecycleStep::Progress(empty)).map_err(Into::into);
        }
        if closing && active.terminal() && active.inputs_are_empty() {
            let demand = active.terminal_removal_demands();
            if demand.release_bytes != 0 {
                drop(std::mem::take(&mut active.fault));
            }
            drop(active);
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress {
                copied_bytes: demand.copy_bytes,
                released_bytes: demand.release_bytes,
                ..item()
            }));
        }
        let step=self.ingest.advance_document_archive_load(&mut *self.stores, &mut *self.child_composition, &mut active, grant, cx);
        let step=match step{
            Err(failure) if active.terminal_target.is_none()&&failure.receipt_refusal.is_none()&&failure.context_refusal.is_none()&&failure.work_refusal.is_none() => match &failure.primary{
                semio_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault)=>{active.request_fault(fault);Ok(PluginLifecycleStep::Progress(failure.effects))}
                _=>Err(failure),
            },
            step=>step,
        };
        self.ingest.document_archive_loads.insert_admitted(operation, active);
        step
    }
}

/// 🧮️ One archive-load step may post many owned items in one receipt (the history decoder admission alone posts 6), so it never starts in a turn whose remaining item loan is smaller.
#[doc(hidden)]
pub const DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS:usize=16;

/// ⏳️ How long a whole-document load may wait on its host, worker or replacement owner while nothing it reports moves (phase, count, fold, stage, member passes, decoded records). A hung host then ends the load as a named `plugin.document-load.failed` fault carrying the phase instead of waiting silently. The load keeps one wait witness, so real progress of any kind restarts the wait.
#[doc(hidden)]
pub const DOCUMENT_ARCHIVE_WAIT_CEILING_US:u64=120_000_000;

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    /// 🌡️ Pressure beats fairness: a displaced-owner or child retirement queue a quarter full is drained out of turn.
    pub fn maintenance_ladder_under_pressure(&self) -> bool {
        self.history.command_prune_requested() || self.history.pending_command_prune.is_some() || !self.history.command_log.pruned_terminal_is_empty() || self.tools.next_closing_private_child_group(&*self.child_composition).is_some() || self.stores.store.maintenance_retirements_under_pressure() || self.stores.config_lane_under_pressure() || self.child_composition.child_retirements_under_pressure()
    }

    /// 🪹️ Every live retirement frontier and its retained directory backing is empty.
    pub fn maintenance_ladder_terminal_is_empty(&self) -> bool {
        self.maintenance_ladder_settled(false)
    }

    /// 🫧️ Quiescence of every maintenance frontier except a live transient press preview: a preview is a deliberate, committed-nowhere overlay that only its own release or cancel retires, so the tool-overlay unit that merely awaits the already-returned foreground source never blocks the settle of a tick.
    pub fn maintenance_ladder_quiescent_under_live_preview(&self) -> bool {
        self.maintenance_ladder_settled(true)
    }

    fn maintenance_ladder_settled(&self, live_preview_allowed: bool) -> bool {
        let overlay = self.render_ref();
        self.stores.window_config_store.direct_ingress_quiescent()
            && !(overlay.tool_overlay_retirement_pending() && !(live_preview_allowed && overlay.tool_overlay_waits_for_foreground()))
            && self.tool_run_ref().tool_run_retirement_demand(0).is_ok_and(|demand| demand == RetirementDemand::default())
            && self.time_travel_ref().time_travel_retirement_demands(0).is_ok_and(|demand| demand == RetirementDemand::default())
            && self.history.pending_command_prune.is_none()
            && !self.history.command_prune_requested()
            && self.history.command_log.pruned_terminal_is_empty()
            && self.child_composition.private_child_groups.is_empty()
            && self.tools.tool_operations.is_empty()
            && self.tools.original_emission_admission.is_none()
            && self.tools.original_emission_receipt.is_none()
            && self.tools.original_emission_result.is_none()
            && self.tools.original_emission_construction.as_ref().is_none_or(|admission| admission.terminal_is_empty())
            && self.tools.tool_cancellations.active_operation_count() == 0
            && self.tools.latest_wins_commands.is_empty()
            && self.tools.latest_wins_keys.map.as_ref().is_some_and(|map| map.is_empty())
            && self.tools.latest_wins_keys.can_begin()
            && self.tools.agent_lane_preview.is_none()
            && self.media.media_submission_refusal_terminal_is_empty()
            && self.ingress.action_ingress_sources_idle()
            && self.media.media_closures.is_empty()
            && self.media.segmented_closures.is_empty()
            && self.media.snapshot_retirements.is_empty()
            && self.child_composition.child_content_retirements.is_empty()
            && self.child_composition.child_member_retirements.is_empty()
            && self.roster.peer_roster_publications.is_empty()
            && self.roster.peer_presence_retirements.is_empty()
            && self.roster.presence_peer_retirements.is_empty()
            && self.ingest.envelope_ingress.is_empty()
            && self.ingest.envelope_decode_jobs.is_empty()
            && self.ingest.store_replacement_jobs.is_empty()
            && self.ingest.document_archive_loads.is_empty()
            && self.ingest.envelope_field_decoder_retirements.is_empty()
            && self.ingest.envelope_field_decoders.get().is_none_or(|owner|owner.next_returned_ticket().is_none())
            && self.ingest.envelope_completed_record_retirements.is_empty()
            && self.ingest.envelope_completed_records.get().is_none_or(|owner|owner.terminal_is_empty())
            && !self.stores.store.has_pending_maintenance_retirements()
            && !self.stores.config_store.has_pending_maintenance_retirements()
            && !self.stores.draft_store.has_pending_maintenance_retirements()
            && !self.stores.interaction_store.has_pending_maintenance_retirements()
            && self.stores.window_config_store.maintenance_retirements_terminal_is_empty()
            && self.stores.window_transient_store.maintenance_terminal_is_empty()
            && self.stores.retired_window_transient_stores.is_empty()
            && self.stores.presence_store.local_read_maintenance_is_idle()
            && self.ingest.snapshot_read_returns_terminal_is_empty(&*self.stores, &*self.interaction)
            && !semio_framework_job_worker::worker_job_retirements_are_parked()
            && self.tools.live_runtime_instance_id.is_none_or(|instance_id| {
                if self.ingest.document_blocks_pending_effects() {
                    A::mounted_jobs_terminal_is_empty(instance_id)
                } else {
                    A::mounted_job_maintenance_phase(instance_id).is_none()
                }
            })
    }

    pub fn maintenance_unit_at(&self, position: usize, cursor: u8) -> Option<MaintenanceUnit> {
        let Some(position) = position.checked_sub(1) else { return self.ingest.snapshot_read_returns_under_pressure(&*self.stores).then_some(MaintenanceUnit::Pressure(MAINTENANCE_SNAPSHOT_RETURN_STAGE)) };
        match position {
            0 => (self.history.command_prune_requested() || self.history.pending_command_prune.is_some() || !self.history.command_log.pruned_terminal_is_empty()).then_some(MaintenanceUnit::CommandPrune),
            1 => self.tools.next_closing_private_child_group(&*self.child_composition).map(MaintenanceUnit::PrivateChildGroup),
            2 => (!self.stores.window_config_store.direct_ingress_quiescent()).then_some(MaintenanceUnit::DirectIngress),
            3 => self.next_granted_mounted_retirement().is_some().then_some(MaintenanceUnit::GrantedMounted),
            4 => self.render_ref().tool_overlay_retirement_pending().then_some(MaintenanceUnit::ToolOverlay),
            5 => Some(MaintenanceUnit::ToolRuns),
            6 => Some(MaintenanceUnit::TimeTravel),
            7 => self.stores.store.maintenance_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_DOCUMENT_DISPLACED_STAGE)),
            8 => self.stores.config_lane_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE)),
            9 => self.child_composition.child_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CHILD_ROOT_STAGE)),
            10 => self.child_composition.child_retirements_under_pressure().then_some(MaintenanceUnit::Pressure(MAINTENANCE_CHILD_MEMBER_STAGE)),
            11 => self.media.media_submission_refusal_retained().then_some(MaintenanceUnit::MediaRefusal),
            12=>self.tools.agent_lane_preview.as_ref().filter(|state|state.closing).map(|_|MaintenanceUnit::AgentPreview),
            13=>(!self.ingress.action_ingress_sources_idle()).then_some(MaintenanceUnit::ActionIngress),
            _ => Some(MaintenanceUnit::Stage(((usize::from(cursor) + position - ROTATION_START) % usize::from(MAINTENANCE_STAGES)) as u8)),
        }
    }

    pub fn maintenance_unit_demand(&self, unit: MaintenanceUnit, body: usize,maximum_work_units:u64) -> Result<Option<RetirementDemand>, ValueError> {
        let present = |demand: RetirementDemand| (demand != RetirementDemand::default()).then_some(demand);
        Ok(match unit {
            MaintenanceUnit::AgentPreview=>self.tools.agent_lane_preview.as_ref().map(|state|state.retirement_demands(body,self.tools.tool_cancellations.handle())).transpose()?,
            MaintenanceUnit::MediaRefusal => Some(self.media.media_submission_refusal_demands(&*self.stores,&*self.tools, body)?),
            MaintenanceUnit::ActionIngress=>Some(self.ingress.action_ingress_sources_demands(body, false)?),
            MaintenanceUnit::CommandPrune => Some(self.history.command_prune_demands(false, body)?),
            MaintenanceUnit::PrivateChildGroup(operation) => Some(self.child_composition.private_child_group_operation_close_demands(operation, body)?),
            MaintenanceUnit::DirectIngress => Some(self.stores.window_config_store.direct_ingress_demands(body)?),
            MaintenanceUnit::GrantedMounted => Some(self.granted_mounted_retirement_demands(body)?),
            MaintenanceUnit::ToolOverlay => Some(self.render_ref().tool_overlay_retirement_demand(body,maximum_work_units)?),
            MaintenanceUnit::ToolRuns => present(self.tool_run_ref().tool_run_retirement_demand(body)?),
            MaintenanceUnit::TimeTravel => present(self.time_travel_ref().time_travel_retirement_demands(body)?),
            MaintenanceUnit::Pressure(stage) | MaintenanceUnit::Stage(stage) => self.maintenance_stage_demand(stage, body)?,
        })
    }

    /// 📏️ Quotes the first unit with work, in the exact order [`Self::maintenance_ladder_step`] visits them.
    pub fn maintenance_ladder_demands(&self, body: usize,maximum_work_units:u64) -> Result<RetirementDemand, ValueError> {
        for position in 0..MAINTENANCE_POSITIONS {
            let Some(unit) = self.maintenance_unit_at(position, self.maintenance.maintenance_stage) else { continue };
            if let Some(demand) = self.maintenance_unit_demand(unit, body,maximum_work_units)?.filter(|demand| *demand != RetirementDemand::default()) {
                return Ok(demand);
            }
        }
        Ok(RetirementDemand::default())
    }

    fn maintenance_tool_selection(&self) -> MaintenanceTool {
        let mut awaits_publication = self.tools.original_emission_admission.is_some()
            || self.tools.original_emission_receipt.is_some()
            || self.tools.original_emission_result.is_some()
            || self.tools.original_emission_construction.as_ref().is_some_and(|owner| !owner.terminal_is_empty());
        let mut awaits_ack = false;
        let mut retiring = false;
        for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let index = (self.maintenance.maintenance_tool_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            let Some((operation_id, operation)) = self.tools.tool_operations.entry(index).map(|entry| (entry.0, &entry.1)) else { continue };
            retiring |= operation.granted_retirement_ready() && operation.granted_retirement_pending();
            match operation.stage {
                MountedTypedCommandFullOperationStage::Worker if operation.producer_build_pending() => awaits_publication = true,
                MountedTypedCommandFullOperationStage::Worker => return MaintenanceTool::Worker(index, operation_id),
                MountedTypedCommandFullOperationStage::AwaitingAck if operation.result_page_presented => awaits_ack = true,
                MountedTypedCommandFullOperationStage::Publishing | MountedTypedCommandFullOperationStage::AwaitingAck => awaits_publication = true,
                MountedTypedCommandFullOperationStage::Retiring => retiring = true,
            }
        }
        if retiring { return MaintenanceTool::Idle; }
        if awaits_publication { return MaintenanceTool::AwaitingPublication; }
        if awaits_ack { return MaintenanceTool::Awaiting; }
        MaintenanceTool::Idle
    }

    fn maintenance_stage_demand(&self, stage: u8, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        Ok(match stage {
            0 => match self.maintenance_tool_selection() {
                MaintenanceTool::Worker(_, operation_id) => self.tools.tool_operations.get(operation_id).map(|operation| operation.worker_step_demands(body)).transpose()?,
                MaintenanceTool::Awaiting | MaintenanceTool::AwaitingPublication => Some(RetirementDemand::default()),
                MaintenanceTool::Idle => None,
            },
            1 => {
                let Some((_, operation_id)) = self.media.media_closures.next_id_from(self.maintenance.maintenance_media_cursor) else { return Ok(None) };
                if self.media.media_closures.get(operation_id).is_some_and(|active|active.download_pending){return Ok(Some(RetirementDemand::default()))}
                self.media.media_closures.get(operation_id).map(|active| active.retirement_demands(body,&self.tools.tool_cancellations)).transpose()?
            }
            2=>{let Some((_,operation_id))=self.media.segmented_closures.next_id_from(self.maintenance.maintenance_segment_cursor)else{return Ok(None)};Some({let original=self.media.segmented_closures.get(operation_id).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original segmented close owner is missing"))?;if original.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<semio_plugin_document_media::media_export::MediaDownloadRetirement>(),depth:1,..Default::default()}}else{original.retirement_demands(body)?}})},
            3 => {
                let Some((_, operation_id)) = self.media.snapshot_retirements.next_id_from(self.maintenance.maintenance_snapshot_cursor) else { return Ok(None) };
                if self.media.media_exports.get(operation_id).is_some() || self.media.media_closures.get(operation_id).is_some() || self.media.segmented_downloads.get(operation_id).is_some() || self.media.segmented_closures.get(operation_id).is_some() || self.media.media_download_preparation.as_ref().is_some_and(|pending|pending.operation_id==operation_id) {
                    return Ok(Some(RetirementDemand::default()));
                }
                self.media.snapshot_retirements.get(operation_id).map(|retention| retention.retirement_demands(body)).transpose()?
            }
            4 => (!self.child_composition.child_content_retirements.is_empty()).then(|| self.child_root_retirement_demands(body)).transpose()?,
            5 => {
                let Some((_, generation)) = self.roster.peer_presence_retirements.next_id_from(self.maintenance.maintenance_peer_presence_cursor) else { return Ok(None) };
                self.roster.peer_presence_retirements.get(generation).map(|retirement| retirement.retirement_demands(body)).transpose()?
            }
            6 => {
                let Some((_, generation)) = self.roster.presence_peer_retirements.next_id_from(self.maintenance.maintenance_presence_peer_cursor) else { return Ok(None) };
                self.roster.presence_peer_retirements.get(generation).map(|retirement| retirement.retirement_demands(body)).transpose()?
            }
            7 => {
                let Some(generation) = self.roster.maintenance_peer_roster_generation() else { return Ok(None) };
                let Some(publication) = self.roster.peer_roster_publications.get(generation) else { return Ok(None) };
                Some(if publication.is_faulted() {
                    publication.close_demands(body)?
                } else if publication.stage == PeerRosterPublicationStage::Ready {
                    RetirementDemand { capacity_bytes: child_content_arc_bytes::<PeerPresenceRoot>(), depth: 1, ..Default::default() }
                } else {
                    publication.step_demands()?
                })
            }
            8 => (!self.ingest.snapshot_read_returns_terminal_is_empty(&*self.stores, &*self.interaction)).then(|| self.ingest.snapshot_read_returns_demands(&*self.stores, &*self.interaction, body)).transpose()?,
            MAINTENANCE_DOCUMENT_DISPLACED_STAGE => self.stores.store.has_pending_maintenance_retirements().then(|| self.stores.store.maintenance_retirements_demands(body)).transpose()?,
            10 => self.envelope_ingress_target(false).map(|_| self.envelope_ingress_demands(false, body)).transpose()?,
            11 => (!self.ingest.envelope_decode_jobs.is_empty()).then(|| self.envelope_decode_jobs_demands(false, body)).transpose()?,
            12 => (!self.ingest.envelope_field_decoder_retirements.is_empty() || self.ingest.envelope_field_decoders.get().is_some_and(|owner|owner.next_returned_ticket().is_some())).then(|| self.envelope_field_decoder_returns_demands(false, body)).transpose()?,
            13 => (!self.ingest.envelope_completed_record_retirements.is_empty() || !self.ingest.envelope_completed_records.get().is_none_or(|owner|owner.terminal_is_empty())).then(|| self.envelope_completed_record_returns_demands(false, body)).transpose()?,
            14 => self.store_replacement_target(false).map(|_| self.store_replacement_demands(false, body)).transpose()?,
            15 => match self.tools.live_runtime_instance_id {
                Some(instance_id) if self.ingest.document_blocks_pending_effects() => {
                    (!A::mounted_jobs_terminal_is_empty(instance_id)).then(|| A::mounted_job_close_demands(instance_id, body)).transpose()?
                }
                Some(instance_id) if A::mounted_job_maintenance_phase(instance_id) == Some(MountedOwnerPhaseV1::Maintenance) => Some(A::mounted_job_maintenance_demands(instance_id, body)?),
                _ => None,
            },
            16 => (!self.tools.instance_operation_owner.terminal_is_empty().unwrap_or(false)).then(|| self.tools.instance_operation_owner.retirement_demands(body)).transpose()?,
            17 => (!(self.tools.latest_wins_keys.map.as_ref().is_some_and(|map| map.is_empty()) && self.tools.latest_wins_keys.can_begin())).then(|| RetirementDemand { depth: 1, ..Default::default() }),
            26 => self.pending.pending_document_window_admission.as_ref().filter(|admission|admission.committed).map(|_|self.pending.document_window_admission_demands()),
            18 if !self.tools.tool_cancellations.has_cleanup_work() => None,
            18 if self.tools.tool_cancellations.live_cleanup_awaits_custody() => Some(RetirementDemand::default()),
            18 => {let mut demand=self.tools.tool_cancellations.cancellation_cleanup_demands(self.maintenance.maintenance_cancellation_cursor,body,true)?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original maintenance cancellation cursor quote overflow"))?;Some(demand)},
            19 => (!self.stores.presence_store.local_read_maintenance_is_idle()).then(|| self.stores.presence_store.maintenance_local_reads_demands(body)).transpose()?,
            MAINTENANCE_CHILD_MEMBER_STAGE => (!self.child_composition.child_member_retirements.is_empty()).then(|| self.child_member_retirement_demands(body)).transpose()?,
            21 => (!self.stores.retired_window_transient_stores.is_empty()).then(|| self.document_windows_retirement_demands(body, false)).transpose()?,
            22 => Some(self.stores.window_transient_store.maintenance_demands(body)?).filter(|demand| *demand != RetirementDemand::default()),
            23 => semio_framework_job_worker::worker_job_retirements_are_parked().then(item_demand),
            24 => self.document_archive_target(false).map(|_| self.document_archive_demands(false, body)).transpose()?,
            MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE => {
                if self.stores.config_store.has_pending_maintenance_retirements() {
                    Some(self.stores.config_store.maintenance_retirements_demands(body)?)
                } else if self.stores.draft_store.has_pending_maintenance_retirements() {
                    Some(self.stores.draft_store.maintenance_retirements_demands(body)?)
                } else if self.stores.interaction_store.has_pending_maintenance_retirements() {
                    Some(self.stores.interaction_store.maintenance_retirements_demands(body)?)
                } else if !self.stores.window_config_store.maintenance_retirements_terminal_is_empty() {
                    Some(self.stores.window_config_store.maintenance_retirements_demands(body)?)
                } else {
                    None
                }
            }
            26 => unreachable!("original window admission source return requires its actual caller"),
            _ => unreachable!("fixed maintenance stage"),
        })
    }

    fn envelope_ingress_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close.close_envelope_ingress_cursor } else { self.maintenance.maintenance_envelope_ingress_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.ingest.envelope_ingress.entry(index).filter(|entry| closing || entry.1.closing).map(|entry| (index, entry.0))
        })
    }

    /// 📃️ Quotes the next envelope-ingress page release: the page payload, then the slot backing's whole physical release.
    pub fn envelope_ingress_demands(&self, closing: bool, _body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation_id)) = self.envelope_ingress_target(closing) else { return Ok(RetirementDemand::default()) };
        Ok(self.ingest.envelope_ingress.get(operation_id).map_or_else(RetirementDemand::default, ActiveArtifactEnvelopeIngress::retirement_demands))
    }

    fn store_replacement_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close.close_store_replacement_cursor } else { self.maintenance.maintenance_store_replacement_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.ingest.store_replacement_jobs
                .entry(index)
                .filter(|(_, replacement)| {
                    use ActiveArtifactStoreReplacementState::{AwaitingMembers, Complete};
                    if closing { return true; }
                    if replacement.state == Complete { return false; }
                    if replacement.state != AwaitingMembers || replacement.cancelled() { return true; }
                    replacement.member_ingress.is_none() && !(0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|slot| {
                        self.ingest.document_archive_loads.entry(slot).is_some_and(|(_, archive)| {
                            archive.replacement.is_some_and(|handle| {
                                handle.operation == replacement.operation && handle.generation == replacement.generation
                            })
                        })
                    })
                })
                .map(|entry| (index, entry.0))
        })
    }

    /// 🏪️ Quotes the next store-replacement turn selected by the target replacement's own state.
    pub fn store_replacement_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation_id)) = self.store_replacement_target(closing) else { return Ok(RetirementDemand::default()) };
        self.ingest.store_replacement_jobs.get(operation_id).map_or(Ok(RetirementDemand::default()),|active|{
            let mounted_cancelled=active.mounted_genesis.is_some_and(|(operation,_)|
                self.tools.mounted_genesis_lease(operation,active.operation,active.generation).is_none_or(|lease|lease.publication_is_cancelled()||lease.publication_claim().is_finished()));
            store_replacement_unit_demands::<A,M>(active,closing||mounted_cancelled,body)
        })
    }

    /// 🚞️ Arms another turn only while the original stock can drive the same archive lane as maintenance.
    pub fn document_archive_maintenance_is_runnable(&self, stock: RetainedCloneGrant) -> Result<bool, ValueError> {
        if stock.maximum_items < DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS || self.document_archive_target(false).is_none() {
            return Ok(false);
        }
        let demand = self.document_archive_demands(false, stock.maximum_copy_bytes)?;
        Ok(semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand, stock))
    }

    /// 🔭️ Names every live archive load phase and store replacement state with the next quoted demand of each against the maintenance grant, for the live cleanup stall report.
    pub fn document_load_diagnostic(&self, grant: RetainedCloneGrant) -> String {
        let mut lines = Vec::new();
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            if let Some((operation, load)) = self.ingest.document_archive_loads.entry(index) {
                let demand = self.ingest.document_archive_load_demands(load, false, grant.maximum_copy_bytes);
                let funded = demand.as_ref().ok().map(|demand| semio_plugin_ownership::bounded_ownership::plugin_grant_funds(*demand, grant));
                lines.push(format!("archive op={operation} state={:?} phase={:?} target={:?} merge={} terminal={} awaits_merge={} replacement={:?} members={} demand={demand:?} funded={funded:?}", load.state, load.phase, load.terminal_target, load.merge, load.terminal(), load.awaits_merge(), load.replacement.map(|handle| handle.operation.0), load.archive.as_ref().map_or(0, |archive| archive.members.len())));
            }
            if let Some((operation, replacement)) = self.ingest.store_replacement_jobs.entry(index) {
                let demand = store_replacement_unit_demands::<A, M>(replacement, false, grant.maximum_copy_bytes);
                let funded = demand.as_ref().ok().map(|demand| semio_plugin_ownership::bounded_ownership::plugin_grant_funds(*demand, grant));
                let worker = replacement.session.as_ref().map(|session| session.poll());
                lines.push(format!("replacement op={operation} state={:?} refusal={:?} cancelled={} committed={} mounted_genesis={} member_ingress={} worker={worker:?} outcome_pending={} resume_pending={} terminal_target={:?} terminal_empty={} store_close={} roots=[{}] demand={demand:?} funded={funded:?}", replacement.state, replacement.refusal, replacement.cancelled(), replacement.committed, replacement.mounted_genesis.is_some(), replacement.member_ingress.is_some(), replacement.outcome_pending, replacement.resume_pending, replacement.terminal_target, replacement.terminal_is_empty(), replacement.retained_store.as_ref().map_or_else(|| "none".to_string(), |store| format!("{} leases_empty={}", store.close_owned_phase_witness(), store.snapshot_read_leases_terminal_is_empty())), replacement.retained_store.as_ref().map_or_else(|| "none".to_string(), |store| store.close_owned_roots_witness())));
            }
        }
        format!("grant={grant:?} archive_target={:?} replacement_target={:?} waits_for_host={} [{}]", self.document_archive_target(false), self.store_replacement_target(false), self.document_archive_waits_for_host(), lines.join("; "))
    }

    pub fn document_archive_waits_for_host(&self) -> bool {
        !self.ingest.document_archive_loads.is_empty() && self.document_archive_target(false).is_none()
    }

    fn document_archive_target(&self, closing: bool) -> Option<(usize, u64)> {
        let cursor = if closing { self.close.close_document_archive_cursor } else { self.maintenance.maintenance_document_archive_cursor };
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.ingest.document_archive_loads.entry(index).filter(|entry| closing || (!entry.1.terminal() && !entry.1.awaits_merge())).map(|entry| (index, entry.0))
        })
    }

    /// 🌈️ Quotes the next archive-load turn of the target load, in the phase its state machine is about to run.
    pub fn document_archive_demands(&self, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_, operation)) = self.document_archive_target(closing) else {
            let backing = if closing { self.ingest.document_archive_loads.empty_backing_byte_demand().unwrap_or(0) } else { 0 };
            return Ok(if backing == 0 { RetirementDemand::default() } else { RetirementDemand { release_bytes: backing, depth: 1, ..Default::default() } });
        };
        self.ingest.document_archive_loads.get(operation).map_or(Ok(RetirementDemand::default()), |active| self.ingest.document_archive_load_demands(active, closing, body))
    }
}

pub(crate) fn store_replacement_commit_capacity() -> usize {
    ARTIFACT_LIVE_OUTPUT_SLOTS * size_of::<(u64, ChildContentRetirement)>()
}

pub(crate) fn store_replacement_unit_demands<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static>(active: &ActiveArtifactStoreReplacement<A::Snapshot, A::Mutation, M>, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
    use ActiveArtifactStoreReplacementState as State;
    let live = matches!(active.state, State::AwaitingMembers | State::OpeningMembers | State::ClosingRejectedMember | State::ValidatingClosure | State::PreparingCandidateViews | State::CandidateReady);
    Ok(match active.state {
        State::Initializing => active.initializer_demands_for_turn(body, closing)?,
        _ if live && (closing || active.cancelled()) => item_demand(),
        State::AwaitingMembers if active.mounted_genesis.is_some() && active.candidate_children.is_none()=>ChildMemberRegistry::<M>::birth_demands()?,
        State::AwaitingMembers | State::ClosingRejectedMember => item_demand(),
        State::OpeningMembers => active.member_open_demands(body)?,
        State::ValidatingClosure => active.closure_demands(),
        State::PreparingCandidateViews => active.candidate_views_demands(),
        State::RetiringRejectedMembers | State::RetiringCommittedMembers | State::RetiringCommittedStore | State::RetiringRejectedCandidate => active.retirement_demands(body)?,
        State::CandidateReady => RetirementDemand { capacity_bytes: store_replacement_commit_capacity(), depth: 1, ..Default::default() },
        State::Complete => if closing { item_demand() } else { RetirementDemand::default() },
    })
}

#[path = "📥️staging/🏪️lifecycle-stores/🦀️.rs"]
pub mod ladders_lifecycle_stores;
#[path = "📥️staging/🔁️document-replacement/🦀️.rs"]
pub mod ladders_document_replacement;
#[path = "📥️staging/🔄️full-operation/🦀️.rs"]
pub mod ladders_full_operation;
#[path = "📥️staging/🔌️plugin-app-impl/🦀️.rs"]
pub mod ladders_plugin_app_impl;
#[path = "📥️staging/🔒️close-ladder/🦀️.rs"]
pub mod ladders_close_ladder;
#[path = "📥️staging/🗝️envelope-ladder/🦀️.rs"]
pub mod ladders_envelope_ladder;
#[path = "📥️staging/🚪️ingress/🦀️.rs"]
pub mod ladders_ingress;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod ladder_bundle;
#[path = "📥️staging/🧱️components/🦀️.rs"]
pub mod ladders_components;

/// 🧾️ A producer's whole receipt always covers the effects its nested owners already posted on the same original context.
fn whole_receipt(step: PluginLifecycleStep, posted: RetainedCloneProgress) -> PluginLifecycleStep {
    let cover = |whole: RetainedCloneProgress| RetainedCloneProgress {
        copied_items: whole.copied_items.max(posted.copied_items),
        copied_bytes: whole.copied_bytes.max(posted.copied_bytes),
        retained_capacity_bytes: whole.retained_capacity_bytes.max(posted.retained_capacity_bytes),
        released_bytes: whole.released_bytes.max(posted.released_bytes),
    };
    match step {
        PluginLifecycleStep::Progress(progress) => PluginLifecycleStep::Progress(cover(progress)),
        PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Complete(cover(progress)),
        other => other,
    }
}

