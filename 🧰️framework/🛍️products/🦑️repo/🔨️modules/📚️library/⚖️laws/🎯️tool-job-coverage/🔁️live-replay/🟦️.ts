/** 🔂️ Tool-job coverage: live fixed replay, mounted sessions, retained ownership and the FEM numerical microcursor.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { toolJobProductionProtocolSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";

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
  const beginCaptureStart = actor.indexOf("pub fn begin_capture(");
  const beginCapture = beginCaptureStart < 0 ? undefined : toolJobRustBlock(actor, actor.indexOf("{", beginCaptureStart));
  const captureStart = actor.indexOf("pub fn capture_step(");
  const capture = captureStart < 0 ? undefined : toolJobRustBlock(actor, actor.indexOf("{", captureStart));
  const acknowledgeStart = actor.indexOf("pub fn acknowledge_publication(");
  const acknowledge = acknowledgeStart < 0 ? undefined : toolJobRustBlock(actor, actor.indexOf("{", acknowledgeStart));
  const actorCloseStart = actor.indexOf("pub fn close_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayCloseStep");
  const actorClose = actorCloseStart < 0 ? undefined : toolJobRustBlock(actor, actor.indexOf("{", actorCloseStart));
  const shardStepStart = shard.indexOf("let job_outcome = {");
  const shardStep = shardStepStart < 0 ? undefined : toolJobRustBlock(shard, shard.indexOf("{", shardStepStart));
  const replayDriveStart = shard.indexOf("async fn drive_replay_seed(");
  const replayDrive = replayDriveStart < 0 ? undefined : toolJobRustBlock(shard, shard.indexOf("{", replayDriveStart));
  const refusalDriveStart = shard.indexOf("async fn drive_replay_refusal(");
  const refusalDrive = refusalDriveStart < 0 ? undefined : toolJobRustBlock(shard, shard.indexOf("{", refusalDriveStart));
  const executeTurnStart = shard.indexOf("async fn execute_turn_for(");
  const executeTurn = executeTurnStart < 0 ? undefined : toolJobRustBlock(shard, shard.indexOf("{", executeTurnStart));
  const cancelStart = shard.indexOf("Effect::CancelJob { job } => {", executeTurnStart);
  const cancelOpen = cancelStart < 0 ? -1 : shard.indexOf("=> {", cancelStart) + 3;
  const cancel = cancelOpen < 3 ? undefined : toolJobRustBlock(shard, cancelOpen);
  const captureCallerStart = wgpu.indexOf("fn begin_job_replay_capture(");
  const captureCaller = captureCallerStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", captureCallerStart));
  const replayRequestStart = wgpu.indexOf("async fn request_job_replay(");
  const replayRequest = replayRequestStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", replayRequestStart));
  const runTurnStart = wgpu.indexOf("async fn run_turn(&mut self");
  const runTurn = runTurnStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", runTurnStart));
  const maintenanceStart = wgpu.indexOf("fn job_replay_maintenance_step(");
  const maintenance = maintenanceStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", maintenanceStart));
  const recoveryDropStart = wgpu.indexOf("impl Drop for MountedJobReplay");
  const recoveryDrop = recoveryDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", recoveryDropStart));
  const recoveryCloseStart = wgpu.indexOf("fn close_one(&mut self) -> bool", wgpu.indexOf("impl MountedReplayRecoveryRegistry"));
  const recoveryClose = recoveryCloseStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", recoveryCloseStart));
  const recoveryOwnerCloseStart = wgpu.indexOf("fn mounted_replay_recovery_owner_close_one(");
  const recoveryOwnerClose = recoveryOwnerCloseStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", recoveryOwnerCloseStart));
  const poolNewStart = wgpu.indexOf("async fn new() -> Self", wgpu.indexOf("impl KernelPoolState"));
  const poolNew = poolNewStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", poolNewStart));
  const replaySequenceStart = wgpu.indexOf("fn replay_submission_sequence(&mut self)");
  const replaySequence = replaySequenceStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", replaySequenceStart));
  const pendingReplayStart = wgpu.indexOf("fn pending_replay_start_is_exact(&self");
  const pendingReplay = pendingReplayStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", pendingReplayStart));
  const replayAcceptStart = wgpu.indexOf("fn accept_replay_submission(&mut self");
  const replayAccept = replayAcceptStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", replayAcceptStart));
  const commandPortStart = wgpu.indexOf("fn persistent_command_completion_port_ready()");
  const commandPort = commandPortStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", commandPortStart));
  const actionDispatchStart = actionBus.indexOf("pub fn dispatch(&self, mut spec: ToolOperationSpec)");
  const actionDispatch = actionDispatchStart < 0 ? undefined : toolJobRustBlock(actionBus, actionBus.indexOf("{", actionDispatchStart));
  const completePipelineStart = pluginApp.indexOf("fn require_complete_tool_operation_pipeline(");
  const completePipeline = completePipelineStart < 0 ? undefined : toolJobRustBlock(pluginApp, pluginApp.indexOf("{", completePipelineStart));
  const pluginCommandStart = pluginApp.indexOf("async fn dispatch_typed_command_inner(");
  const pluginCommand = pluginCommandStart < 0 ? undefined : toolJobRustBlock(pluginApp, pluginApp.indexOf("{", pluginCommandStart));
  const guestStartStart = pluginHost.indexOf("async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>)", pluginHost.indexOf("impl GuestRuntime for WasmtimeRuntime"));
  const guestStart = guestStartStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", guestStartStart));
  const takeProductStart = wgpu.indexOf("pub(crate) fn take_product_replay_authority(");
  const takeProduct = takeProductStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", takeProductStart));
  const rawProductStart = wgpu.indexOf("impl RawSpawnJobOwner");
  const rawProduct = rawProductStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", rawProductStart));
  const productRequestStart = wgpu.indexOf("impl MountedProductReplayRequest");
  const productRequest = productRequestStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRequestStart));
  const productRecoveryOwnerStart = wgpu.indexOf("impl MountedProductReplayRecoveryOwner");
  const productRecoveryOwner = productRecoveryOwnerStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRecoveryOwnerStart));
  const productValidateStart = wgpu.indexOf("fn validate(self, expected: MountedProductReplayExpected", wgpu.indexOf("impl MountedProductReplayAuthority"));
  const productValidate = productValidateStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productValidateStart));
  const productRawRetirementStart = wgpu.indexOf("fn acknowledge_raw_retirement_boundary(&mut self)");
  const productRawRetirement = productRawRetirementStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRawRetirementStart));
  const productRecoveryStart = wgpu.indexOf("impl MountedProductReplayRecoveryRegistry");
  const productRecovery = productRecoveryStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRecoveryStart));
  const productRecoveryCloseStart = wgpu.indexOf("fn close_one(&mut self) -> bool", productRecoveryStart);
  const productRecoveryClose = productRecoveryCloseStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRecoveryCloseStart));
  const productRefusalStart = wgpu.indexOf("impl MountedProductReplayRefusalRegistry");
  const productRefusal = productRefusalStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRefusalStart));
  const productRefusalStepStart = wgpu.indexOf("fn step_one(&mut self, retry: bool)", productRefusalStart);
  const productRefusalStep = productRefusalStepStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRefusalStepStart));
  const productPermitDropStart = wgpu.indexOf("impl Drop for MountedProductReplayAdmissionPermit");
  const productPermitDrop = productPermitDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productPermitDropStart));
  const productRefusalDropStart = wgpu.indexOf("impl Drop for RefusedProductReplay");
  const productRefusalDrop = productRefusalDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRefusalDropStart));
  const productRequestDropStart = wgpu.indexOf("impl Drop for MountedProductReplayRequest");
  const productRequestDrop = productRequestDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productRequestDropStart));
  const productClaimDropStart = wgpu.indexOf("impl Drop for MountedProductReplayClaim");
  const productClaimDrop = productClaimDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productClaimDropStart));
  const productAuthorityDropStart = wgpu.indexOf("impl Drop for MountedProductReplayAuthority");
  const productAuthorityDrop = productAuthorityDropStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productAuthorityDropStart));
  const productAbortStart = wgpu.indexOf("fn abort_product_replay_one(");
  const productAbort = productAbortStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productAbortStart));
  const commandMaintenanceStart = wgpu.indexOf("fn command_maintenance_step(");
  const commandMaintenance = commandMaintenanceStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", commandMaintenanceStart));
  const destroyAppStart = wgpu.indexOf("async fn destroy_app_step(");
  const destroyApp = destroyAppStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", destroyAppStart));
  const closeRealmStart = wgpu.indexOf("fn close_realm_progress_step(");
  const closeRealm = closeRealmStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", closeRealmStart));
  const productMountStart = wgpu.indexOf("fn mount_product_replay(&mut self");
  const productMount = productMountStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productMountStart));
  const productAdvanceStart = wgpu.indexOf("async fn advance_product_replay(&mut self");
  const productAdvance = productAdvanceStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productAdvanceStart));
  const productClientMountStart = wgpu.indexOf("pub(crate) async fn mount_product_replay(&self");
  const productClientMount = productClientMountStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productClientMountStart));
  const productClientAdvanceStart = wgpu.indexOf("pub(crate) async fn advance_product_replay(&self");
  const productClientAdvance = productClientAdvanceStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productClientAdvanceStart));
  const productClientRetireStart = wgpu.indexOf("pub(crate) async fn retire_product_replay(&self");
  const productClientRetire = productClientRetireStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productClientRetireStart));
  const productClientRefusalRetireStart = wgpu.indexOf("pub(crate) async fn retire_product_replay_refusal(&self");
  const productClientRefusalRetire = productClientRefusalRetireStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", productClientRefusalRetireStart));
  const kernelPoolRunStart = wgpu.indexOf("async fn run_kernel_pool(");
  const kernelPoolRun = kernelPoolRunStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", kernelPoolRunStart));
  const productionActionStart = programBridge.indexOf("pub async fn handle_action(");
  const productionAction = productionActionStart < 0 ? undefined : toolJobRustBlock(programBridge, programBridge.indexOf("{", productionActionStart));
  const productionCommandStart = programBridge.indexOf("pub async fn handle_command(");
  const productionCommand = productionCommandStart < 0 ? undefined : toolJobRustBlock(programBridge, programBridge.indexOf("{", productionCommandStart));
  const consumeFuel = capture?.body.indexOf("cx.consume_fuel(1);") ?? -1;
  const copyPage = capture?.body.indexOf("JobReplayPage::try_copy(chunk)") ?? -1;
  const pageOpportunity = capture && copyPage >= 0 ? capture.body.slice(Math.max(0, copyPage - 420), copyPage) : "";
  const productValidation = productAdvance?.body.indexOf("authority.validate(expected)") ?? -1;
  const productProfileWrite = productAdvance?.body.indexOf(".worker_count =") ?? -1;
  const productBeginWrite = productAdvance?.body.indexOf(".begin =") ?? -1;
  const productKindPreflight = takeProduct?.body.indexOf("kind.len() > PRODUCT_REPLAY_KIND_BYTES") ?? -1;
  const productInputPreflight = takeProduct?.body.indexOf("input.len() > PRODUCT_REPLAY_INPUT_BYTES") ?? -1;
  const productRecoveryPreflight = takeProduct?.body.indexOf('mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()') ?? -1;
  const productEffectMove = takeProduct?.body.indexOf("self.effects.remove(index)") ?? -1;
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



/** 🏗️ Phase-6 mounted FEM2D job ownership and live-route contract. */
export function toolJobFem2dMountedSessionExact(session: string, editor: string, model: string, mesh: string, pluginRoot: string, glue: string, jobs: string, analyses: string, reactor: string, store: string, frameworkPlugin: string): boolean {
  const reconcileStart = session.indexOf("pub fn reconcile(");
  const reconcileOpen = reconcileStart < 0 ? -1 : session.indexOf("{", reconcileStart);
  const reconcile = reconcileOpen < 0 ? undefined : toolJobRustBlock(session, reconcileOpen);
  const stepStart = session.indexOf("fn step(&mut self, budget: JobBudget)");
  const stepOpen = stepStart < 0 ? -1 : session.indexOf("{", stepStart);
  const step = stepOpen < 0 ? undefined : toolJobRustBlock(session, stepOpen);
  const pluginStart = pluginRoot.indexOf("pub fn plugin()");
  const pluginOpen = pluginStart < 0 ? -1 : pluginRoot.indexOf("{", pluginStart);
  const plugin = pluginOpen < 0 ? undefined : toolJobRustBlock(pluginRoot, pluginOpen);
  const snapshotAdmission = frameworkPlugin.indexOf("let snapshot_is_admitted = {");
  const snapshotIssue = frameworkPlugin.indexOf("self.store.snapshot_read()", snapshotAdmission);
  const renderStart = model.indexOf("pub fn render_with_progress(");
  const renderOpen = renderStart < 0 ? -1 : model.indexOf("{", renderStart);
  const render = renderOpen < 0 ? undefined : toolJobRustBlock(model, renderOpen);
  const classifyStart = mesh.lastIndexOf("MeshJobStage::Classify =>");
  const classifyEnd = classifyStart < 0 ? -1 : mesh.indexOf("MeshJobStage::Finalize =>", classifyStart);
  const classify = classifyEnd < 0 ? "" : mesh.slice(classifyStart, classifyEnd);
  return (
    session.includes('pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis"') &&
    session.includes("const SESSION_ACTIVE_CAPACITY: usize = 32") &&
    session.includes("const SESSION_SHELL_CAPACITY: usize = 64") &&
    session.includes("const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096") &&
    session.includes("const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024") &&
    session.includes("struct MountedProcessOwnerCatalog") &&
    session.includes("claims: [MountedOwnerClaim; 30]") &&
    session.includes("MountedOwnerClass::AssemblyDofStrings") &&
    session.includes("MountedOwnerClass::MeshPreparationIndexVector") &&
    session.includes("MountedOwnerClass::MeshTriangulationWorkspaceVectors") &&
    session.includes("MountedOwnerClass::MeshEdgeIndexVectors") &&
    session.includes("process_owner_inventory_admits_exact_maximum_and_returns_exact_credit") &&
    session.includes("const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000") &&
    session.includes("job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG") &&
    session.includes("shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY]") &&
    session.includes("current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY]") &&
    session.includes("retiring: [Option<u16>; SESSION_SHELL_CAPACITY]") &&
    session.includes("free: [u16; SESSION_SHELL_CAPACITY]") &&
    session.includes("canonical_base_revision: [u8; 32]") &&
    session.includes("preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY]") &&
    session.includes("struct SnapshotAdmissionCursor") &&
    session.includes("fn step_one(&mut self, snapshot: &Fem2dSnapshot)") &&
    session.includes("credit_items: [usize; SESSION_SHELL_CAPACITY]") &&
    session.includes("credit_bytes: [usize; SESSION_SHELL_CAPACITY]") &&
    !session.includes("region.holes.iter().map") &&
    !session.includes("case.loads.iter().map") &&
    !session.includes("combination.terms.iter().map") &&
    session.includes("register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND") &&
    session.includes("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep") &&
    session.includes("fn terminal_is_empty(&self) -> bool") &&
    session.includes("pub fn maintenance_step(") &&
    session.includes("pub fn close_step(") &&
    session.includes("pub fn terminal_is_empty(") &&
    session.includes("visual_job_candidate: Option<Fem2dVisualJob>") &&
    session.includes("visual_rejected: Option<Fem2dVisualJob>") &&
    session.includes("visual_current: Option<Fem2dMountedVisualLease>") &&
    session.includes("visual_displaced: Option<Fem2dMountedVisualLease>") &&
    session.includes("fn publish_visual_candidate(&mut self, candidate: Fem2dMountedVisualLease) -> Result<(), Fem2dMountedVisualLease>") &&
    session.includes("return_to_registry_witness()") &&
    session.includes("witness.terminal_is_empty()") &&
    !!step &&
    (step.body.match(/\.step\(&mut cx\)/g) ?? []).length === 4 &&
    !step.body.includes("while ") &&
    step.body.includes("current_identity(self.identity.app_instance_id)") &&
    step.body.includes("commit_authority_matches(") &&
    step.body.includes("self.cancel.is_cancelled_now()") &&
    step.body.indexOf("commit_authority_matches(") < step.body.indexOf("JobStep::Done(output)") &&
    !!reconcile &&
    reconcile.body.includes("doc.take_snapshot_read()") &&
    reconcile.body.includes("Effect::CancelJob") &&
    reconcile.body.includes("Effect::SpawnJob") &&
    reconcile.body.indexOf("Effect::CancelJob") < reconcile.body.indexOf("Effect::SpawnJob") &&
    reconcile.body.includes("JobPlacement::Isolated") &&
    !reconcile.body.includes("run_to_completion") &&
    !reconcile.body.includes("block_on") &&
    editor.includes("fn pending_effects(") &&
    editor.includes("crate::editor::fem2d::session::reconcile(doc)") &&
    editor.includes("session::with_live_visual(doc.render_operation()") &&
    editor.includes("model_window::render_with_progress(doc.snapshot, camera, visual)") &&
    editor.includes("fn mounted_job_maintenance_step(") &&
    editor.includes("fn mounted_job_close_step(") &&
    editor.includes("fn mounted_jobs_terminal_is_empty(") &&
    editor.includes("fn mounted_job_prepare_snapshot_read(") &&
    !!render &&
    render.body.includes("progress.map(Fem2dMountedVisualLease::snapshot)") &&
    render.body.includes("fem2d_structure_layers(doc") &&
    !render.body.includes("fem2d_live_visual_layers") &&
    !render.body.includes("fem2d_region_triangles") &&
    !render.body.includes("serde_json") &&
    !render.body.includes("sort") &&
    !render.body.includes("collect") &&
    model.includes("pub struct Fem2dVisualJob") &&
    model.includes("Fem2dVisualJobStage::ReserveSnapshot") &&
    model.includes("canvas2d_snapshot_begin(") &&
    model.includes("self.output.admit_page(") &&
    model.includes("canvas2d_snapshot_seal(token)") &&
    model.includes("Fem2dVisualJobStage::OrderRegionKey") &&
    model.includes("Fem2dVisualJobStage::OrderElementKey") &&
    model.includes("fn order_field_one(&mut self, visual: &Fem2dLiveVisual)") &&
    model.includes("Fem2dVisualJobStage::BuildRegion") &&
    model.includes("fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize)") &&
    model.includes("mounted_visual_output_exact_maximum_plus_one_and_page_handback") &&
    model.includes("fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer") &&
    model.includes("fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid") &&
    model.includes("fem2d_visual_job_replay_accessibility_and_each_step_are_bounded") &&
    model.includes("assert!(packet_equal(&first, &second))") &&
    model.includes("started.elapsed().as_micros() < 8_000") &&
    classify.includes("self.advance_face_classification()") &&
    !classify.includes("collect(") &&
    !classify.includes("Vec<") &&
    mesh.includes("fn advance_face_classification(&mut self)") &&
    mesh.includes("triangulation.triangles.get(self.face_cursor)") &&
    mesh.includes("MeshJobStage::ReservePreparation") &&
    mesh.includes("MeshJobStage::ReserveEdgeAuthorities") &&
    mesh.includes("mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly") &&
    !!plugin &&
    plugin.body.indexOf("crate::editor::fem2d::session::initialize()") < plugin.body.indexOf('Plugin::<FemApps>::builder("fem")') &&
    glue.includes("✏️editor/🧵️session/🦀️.rs") &&
    jobs.includes("pub trait BoundedJob") &&
    jobs.includes("JobBody::Bounded(owner)") &&
    !jobs.includes("#[cfg(not(test))]") &&
    !jobs.includes("JobBody::ExplicitStateMachineRequired") &&
    analyses.includes("pub struct AssemblyJobConstruction") &&
    analyses.includes("AssemblyConstructionStage::ValidateNodePairs") &&
    analyses.includes("AssemblyConstructionStage::DiscoverDofs") &&
    analyses.includes("AssemblyConstructionStage::CommitDofOwner") &&
    analyses.includes("struct PendingElementBuild") &&
    analyses.includes("PendingElementBuildStage::ReserveIndices") &&
    analyses.includes("PendingElementBuildStage::ReservePositions") &&
    analyses.includes("PendingElementBuildStage::ReserveStiffnessCredit") &&
    analyses.includes("PendingElementBuildStage::AllocateStiffness") &&
    analyses.includes("PendingElementBuildStage::ObserveStiffnessBacking =>") &&
    analyses.includes("PendingElementBuildStage::AdmitStiffnessBacking") &&
    analyses.includes("mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing") &&
    analyses.includes("fn advance_element_build(&mut self)") &&
    analyses.includes("fn reclaim_element_owner(&mut self)") &&
    analyses.includes("mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn") &&
    !analyses.includes("pending_node_ids: Vec<String>") &&
    analyses.includes("pub struct AssemblyCsrBuild") &&
    session.includes("AssemblyJobConstruction::new_owned") &&
    session.includes("AssemblyCsrBuild::new") &&
    session.includes("PcgJobConstruction::new") &&
    session.includes("assembly_build.close_step(maximum_bytes)") &&
    !step.body.includes("AssemblyJob::new_owned(") &&
    !step.body.includes("into_full_matrix(") &&
    !step.body.includes("PcgJob::new(") &&
    reactor.includes("Event::JobProgress { job, .. }") &&
    reactor.includes("JOB_RENDER_BINDINGS.with") &&
    reactor.includes(".try_surface(binding.instance, surface)") &&
    store.includes("pub fn commit_authority_matches") &&
    store.includes("fn publish_authority") &&
    store.includes("pub struct SnapshotReadReturn") &&
    store.includes("pub fn return_to_registry_witness") &&
    snapshotAdmission >= 0 &&
    snapshotIssue > snapshotAdmission &&
    frameworkPlugin.slice(snapshotAdmission, snapshotIssue).includes("A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())") &&
    session.includes("mounted_revision_restart_keeps_cancel_before_spawn") &&
    session.includes("mounted_close_and_capacity_are_fixed_and_terminal_witnessed")
  );
}

