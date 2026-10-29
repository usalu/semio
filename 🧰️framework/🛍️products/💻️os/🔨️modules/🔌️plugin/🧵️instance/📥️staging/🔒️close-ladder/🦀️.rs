//! 🪁️ Staged slice of the `close-ladder` band of the mounted artifact instance: its `instance-app` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The retained-field rung of one hosted app's close ladder: every owned field the app retains
//! beyond its stores is released through exact per-axis quotes and an unchanged caller grant.
use store::os_store::ArtifactStoreNativeVcsTurn as _;
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedRegistry, PluginLifecycleStep};
use semio_framework_plugin_instance_stores::stores_components::{ActionIngress, InstanceStores, PeerRoster};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_SLOTS;
use semio_framework_plugin_child::ChildMemberRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use crate::VcsArtifactApp;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::{idle, removed};
use semio_framework_plugin_instance_jobs::jobs_close_ladder::{handoff, handoff_demand};
use semio_framework_plugin_instance_ladders::ladders_close_ladder::{CloseRung, ClosingOwner, child_grant};

/// 📏️ Quotes the registry owner at `cursor`, or one hand-off when it only awaits removal.
fn registry_demands<T: ClosingOwner>(registry: &ArtifactFixedRegistry<T>, cursor: usize, body: usize) -> Result<RetirementDemand, ValueError> {
    let Some((_, id)) = registry.next_id_from(cursor) else { return Ok(RetirementDemand::default()) };
    let Some(owner) = registry.get(id) else { return Ok(handoff_demand()) };
    if owner.closing_terminal_is_empty() { return Ok(handoff_demand()); }
    owner.closing_demands(body)
}

