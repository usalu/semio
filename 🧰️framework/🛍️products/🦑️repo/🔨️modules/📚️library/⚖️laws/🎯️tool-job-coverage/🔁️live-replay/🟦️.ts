import { toolJobRustBlock } from "../📋️inventory/🟦️.ts";
import { buildBeginCaptureStartState } from "./🔶️begin-capture-start/🟦️.ts";

export function toolJobActorProgressOverlayExact(actor: string, shard: string, wgpu: string): boolean {
  const regionStart = actor.indexOf("//#region 🎨️JobProgressOverlay");
  const regionEnd = actor.indexOf("//#endregion 🎨️JobProgressOverlay", regionStart);
  const overlay = regionStart < 0 || regionEnd < 0 ? "" : actor.slice(regionStart, regionEnd);
  const outcomeStart = wgpu.indexOf("for outcome in outcomes");
  const outcomeOpen = outcomeStart < 0 ? -1 : wgpu.indexOf("{", outcomeStart);
  const outcomes = outcomeOpen < 0 ? undefined : toolJobRustBlock(wgpu, outcomeOpen);
  const destroyStart = wgpu.indexOf("async fn destroy_app_step(&mut self");
  const destroyOpen = destroyStart < 0 ? -1 : wgpu.indexOf("{", destroyStart);
  const destroy = destroyOpen < 0 ? undefined : toolJobRustBlock(wgpu, destroyOpen);
  const publishStart = wgpu.indexOf("fn publish_captured_job_progress(&mut self");
  const publishOpen = publishStart < 0 ? -1 : wgpu.indexOf("{", publishStart);
  const publish = publishOpen < 0 ? undefined : toolJobRustBlock(wgpu, publishOpen);
  return (
    overlay.includes("pub const JOB_PROGRESS_ACTIVE_CAPACITY: usize = 64") &&
    overlay.includes("pub const JOB_PROGRESS_RETIREMENT_CAPACITY: usize = 128") &&
    overlay.includes("pub const JOB_PROGRESS_PAGE_MAXIMUM_BYTES: usize = 16 * 1024") &&
    overlay.includes("pub const JOB_PROGRESS_TOTAL_MAXIMUM_BYTES: usize = 4 * 1024 * 1024") &&
    overlay.includes("pub const JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS: usize = 512") &&
    overlay.includes("active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY]") &&
    overlay.includes("retirements: [JobProgressRetirementSlot; JOB_PROGRESS_RETIREMENT_CAPACITY]") &&
    overlay.includes("pub fn begin_operation(") &&
    overlay.includes("pub fn preflight(") &&
    overlay.includes("cx.operation().0 != identity.operation") &&
    overlay.includes("cx.generation().0 != identity.generation") &&
    overlay.includes("!live.accepts(identity)") &&
    overlay.includes("pub fn publish_reserved(") &&
    overlay.includes("pub fn acknowledge(") &&
    overlay.includes("pub fn abort(") &&
    overlay.includes("pub fn retain_rejected(") &&
    overlay.includes("pub fn take(") &&
    overlay.includes("impl Drop for JobProgressCheckout") &&
    overlay.includes("pub fn begin_close_actor(") &&
    overlay.includes("pub fn close_step(") &&
    overlay.includes("pub fn terminal_is_empty(") &&
    !overlay.includes("HashMap<") &&
    !overlay.includes("VecDeque<") &&
    shard.includes("authority: JobTurn,") &&
    shard.includes("job_authorities: HashMap<(u64, u64), JobAuthority>") &&
    shard.includes("replay_seeds: [Option<MountedReplaySeed>; JOB_REPLAY_SEED_SLOT_CAPACITY]") &&
    shard.includes("MountedReplaySeed::new(actor_id, job, authority, request, placement, kind, input)") &&
    shard.includes("*slot = Some(seed)") &&
    shard.includes("self.runtime.start_job(instance, job, &kind, input).await") &&
    shard.includes("self.job_authorities.insert((actor, seed.job), JobAuthority { turn: seed.authority, request:") &&
    shard.includes("ShardOutcome::Job { actor: actor_id, authority: authority.turn, request: authority.request, placement, publication: JobPublication") &&
    shard.includes("replay_seed_max_plus_one_returns_the_exact_spawn_owners_unchanged") &&
    shard.includes("mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity") &&
    !!publish &&
    publish.body.includes("stable_identity_matches") &&
    publish.body.includes("self.job_progress.begin_operation(actor, authority.job, live)") &&
    !publish.body.includes("self.job_progress.acknowledge(receipt)") &&
    !!outcomes &&
    outcomes.body.includes("ShardOutcome::Job { actor: reported, authority, request, placement, publication }") &&
    outcomes.body.includes("self.begin_job_replay_capture(ActorId(reported), authority, request, placement, self.worker_count, worker_slot, publication)") &&
    wgpu.includes("self.publish_captured_job_progress(actor, authority, publication)") &&
    !!destroy &&
    destroy.body.includes("self.pending_job_progress_presentations") &&
    destroy.body.includes("self.job_progress.actor_terminal_is_empty") &&
    wgpu.includes("job_progress: JobProgressOverlayStore") &&
    wgpu.includes("rejected_job_progress: [Option<JobProgressRejected>; 64]") &&
    wgpu.includes("pending_job_progress_presentations: [Option<PendingJobProgressPresentation>; JOB_PROGRESS_PRESENTATION_CAPACITY]") &&
    wgpu.includes("slots: [JobProgressPresentationSlot; JOB_PROGRESS_PRESENTATION_CAPACITY]") &&
    wgpu.includes("admission_sequence: u64") &&
    wgpu.includes("next_admission_sequence: u64") &&
    wgpu.includes("let index = self.oldest_ready_index()?") &&
    wgpu.includes("self.job_progress.preflight(&mut context, actor, &publication, live)") &&
    wgpu.includes("self.job_progress.publish_reserved(&mut context, admission, publication, live)") &&
    wgpu.includes("self.job_progress.acknowledge(receipt)") &&
    wgpu.includes("self.job_progress.abort(receipt)") &&
    wgpu.includes("AppPresentPhase::ProgressAcknowledge") &&
    wgpu.includes("KernelRequest::AcknowledgeJobProgress { token }") &&
    wgpu.includes("progress.acknowledge_presented()") &&
    wgpu.includes("fn close_realm_progress_step(&mut self) -> bool") &&
    wgpu.includes("self.job_progress.begin_close_all()") &&
    wgpu.includes("self.pending_job_progress_presentations.iter().position(Option::is_some)") &&
    wgpu.includes('job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty()') &&
    wgpu.includes("fn begin_fault_close(&mut self, actor: ActorId)") &&
    wgpu.includes("mounted_job_progress_presentation_bridge_is_fixed_fifo_and_generation_checked") &&
    wgpu.includes("cancelled_head_does_not_strand_later_ready_presentations") &&
    actor.includes("job_progress_preview_is_distinct_owned_and_checked_out_drop_hands_back_exactly") &&
    actor.includes("abort must restore the last valid preview owner rather than exposing the staged replacement") &&
    actor.includes("job_progress_fixed_capacity_and_aba_admission_fail_closed") &&
    actor.includes("job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly") &&
    actor.includes("job_progress_page_boundary_plus_one_stale_order_and_cancel_preserve_owner") &&
    actor.includes("job_progress_commit_validates_live_authority_and_rejected_close_is_incremental") &&
    actor.includes("job_progress_replay_is_deterministic_for_identical_publications")
  );
}
/** 🔁️ P2c fixed replay capture, mounted publication, and recovery contract. */
export function toolJobLiveFixedReplayExact(actor: string, shard: string, executor: string, wgpu: string, actionBus: string, pluginApp: string, pluginHost: string, programBridge: string): boolean {
  const { beginCapture, capture, consumeFuel, copyPage, pageOpportunity, acknowledge, actorClose, refusalDrive, replayDrive, executeTurn, cancel, shardStep, captureCaller, replayRequest, runTurn, replaySequence, pendingReplay, replayAccept, commandPort, maintenance, recoveryClose, recoveryOwnerClose, recoveryDrop, poolNew, actionDispatch, completePipeline, pluginCommand, guestStart, takeProduct, productKindPreflight, productInputPreflight, productRecoveryPreflight, productEffectMove, rawProduct, productRequest, productRecoveryOwner, productValidate, productRawRetirement, productRecovery, productRecoveryClose, productRefusal, productRefusalStep, productPermitDrop, productRefusalDrop, commandMaintenance, productRequestDrop, productClaimDrop, productAuthorityDrop, productAbort, destroyApp, closeRealm, productMount, productAdvance, productValidation, productProfileWrite, productBeginWrite, productClientMount, productClientAdvance, productClientRetire, productClientRefusalRetire, kernelPoolRun, productionAction, productionCommand } = buildBeginCaptureStartState({ actor, shard, wgpu, actionBus, pluginApp, pluginHost, programBridge });
  return (
    actor.includes("pub const JOB_REPLAY_RECORD_CAPACITY: usize = 256") &&
    actor.includes("Box<[Option<JobReplayRecord>; JOB_REPLAY_RECORD_CAPACITY]>") &&
    actor.includes("pages: ManuallyDrop<[Option<JobReplayPage>; JOB_REPLAY_RECORD_PAGE_CAPACITY]>") &&
    actor.includes("JOB_REPLAY_PROCESS_PAGES") &&
    actor.includes("fetch_update(std::sync::atomic::Ordering::AcqRel") &&
    actor.includes("pub struct JobReplayRequest") &&
    actor.includes("pub fn from_spawn(kind: &str, input: &[u8])") &&
    actor.includes("pub fn maintenance_step(&mut self, cx: &mut job::StepContext<'_>)") &&
    actor.includes("pub fn acknowledge_publication(&mut self, cx: &mut job::StepContext<'_>") &&
    !!beginCapture &&
    beginCapture.body.includes("self.closing || cx.is_cancelled()") &&
    beginCapture.body.includes("cx.should_yield()") &&
    beginCapture.body.includes("cx.generation().0 != publication.turn.operation.generation") &&
    !actor.includes("entries: Vec<JobPublication>") &&
    !actor.includes("while !record.terminal_is_empty()") &&
    !!capture &&
    consumeFuel >= 0 &&
    copyPage > consumeFuel &&
    pageOpportunity.includes("cx.consume_fuel(1);") &&
    pageOpportunity.includes("cx.is_cancelled() || cx.deadline_exceeded()") &&
    !!acknowledge &&
    acknowledge.body.indexOf("cx.consume_fuel(1);") >= 0 &&
    acknowledge.body.indexOf("cx.consume_fuel(1);") < acknowledge.body.indexOf("self.ready_record.take()") &&
    !!actorClose &&
    actorClose.body.includes("cx.consume_fuel(1);") &&
    actorClose.body.includes("record.close_step(JOB_REPLAY_PAGE_BYTES)") &&
    actor.includes("for worker_count in [1, 2, 4, host_default]") &&
    actor.includes("replay_record_matches(expected, record)") &&
    actor.includes("cancellation_observed: matches!(publication.outcome, JobStepOutcome::Cancelled)") &&
    actor.includes("u64::from(header.cancellation_observed)") &&
    actor.includes("mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification") &&
    actor.includes("mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged") &&
    actor.includes("mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay") &&
    shard.includes("request: JobReplayRequest") &&
    shard.includes("request.pack_encode(out).await") &&
    shard.includes("kind: ManuallyDrop<[Option<FixedReplaySeedPage>; JOB_REPLAY_KIND_PAGE_CAPACITY]>") &&
    shard.includes("input: ManuallyDrop<[Option<FixedReplaySeedPage>; JOB_REPLAY_INPUT_PAGE_CAPACITY]>") &&
    shard.includes("checkpoint: ManuallyDrop<[Option<FixedReplaySeedPage>; JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY]>") &&
    shard.includes("JOB_REPLAY_SEED_PAGES.fetch_update(Ordering::AcqRel") &&
    shard.includes("replay_seed_refusals: [Option<ReplaySpawnRefusal>; JOB_REPLAY_REFUSAL_SLOT_CAPACITY]") &&
    shard.includes("struct ReplaySpawnRefusal") &&
    shard.includes("input: ManuallyDrop<Option<Vec<u8>>>") &&
    shard.includes("kind: ManuallyDrop<Option<String>>") &&
    !!refusalDrive &&
    refusalDrive.body.indexOf("consume_replay_close_opportunity(actor)") >= 0 &&
    refusalDrive.body.indexOf("consume_replay_close_opportunity(actor)") < refusalDrive.body.indexOf("refusal.input.take()") &&
    refusalDrive.body.includes('drop(refusal.input.take().expect("rejected input closes once"));') &&
    refusalDrive.body.includes('drop(refusal.kind.take().expect("rejected kind closes once"));') &&
    refusalDrive.body.includes("self.replay_seed_refusals[index].take()") &&
    !!replayDrive &&
    replayDrive.body.indexOf("consume_replay_opportunity(actor)") >= 0 &&
    replayDrive.body.indexOf("consume_replay_opportunity(actor)") < replayDrive.body.indexOf("match phase") &&
    replayDrive.body.includes("self.runtime.checkpoint(instance).await") &&
    replayDrive.body.includes("self.runtime.restore(instance, checkpoint).await") &&
    replayDrive.body.match(/self\.runtime\.start_job\(instance, job, &kind, input\)\.await/g)?.length === 2 &&
    replayDrive.body.includes("seed.materialize_page += 1") &&
    replayDrive.body.includes("ReplaySeedPhase::ActivateAuthority") &&
    replayDrive.body.includes("ReplaySeedPhase::ActivateTurn") &&
    replayDrive.body.includes("ReplaySeedPhase::ActivateRunning") &&
    replayDrive.body.includes("ReplaySeedPhase::ActivatePlacement") &&
    replayDrive.body.includes("ReplaySeedPhase::RetireSeedShell") &&
    replayDrive.body.includes("ReplaySeedPhase::RetireMountedShell") &&
    replayDrive.body.includes('self.replay_seeds[index].as_mut().expect("stale mounted replay seed").phase = ReplaySeedPhase::Closing') &&
    replayDrive.body.includes('self.replay_seeds[index].as_mut().expect("stale retained replay seed").phase = ReplaySeedPhase::Closing') &&
    replayDrive.body.includes("!self.instances.contains_key(&seed.actor)") &&
    !!executeTurn &&
    executeTurn.body.includes("JobReplayRequest::from_spawn(&kind, &input)") &&
    executeTurn.body.indexOf("to_actor_turn_result_in_place(&mut result") >= 0 &&
    executeTurn.body.indexOf("to_actor_turn_result_in_place(&mut result") < executeTurn.body.indexOf("std::mem::take(&mut result.effects)") &&
    !executeTurn.body.includes("input.clone()") &&
    !!cancel &&
    cancel.body.match(/seed\.phase = ReplaySeedPhase::Closing/g)?.length === 2 &&
    shard.includes("JobStep::Done { output } => match self.runtime.checkpoint(instance).await {") &&
    shard.includes("JobStep::Failed { error } => {") &&
    shard.includes("JobStepOutcome::Fault { detail: error }") &&
    !shardStep?.body.includes("unwrap_or_default") &&
    shard.includes("turn.step_sequence == u64::MAX || turn.operation.preview_sequence == u64::MAX") &&
    shard.includes("published_turn.operation.preview_sequence += 1") &&
    shard.includes("step_sequence: turn.step_sequence + 1") &&
    !shard.includes("preview_sequence.saturating_add(1)") &&
    !shard.includes("step_sequence: turn.step_sequence.saturating_add(1)") &&
    !shard.includes("next_job_operation.wrapping_add") &&
    !shard.includes("running_jobs.iter().find_map") &&
    shard.includes("replay_seed_max_plus_one_returns_the_exact_spawn_owners_unchanged") &&
    shard.includes("mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity") &&
    shard.includes("mounted_cancel_marks_the_exact_replay_seed_for_incremental_close_before_another_job_step") &&
    !!captureCaller &&
    captureCaller.body.includes("mounted_replay_recovery_registry().lock().expect(\"replay recovery lock\").reserve()") &&
    captureCaller.body.indexOf("reserve()") < captureCaller.body.indexOf("JobReplayLog::new(") &&
    wgpu.includes("replay_routes: [Option<MountedReplayRouteSeed>; JOB_PROGRESS_ACTIVE_CAPACITY]") &&
    wgpu.includes("job_replays: [Option<MountedJobReplay>; JOB_PROGRESS_ACTIVE_CAPACITY]") &&
    wgpu.includes("fn publish_captured_job_progress(") &&
    wgpu.includes("self.publish_captured_job_progress(actor, authority, publication)") &&
    shard.includes("placement: JobPlacement") &&
    shard.includes("request.pack_encode(out).await") &&
    shard.includes('pack::read_u8(bytes, pos, "ShardOutcome::Job::placement")') &&
    wgpu.includes("ShardOutcome::Job { actor: reported, authority, request, placement, publication }") &&
    wgpu.includes("self.begin_job_replay_capture(ActorId(reported), authority, request, placement, self.worker_count, worker_slot, publication)") &&
    wgpu.includes("if replay_capture_started {\n                    break;") &&
    !!replayRequest &&
    replayRequest.body.includes("!matches!(worker_count, 1 | 2 | 4) && worker_count != self.worker_count") &&
    replayRequest.body.includes("mounted_product_replay_worker_slot(process_slot, worker_count) != Some(worker_slot)") &&
    replayRequest.body.includes("entry.accepted_replay_sequence != restore_start_ordinal") &&
    replayRequest.body.indexOf("if entry.replay_requested") >= 0 &&
    replayRequest.body.indexOf("if entry.replay_requested") < replayRequest.body.indexOf("entry.log.begin_replay(entry.authority.operation.generation)") &&
    replayRequest.body.includes("!entry.pending_replay_start_is_exact(worker_count, worker_slot)") &&
    replayRequest.body.includes("return Ok(())") &&
    replayRequest.body.includes("entry.log.begin_replay(entry.authority.operation.generation)") &&
    replayRequest.body.includes("entry.replay_worker_slot = worker_slot") &&
    !!runTurn &&
    runTurn.body.includes("Payload::JobReplay { turn, request: entry.request, worker_count: entry.replay_worker_count, worker_slot: entry.replay_worker_slot }") &&
    runTurn.body.includes("Payload::JobStep { turn }") &&
    runTurn.body.includes("let mut replay_start_index = None") &&
    runTurn.body.includes("entry.replay_submission_sequence()?") &&
    runTurn.body.indexOf("self.runtime.submit(envelope).await") >= 0 &&
    runTurn.body.indexOf("self.runtime.submit(envelope).await") < runTurn.body.indexOf("entry.accept_replay_submission(envelope.seq)?") &&
    runTurn.body.includes("if replay_start_index.is_some()") &&
    runTurn.body.includes("command_ingress: semio_framework::kernel::CommandIngressStatus::Idle") &&
    !runTurn.body.includes("entry.replay_started = true") &&
    !!replaySequence &&
    replaySequence.body.includes("Some(sequence) => Ok(sequence)") &&
    replaySequence.body.includes("self.replay_submit_sequence = Some(sequence)") &&
    !!pendingReplay &&
    pendingReplay.body.includes("self.replay_requested && !self.replay_started") &&
    pendingReplay.body.includes("self.replay_worker_count == worker_count && self.replay_worker_slot == worker_slot") &&
    !!replayAccept &&
    replayAccept.body.includes("self.replay_submit_sequence != Some(sequence)") &&
    replayAccept.body.includes("self.replay_started = true") &&
    replayAccept.body.includes("self.replay_submit_sequence = None") &&
    !!commandPort &&
    commandPort.body.includes("KERNEL_REQUEST_QUEUE_CAPACITY != 0") &&
    commandPort.body.includes("COMMAND_MAXIMUM_PAGES != 0") &&
    commandPort.body.includes("COMMAND_MAXIMUM_BYTES >= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES") &&
    !commandPort.body.includes("false") &&
    wgpu.includes("KernelRequest::MountProductReplay { owner }") &&
    wgpu.includes("KernelRequest::RetireProductReplay { owner }") &&
    wgpu.includes("KernelRequest::AdvanceProductReplay { instance }") &&
    !wgpu.includes("pub(crate) async fn replay_job_turn(&self") &&
    wgpu.includes("rejected_production_replay_submit_retries_exact_restore_start_identity_before_job_step") &&
    !!maintenance &&
    maintenance.body.includes("log.take_captured_publication()") &&
    maintenance.body.includes("log.acknowledge_publication(&mut context, policy)") &&
    wgpu.includes("struct MountedReplayRecoveryRegistry") &&
    !!recoveryClose &&
    recoveryClose.body.includes("self.abandoned[index]") &&
    !!recoveryOwnerClose &&
    recoveryOwnerClose.body.includes("let _ = log.close_step(&mut context);") &&
    wgpu.includes("abandoned: [Option<MountedReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY]") &&
    !wgpu.includes("std::mem::forget(owner)") &&
    !!recoveryDrop &&
    recoveryDrop.body.includes("mounted_replay_recovery_registry().lock().expect(\"replay recovery lock\").publish(token, owner)") &&
    wgpu.includes("panic_after_mounted_capture_transfers_the_exact_generation_to_incremental_recovery") &&
    wgpu.includes("entry.log.begin_close();") &&
    wgpu.includes("log.close_step(&mut context)") &&
    !!poolNew &&
    poolNew.body.includes("let pool = Arc::new(crate::renderer_worker_pool());") &&
    !poolNew.body.includes("WorkerPool::new(") &&
    !wgpu.includes("fn publish_job_progress(") &&
    !!actionDispatch &&
    actionDispatch.body.includes("factory.create_job(&mut spec)") &&
    actionDispatch.body.includes("Ok(ToolJobDispatch { spec, job })") &&
    !!completePipeline &&
    completePipeline.body.includes("self.live_runtime_instance_id != Some(meta.instance_id)") &&
    completePipeline.body.includes("admission.proof.contract()") &&
    completePipeline.body.includes("ToolCancellationPolicy::PerOperation") &&
    completePipeline.body.includes("ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure") &&
    completePipeline.body.includes("ToolExecutionShape::Resumable | semio_framework::ToolExecutionShape::BoundedFirstStep") &&
    completePipeline.body.includes("Ok(())") &&
    !completePipeline.body.includes("interactive-job.full-operation-pending") &&
    !!pluginCommand &&
    pluginCommand.body.indexOf("self.require_complete_tool_operation_pipeline(&admission, meta)?") >= 0 &&
    pluginCommand.body.indexOf("self.require_complete_tool_operation_pipeline(&admission, meta)?") < pluginCommand.body.indexOf("let dispatch = self.tool_jobs.dispatch(operation_spec)") &&
    pluginCommand.body.includes("let dispatch = self.tool_jobs.dispatch(operation_spec)") &&
    pluginCommand.body.includes("MountedWorkerJobSession::try_new(dispatch.job, params)") &&
    pluginApp.includes("session.pump_one(pool, semio_framework_async::Lane::Interactive)") &&
    pluginApp.includes("$crate::reactor::jobs::start_job(input.job, &input.kind, &input.input)") &&
    !!guestStart &&
    guestStart.body.includes("call_start_job(accessor, job, kind, input).await") &&
    shard.includes("self.runtime.start_job(instance, job, &kind, input).await") &&
    !!takeProduct &&
    takeProduct.body.includes("matches!(effect, Effect::SpawnJob { .. })") &&
    wgpu.includes("pub(crate) fn take_product_replay_authority(&mut self, instance: u32, permit: MountedProductReplayAdmissionPermit) -> MountedProductReplayAdmission") &&
    productKindPreflight >= 0 &&
    productInputPreflight > productKindPreflight &&
    productRecoveryPreflight > productInputPreflight &&
    productEffectMove > productRecoveryPreflight &&
    takeProduct.body.includes("let spawn = self.effects.remove(index)") &&
    takeProduct.body.includes("MountedProductReplayRequest::from_admitted_effect(instance, index, spawn, recovery)") &&
    !takeProduct.body.includes("drop(self.effects.remove(index))") &&
    takeProduct.body.includes("MountedProductReplayRefusalCause::KindCapacity") &&
    takeProduct.body.includes("MountedProductReplayRefusalCause::InputCapacity") &&
    takeProduct.body.includes("MountedProductReplayRefusalCause::RecoveryCapacity") &&
    takeProduct.body.includes("let remaining_effects = std::mem::take(&mut self.effects)") &&
    takeProduct.body.includes("MountedProductReplayAdmission::Refused(permit.refuse(cause, index, spawn, remaining_effects))") &&
    !takeProduct.body.includes("self.effects.insert(index, effect)") &&
    !takeProduct.body.includes("Result<Option<MountedProductReplayRequest>") &&
    wgpu.includes("const PRODUCT_REPLAY_INPUT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY") &&
    wgpu.includes("const PRODUCT_REPLAY_INPUT_BYTES: usize = semio_framework_actor::JOB_REPLAY_PAGE_BYTES * PRODUCT_REPLAY_INPUT_PAGE_CAPACITY") &&
    wgpu.includes("#[derive(Debug)]\n    struct RawSpawnJobOwner") &&
    wgpu.includes("raw: Option<RawSpawnJobOwner>") &&
    wgpu.includes("raw: RawSpawnJobOwner") &&
    !!rawProduct &&
    rawProduct.body.includes("let Effect::SpawnJob { job, kind, input, placement } = effect") &&
    rawProduct.body.includes("kind: Some(kind)") &&
    rawProduct.body.includes("input: Some(input)") &&
    rawProduct.body.includes("self.fixed_witness_acked = true") &&
    rawProduct.body.includes("self.mount_acked = true") &&
    rawProduct.body.includes("self.qualification_acked = true") &&
    rawProduct.body.includes("accepted_replay_ordinal.is_none()") &&
    rawProduct.body.includes("self.replay_acked = true") &&
    rawProduct.body.includes("self.disposition = RawSpawnJobDisposition::Accepted") &&
    rawProduct.body.includes('drop(self.input.take().expect("raw admitted product input backing closes once"))') &&
    rawProduct.body.includes('drop(self.kind.take().expect("raw admitted product kind backing closes once"))') &&
    !!productRequest &&
    productRequest.body.includes("RawSpawnJobOwner::from_effect(selected_index, effect)") &&
    productRequest.body.includes("JobReplayRequest::from_spawn(raw.kind(), raw.input())") &&
    productRequest.body.indexOf("raw.acknowledge_fixed_witness()") > productRequest.body.indexOf("copy_from_slice(raw.kind().as_bytes())") &&
    productRequest.body.includes('self.raw.take().expect("mounted product replay raw SpawnJob transfers once")') &&
    productRequest.body.includes("self.raw_mut().reject()") &&
    productRequest.body.includes("MountedProductReplayRecoveryOwner::Request(retained)") &&
    !!productRecoveryOwner &&
    productRecoveryOwner.body.includes("request.raw.close_step()") &&
    !!productValidate &&
    productValidate.body.includes("request_owner.instance == expected.instance") &&
    productValidate.body.includes("request_owner.job == expected.turn.job") &&
    productValidate.body.includes("request_owner.raw().job == request_owner.job") &&
    productValidate.body.includes("request_owner.raw().kind().as_bytes() == request_owner.job_kind()") &&
    productValidate.body.includes("JobReplayRequest::from_spawn(request_owner.raw().kind(), request_owner.raw().input())") &&
    productValidate.body.includes("request_owner.raw().placement == request_owner.placement") &&
    productValidate.body.includes("kind.tool == expected.request.tool") &&
    productValidate.body.includes("request_owner.request == expected.request") &&
    productValidate.body.includes("request_owner.placement == expected.placement") &&
    productValidate.body.includes("self.actor == expected.actor") &&
    productValidate.body.includes("self.operation == expected.turn.operation") &&
    productValidate.body.includes("self.generation == expected.turn.operation.generation") &&
    productValidate.body.includes("self.seed == expected.turn.operation.seed") &&
    productValidate.body.includes("self.route == expected.route") &&
    productValidate.body.includes("self.checkpoint == expected.checkpoint") &&
    productValidate.body.includes("self.terminal == expected.terminal") &&
    productValidate.body.includes("expected.terminal.worker_count == expected.process_worker_count") &&
    productValidate.body.includes("expected.terminal.worker_slot == expected.process_worker_slot") &&
    productValidate.body.includes("self.worker_count == expected.logical_worker_count") &&
    productValidate.body.includes("self.worker_slot == expected.logical_worker_slot") &&
    productValidate.body.includes("self.begin == expected.begin") &&
    productValidate.body.includes("self.restore_start_ordinal == expected.restore_start_ordinal") &&
    productValidate.body.includes("Err(self)") &&
    !!productRawRetirement &&
    productRawRetirement.body.includes("self.profile_cursor != PRODUCT_REPLAY_PROFILE_COUNT || self.profile_started") &&
    productRawRetirement.body.includes("self.request_owner_mut().raw_mut().acknowledge_replay_boundary(terminal, accepted_replay_ordinal)") &&
    !!productRecovery &&
    productRecovery.body.includes("generation.checked_add(1)") &&
    productRecovery.body.includes("slot.generation == token.generation && slot.reserved && slot.owner.is_none()") &&
    productRecovery.body.includes("self.abandoned.iter_mut().find") &&
    !!productRecoveryClose &&
    productRecoveryClose.body.indexOf(".close_step()") >= 0 &&
    productRecoveryClose.body.indexOf(".close_step()") < productRecoveryClose.body.indexOf("self.slots[index].owner.take()") &&
    productRecoveryClose.body.includes("self.slots[index].owner.take()") &&
    productRecoveryClose.body.includes("self.slots[index].reserved = false") &&
    productRecoveryClose.body.includes("self.abandoned[index].take()") &&
    !!productRefusal &&
    productRefusal.body.includes("generation.checked_add(1)") &&
    productRefusal.body.includes("slot.generation == token.generation && slot.instance == Some(owner.instance) && slot.reserved && slot.owner.is_none()") &&
    productRefusal.body.includes("self.abandoned.iter_mut().find") &&
    productRefusal.body.includes("fn close_instance_one(&mut self, instance: u32)") &&
    !!productRefusalStep &&
    productRefusalStep.body.includes("owner.retry()") &&
    productRefusalStep.body.includes("owner.close_step()") &&
    productRefusalStep.body.includes("self.slots[index].owner.take()") &&
    productRefusalStep.body.includes("self.slots[index].reserved = false") &&
    wgpu.includes("struct RefusedProductReplay") &&
    wgpu.includes("spawn: Option<Effect>") &&
    wgpu.includes("let Some(spawn) = self.spawn.take()") &&
    wgpu.includes("MountedProductReplayRequest::from_admitted_effect(self.instance, self.selected_index, spawn, recovery)") &&
    wgpu.includes("remaining_effects: Option<Vec<Effect>>") &&
    wgpu.includes("drop(self.spawn_input.take().expect(\"refused product input backing closes once\"))") &&
    wgpu.includes("drop(self.spawn_kind.take().expect(\"refused product kind backing closes once\"))") &&
    wgpu.includes("drop(self.remaining_effects.take().expect(\"refused product outcome backing closes once\"))") &&
    !!productPermitDrop &&
    productPermitDrop.body.includes("release(token)") &&
    !!productRefusalDrop &&
    productRefusalDrop.body.includes("mounted_product_replay_refusal_registry().lock().expect(\"product replay refusal lock\").publish(token, retained)") &&
    !!commandMaintenance &&
    commandMaintenance.body.includes('mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one()') &&
    commandMaintenance.body.includes('mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true)') &&
    commandMaintenance.body.includes("self.mount_product_replay(owner)") &&
    !!productRequestDrop &&
    productRequestDrop.body.includes("raw.reject()") &&
    productRequestDrop.body.includes("MountedProductReplayRecoveryOwner::Request(retained)") &&
    (wgpu.match(/request\.raw_mut\(\)\.reject\(\)/g) ?? []).length >= 4 &&
    !!productClaimDrop &&
    productClaimDrop.body.includes("request.raw_mut().reject()") &&
    productClaimDrop.body.includes("MountedProductReplayRecoveryOwner::Claim") &&
    !!productAuthorityDrop &&
    productAuthorityDrop.body.includes("request.raw_mut().reject()") &&
    productAuthorityDrop.body.includes("MountedProductReplayRecoveryOwner::Authority") &&
    !!productAbort &&
    productAbort.body.includes('take().expect("faulted product replay authority remains exact").retire()') &&
    productAbort.body.includes('take().expect("faulted product replay claim remains exact").retire()') &&
    !productAbort.body.includes("= None") &&
    !!destroyApp &&
    destroyApp.body.includes('take().expect("destroyed app product replay claim remains exact").retire()') &&
    destroyApp.body.includes('take().expect("destroyed app product replay authority remains exact").retire()') &&
    destroyApp.body.includes('mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").close_instance_one(instance)') &&
    destroyApp.body.includes("close_instance_one(instance)") &&
    !!closeRealm &&
    closeRealm.body.includes('take().expect("realm product replay claim remains exact").retire()') &&
    closeRealm.body.includes('take().expect("realm product replay authority remains exact").retire()') &&
    closeRealm.body.includes('mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false)') &&
    closeRealm.body.includes('mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one()') &&
    !!productMount &&
    productMount.body.includes("self.instances.get(&owner.instance)") &&
    productMount.body.includes("owner.raw().job != owner.job") &&
    productMount.body.includes("owner.raw().kind().as_bytes() != owner.job_kind()") &&
    productMount.body.includes("owner.raw().placement != owner.placement") &&
    productMount.body.includes("JobReplayRequest::from_spawn(owner.raw().kind(), owner.raw().input()) != owner.request") &&
    productMount.body.includes("JobReplayRequest::from_spawn(kind, &[])") &&
    productMount.body.includes("kind_identity.controller != owner.request.controller") &&
    productMount.body.includes("return Err(owner)") &&
    productMount.body.indexOf("owner.raw_mut().acknowledge_mount()") >= 0 &&
    productMount.body.indexOf("owner.raw_mut().acknowledge_mount()") < productMount.body.indexOf("MountedProductReplayClaim { request: Some(owner)") &&
    productMount.body.includes("MountedProductReplayClaim { request: Some(owner), actor, record_cursor: 0, checkpoint: None }") &&
    !!productAdvance &&
    productAdvance.body.includes("claim.record_cursor += 1") &&
    !productAdvance.body.includes("while ") &&
    !productAdvance.body.includes("WorkerPool::new") &&
    productAdvance.body.includes("JobReplayPublicationKind::Checkpoint") &&
    productAdvance.body.includes("terminal.kind, JobReplayPublicationKind::Commit | JobReplayPublicationKind::Cancelled | JobReplayPublicationKind::Fault") &&
    productAdvance.body.match(/entry\.log\.last_checkpoint_header\(\)/g)?.length === 2 &&
    productAdvance.body.includes("terminal.worker_count != self.worker_count || terminal.worker_slot != process_slot") &&
    productAdvance.body.includes("MountedProductReplayAuthority {") &&
    productAdvance.body.indexOf("request_owner.raw_mut().acknowledge_qualification(claim.checkpoint)") >= 0 &&
    productAdvance.body.indexOf("request_owner.raw_mut().acknowledge_qualification(claim.checkpoint)") < productAdvance.body.indexOf("MountedProductReplayAuthority {") &&
    productAdvance.body.includes("mounted_product_replay_profile(self.worker_count, profile_cursor)") &&
    productAdvance.body.includes("self.request_job_replay(actor, job, worker_count, worker_slot, restore_start_ordinal).await?") &&
    productAdvance.body.includes("let outcome = self.run_turn(actor, instance, Vec::new()).await?") &&
    productAdvance.body.includes("authority.begin = !replay_started") &&
    productAdvance.body.includes("authority.restore_start_ordinal = restore_start_ordinal") &&
    productAdvance.body.includes("acknowledge_raw_retirement_boundary()") &&
    productAdvance.body.indexOf("acknowledge_raw_retirement_boundary()") < productAdvance.body.indexOf('take().expect("completed product replay authority remains exact").retire()') &&
    productAdvance.body.includes('take().expect("completed product replay authority remains exact").retire()') &&
    productValidation >= 0 &&
    productValidation < productProfileWrite &&
    productValidation < productBeginWrite &&
    !!productClientMount &&
    productClientMount.body.includes("KernelRequest::MountProductReplay { owner }") &&
    productClientMount.body.includes("KernelOutcome::ProductReplayMounted(result) => result") &&
    !!productClientAdvance &&
    productClientAdvance.body.includes("KernelRequest::AdvanceProductReplay { instance }") &&
    !!productClientRetire &&
    productClientRetire.body.includes("KernelRequest::RetireProductReplay { owner }") &&
    productClientRetire.body.includes("owner.retire()") &&
    !!productClientRefusalRetire &&
    productClientRefusalRetire.body.includes("KernelRequest::RetireProductReplayRefusal { owner }") &&
    wgpu.includes("KernelRequest::RetireProductReplayRefusal { owner } => {") &&
    !!kernelPoolRun &&
    kernelPoolRun.body.indexOf("let ready = queue.try_next()") >= 0 &&
    kernelPoolRun.body.indexOf("let ready = queue.try_next()") < kernelPoolRun.body.indexOf("state.product_replay_pending_instance()") &&
    (kernelPoolRun.body.match(/state\.advance_product_replay\(instance\)\.await/g) ?? []).length >= 2 &&
    kernelPoolRun.body.includes("state.abort_product_replay_one(instance)") &&
    kernelPoolRun.body.includes("yield_kernel_maintenance_turn().await") &&
    wgpu.includes("[1, 2, 4, process_default]") &&
    wgpu.includes("process_slot % worker_count") &&
    wgpu.includes("production_action_bus_product_authority_carries_complete_identity_and_returns_exact_owner_on_wrong_field") &&
    wgpu.includes("production_product_authority_qualifies_every_minted_field_before_replay_work") &&
    wgpu.includes("product_authority_fault_abort_and_close_rediscover_exact_generation_once") &&
    wgpu.includes("product_claim_fault_retains_exact_request_cursor_checkpoint_and_generation") &&
    wgpu.includes("product_effect_after_normal_exchange_is_moved_into_fixed_authority_without_json_escape_hatch") &&
    wgpu.includes("product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder") &&
    wgpu.includes("product_ingress_recovery_max_plus_one_retries_exact_spawn_then_closes_remainder_incrementally") &&
    wgpu.includes("product_refusal_slot_max_plus_one_stops_before_exchange_and_drop_publishes_exact_generation") &&
    wgpu.includes("admitted_product_mount_refusal_returns_exact_raw_allocation_then_closes_one_backing_per_grant") &&
    wgpu.includes("admitted_product_request_claim_and_authority_drop_preserve_raw_allocation_identity") &&
    wgpu.includes("admitted_raw_backing_retires_only_after_fixed_mount_qualification_and_all_replay_acks") &&
    wgpu.includes("production_mounted_process_replay_supports_one_two_four_and_default_without_private_pool") &&
    wgpu.includes("production_product_authority_opportunities_remain_sub_eight_ms") &&
    !programBridge.includes("mountedJobReplay") &&
    !programBridge.includes("requested_effects: outcome.effects.clone()") &&
    !!productionAction &&
    !productionAction.body.includes("serde_json::from_value") &&
    productionAction.body.indexOf("client.reserve_product_replay_admission(instance_id)?") >= 0 &&
    productionAction.body.indexOf("client.reserve_product_replay_admission(instance_id)?") < productionAction.body.indexOf("let mut outcome = exchange(client, instance_id, commands).await?") &&
    productionAction.body.indexOf("let mut outcome = exchange(client, instance_id, commands).await?") >= 0 &&
    productionAction.body.indexOf("let mut outcome = exchange(client, instance_id, commands).await?") < productionAction.body.indexOf("outcome.take_product_replay_authority(instance_id, admission)") &&
    productionAction.body.indexOf("outcome.take_product_replay_authority(instance_id, admission)") < productionAction.body.indexOf("MountedProductReplayAdmission::Refused(refusal)") &&
    productionAction.body.includes("client.retire_product_replay_refusal(refusal).await") &&
    productionAction.body.indexOf("client.retire_product_replay_refusal(refusal).await") < productionAction.body.indexOf("invocation_from_frames(&mut outcome, seq)") &&
    !productionAction.body.includes("take_product_replay_authority(instance_id)?") &&
    productionAction.body.indexOf("invocation_from_frames(&mut outcome, seq)?") < productionAction.body.indexOf("client.mount_product_replay(authority).await") &&
    productionAction.body.includes("client.retire_product_replay(authority).await") &&
    productionAction.body.indexOf("client.mount_product_replay(authority).await") < productionAction.body.indexOf("client.advance_product_replay(instance_id).await?") &&
    !!productionCommand &&
    !productionCommand.body.includes("serde_json::from_value") &&
    productionCommand.body.indexOf("client.reserve_product_replay_admission(instance_id)?") >= 0 &&
    productionCommand.body.indexOf("client.reserve_product_replay_admission(instance_id)?") < productionCommand.body.indexOf("let mut outcome = exchange(client, instance_id") &&
    productionCommand.body.indexOf("let mut outcome = exchange(client, instance_id") >= 0 &&
    productionCommand.body.indexOf("let mut outcome = exchange(client, instance_id") < productionCommand.body.indexOf("outcome.take_product_replay_authority(instance_id, admission)") &&
    productionCommand.body.indexOf("outcome.take_product_replay_authority(instance_id, admission)") < productionCommand.body.indexOf("MountedProductReplayAdmission::Refused(refusal)") &&
    productionCommand.body.includes("client.retire_product_replay_refusal(refusal).await") &&
    productionCommand.body.indexOf("client.retire_product_replay_refusal(refusal).await") < productionCommand.body.indexOf("invocation_from_frames(&mut outcome, seq)") &&
    !productionCommand.body.includes("take_product_replay_authority(instance_id)?") &&
    productionCommand.body.indexOf("invocation_from_frames(&mut outcome, seq)?") < productionCommand.body.indexOf("client.mount_product_replay(authority).await") &&
    productionCommand.body.includes("client.retire_product_replay(authority).await") &&
    productionCommand.body.indexOf("client.mount_product_replay(authority).await") < productionCommand.body.indexOf("client.advance_product_replay(instance_id).await?")
  );
}