export function toolJobLayoutColdRelayRetainedExact(layoutExport: string, layoutWasm: string, pluginHost: string): boolean {
  const relayCompletionStart = pluginHost.indexOf("enum GuestRelayCompletion");
  const relayCompletion = relayCompletionStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relayCompletionStart));
  const relaySenderStart = pluginHost.indexOf("struct GuestRelayCompletionSender");
  const relaySender = relaySenderStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relaySenderStart));
  const relayRunStart = pluginHost.indexOf("async fn run_job_on_worker");
  const relayRun = relayRunStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relayRunStart));
  return (
    layoutExport.includes("struct LayoutExportPublication") &&
    layoutExport.includes("bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES]") &&
    layoutExport.includes("writer: Option<RetainedJobPayloadWriter>") &&
    layoutExport.includes("fn encode_checkpoint(") &&
    layoutExport.includes("Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String>") &&
    layoutExport.includes("fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact()") &&
    layoutExport.includes("fn begin_close(&mut self)") &&
    layoutExport.includes("self.close_stage = LayoutExportCloseStage::Publication") &&
    layoutExport.includes("snapshot_placeholder: Option<Arc<LayoutSnapshot>>") &&
    layoutWasm.includes("WorkerJobSession::try_new(job, params)") &&
    layoutWasm.includes("const LAYOUT_EXPORT_REJECTION_SLOTS: usize = 8") &&
    layoutWasm.includes("ClosingSession { session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> }") &&
    layoutWasm.includes("session.try_step_on_caller()") &&
    layoutWasm.includes("session.take_outcome(ticket)") &&
    layoutWasm.includes("session.take_terminal()") &&
    layoutWasm.includes("owner.resume()") &&
    layoutWasm.includes("owner.begin_close()") &&
    layoutWasm.includes("outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES)") &&
    layoutWasm.includes("retain_layout_export_session(") &&
    layoutWasm.includes("retain_layout_export_rejection(") &&
    !layoutWasm.includes("session.step(&self.pool") &&
    relayCompletion !== undefined &&
    relayCompletion.body.includes("Stepped(Result<GuestRelayStepCompletion, TurnFault>)") &&
    relayCompletion.body.includes("Rejected(GuestRelayOwnedBytes)") &&
    relayCompletion.body.includes("Fault(GuestRelayOwnedBytes)") &&
    !relayCompletion.body.includes("Result<JobStep") &&
    !relayCompletion.body.includes("Rejected(Vec<u8>)") &&
    !relayCompletion.body.includes("Fault(Vec<u8>)") &&
    pluginHost.includes("Running { progress: Option<GuestRelayOwnedBytes> }") &&
    pluginHost.includes("Done { output: GuestRelayOwnedBytes }") &&
    pluginHost.includes("Failed { error: GuestRelayOwnedBytes }") &&
    relaySender !== undefined &&
    relaySender.body.includes("slot: Arc<GuestRelayCompletionSlot>") &&
    !relaySender.body.includes("oneshot") &&
    pluginHost.includes("struct GuestRelayPublication") &&
    pluginHost.includes("writer: Option<semio_framework_job::RetainedJobPayloadWriter>") &&
    pluginHost.includes("const GUEST_RELAY_MOUNTED_SLOTS: usize = 16") &&
    pluginHost.includes("struct GuestRelayMountedRegistry") &&
    pluginHost.includes("struct GuestRelayMountedOutput") &&
    pluginHost.includes("Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>") &&
    !pluginHost.includes("terminal_bytes: Vec<u8>") &&
    pluginHost.includes("next_generation: std::sync::atomic::AtomicU64") &&
    pluginHost.includes("u64::MAX => Some(0)") &&
    pluginHost.includes("fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact()") &&
    pluginHost.includes("assert_eq!(output.storage.as_ptr(), output_identity)") &&
    pluginHost.includes("fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source()") &&
    pluginHost.includes("impl std::future::Future for GuestRelayMountedFuture") &&
    pluginHost.includes("self.registry.pump(self.index, self.generation, context.waker())") &&
    pluginHost.includes("fn begin_close(&mut self)") &&
    pluginHost.includes("fn terminal_is_empty(&self) -> bool") &&
    relayRun !== undefined &&
    relayRun.body.includes("WorkerJobSession::try_new(relay, params)") &&
    relayRun.body.includes("self.relay_registry.mount(index, mounted_generation, owner)") &&
    !relayRun.body.includes("loop {") &&
    !relayRun.body.includes("session.step(")
  );
}