/// 🎟️ Advances the registry owner at `cursor` by one granted turn and removes it at its terminal-empty witness; answers the removed id.
fn registry_step<T: ClosingOwner>(registry: &mut ArtifactFixedRegistry<T>, cursor: &mut usize, cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<(PluginLifecycleStep, Option<u64>),PluginLifecycleFailure> {
    let idle = RetainedCloneProgress::default();
    let Some((index, id)) = registry.next_id_from(*cursor) else { return Ok((PluginLifecycleStep::Complete(idle), None)) };
    let owner = registry.get_mut(id).ok_or_else(|| plugin_sdk_fault("closing registry owner changed during one fixed step"))?;
    let progress = if owner.closing_terminal_is_empty() {
        idle
    } else {
        match owner.closing_step(cx,grant)? {
            PluginLifecycleStep::Progress(progress) => {
                if !owner.closing_terminal_is_empty() {
                    *cursor = index;
                    return Ok((PluginLifecycleStep::Progress(progress), None));
                }
                progress
            }
            PluginLifecycleStep::Complete(progress) => {
                if !owner.closing_terminal_is_empty() { return Err(plugin_sdk_fault("closing registry owner reported Complete without its exact terminal-empty witness").into()); }
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

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    fn uncovered_close_frontier() -> ValueError {
        ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plugin close retains an original frontier without its granted owner authority")
    }

    /// 📏️ Quotes the next close turn on every independent axis; a closed app owes nothing.
    pub(crate) fn close_ladder_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let fault = |fault: Fault| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, fault.message);
        match self.ladder_ref().close_rung() {
            CloseRung::Begin | CloseRung::PendingReserved | CloseRung::ReservedOutcome | CloseRung::DetachChildRoot | CloseRung::DetachChildren | CloseRung::PeerRosterOutcomes | CloseRung::DetachPeerPresence => Ok(handoff_demand()),
            CloseRung::ReservedCommit=>{if let Some(commit)=self.pending.reserved_commits.front(){if let semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitStage::Repin{pins,publication,retirement,closing,..}=&commit.stage{if pins.is_some()||publication.is_some()||!retirement.is_empty(){return self.stores.checkpoint_composition_pins_close_demands(pins,publication,retirement,*closing,4096)}}if let semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitStage::Revert{command:Some(command),..}=&commit.stage{return if command.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::ArtifactNativeVcsCommand<A::Snapshot,A::Mutation>>>(),depth:1,..Default::default()})}else{self.stores.store.native_vcs_command_close_demands(command,body)}}}Ok(handoff_demand())},
            CloseRung::CancellationSlots => {let mut demand=self.tools.tool_cancellations.cancellation_cleanup_demands(self.close.close_cancellation_cursor,body,false)?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original cancellation cursor quote overflow"))?;Ok(demand)},
            CloseRung::DirectIngress => self.stores.window_config_store.direct_ingress_demands(body),
            CloseRung::CommandPrune => self.history.command_prune_demands(true, body),
            CloseRung::GrantedMounted => self.ladder_ref().granted_mounted_retirement_demands(body),
            CloseRung::PrivateChildGroups => self.child_composition.private_child_group_close_demands(body),
            CloseRung::ToolRuns => self.tool_run_ref().tool_run_retirement_demand(body),
            CloseRung::TimeTravel => self.time_travel_ref().time_travel_retirement_demands(body),
            CloseRung::ToolOverlay => self.render_ref().tool_overlay_retirement_demand(body,true),
            CloseRung::LocalQuery => self.interaction.local_interaction_query.as_ref().map_or(Ok(Default::default()), |query| if query.is_closing() { query.retirement_demands(body) } else { Ok(handoff_demand()) }),
            CloseRung::SnapshotReturns => self.ingest.snapshot_read_returns_demands(&self.stores, &self.interaction, body),
            CloseRung::PresenceLocalReads => self.stores.presence_store.maintenance_local_reads_demands(body),
            CloseRung::MountedJobs => self.tools.live_runtime_instance_id.map_or(Ok(Default::default()), |instance_id| A::mounted_job_close_demands(instance_id, body)),
            CloseRung::LatestWinsKeys => self.tools.latest_wins_keys.advance_demands(),
            CloseRung::LatestWinsCommands => self.tools.latest_wins_order.items.front().copied().map_or(Ok(Default::default()), |operation_id| self.tools.close_latest_wins_command_demands(operation_id, body)),
            CloseRung::TypedOperations => if self.original_typed_tag_pending(){self.original_typed_tag_close_demands(body)}else if self.original_typed_command.pending(){self.original_typed_command.close_demands()}else if self.original_typing.pending(){self.original_typing.close_demands()}else if self.original_emission.pending(){self.original_emission.close_demands()}else{self.tools.close_typed_operation_demands(&self.child_composition, &self.stores, body).map_err(fault)},
            CloseRung::InstanceOwner=>if self.close.close_instance_operation_owner_drained{self.tools.instance_operation_owner.inference_retirement_demands(body)}else{self.tools.instance_operation_owner.retirement_demands(body).map(|demand|if demand==Default::default(){handoff_demand()}else{demand})},
            CloseRung::AgentPreview=>self.tools.agent_lane_preview.as_ref().map_or(Ok(Default::default()),|state|state.retirement_demands(body,self.tools.tool_cancellations.handle())),
            CloseRung::ConfigurationFailure=>self.pending.configuration_failure_demands(body),
            CloseRung::RenderFailure=>self.pending.render_failure_demands(body),
            CloseRung::MediaRefusal => self.media.media_submission_refusal_demands(&self.stores,&self.tools, body),
            CloseRung::ActionIngress=>self.ingress.action_ingress_sources_demands(body, true),
            CloseRung::MediaExports => self.tools.close_media_demands(&self.media.media_exports, body),
            CloseRung::MediaClosures => self.tools.close_media_demands(&self.media.media_closures, body),
            CloseRung::Segments => Ok(self.media.close_segment_demands()),
            CloseRung::SegmentClosures => self.media.close_segment_retirement_demands(body),
            CloseRung::SnapshotRetentions => registry_demands(&self.media.snapshot_retirements, self.close.close_snapshot_cursor, body),
            CloseRung::ChildMemberRetirements => {
                let member = self.ladder_ref().child_member_retirement_demands(body)?;
                if self.child_composition.child_content_retirements.is_empty() { Ok(member) } else { Ok(widest_demand(member, self.ladder_ref().child_root_retirement_demands(body)?)) }
            }
            CloseRung::ChildRootRetirements => self.ladder_ref().child_root_retirement_demands(body),
            CloseRung::ClosingChildMember => self.close.close_child_member.as_ref().map_or(Ok(Default::default()), |retirement| retirement.retirement_demands(body)),
            CloseRung::PeerRoster => registry_demands(&self.roster.peer_roster_publications, self.close.close_peer_roster_cursor, body),
            CloseRung::PeerPresenceRetirements => registry_demands(&self.roster.peer_presence_retirements, self.close.close_peer_presence_cursor, body),
            CloseRung::PresencePeerRetirements => registry_demands(&self.roster.presence_peer_retirements, self.close.close_presence_peer_cursor, body),
            CloseRung::EnvelopeIngress => self.ladder_ref().envelope_ingress_demands(true, body),
            CloseRung::EnvelopeDecode => self.ladder_ref().envelope_decode_close_demands(body),
            CloseRung::StoreReplacement => self.ladder_ref().store_replacement_demands(true, body),
            CloseRung::DocumentArchive => self.ladder_ref().document_archive_demands(true, body),
            CloseRung::WindowRetirements => self.ladder_ref().document_windows_retirement_demands(body, true),
            CloseRung::OwnedStores => self.ladder_ref().close_owned_stores_demands(body),
            CloseRung::RetainedFields => self.ladder_ref().retained_fields_close_demand(body),
            CloseRung::ToolLeaseAdmission => self.media.original_tool_lease_close_demands(&self.tools, body),
            CloseRung::DocumentWindowAdmission => Ok(self.pending.document_window_admission_demands()),
            CloseRung::CancellationIssuer => self.tools.tool_cancellations.retirement_demands(body),
            CloseRung::Uncovered => if self.close_terminal_is_empty() { Ok(Default::default()) } else { Err(Self::uncovered_close_frontier()) },
        }
    }

    /// 🎟️ Advances exactly one frontier under the caller's unchanged grant; a closed app answers `Complete`.
    pub(crate) fn close_ladder_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, PluginLifecycleFailure> {
        let idle=RetainedCloneProgress::default();let child=child_grant(grant);
        match self.ladder_ref().close_rung(){
            CloseRung::Begin=>{self.tools.tool_cancellations.cancel_scope_generation();self.roster.peer_roster_scope.cancel_now();self.interaction.local_interaction_authority.close();if let Some(query)=self.interaction.local_interaction_query.as_mut(){query.begin_close()}self.tool_runs.begin_close();self.tools.latest_wins_keys.begin_close();self.close.close_started=true;self.render_ctx().begin_gesture_retirement();Ok(RetainedCloneStep::Progress(handoff()))}
            CloseRung::DirectIngress=>self.stores.window_config_store.close_direct_ingress(grant).map_err(PluginLifecycleFailure::from),
            CloseRung::CommandPrune=>self.history.advance_command_prune_step(&mut self.stores, grant, true).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::GrantedMounted=>self.ladder_ctx().granted_mounted_retirement_step(cx),
            CloseRung::PrivateChildGroups=>Ok(match self.child_composition.close_private_child_group_step(&mut self.stores, cx, grant)?{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>RetainedCloneStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>RetainedCloneStep::Progress(idle)}),
            CloseRung::PendingReserved=>{if let Some((_,id))=self.pending.pending_reserved.next_id_from(0){if let Some(pending)=self.pending.pending_reserved.remove(id){pending.permit.finish()}}Ok(RetainedCloneStep::Progress(handoff()))}
            CloseRung::ReservedCommit=>{
                if let Some(commit)=self.pending.reserved_commits.front_mut(){if let semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitStage::Repin{pins,publication,retirement,closing,..}=&mut commit.stage{if pins.is_some()||publication.is_some()||!retirement.is_empty(){let before=cx.retained_progress();let complete=self.stores.checkpoint_composition_pins_unit(pins,publication,retirement,closing,false,4096,cx)?;let p=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;if !complete||p!=RetainedCloneProgress::default(){return Ok(RetainedCloneStep::Progress(p))}}}}
                if let Some(commit)=self.pending.reserved_commits.front_mut(){if let semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitStage::Revert{command:Some(command),..}=&mut commit.stage{
                    if !command.terminal_is_empty(){let before=cx.retained_progress();cx.with_original_retirement(|cx|self.stores.store.close_native_vcs_command(command,cx))?;let p=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;return Ok(RetainedCloneStep::Progress(p))}
                    if let semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitStage::Revert{command,..}=&mut commit.stage{*command=None;}let d=RetirementDemand{copy_bytes:size_of::<Option<store::os_store::ArtifactNativeVcsCommand<A::Snapshot,A::Mutation>>>(),depth:1,..Default::default()};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
                }}
                if let Some(commit)=self.pending.reserved_commits.pop_front(){commit.permit.finish()}Ok(RetainedCloneStep::Progress(handoff()))
            }
            CloseRung::ReservedOutcome=>{drop(self.pending.reserved_commit_outcome.take());Ok(RetainedCloneStep::Progress(handoff()))}
            CloseRung::ToolRuns=>self.tool_run_ctx().tool_run_close_step(cx).map_err(PluginLifecycleFailure::from),
            CloseRung::TimeTravel=>self.time_travel_ctx().time_travel_close_step_context(grant,cx).map_err(PluginLifecycleFailure::from),
            CloseRung::ToolOverlay=>self.render_ctx().tool_overlay_retirement_step(cx,grant,true).map_err(PluginLifecycleFailure::from),
            CloseRung::LocalQuery=>{let Some(query)=self.interaction.local_interaction_query.as_mut()else{return Ok(RetainedCloneStep::Progress(idle))};if !query.is_closing(){query.begin_close();return Ok(RetainedCloneStep::Progress(handoff()))}let step=query.close_step(child)?;if query.terminal_is_empty(){drop(self.take_local_interaction_query_reply());self.interaction.local_interaction_query=None}Ok(RetainedCloneStep::Progress(step.progress()))}
            CloseRung::SnapshotReturns=>{if let Some(stage)=self.ingest.next_snapshot_read_return_stage(&self.stores, &self.interaction){self.interaction.snapshot_read_return_stage=stage}self.ingest.advance_snapshot_read_returns_one(&mut self.stores, &mut self.interaction, grant).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(PluginLifecycleFailure::from)}
            CloseRung::PresenceLocalReads=>self.stores.presence_store.maintenance_local_reads_step(grant).map_err(PluginLifecycleFailure::from),
            CloseRung::MountedJobs=>{let Some(instance_id)=self.tools.live_runtime_instance_id else{return Ok(RetainedCloneStep::Progress(idle))};A::mounted_job_close_step(instance_id, cx, grant).map(ToolOperations::<A>::lifecycle_retained)}
            CloseRung::CancellationSlots=>{
                let before=cx.retained_progress();let complete=self.tools.tool_cancellations.cleanup_slot(self.close.close_cancellation_cursor,cx,false)?;
                let progress=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;
                if complete&&progress==RetainedCloneProgress::default(){
                    let bytes=std::mem::size_of::<usize>();if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_copy_bytes<bytes||cx.retained_grant().maximum_depth==0{return Ok(RetainedCloneStep::Progress(idle))}
                    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.close.close_cancellation_cursor+=1;
                }
                Ok(RetainedCloneStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
            },
            CloseRung::LatestWinsKeys=>Ok(RetainedCloneStep::Progress(self.tools.latest_wins_keys.advance(grant)?.progress())),
            CloseRung::LatestWinsCommands=>{let Some(operation_id)=self.tools.latest_wins_order.items.front().copied()else{return Ok(RetainedCloneStep::Progress(idle))};self.tools.close_latest_wins_command_step(operation_id, grant,cx).map(|step|RetainedCloneStep::Progress(step.progress()))}
            CloseRung::TypedOperations=>{if self.original_typed_tag_pending(){Ok(self.close_original_typed_tag(cx)?)}else if self.original_typed_command.pending(){let before=cx.retained_progress();self.original_typed_command.close_step(cx)?;Ok(RetainedCloneStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))}else if self.original_typing.pending(){let before=cx.retained_progress();self.original_typing.close_step(cx)?;Ok(RetainedCloneStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))}else if self.original_emission.pending(){let before=cx.retained_progress();self.original_emission.close_step(cx)?;Ok(RetainedCloneStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))}else{self.ladder_ctx().close_typed_operation_unit(cx).map(ToolOperations::<A>::lifecycle_retained)}},
            CloseRung::InstanceOwner=>{
                if self.close.close_instance_operation_owner_drained{return self.tools.instance_operation_owner.close_inference_step(cx,grant).map(ToolOperations::<A>::lifecycle_retained)}
                let step=self.tools.instance_operation_owner.close_step(cx,grant)?;let mut progress=step.progress().unwrap_or_default();
                if matches!(step,PluginLifecycleStep::Complete(_)){if !self.tools.instance_operation_owner.close_terminal_is_empty()?{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original dynamic owner completed with retained inferred payload").with_retained_progress(progress).into())}self.close.close_instance_operation_owner_drained=true;if progress==Default::default(){progress.copied_items=1}}
                Ok(RetainedCloneStep::Progress(progress))
            }
            CloseRung::AgentPreview=>self.tools.close_agent_lane_preview(cx, grant).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::ConfigurationFailure=>self.tools.close_configuration_failure(&mut self.pending, cx, grant).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::RenderFailure=>self.pending.close_render_failure(cx, grant).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::MediaRefusal=>self.media.close_media_submission_refusal(&mut self.stores,&mut self.tools, cx, grant).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::ActionIngress=>self.ingress.close_action_ingress_sources(cx, grant, true).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::MediaExports=>self.media.close_media_step(&mut self.tools, false, cx, grant),
            CloseRung::MediaClosures=>self.media.close_media_step(&mut self.tools, true, cx, grant),
            CloseRung::Segments=>self.media.close_segment_step(grant),
            CloseRung::SegmentClosures=>self.media.close_segment_retirement_step(cx, grant),
            CloseRung::SnapshotRetentions=>registry_step(&mut self.media.snapshot_retirements,&mut self.close.close_snapshot_cursor,cx,grant).map(|(step,_)|ToolOperations::<A>::lifecycle_retained(step)).map_err(PluginLifecycleFailure::from),
            CloseRung::ChildMemberRetirements=>{let progress=self.ladder_ctx().child_member_retirement_step(grant)?;if progress==idle&&!self.child_composition.child_content_retirements.is_empty(){return Ok(RetainedCloneStep::Progress(self.ladder_ctx().child_root_retirement_step(grant)?))}Ok(RetainedCloneStep::Progress(progress))}
            CloseRung::ChildRootRetirements=>Ok(RetainedCloneStep::Progress(self.ladder_ctx().child_root_retirement_step(grant)?)),
            CloseRung::DetachChildRoot=>self.ladder_ctx().detach_child_root_step().map_err(PluginLifecycleFailure::from),
            CloseRung::ClosingChildMember=>{let Some(retirement)=self.close.close_child_member.as_mut()else{return Ok(RetainedCloneStep::Progress(idle))};let step=retirement.close_step(grant)?;if let RetainedCloneStep::Complete(progress)=step{if !retirement.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"child member retirement reported Complete without its exact terminal-empty witness").with_retained_progress(progress).into())}drop(self.close.close_child_member.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:progress.copied_items.max(1),..progress}))}Ok(step)}
            CloseRung::DetachChildren=>{if self.close.close_child_member_cursor>=CHILD_CONTENT_SLOTS{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixed child-member close cursor exhausted before every exact owner was detached").into())}let index=self.close.close_child_member_cursor;self.close.close_child_member_cursor+=1;if let Some(entry)=self.child_composition.children.take_at(index){*self.close.close_child_member=Some(ChildMemberRetirement::new(entry))}Ok(RetainedCloneStep::Progress(handoff()))}
            CloseRung::PeerRoster=>{let(step,removed)=registry_step(&mut self.roster.peer_roster_publications,&mut self.close.close_peer_roster_cursor,cx,grant)?;if let Some(generation)=removed{let slot=InstanceStores::<A>::peer_roster_slot(generation);if self.roster.peer_roster_reservations[slot].is_some_and(|(reserved,_)|reserved==generation){self.roster.peer_roster_reservations[slot]=None}}Ok(ToolOperations::<A>::lifecycle_retained(step))}
            CloseRung::PeerRosterOutcomes=>{if let Some((index,generation))=self.roster.peer_roster_outcomes.next_id_from(self.close.close_peer_roster_cursor){drop(self.roster.peer_roster_outcomes.remove(generation).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"peer roster outcome changed before exact removal"))?);self.close.close_peer_roster_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS}Ok(RetainedCloneStep::Progress(handoff()))}
            CloseRung::PeerPresenceRetirements=>registry_step(&mut self.roster.peer_presence_retirements,&mut self.close.close_peer_presence_cursor,cx,grant).map(|(step,_)|ToolOperations::<A>::lifecycle_retained(step)).map_err(PluginLifecycleFailure::from),
            CloseRung::PresencePeerRetirements=>registry_step(&mut self.roster.presence_peer_retirements,&mut self.close.close_presence_peer_cursor,cx,grant).map(|(step,_)|ToolOperations::<A>::lifecycle_retained(step)).map_err(PluginLifecycleFailure::from),
            CloseRung::DetachPeerPresence=>self.ladder_ctx().detach_peer_presence_step().map_err(PluginLifecycleFailure::from),
            CloseRung::EnvelopeIngress=>self.ladder_ctx().drive_envelope_ingress(grant,true).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::EnvelopeDecode=>self.ladder_ctx().envelope_decode_close_step(cx,grant).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::StoreReplacement=>self.ladder_ctx().drive_store_replacement_jobs(cx,grant,true).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::DocumentArchive=>self.ladder_ctx().drive_document_archive_load_retirements(cx,grant,true).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::WindowRetirements=>self.ladder_ctx().retire_document_windows_step(grant,true).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::OwnedStores=>self.ladder_ctx().close_owned_stores_step(grant).map(ToolOperations::<A>::lifecycle_retained).map_err(PluginLifecycleFailure::from),
            CloseRung::RetainedFields=>self.ladder_ctx().retained_fields_close_step(grant).map_err(PluginLifecycleFailure::from),
            CloseRung::ToolLeaseAdmission=>self.media.close_original_tool_lease(&mut self.tools, cx, grant).map_err(Into::into),
            CloseRung::DocumentWindowAdmission=>self.pending.close_document_window_admission(cx, true).map(ToolOperations::<A>::lifecycle_retained),
            CloseRung::CancellationIssuer=>{let parent=cx.original_source_parent()?;let step=self.tools.tool_cancellations.close_step(parent,cx)?;Ok(match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>RetainedCloneStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>RetainedCloneStep::Complete(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>RetainedCloneStep::Progress(Default::default())})},
            CloseRung::Uncovered=>if self.close_terminal_is_empty(){Ok(RetainedCloneStep::Complete(idle))}else{Err(Self::uncovered_close_frontier().into())},
        }
    }
}

fn widest_demand(left: RetirementDemand, right: RetirementDemand) -> RetirementDemand {
    RetirementDemand { copy_bytes: left.copy_bytes.max(right.copy_bytes), capacity_bytes: left.capacity_bytes.max(right.capacity_bytes), release_bytes: left.release_bytes.max(right.release_bytes), depth: left.depth.max(right.depth) }
}