export function toolJobUniversalRetainedOwnershipExact(jobRuntime: string, nativeIo: string, wgpu: string, rustFiles: ReadonlyMap<string, string>): boolean {
  const mandatoryClose = [...rustFiles.entries()].every(([, raw]) => {
    const source = toolJobProductionProtocolSource(raw);
    for (const match of source.matchAll(/\bInteractiveJob\s+for\s+[^\{]+\{/g)) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block || !block.body.includes("fn begin_close(") || !block.body.includes("fn close_step(") || !block.body.includes("fn terminal_is_empty(")) return false;
    }
    return true;
  });
  const production = [...rustFiles.values()].map(toolJobProductionProtocolSource).join("\n");
  const layoutExport = [...rustFiles.values()].find((source) => source.includes("pub struct LayoutExportJob"));
  const layoutWasm = [...rustFiles.values()].find((source) => source.includes("pub struct LayoutExportOperation"));
  const pluginHost = [...rustFiles.values()].find((source) => source.includes("struct GuestColdRelayJob"));
  const layoutColdRelay = layoutExport === undefined && layoutWasm === undefined && pluginHost === undefined ? true : layoutExport !== undefined && layoutWasm !== undefined && pluginHost !== undefined && toolJobLayoutColdRelayRetainedExact(layoutExport, layoutWasm, pluginHost);
  const animateRaw = [...rustFiles.values()].find((source) => source.includes("pub struct PresentationEnvelopeMaterializeHandle"));
  const animate = animateRaw === undefined ? undefined : toolJobProductionProtocolSource(animateRaw);
  const futureStart = wgpu.indexOf("impl std::future::Future for RendererIoHandle");
  const future = futureStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", futureStart));
  const presenterStart = wgpu.indexOf("pub(crate) fn present_step(&mut self)");
  const presenter = presenterStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", presenterStart));
  const closeCounterStart = jobRuntime.indexOf("fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release()");
  const closeCounter = closeCounterStart < 0 ? undefined : toolJobRustBlock(jobRuntime, jobRuntime.indexOf("{", closeCounterStart));
  return (
    mandatoryClose &&
    layoutColdRelay &&
    !production.includes("WorkerJobSession::new") &&
    !/semio_framework_job::run_(?:to_completion|on_worker|on_worker_async)\s*\(/.test(production) &&
    !/semio_framework_job::drive_step\s*\(/.test(production) &&
    !/use\s+semio_framework_job::\{[^}]*\bdrive_step\b/s.test(production) &&
    !/\bjob::drive_step\s*\(/.test(production) &&
    !production.includes("Receiver<RuntimeFrameResult>") &&
    production.includes("session: Option<semio_framework_job::WorkerJobSession<ActiveFrameBuild>>") &&
    production.includes("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>") &&
    jobRuntime.includes("pub trait InteractiveJob: Send") &&
    jobRuntime.includes("fn begin_close(&mut self);") &&
    jobRuntime.includes("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)") &&
    jobRuntime.includes("fn terminal_is_empty(&self) -> bool;") &&
    jobRuntime.includes("preadmitted_fault: Option<RetainedJobPayload>") &&
    jobRuntime.includes("preadmitted_static_payload(&payload_ledger") &&
    jobRuntime.includes('authority.preadmitted_fault.take().expect("worker panic retains its pre-admitted terminal fault page")') &&
    jobRuntime.includes("fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent()") &&
    jobRuntime.includes("assert_eq!(returned_fault_pointer, preadmitted_fault_pointer)") &&
    closeCounter !== undefined &&
    (closeCounter.body.match(/batch\.close_step\(1, JOB_PAYLOAD_PAGE_BYTES\)[\s\S]{0,160}?released_items:\s*0/g)?.length ?? 0) >= 2 &&
    jobRuntime.includes("child begin-close transfers control without claiming an owner release") &&
    !jobRuntime.includes("fn retained_static_payload(") &&
    jobRuntime.includes("pub fn try_step_on_caller(&self)") &&
    jobRuntime.includes("pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>>") &&
    jobRuntime.includes("fault_source: ManuallyDrop<Option<JobPayloadPageSource>>") &&
    nativeIo.includes("pub struct NativePathSet") &&
    nativeIo.includes("pub struct NativeModifiedSet") &&
    nativeIo.includes("entries: std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]>") &&
    nativeIo.includes("entries: std::mem::ManuallyDrop<[Option<(PathBuf, std::time::SystemTime)>; NATIVE_IO_PATH_CAPACITY]>") &&
    nativeIo.includes("fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact()") &&
    nativeIo.includes("assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer)") &&
    nativeIo.includes("ReadingModified { paths: NativePathSet, modified: NativeModifiedSet }") &&
    nativeIo.includes("ClosingScanFault") &&
    nativeIo.includes("ClosingModifiedFault") &&
    nativeIo.includes("pub fn retained_request_backing_identity(&self) -> Option<*const u8>") &&
    !nativeIo.includes("Paths(Vec<PathBuf>)") &&
    !nativeIo.includes("Modified(Vec<") &&
    !nativeIo.includes("WriteBytes") &&
    wgpu.includes("const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job::WORKER_JOB_SESSION_SLOTS + 1") &&
    wgpu.includes("generation: std::sync::atomic::AtomicU64") &&
    wgpu.includes("cancel_requested: std::sync::atomic::AtomicBool") &&
    wgpu.includes("fn pump_renderer_io_sessions(maximum_sessions: usize)") &&
    wgpu.includes("fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact()") &&
    wgpu.includes("assert_eq!(returned_pointer, plus_one_request_pointer)") &&
    wgpu.includes("assert_eq!(pump_renderer_io_sessions(0), 0)") &&
    wgpu.includes('assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity")') &&
    wgpu.includes("session: Option<semio_framework_job::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>") &&
    wgpu.includes("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>>") &&
    wgpu.includes("checked_out_job_mut()?.take_packet()") &&
    wgpu.includes("slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation") &&
    future !== undefined &&
    future.body.includes("renderer_io_with_node") &&
    future.body.includes("node.waker = Some(cx.waker().clone())") &&
    !future.body.includes("try_submit_step") &&
    !future.body.includes("close_step(") &&
    !future.body.includes("pump_one(") &&
    presenter !== undefined &&
    presenter.body.includes("pump_renderer_io_sessions(1)") &&
    presenter.body.includes("pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)") &&
    animate !== undefined &&
    animate.includes("rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<PresentationEnvelopeMaterializeJob>>>") &&
    animate.includes("pending: Option<semio_framework_job::WorkerJobTicket>") &&
    animate.includes("retained_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>") &&
    animate.includes("fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>") &&
    animate.includes("retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>") &&
    animate.includes("WorkerJobSession::try_new(job, params)") &&
    animateRaw !== undefined &&
    animateRaw.includes("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack") &&
    animateRaw.includes("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output") &&
    animateRaw.includes("registry.close_step(operation, generation, &pool, 0, 0)") &&
    !animate.includes("oneshot::Receiver")
  );
}

//#region 🧮️P6hFemNumericalMicrocursors
/** 🧮️ Phase-6h FEM numerical hot-loop, fixed-owner, identity and hostile-law contract. */
export function toolJobFemNumericalMicrocursorExact(sparse: string, mesh: string, analyses: string, model: string, elements: string, session: string, runtime: string): boolean {
  const namedBlock = (source: string, signature: string) => {
    const start = source.indexOf(signature);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const ordered = (source: string, tokens: readonly string[]) => {
    let cursor = 0;
    for (const token of tokens) {
      const next = source.indexOf(token, cursor);
      if (next < 0) return false;
      cursor = next + token.length;
    }
    return true;
  };
  const loopFree = (block: { body: string } | undefined) => !!block && !/\b(?:while|for)\b/.test(block.body);
  const ldltStart = sparse.indexOf("impl InteractiveJob for LdltJob");
  const ldltOpen = ldltStart < 0 ? -1 : sparse.indexOf("{", ldltStart);
  const ldlt = ldltOpen < 0 ? undefined : toolJobRustBlock(sparse, ldltOpen);
  const subspaceStart = sparse.indexOf("impl InteractiveJob for SubspaceIterationJob");
  const subspaceOpen = subspaceStart < 0 ? -1 : sparse.indexOf("{", subspaceStart);
  const subspace = subspaceOpen < 0 ? undefined : toolJobRustBlock(sparse, subspaceOpen);
  const constraintStart = mesh.lastIndexOf("MeshJobStage::ConstrainBoundary =>");
  const constraintEnd = constraintStart < 0 ? -1 : mesh.indexOf("MeshJobStage::ReservePointIndex =>", constraintStart);
  const constraint = constraintEnd < 0 ? "" : mesh.slice(constraintStart, constraintEnd);
  const elementStart = analyses.indexOf("fn advance_element_build(&mut self)");
  const elementOpen = elementStart < 0 ? -1 : analyses.indexOf("{", elementStart);
  const element = elementOpen < 0 ? undefined : toolJobRustBlock(analyses, elementOpen);
  const ldltCheckpoint = namedBlock(sparse, "fn advance_checkpoint_entry(state: &LdltCheckpoint");
  const ldltColumn = namedBlock(sparse, "fn advance_column_microcursor(&mut self)");
  const subspaceCheckpoint = namedBlock(sparse, "fn advance_checkpoint_entry(state: &SubspaceCheckpoint");
  const ldltOutput = namedBlock(sparse, "fn advance_output_entry(");
  const subspacePreview = namedBlock(sparse, "fn advance_preview_entry(");
  const ldltRestore = namedBlock(sparse, "fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault>");
  const secondRestoreStart = sparse.indexOf("fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault>", (ldltRestore?.end ?? 0) + 1);
  const subspaceRestore = secondRestoreStart < 0 ? undefined : toolJobRustBlock(sparse, sparse.indexOf("{", secondRestoreStart));
  const ldltRestoreImpl = namedBlock(sparse, "impl LdltRestoreCursor");
  const subspaceRestoreImpl = namedBlock(sparse, "impl SubspaceRestoreCursor");
  const restoreHelpers = ["fn restore_u32_entry(", "fn restore_usize_entry(", "fn restore_f64_entry(", "fn restore_pair_entry(", "fn restore_matrix_entry("].map((signature) => namedBlock(sparse, signature));
  const ownerWriters = ["fn advance_u32_owner(", "fn advance_u64_owner(", "fn advance_u64_values(", "fn advance_f64_owner(", "fn advance_pair_owner(", "fn advance_matrix_owner("].map((signature) => namedBlock(sparse, signature));
  const meshStep = namedBlock(mesh, "impl InteractiveJob for MeshJob");
  const meshPreparation = namedBlock(mesh, "impl MeshInputPreparation");
  const meshMountedBegin = namedBlock(mesh, "fn begin_mounted(points: Vec<[f64; 2]>");
  const meshMountedInitialization = namedBlock(mesh, "fn advance_mounted_initialization(&mut self)");
  const meshFinishInsertion = namedBlock(mesh, "fn advance_finish_insertion(&mut self)");
  const meshPayload = namedBlock(mesh, "fn advance_mesh_payload(&mut self)");
  const meshPublication = namedBlock(mesh, "fn advance_publication(&mut self");
  const meshConstraint = namedBlock(mesh, "fn advance_constraint_recovery(&mut self)");
  const faceCursor = namedBlock(mesh, "fn advance_face_classification(&mut self)");
  const meshLaw = namedBlock(mesh, "fn p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies()");
  const meshCursorLaw = namedBlock(mesh, "fn p6h_mounted_mesh_preparation_initialization_finish_publication_interrupt_replay_timing_and_close()");
  const elementLaw = namedBlock(analyses, "fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws()");
  const assemblyCursorLaw = namedBlock(analyses, "fn p6h_owned_assembly_lookup_partition_scan_transfer_interrupt_replay_and_timing()");
  const elementAdmissionLaw = namedBlock(analyses, "fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing()");
  const partitionMerge = namedBlock(analyses, "fn advance_partition_merge(&mut self");
  const subspaceConstructor = namedBlock(sparse, "pub fn new(operation: Operation, k_factor: LdltFactor");
  const identityMatches = namedBlock(sparse, "fn matches(self, operation: Operation) -> bool");
  const stagedWriterStart = runtime.indexOf("impl RetainedJobPayloadWriter");
  const stagedCloseStart = runtime.indexOf("pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep", stagedWriterStart);
  const stagedClose = stagedCloseStart < 0 ? undefined : toolJobRustBlock(runtime, runtime.indexOf("{", stagedCloseStart));
  const stagedBegin = namedBlock(runtime, "pub fn begin_staged_page(&mut self");
  const stagedCommit = namedBlock(runtime, "pub fn commit_staged_page(&mut self)");
  const ldltStages = ["ReserveColumn", "SourceEntry", "ContributorLookup", "ContributorEntry", "PivotRead", "DiagonalCommit", "EmitRow", "PublishColumn", "CompleteColumn"];
  const subspaceStages = [
    "ReserveIteration",
    "ApplyOperatorColumnRow",
    "FactorForwardEntry",
    "FactorDiagonalEntry",
    "FactorBackwardEntry",
    "OrthogonalizePairElement",
    "NormalizeColumnElement",
    "ProjectedMatrixCellEntry",
    "JacobiFindPairCell",
    "JacobiRotateCell",
    "JacobiConvergenceCell",
    "ModeSortCompare",
    "ModePermuteElement",
    "ResidualColumnRow",
    "ConvergenceMode",
    "PublishIteration",
  ];
  const constraintStages = [
    "ReserveConstraintWorkspace",
    "IndexTriangleEdge",
    "SearchConstraintEdge",
    "ClassifyIntersection",
    "SelectDeterministicFlip",
    "ValidateFlip",
    "ApplyFlip",
    "RetireFormerEdge",
    "PublishConstraintProgress",
    "ConstraintComplete",
  ];
  const elementStages = [
    "ReferenceQuadraturePoint",
    "ShapeFunctionDerivativeScalar",
    "JacobianCell",
    "DeterminantInverseCell",
    "StrainDisplacementCell",
    "ConstitutiveCell",
    "LocalStiffnessMultiplyCell",
    "BodyTractionLoadCell",
    "LocalToGlobalTripletCell",
  ];
  return (
    ldltStages.every((stage) => sparse.includes(stage)) &&
    subspaceStages.every((stage) => sparse.includes(stage)) &&
    constraintStages.every((stage) => mesh.includes(stage)) &&
    elementStages.every((stage) => analyses.includes(stage)) &&
    sparse.includes("const LDLT_MAXIMUM_ORDER: usize = 40") &&
    sparse.includes("const SUBSPACE_MAXIMUM_ORDER: usize = 40") &&
    sparse.includes("const SUBSPACE_MAXIMUM_COLUMNS: usize = 40") &&
    sparse.includes("struct NumericalCheckpointIdentity") &&
    sparse.includes("revision: operation.base_revision.0") &&
    sparse.includes("generation: operation.generation.0") &&
    sparse.includes("seed: operation.seed") &&
    sparse.includes('b"FEMLCP1\\0"') &&
    sparse.includes('b"FEMSCP1\\0"') &&
    sparse.includes("NUMERICAL_CHECKPOINT_VERSION") &&
    sparse.includes("struct NumericalPageCursor") &&
    sparse.includes("pub struct LdltRestoreCursor") &&
    sparse.includes("pub struct SubspaceRestoreCursor") &&
    sparse.includes("fn decode_page_entry(&mut self, bytes: &[u8])") &&
    sparse.includes("pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)") &&
    sparse.includes("pub fn terminal_is_empty(&self) -> bool") &&
    sparse.includes("reserve_lane: u8") &&
    sparse.includes("fn advance_reservation(&mut self)") &&
    sparse.includes("initialization_cursor: usize") &&
    sparse.includes("retiring_work: Option<SubspaceWork>") &&
    sparse.includes("fn advance_preview_entry(state: &mut SubspaceCheckpoint") &&
    sparse.includes("fn advance_checkpoint_entry(state: &LdltCheckpoint") &&
    sparse.includes("fn advance_checkpoint_entry(state: &SubspaceCheckpoint") &&
    sparse.includes("begin_staged_page(context)") &&
    sparse.includes("JobPayloadStream::CheckpointState") &&
    sparse.includes("terminal_writer: Option<RetainedJobPayloadWriter>") &&
    sparse.includes("NumericalCheckpointFault::Truncated") &&
    sparse.includes("interrupted_restore.close_step(1, usize::MAX)") &&
    !sparse.includes("publication_bytes") &&
    !sparse.includes("preview_bytes") &&
    sparse.includes("p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity") &&
    sparse.includes("p6h_subspace_cancellation_is_observed_at_every_nested_stage_and_worker_replay_is_exact") &&
    mesh.includes("indexed_constraint_edges: Vec<IndexedConstraintEdge>") &&
    mesh.includes("p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies") &&
    analyses.includes("p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws") &&
    elements.includes("p6h_mounted_element_fixed_schema_cells_match_batch_and_reject_maximum_plus_one") &&
    model.includes("fn mounted_stiffness_cell(&self, _ctx: &ElementContext, _row: usize, _column: usize) -> Option<f64>") &&
    (elements.match(/fn mounted_stiffness_cell\(/g) ?? []).length === 3 &&
    session.includes("MountedOwnerClaim { class: MountedOwnerClass::MeshEdgeIndexVectors, roots: 3") &&
    !!ldlt &&
    ordered(ldlt.body, ["context.should_yield()", "context.consume_fuel(1)", "writer.staged_page_len().is_none()", "return match writer.begin_staged_page(context)", "advance_checkpoint_entry"]) &&
    !ldlt.body.includes("ldlt_column(") &&
    !ldlt.body.includes("while ") &&
    !ldlt.body.includes("unwrap_or_default") &&
    !ldlt.body.includes("checkpoint_bytes()") &&
    !ldlt.body.includes("serde_json") &&
    !ldlt.body.includes("expect(") &&
    !!ldltColumn && loopFree(ldltColumn) && ldltColumn.body.includes("lookup_lower") && ldltColumn.body.includes("lookup_upper") && ldltColumn.body.includes("lookup_mid") && ldltColumn.body.includes("lookup_comparison") && !ldltColumn.body.includes("binary_search") &&
    !!subspace &&
    ordered(subspace.body, ["context.should_yield()", "context.consume_fuel(1)", "writer.staged_page_len().is_none()", "return match writer.begin_staged_page(context)", "advance_checkpoint_entry"]) &&
    !subspace.body.includes("self.iterate(") &&
    !subspace.body.includes("while ") &&
    !subspace.body.includes("serde_json::to_vec(&self.preview())") &&
    !subspace.body.includes("checkpoint_bytes()") &&
    !subspace.body.includes("expect(") &&
    !!subspaceConstructor && !subspaceConstructor.body.includes(".iter().all") && subspaceConstructor.body.includes("factor_validation_complete: admission_fault") && subspace.body.includes("fem.subspace.validate-factor-owner") && ordered(subspace.body, ["!self.state.factor_validation_complete", "context.should_yield()", "context.consume_fuel(1)", "self.state.k_factor.l_cols.get(self.state.factor_validation_cursor)", "self.state.factor_validation_cursor += 1"]) &&
    constraint.includes("advance_constraint_recovery()") &&
    !constraint.includes("recover_constraint(") &&
    !constraint.includes(".edges()") &&
    !!element &&
    element.body.includes("mounted_stiffness_cell") &&
    !element.body.includes("stiffness_global(") &&
    ownerWriters.every((block) => loopFree(block) && block?.body.includes("cursor.item += 1") && block.body.includes("return Ok(false)")) &&
    restoreHelpers.every(loopFree) &&
    loopFree(ldltCheckpoint) &&
    loopFree(subspaceCheckpoint) &&
    loopFree(ldltOutput) &&
    loopFree(subspacePreview) &&
    !!ldltCheckpoint && ordered(ldltCheckpoint.body, ["advance_numerical_page_header", "advance_", "writer.commit_staged_page()", "cursor.item = 0", "cursor.owner = 0"]) &&
    !!subspaceCheckpoint && ordered(subspaceCheckpoint.body, ["advance_numerical_page_header", "advance_", "writer.commit_staged_page()", "cursor.item = 0", "cursor.owner = 0"]) &&
    !!ldltRestore && !ldltRestore.body.includes("[0u8;") && !ldltRestore.body.includes("copy_from_slice(source)") && ldltRestore.body.includes("page_entry") &&
    !!subspaceRestore && !subspaceRestore.body.includes("[0u8;") && !subspaceRestore.body.includes("copy_from_slice(source)") && subspaceRestore.body.includes("page_entry") &&
    !!ldltRestoreImpl && !!subspaceRestoreImpl && !ldltRestoreImpl.body.includes("copy_from_slice(source)") && !subspaceRestoreImpl.body.includes("copy_from_slice(source)") && ldltRestoreImpl.body.includes("payload.take()") && subspaceRestoreImpl.body.includes("payload.take()") &&
    !!identityMatches && identityMatches.body.includes("self == Self::from_operation(operation)") &&
    !!meshStep && loopFree(meshStep) && ordered(meshStep.body, ["context.should_yield()", "context.consume_fuel(1)", "match self.stage"]) && meshStep.body.includes("MeshJobStage::CountInput") && meshStep.body.includes("triangulation.insert_next();") &&
    !!meshPreparation && loopFree(meshPreparation) && meshPreparation.body.includes("point_lookup_cursor") && meshPreparation.body.includes("grid_polygon") && meshPreparation.body.includes("grid_edge") && !meshPreparation.body.includes(".iter().find") && !meshPreparation.body.includes("point_in_polygon(") && !meshPreparation.body.includes("holes.iter()") &&
    !!meshMountedBegin && loopFree(meshMountedBegin) && !meshMountedBegin.body.includes(".iter()") && !meshMountedBegin.body.includes("sort_by") && !meshMountedBegin.body.includes("sort_unstable") && meshMountedBegin.body.includes("MountedTriangulationStage::BoundsPoint") &&
    !!meshMountedInitialization && loopFree(meshMountedInitialization) && !meshMountedInitialization.body.includes("sort_by") && !meshMountedInitialization.body.includes("extend(") && meshMountedInitialization.body.includes("sort_inner") && meshMountedInitialization.body.includes("insertion_order.swap") &&
    !!meshFinishInsertion && loopFree(meshFinishInsertion) && !meshFinishInsertion.body.includes(".retain(") && !meshFinishInsertion.body.includes("sort_unstable") && meshFinishInsertion.body.includes("finish.read") && meshFinishInsertion.body.includes("finish.write") && meshFinishInsertion.body.includes("triangles.swap") &&
    !!meshPayload && loopFree(meshPayload) && !meshPayload.body.includes("Vec::with_capacity") && !meshPayload.body.includes("extend_from_slice") && meshPayload.body.includes("write_staged") && meshPayload.body.includes("commit_staged_page") && meshPayload.body.includes("MeshPayloadStage::PointCoordinate") && meshPayload.body.includes("MeshPayloadStage::TriangleIndex") &&
    !!meshPublication && loopFree(meshPublication) && ordered(meshPublication.body, ["RetainedJobPayloadWriter::new", "writer.staged_page_len().is_none()", "writer.begin_staged_page(context)", "self.advance_mesh_payload()", "writer.finish()"]) &&
    !!meshConstraint && loopFree(meshConstraint) && meshConstraint.body.includes("constraint_retire_adjacency_cursor") && meshConstraint.body.includes("slot.adjacent[self.constraint_retire_adjacency_cursor]") && ordered(meshConstraint.body, ["self.constraint_retire_adjacency_cursor += 1", "slot.active =", "self.constraint_retire_cursor += 1"]) &&
    !!faceCursor && loopFree(faceCursor) && faceCursor.body.includes("FaceClassificationStage::HoleEdge") && faceCursor.body.includes("self.point_index.get(cursor.lookup)") &&
    !!meshLaw && meshLaw.body.includes("job.step(&mut deadline)") && meshLaw.body.includes("job.step(&mut stale)") && meshLaw.body.includes("job.step(&mut cancelled)") && meshLaw.body.includes("maximum_micros < 8_000") && !meshLaw.body.includes("job.advance_constraint_recovery()") &&
    !!meshCursorLaw && meshCursorLaw.body.includes("MeshJob::new_bounded") && meshCursorLaw.body.includes("preparation_lookup_seen") && meshCursorLaw.body.includes("MountedTriangulationStage::OrderInsertion") && meshCursorLaw.body.includes("TriangulationFinishStage::OrderTriangles") && meshCursorLaw.body.includes("MeshPayloadStage::CommitPage") && meshCursorLaw.body.includes("StepBudget::new(1, 0)") && meshCursorLaw.body.includes("InteractiveJob::close_step(&mut interrupted, 1, 0)") && meshCursorLaw.body.includes("first.1.max(second.1) < 8_000") &&
    !!elementLaw && elementLaw.body.includes("AssemblyJobConstruction::new_owned") && elementLaw.body.includes("Bar2") && elementLaw.body.includes("BeamEb2") && elementLaw.body.includes("Tri3Cst") && elementLaw.body.includes("mounted stiffness mismatch") && elementLaw.body.includes("maximum_micros < 8_000") &&
    element.body.includes("lookup_cursor") && element.body.includes("PendingElementBuildStage::PublishIndex") && element.body.includes("PendingElementBuildStage::PublishPosition") && !element.body.includes("dof_map.get(") && !element.body.includes("nodes.iter().find") &&
    !!partitionMerge && loopFree(partitionMerge) && partitionMerge.body.includes("merge_scan_partition") && partitionMerge.body.includes("merge_candidate") && !partitionMerge.body.includes(".iter()") && !partitionMerge.body.includes("filter_map") && !partitionMerge.body.includes("min_by_key") && !analyses.includes("fn next_partition_triplet") &&
    !!assemblyCursorLaw && assemblyCursorLaw.body.includes("AssemblyJobConstruction::new_owned") && assemblyCursorLaw.body.includes("merge_scan_partition") && assemblyCursorLaw.body.includes("lookup_cursor") && assemblyCursorLaw.body.includes("StepBudget::new(1, 0)") && assemblyCursorLaw.body.includes("first.2.max(second.2) < 8_000") &&
    !!elementAdmissionLaw && elementAdmissionLaw.body.includes("reserve_exact_owner_page") && elementAdmissionLaw.body.includes("observed - 1") && elementAdmissionLaw.body.includes("rejected.as_ptr()") && !elementAdmissionLaw.body.includes("include_str!") &&
    !!stagedBegin && stagedBegin.body.includes("self.admit_page(cx)?.stage()") &&
    !!stagedCommit && stagedCommit.body.includes("self.staged.take()") &&
    !!stagedClose && ordered(stagedClose.body, ["self.staged.as_ref()", "ledger.release(stream)", "self.rejected.is_some()", "payload.close_step"])
  );
}


//#endregion 🧮️P6hFemNumericalMicrocursors
