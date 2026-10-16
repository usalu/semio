/** 🛢️ Interactivity audit: prepared raster, shard, store sync, database I/O, capability open and catalog read sources; the catalog lifecycle, history, CLI and MCP sources live in the child `🌱️database-lifecycle`.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { policyTestModSpans, policyLineInTestMod, interactivityCfgTestItemSpans } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { POLICY_RUST_TEST_EVIDENCE_BOUNDARY } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

//#region 🔖️InteractivityAudit
export function interactivityPreparedRasterProducerFailures(preparedSource: string, drawSource: string, gpuSource: string, canvasSource: string, interpreterSource: string, glueSource: string, hostSource: string): string[] {
  const prepared = interactivityProductionSource(preparedSource);
  const canvas = interactivityProductionSource(canvasSource);
  const canvasQueue = canvas.slice(canvas.indexOf("pub(crate) fn queue_canvas_image_upload_with"), canvas.indexOf("/** Clamps checkerboard"));
  const interpreter = interactivityProductionSource(interpreterSource);
  const interpreterSvg = interpreter.slice(interpreter.indexOf("fn resolve_ui_image_svg"), interpreter.indexOf("fn resolve_ui_image_url"));
  const interpreterResolve = interpreter.slice(interpreter.indexOf("pub(crate) fn resolve_ui_image"), interpreter.indexOf("fn object_contain_rect"));
  const glue = interactivityProductionSource(glueSource);
  const host = interactivityProductionSource(hostSource);
  const failures: string[] = [];
  for (const cap of [
    "PREPARED_RASTER_PAGE_BYTES: usize = 16 * 1024",
    "PREPARED_RASTER_ITEM_BYTES: usize = 16 * 1024 * 1024",
    "PREPARED_RASTER_PRODUCER_CAPACITY: usize = 256",
    "PREPARED_RASTER_PRODUCER_ITEMS: usize = 4_096",
    "PREPARED_RASTER_PRODUCER_BYTES: usize = 32 * 1024 * 1024",
  ]) if (!prepared.includes(cap)) failures.push(`prepared raster fixed credit changed or disappeared: ${cap}`);
  if (!prepared.includes("slots: Vec<PreparedRasterPage>") || !prepared.includes("slots: Vec::with_capacity(claim.page_capacity)") || !prepared.includes("backing: Vec<u8>") || !prepared.includes("retained_source: Vec<u8>") || !prepared.includes("pages.backing = std::mem::take(&mut self.source)") || prepared.includes("source.split_off(") || prepared.includes("pixels.to_vec()") || prepared.includes("source.clone()")) failures.push("prepared raster page publication does not retain one exact codec backing and its encoded source owner");
  if (!prepared.includes("pub fn try_reserve_source") || !prepared.includes("pub fn claim_with_retained") || !prepared.includes("source_bytes.checked_mul(2)") || !prepared.includes("source_peak_bytes.checked_add(key_bytes)") || !prepared.includes("page_capacity.checked_add(8)") || !prepared.includes("key.capacity().checked_mul(2)") || !prepared.includes("page_capacity.checked_mul(size_of::<PreparedRasterPage>())") || !prepared.includes("self.source_peak_bytes.checked_add(byte_len)") || !prepared.includes("retained_source.capacity() > self.source_bytes") || !prepared.includes("ledger.resize(credit, items, bytes)")) failures.push("prepared raster simultaneous source, decoded backing, key, page-slot, item, and byte owners are not jointly credited before materialization");
  if (!prepared.includes("retire_backing_step") || !prepared.includes("self.backing = Vec::new()") || !prepared.includes("self.source = Vec::new()") || !prepared.includes("self.retained_source = Vec::new()") || !prepared.includes("self.key = String::new()") || !prepared.includes("self.slots.capacity() == 0") || !prepared.includes("self.backing.capacity() == 0") || !prepared.includes("self.retained_source.capacity() == 0") || !prepared.includes("self.key_released && self.credit.is_none()")) failures.push("prepared raster backing/source/key/page owners can release after ledger credit or outside retained retirement");
  if (!prepared.includes("if self.frame_generation != Some(expected_generation)") || prepared.indexOf("if self.frame_generation != Some(expected_generation)") > prepared.indexOf("pages.backing = std::mem::take(&mut self.source)")) failures.push("stale prepared raster generation can consume source bytes");
  if (!prepared.includes("raster_producers.front_mut()") || !prepared.includes("return StepOutcome::Yield") || !prepared.includes("producer.begin_close()") || !prepared.includes("producer.close_step()")) failures.push("prepared raster job does not advance or retire one retained producer opportunity");
  const producerStep = prepared.slice(prepared.indexOf("impl InteractiveJob for PreparedRenderJob"));
  if (!producerStep.includes("if cx.should_yield()") || !producerStep.includes("cx.consume_fuel(1)") || !producerStep.includes("producer.step(cx.generation().0)") || producerStep.indexOf("if cx.should_yield()") > producerStep.indexOf("producer.step(cx.generation().0)") || producerStep.indexOf("cx.consume_fuel(1)") > producerStep.indexOf("producer.step(cx.generation().0)")) failures.push("prepared raster page advancement bypasses StepContext fuel or deadline");
  if (!canvas.includes("slots: Box<[Option<PreparedRasterProducer>; RASTER_UPLOADS_PER_SURFACE_CAPACITY]>") || !canvas.includes("fn checkout_front(&mut self)") || !canvas.includes("struct PendingRasterCheckedOut") || !canvas.includes("surface.queue.hand_back(self.queue)")) failures.push("Canvas raster FIFO checkout lacks exact retained handback");
  const reserveAt = canvasQueue.indexOf("PreparedRasterReservation::try_reserve_source");
  const dimensionAt = canvasQueue.indexOf("dimensions()");
  const claimAt = canvasQueue.indexOf("reservation.claim_with_retained(width, height, retained_source)");
  const decodeAt = canvasQueue.indexOf("decode(&retained_source)");
  if (reserveAt < 0 || dimensionAt < 0 || claimAt < 0 || decodeAt < 0 || !(reserveAt < dimensionAt && dimensionAt < claimAt && claimAt < decodeAt) || canvasQueue.includes("digest_pixels(")) failures.push("Canvas raster reservation/claim is not before dimension, hash, and RGBA materialization");
  if (canvasQueue.includes("pixels[..expected].to_vec()") || canvas.includes("pending_raster_uploads: Vec") || canvas.includes("PendingRasterUpload {") || canvasQueue.includes("pixels.to_vec()")) failures.push("Canvas raster path reintroduced a contiguous prepared-pixel clone or growable queue");
  if (!canvasQueue.includes("reservation.finalize(pixels, retained_source, width, height)") || !canvasQueue.includes("reject_with_retained") || canvasQueue.includes("PreparedRasterReservation::published_key") || canvasQueue.includes("let src_key") || !glue.includes("producer.bind_frame_generation(input.preview_generation)") || !glue.includes("input.try_push_raster_producer(producer)") || !glue.includes("checked_out.take()")) failures.push("Canvas producer does not move exact source/key owners once with checkout and frame generation witnesses");
  const svgQueueAt = interpreterSvg.indexOf("queue_canvas_image_upload_with");
  const svgParseAt = interpreterSvg.indexOf("parse_svg_data_url_bytes(src)");
  if (interpreter.includes("encode_rgba_png_data_url") || interpreter.includes("pixels.to_vec()") || interpreter.includes("ui_image_digest") || interpreter.includes("UI_IMAGE_SVG_CACHE") || !interpreter.includes("queue_canvas_image_upload_with") || !interpreter.includes("queue_canvas_image_upload_sized") || !interpreter.includes("UI_IMAGE_URL_CACHE.with(|cell| cell.borrow().get(id).cloned())") || svgQueueAt < 0 || svgParseAt < 0 || svgQueueAt > svgParseAt || !interpreterSvg.includes("Ok((measured.0, measured.1, bytes))") || !interpreterSvg.includes("|bytes|") || interpreterResolve.includes("parse_svg_data_url_bytes(src)")) failures.push("Interpreter raster routes parse, digest, clone, or drop source ownership before the stable prepared-key reservation");
  if (!canvas.includes("pub struct PendingRasterAuthorityClose") || !canvas.includes("PENDING_RASTER_CLOSE_OWNER") || !canvas.includes("pub fn begin_pending_raster_authority_close") || !host.includes("raster_uploads: Option<crate::scenes::PendingRasterAuthorityClose>") || !host.includes("begin_pending_raster_authority_close()") || !host.includes("raster_uploads.terminal_is_empty()")) failures.push("pending raster authority is not registered in exact realm close ordering");
  if (!gpuSource.includes("PreparedRenderUpload::RasterPages") || !gpuSource.includes("pixels.frame_generation() != packet.preview_generation()") || !drawSource.includes("Self::Pages(pixels) => pixels.page_for_row(row)")) failures.push("paged producer handoff is not consumed directly by the accepted raster upload cursor");
  const fixtures = [
    "paged_raster_producer_advances_one_page_and_moves_page_identity",
    "stale_generation_does_not_consume_a_prepared_raster_page",
    "raster_item_bytes_exact_and_plus_one_are_claimed_before_materialization",
    "raster_ledger_exact_item_and_generation_slot_caps_reject_plus_one",
    "raster_credit_epoch_rejects_aba_and_cancel_retires_one_owner_per_grant",
    "zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation",
    "pending_raster_ring_is_fixed_fifo_and_returns_cap_plus_one_owner",
    "checked_out_drop_hands_back_exact_fifo_owner_and_rejects_aba",
    "admission_saturation_runs_no_hash_dimension_or_pixel_materialization",
    "realm_close_retires_pending_rasters_before_terminal_and_allows_clean_reopen_fixture",
    "raster_simultaneous_source_decode_peak_exact_and_plus_one",
    "retained_codec_source_moves_once_and_retires_one_page_per_governed_step",
    "inline_svg_saturation_rejects_before_parse_or_source_copy",
  ];
  for (const fixture of fixtures) if (!preparedSource.includes(fixture) && !canvasSource.includes(fixture) && !interpreterSource.includes(fixture)) failures.push(`prepared raster fixture missing ${fixture}`);
  return failures;
}



export function interactivityShardExecutorFailures(executorSource: string, shardSource: string): string[] {
  const executor = executorSource.split("#[cfg(test)]")[0] ?? executorSource;
  const run = executor.slice(executor.indexOf("fn run("));
  const failures: string[] = [];
  if (executor.includes("block_on(")) failures.push("production ShardExecutor retains block_on");
  if (!executor.includes("let drive = shard.drive_one().await") || (executor.match(/let drive = shard\.drive_one\(\)\.await/g) ?? []).length !== 1 || (executor.match(/poll_retained_drive_once\(/g) ?? []).length !== 2) failures.push("one pool admission is not exactly one retained shard drive opportunity");
  if (!run.includes("if admitted_epoch != self.epoch.load(Ordering::Acquire)") || run.indexOf("if admitted_epoch != self.epoch.load(Ordering::Acquire)") > run.indexOf("self.state.lock()")) failures.push("stale admitted epoch can reach mutable shard state");
  if (!executor.includes("self.pool.try_submit(lane, job)") || !executor.includes("rejected.into_job()") || !executor.includes("handoff: Mutex<Option<(PoolLane, PoolJob)>>")) failures.push("pool cap plus one does not retain the exact rejected closure");
  if (!executor.includes("self.pool.callback_at(deadline") || !executor.includes("handoff_retry_generation") || !executor.includes("claim_one_shot(&self.handoff_retry_armed)") || !executor.includes("attempt > 8")) failures.push("quiet-ingress saturation has no finite generation-keyed retry trigger");
  if (!executor.includes("WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned => self.terminalize_handoff") || !executor.includes("terminal_handoff: Mutex<Option<(WorkerSubmitErrorKind, PoolLane, PoolJob)>>") || !executor.includes("pub fn take_terminal_handoff") || !executor.includes("pub fn resume_terminal_handoff")) failures.push("terminal pool rejection loses or strands the exact successor closure");
  if (!executor.includes("executor.request_drive_wake(self.generation)") || !executor.includes("claim_drive_wake(self.drive_generation.load") || !executor.includes("drive_waiting.swap(false") || !run.includes("let Some(drive) = polled else")) failures.push("pending drive lacks a one-shot owner wake transition");
  if (run.includes("ShardDrive::Blocked);") || run.includes("ShardDrive::MoreWork | ShardDrive::Blocked")) failures.push("pending drive is immediately resubmitted instead of wake-parked");
  if (executor.includes("while let Some(bytes)") || executor.includes("loop {")) failures.push("executor closure can drain or loop instead of yielding");
  if (!executor.includes("registrations: FixedOwnerRing<(ActorId, GuestInstance), SHARD_DEFERRED_ITEMS>") || !run.includes("if let Some((_, (actor, instance))) = state.registrations.pop_front()") || run.includes("while let Some((actor, instance))")) failures.push("registration authority is not fixed FIFO at one owner per closure");
  if (!run.includes("ShardDrive::Fault { consumed_epoch, work_remains, terminal_overflow") || !run.includes("if let Some(epoch) = consumed_epoch") || !executor.includes("self.consumed_epoch.compare_exchange(previous, epoch") || !run.includes("ShardDrive::Fault { terminal_frame: true") || !run.includes("self.close_ingress(IngressCloseReason::Closing)")) failures.push("terminal frame fault does not consume its exact ingress epoch once and close ingress");
  if (!executor.includes("pub async fn send_frame(self: &Arc<Self>, bytes: Vec<u8>, lane: ActorLane) -> FrameIngress") || !executor.includes("let ingress = self.ingress_gate.lock()") || !executor.includes("if self.ingress_state.load(Ordering::Acquire) != 0") || executor.indexOf("if self.ingress_state.load(Ordering::Acquire) != 0") > executor.indexOf("self.kernel_side.send_now(bytes)") || !executor.includes("FrameIngress::Rejected(TerminalFrameOwner") || !executor.includes("pub fn into_frame(self) -> Vec<u8>") || !executor.includes("pub fn close_terminal_frame")) failures.push("terminal ingress can enqueue or lose the exact late frame owner");
  if (!shardSource.includes("pub async fn drive_one(&mut self) -> ShardDrive") || !shardSource.includes("rejected_frame: Option<(u64, Vec<u8>)>") || !shardSource.includes("self.rejected_frame = Some((epoch, bytes))") || shardSource.includes("while let Some(bytes) = self.transport.recv().await")) failures.push("full shard ingress can drain or lose its original epoch in one grant");
  if (!shardSource.includes("FixedOwnerRing<T, const N: usize>") || !shardSource.includes("pub fn can_admit(&self, items: usize, bytes: usize)") || !shardSource.includes("generation: entry.generation") || !shardSource.includes("Err(AdmissionRejected { limit: AdmissionLimit::Bytes, owner })")) failures.push("deferred owners lack fixed item/byte caps, ABA keys, or exact handback");
  if (/pending_(?:events|job_steps|cancels):\s*(?:HashMap|Vec|VecDeque|std::collections::BTreeMap)/.test(shardSource)) failures.push("a deferred event, job, or cancel owner remains dynamically queued");
  if (!shardSource.includes("SHARD_FRAME_MAX_BYTES") || !shardSource.includes("bytes.len() > SHARD_FRAME_MAX_BYTES") || !shardSource.includes("split_frame_credit(raw_bytes, envelopes.len(), index)") || shardSource.includes("envelope.pack_encode(&mut encoded).await") || shardSource.includes("serde_json::to_vec(&event)?.len()")) failures.push("raw Grant or generated authority bytes use re-encoding instead of exact retained credit");
  for (const authority of ["Register", "Unregister", "Event", "JobStep", "Cancel", "Suspend", "Resume"]) if (!shardSource.includes(`DeferredAuthority::${authority}`)) failures.push(`${authority} lacks fixed deferred authority admission`);
  if (!shardSource.includes("ShardFrame::Register { .. } | ShardFrame::Unregister { .. } => return self.pending_background.can_admit(1, raw_bytes)")) failures.push("Register or Unregister lacks exact item and identifier-byte preflight");
  if (!shardSource.includes("self.pending_interactive.can_admit(interactive_items, interactive_bytes)") || !shardSource.includes("self.pending_background.can_admit(background_items, background_bytes)")) failures.push("mixed lifecycle item and byte preflight is incomplete");
  if (!shardSource.includes("let authority = if let Some((_, authority)) = self.pending_interactive.pop_front()") || !shardSource.includes("match authority") || !shardSource.includes("self.suspend_one(actor, operation, applied_progress).await?") || !shardSource.includes("self.resume_one(actor, operation, checkpoint).await?")) failures.push("one grant can advance multiple mixed FIFO lifecycle authorities");
  if (!shardSource.includes("terminal_frames: FixedOwnerRing<Vec<u8>, SHARD_DEFERRED_ITEMS>") || !shardSource.includes("terminal_authorities: FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>") || !shardSource.includes("self.terminal_frames.pop_front()") || /terminal_frame\s*=\s*Some\(/.test(shardSource)) failures.push("terminal raw frames or lifecycle authorities can overwrite or remain ready after retrieval");
  if (!shardSource.includes("terminal_frame_overflow: FixedOwnerRing<TerminalFrameOverflow, 1>") || !shardSource.includes("FrameAdmissionError::TerminalCapacity { bytes, error }") || !shardSource.includes("self.terminal_frame_overflow.try_push(TerminalFrameOverflow { epoch, bytes }") || !shardSource.includes("take_terminal_frame_and_rearm") || !shardSource.includes("self.terminal_frames.can_admit(1, overflow.bytes.len())") || !shardSource.includes("self.terminal_frames.try_push(overflow.bytes, byte_len)")) failures.push("terminal frame capacity plus one lacks one bounded FIFO re-arm owner");
  if (!run.includes("terminal_overflow_occupied.store(true") || !run.includes("self.close_ingress(IngressCloseReason::TerminalCapacity)") || !run.includes("!terminal_overflow &&") || !executor.includes("if self.terminal_overflow_occupied.load(Ordering::Acquire)") || executor.indexOf("if self.terminal_overflow_occupied.load(Ordering::Acquire)") > executor.indexOf("self.kernel_side.send_now(bytes)") || !executor.includes("map(ShardLoop::take_terminal_frame_and_rearm)") || !executor.includes("self.terminal_overflow_occupied.store(false") || !executor.includes("self.acknowledge_consumed_epoch(epoch)")) failures.push("terminal overflow can hot-resubmit, overwrite, or strand its exact epoch owner");
  if (!shardSource.includes("self.cancel_one(cursor, lane).await?") || !shardSource.includes("DeferredAuthority::Cancel(CancelCursor") || !shardSource.includes("interrupted close handback rejected") || /while[^{}]*\{[^{}]*self\.cancel_one\(/s.test(shardSource)) failures.push("interrupted actor close can drain or lose more than one exact job authority per grant");
  return failures;
}



export function interactivityProductionSource(source: string): string {
  const boundary = source.indexOf(POLICY_RUST_TEST_EVIDENCE_BOUNDARY);
  const lines = source.slice(0, boundary < 0 ? source.length : boundary).split(/\r?\n/);
  const spans = [...policyTestModSpans(lines), ...interactivityCfgTestItemSpans(lines)];
  return lines.filter((_, index) => !policyLineInTestMod(spans, index + 1)).join("\n");
}

export function interactivityStoreSyncFailures(source: string): string[] {
  const production = interactivityProductionSource(source);
  const runner = production.slice(production.indexOf("struct ActorRunner"), production.indexOf("pub(super) async fn spawn_actor", production.indexOf("struct ActorRunner")));
  const failures: string[] = [];
  for (const forbidden of ["runtime.block_on", "semio_framework_async::block_on", "tokio::spawn", "UnboundedSender<ArtifactActorMsg", "unbounded_channel::<ArtifactActorMsg", "self.remote.drain()", "while let Ok(event) = events.try_recv()"])
    if (production.includes(forbidden)) failures.push(`live store-sync retains ${forbidden}`);
  if (!production.includes("slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS]") || !production.includes("ARTIFACT_MAILBOX_BYTES.saturating_sub(state.bytes)") || !production.includes("artifact_actor_message_bytes(&message)") || !production.includes("pub fn into_message(self) -> ArtifactActorMsg")) failures.push("artifact mailbox lacks fixed item/byte preflight or exact rejected-owner handback");
  if (!production.includes("wake_armed") || !production.includes("wake_requested.compare_exchange(false, true") || !runner.includes("scheduled.compare_exchange(false, true") || !runner.includes("future.as_mut().poll(&mut context)")) failures.push("actor readiness is not a generation-coalesced real-waker one-poll turn");
  if (!production.includes("pub(super) async fn drive_one(&mut self) -> ArtifactDrive") || !production.includes("if let Some(message) = self.cmd_rx.try_recv()") || !production.includes("self.remote.try_pop_front()?") || production.includes("self.remote.drain()")) failures.push("one scheduling admission can consume more than one command or backbone owner");
  if (!runner.includes("self.pool.try_submit(semio_framework_async::Lane::UserVisible, job)") || !runner.includes("error.into_job()") || !runner.includes("self.pool.callback_at") || !runner.includes("retry_generation") || !runner.includes("ACTOR_RUNNER_RETRY_LIMIT")) failures.push("quiet saturation lacks exact successor handback and bounded timer-wheel retry");
  if (!runner.includes("pub fn take_terminal_job") && !runner.includes("fn take_terminal_job")) failures.push("terminal rejected job has no explicit retrieval path");
  if (!runner.includes("close_one_terminal_owner") || !runner.includes("self.mailbox.close_one()") || !runner.includes("terminal_turn")) failures.push("cancel/fault close lacks one-owner-per-grant terminal authority");
  if (!runner.includes("deadline_generation") || !runner.includes("deadline_armed") || runner.includes("saturating_add(4)")) failures.push("idle actor uses polling cadence instead of a coalesced exact deadline callback");
  if (!production.includes("runner: ArtifactActorRunnerHandle") || !production.includes("closing: std::collections::HashMap<u64, ArtifactActorRunnerHandle>") || !production.includes(") -> ArtifactActorRunnerHandle") || !production.includes("runner.set_terminal_empty_callback")) failures.push("host registry lacks a durable generation-keyed strong runner handle");
  if (!runner.includes("self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>>") || !production.includes("Some(runner.clone())") || !runner.includes("self.self_retained.lock()") || !runner.includes("self.terminal_is_empty()")) failures.push("quiet or final runner Arc can deep-drop outside terminal retirement");
  if (!production.includes("runner: ArtifactActorRunnerTicket") || !production.includes("runner: std::sync::Weak<ActorRunner>") || !production.includes("external_tickets: std::sync::atomic::AtomicUsize") || !production.includes("returned: bool") || !production.includes("runner.return_ticket()") || !runner.includes("self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0")) failures.push("external channel ticket can outlive close without exact generation return");
  if (!production.includes("closing.insert(generation, runner.clone())") || !production.includes("runner.request_close()") || !production.includes("pub fn closing_runner") || !production.includes("pub fn close_step") || !production.includes("pub fn terminal_is_empty") || !production.includes("pub fn take_terminal_job")) failures.push("host close cannot drive or retrieve retained terminal ownership");
  for (const fixture of [
    "artifact_mailbox_item_cap_plus_one_returns_exact_owner_and_preserves_fifo",
    "artifact_mailbox_byte_cap_and_plus_one_preflight_before_mutation",
    "artifact_mailbox_wake_storm_coalesces_until_fifo_becomes_empty",
    "artifact_mailbox_stale_late_send_hands_back_exact_owner_and_interrupted_close_drains_one_per_grant",
    "artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order",
    "stale_generation_wake_cannot_schedule_or_mutate_current_turn",
    "turn_fault_and_cancel_retain_then_close_one_owner_per_grant",
    "quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry",
    "idle_runner_is_strongly_retained_and_quiet_late_wake_schedules_once",
    "idle_then_late_send_upgrades_the_host_retained_runner_once",
    "external_ticket_held_across_close_delays_completion_until_exact_return",
    "external_ticket_dropped_before_close_and_generation_aba_are_exact",
    "terminal_job_take_resume_and_close_preserve_exact_owner",
    "host_close_registry_survives_external_ticket_until_return",
    "ticket_return_before_host_close_allows_immediate_retirement",
    "detach_while_pending_retains_future_then_cancel_closes_one_owner",
  ]) if (!source.includes(fixture)) failures.push(`store-sync retained-turn fixture missing: ${fixture}`);
  return failures;
}




export function interactivityDbIoFailures(storageSource: string, sqliteSource: string, engineSource: string, faultStorageSource: string, hubSource: string): string[] {
  const storage = interactivityProductionSource(storageSource);
  const sqlite = interactivityProductionSource(sqliteSource);
  const engine = interactivityProductionSource(engineSource);
  const faultStorage = interactivityProductionSource(faultStorageSource);
  const hub = interactivityProductionSource(hubSource);
  const failures: string[] = [];
  for (const forbidden of ["run_blocking_op", "DbIoRequest", "DbIoAdmission", "DbIoState<", "DbIoOperationKind", "DbIoPages::try_new", "DbIoPages::try_range", "into_owner(self) -> Vec<u8>", "into_vec(self)", "open_inline", "runtime.block_on", "tokio::spawn", "thread::spawn"])
    if (storage.includes(forbidden) || sqlite.includes(forbidden)) failures.push(`live DB I/O source retains forbidden raw/generic authority: ${forbidden}`);
  for (const forbidden of ["struct MemWalSegment { bytes: Vec<u8>", "segment.bytes.extend_from_slice(fragment)"])
    if (storage.includes(forbidden)) failures.push(`memory DB I/O retains a whole raw WAL byte owner: ${forbidden}`);
  if (/async fn (?:append|write_generation|put|cas_root|write_run)\([^)]*(?:&\[u8\]|Vec<u8>)/.test(storage) || /async fn (?:append|write_generation|put|cas_root|write_run)\([^)]*(?:&\[u8\]|Vec<u8>)/.test(sqlite)) failures.push("DB write facade accepts caller-thread byte copies instead of exact pages");
  for (const required of [
    "static DB_IO_PAGE_BACKINGS",
    "pub struct DbIoPageWriter",
    "pub fn try_reserve(reserved_pages: usize)",
    "pub struct DbIoPages",
    "pub fn page(&self, index: u8)",
    "pub fn try_range(mut self, start: usize) -> Result<Self, Self>",
    "pub fn advance(&mut self, len: usize)",
    "pub struct DbIoPlatformBuffer",
    "pub fn db_io_prepare_platform",
    "pub struct DbIoPageHash",
    "pub fn db_io_hash_pages",
    "owner.hasher.update(fragment)",
  ]) if (!storage.includes(required)) failures.push(`DB fixed page/range/prepared authority missing: ${required}`);
  for (const required of [
    "pub const DB_IO_PAGE_BYTES: usize = 16 * 1024",
    "const DB_IO_OPERATION_PAGES: usize = 64",
    "const DB_IO_TOTAL_PAGES: usize = 1024",
    "const DB_IO_OPERATION_ITEMS: usize = 64",
    "DbIoPagePhase::Queued",
    "DbIoPagePhase::Executing",
    "DbIoPagePhase::TerminalResult",
  ]) if (!storage.includes(required)) failures.push(`DB exact page cap/phase missing: ${required}`);
  for (const required of [
    "pub enum DbIoTask",
    "BackendOpen",
    "WalAppend",
    "SnapshotWrite",
    "PayloadPut",
    "CatalogCas",
    "IndexWrite",
    "LeaseAcquire",
    "BackendClose",
    "pub enum DbIoResult",
    "pub trait DbIoTaskExecutor",
    "fn execute_step(&self, operation: u64, task: &mut DbIoTask)",
    "fn close_operation_step(&self, operation: u64, task: &DbIoTask)",
  ]) if (!storage.includes(required)) failures.push(`DB schema-first task/result protocol missing: ${required}`);
  for (const required of [
    "pool.try_submit(Lane::Io, job)",
    "error.into_job()",
    "retry_generation",
    "retry_attempt",
    "DB_IO_RETRY_LIMIT",
    "DbIoTaskHandle { slot, generation, operation }",
    "db_io_slot_matches",
    "pub fn db_io_maintenance_step()",
    "db_io_task_close_step",
    "backend_cleanup_done",
    "backend_to_close",
    "pub fn resume(&self) -> Result<bool, DbError>",
    "pub fn take(&mut self) -> Result<Option<Result<DbIoResultLease, DbIoFault>>, DbError>",
    "db_io_platform_maintenance_step",
    "retired: [Option<(u8, u64)>; DB_IO_PLATFORM_RETIREMENT_SLOTS]",
  ]) if (!storage.includes(required)) failures.push(`DB lane/ABA/retry/close ownership missing: ${required}`);
  if (storage.includes("callback_at(") || storage.includes("db_io_schedule_close")) failures.push("DB close/retry retains recursive timer scheduling instead of mounted maintenance");
  for (const required of [
    "struct FsDbIoExecutor",
    "impl DbIoTaskExecutor for FsDbIoExecutor",
    "struct FsReadState",
    "payload_hashes",
    "readers:",
    "fn list_step",
  ]) if (!storage.includes(required)) failures.push(`filesystem typed cursor missing: ${required}`);
  for (const required of ["struct MemoryDbIoExecutor", "impl DbIoTaskExecutor for MemoryDbIoExecutor", "enum MemoryDbIoCursor", "MemoryDbIoCursor::WalRead", "MemoryDbIoCursor::PageCopy", "MemoryDbIoCursor::List", "MemoryDbIoCursor::PayloadHash", "pub struct MemoryStorage", "memory_execute(self.pool.as_ref(), DbIoTask::", "register_db_io_backend(DbIoBackendKind::Memory", "pool.try_submit(Lane::Io, job)"])
    if (!storage.includes(required)) failures.push(`memory typed page cursor missing: ${required}`);
  for (const required of [
    "struct SqliteDbIoExecutor",
    "impl DbIoTaskExecutor for SqliteDbIoExecutor",
    "CREATE TABLE IF NOT EXISTS db_io_stage",
    "fn write_stage_step",
    "fn read_stage_step",
    "fn payload_stage_step",
    "DELETE FROM db_io_stage WHERE operation",
  ]) if (!sqlite.includes(required)) failures.push(`SQLite typed staged cursor missing: ${required}`);
  if (engine.includes("DbIoPages::try_new") || engine.includes("DbIoPages::try_range")) failures.push("DB engine retains a raw page-owner constructor");
  if (engine.includes("pool: Option<Arc<WorkerPool>>") || engine.includes("None => job()") || engine.includes(".with_pool(")) failures.push("DB engine retains an optional/default pool path");
  if (faultStorage.includes("WorkerPool::new(")) failures.push("DB fault testing creates a subsystem WorkerPool instead of accepting the harness authority");
  if (hub) {
    const connectDb = hub.slice(hub.indexOf("async fn connect_db"), hub.indexOf("async fn connect_directory"));
    const mainSetup = hub.slice(hub.indexOf("async fn main"), hub.indexOf("let db = connect_db"));
    if (!connectDb.includes("Database::open_at(pool, &root, profile).await") || !connectDb.includes("SqliteStorage::open(pool.clone()") || /std::fs::|rusqlite::|create_dir_all/.test(connectDb) || /std::fs::|rusqlite::|create_dir_all/.test(mainSetup)) failures.push("Hub DB setup performs synchronous filesystem/SQLite work before typed pooled open");
  }
  for (const fixture of [
    "db_io_fixed_page_max_plus_one_and_zero_are_exact",
    "db_io_range_moves_the_same_page_leases_without_suffix_copy",
    "db_io_list_capacity_plus_one_does_not_mutate_the_fixed_owner",
    "db_io_process_page_max_plus_one_preflight_is_atomic",
    "db_io_result_page_reservation_plus_one_returns_the_writer",
    "db_io_interrupted_close_retires_one_page_or_owner_per_grant",
    "db_io_real_queued_callback_rejects_a_reused_task_slot_aba",
    "db_io_lost_page_handle_resumes_the_same_retirement_cursor",
  ]) if (!storageSource.includes(fixture)) failures.push(`DB retained-page fixture missing: ${fixture}`);
  for (const fixture of ["typed_lane_is_lossless_at_page_boundary_and_zero", "typed_list_and_catalog_cas_are_stable"])
    if (!sqliteSource.includes(fixture)) failures.push(`SQLite typed-I/O fixture missing: ${fixture}`);
  return failures;
}

export function interactivityDbIoB1B6Failures(storageSource: string, sqliteSource: string, postgresSource: string, neo4jSource: string, hubSource: string): string[] {
  const storage = interactivityProductionSource(storageSource);
  const sqlite = interactivityProductionSource(sqliteSource);
  const postgres = interactivityProductionSource(postgresSource);
  const neo4j = interactivityProductionSource(neo4jSource);
  const hub = interactivityProductionSource(hubSource);
  const failures: string[] = [];
  const ordered = (source: string, tokens: readonly string[]): boolean => {
    let cursor = 0;
    for (const token of tokens) {
      const next = source.indexOf(token, cursor);
      if (next < 0) return false;
      cursor = next + token.length;
    }
    return true;
  };
  const rustItem = (source: string, marker: string): string => {
    const start = source.lastIndexOf(marker);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    if (open < 0) return "";
    let depth = 0;
    for (let cursor = open; cursor < source.length; cursor += 1) {
      if (source[cursor] === "{") depth += 1;
      else if (source[cursor] === "}" && --depth === 0) return source.slice(start, cursor + 1);
    }
    return "";
  };
  for (const required of [
    "struct DbIoOperationLedger",
    "fn db_io_operation_reserve",
    "fn db_io_backend_owner_reserve",
    "fn db_io_operation_attach_task",
    "fn db_io_operation_add_result_lease",
    "fn db_io_operation_return_result_lease",
    "fn db_io_operation_transfer_to_backend",
    "fn db_io_operation_detach_task",
    "backend_owner: bool",
    "pub struct DbIoResultLease",
    "result_handback: Option<DbIoResultHandback>",
    "Ok(()) if owner.cancelled => DbIoTerminal::Cancelled(terminal.ok())",
    "pub struct DbIoAsyncTaskLease",
    "credit_returned: bool",
    "fn db_io_result_lease_credit()",
    "fn db_io_async_lease_credit()",
    "pub async fn take_async_native",
    "pub async fn start_async_native_on_lane_io",
    "pub fn start_on_lane_io",
    "type DbIoAsyncDriverFuture",
    "async_driver: Option<DbIoAsyncDriverFuture>",
    "fn db_io_poll_async_driver",
    "db_io_submit_job(handle, Box::new(move || db_io_poll_async_driver(handle))",
    "pub fn enter_lane_io_driver_turn",
    "pub fn leave_lane_io_driver_turn",
    "pub fn db_io_maintenance_step()",
    "fn db_io_backend_close_step",
    "fn db_io_backend_retirement_turn",
    "fn db_io_request_backend_close",
    "pool.try_submit(Lane::Io",
    "executor: Option<Box<dyn DbIoTaskExecutor>>",
    "fn close_backend_step(&mut self, context: &mut std::task::Context<'_>) -> Result<bool, DbError>",
    "fn backend_terminal_is_empty(&self) -> bool",
    "pub struct DbIoDriverReservation",
    "pub struct DbIoArtifactId",
    "pub fn try_from_text(operation: u64, source: &DbIoText)",
    "value: DbIoText",
    "driver: Option<ArtifactId>",
    "external: Option<DbIoExternalBytes>",
    "reservation: Option<DbIoDriverReservation>",
    "pub struct DbIoExternalBytes",
    "pub type LeaseInfo = DbIoLeaseResult",
    "pub fn try_reserve(operation: u64, maximum_capacity: usize)",
    "pub fn observe_capacity(&mut self, capacity: usize)",
    "pub struct DbIoObservedBytesWrite",
    "pub fn db_io_write_observed_bytes",
    "pub async fn db_io_copy_observed_text",
    "pub struct DbIoListTransfer",
    "pub fn db_io_transfer_list",
    "pub struct DbIoPlatformClose",
    "pub(crate) struct DbIoPlatformSlicesCopy",
    "pub struct DbIoPageWriterSeal",
    "pub fn seal_retained_step",
    "pub async fn close_db_io_backend",
    "pub fn retire_db_io_backend",
    "pub fn close_backend_step(&self) -> Result<bool, DbError>",
    "pub fn backend_terminal_is_empty(&self) -> bool",
    "const DB_IO_LOST_OWNER_SLOTS",
    "const DB_IO_LOST_OWNER_OVERFLOW_SLOTS",
    "static DB_IO_LOST_OWNER_OVERFLOW",
    "static DB_IO_LOST_OWNER_QUARANTINE",
    "static DB_IO_RETIREMENT_PRESSURE_FAULT",
    "struct DbIoRejectedBackendRegistry",
    "fn db_io_park_lost_owner",
    "fn db_io_try_park_lost_owner",
    "DbIoLostOwner::PageWriter",
    "DbIoLostOwner::Pages",
    "DbIoLostOwner::List",
    "DbIoLostOwner::Lease",
    "DbIoLostOwner::Fault",
    "DbIoLostOwner::DriverReservation",
    "DbIoLostOwner::ExternalBytes",
    "DbIoLostOwner::ArtifactId",
    "DbIoLostOwner::Backend",
    "DbIoLostOwner::ResultLease",
  ]) if (!storage.includes(required)) failures.push(`DB aggregate/result/backend authority missing: ${required}`);
  for (const forbidden of ["into_lease_info", "pub struct LeaseInfo {"])
    if (storage.includes(forbidden)) failures.push(`DB result boundary retains an uncensused lease allocation: ${forbidden}`);
  for (const forbidden of ["db_io_schedule_close", "Arc<dyn DbIoTaskExecutor>", "retry_job: Option<(Job", "terminal_job: Option<", "process_worker_pool("])
    if (storage.includes(forbidden)) failures.push(`DB mounted close/retry retains forbidden opaque authority: ${forbidden}`);
  if (storage.split("callback_at(").slice(1).some((call) => !/^[^;]*?, move \|\| (?:db_io_retry\(handle, generation\)|waker\.wake\(\))\)/u.test(call))) failures.push("DB mounted close/retry retains forbidden opaque authority: callback_at( with an opaque closure (only the generation-qualified `db_io_retry(handle, generation)` token or a waker wake may ride the pool timer)");
  for (const forbidden of ["std::mem::forget", "std::mem::ManuallyDrop", "Waker::noop()"])
    if (storage.includes(forbidden)) failures.push(`DB lossless retirement/backend close retains prohibited production path: ${forbidden}`);
  for (const marker of ["impl Future for DbIoPageWriterSeal", "impl Future for DbIoObservedBytesWrite", "impl Future for DbIoListTransfer", "impl Future for DbIoPlatformClose", "impl Future for DbIoPlatformSlicesCopy"]) {
    const body = rustItem(storage, marker);
    if (!body.includes("std::task::Poll::Pending") || !body.includes("context.waker().wake_by_ref()")) failures.push(`DB one-opportunity cursor does not persist a governed Pending turn: ${marker}`);
    if (/while\b|\bloop\b/.test(body)) failures.push(`DB one-opportunity cursor retains a whole-owner loop: ${marker}`);
  }
  const backendRetirementTurn = rustItem(storage, "fn db_io_backend_retirement_turn");
  if (!backendRetirementTurn.includes("db_io_backend_close_step(control, context)")) failures.push("DB registered backend close is not polled by its pre-admitted maintenance hook");
  const backendMaintenance = rustItem(storage, "fn db_io_request_backend_close");
  if (!backendMaintenance.includes("crate::request_retirement(control)") || backendMaintenance.includes("db_io_backend_close_step") || backendMaintenance.includes("try_submit")) failures.push("DB backend retirement request is not an unlosable hook request");
  const rejectedLanePoll = rustItem(storage, "fn db_io_poll_rejected_backend_on_lane_io");
  if (!rejectedLanePoll.includes("executor.close_backend_step(context)")) failures.push("DB rejected backend close is not polled by its typed Lane::Io job");
  for (const body of [rustItem(storage, "pub async fn close_db_io_backend"), backendMaintenance, rustItem(storage, "fn db_io_lost_owner_close_step")])
    if (body.includes(".close_backend_step(")) failures.push("DB facade/drop/maintenance caller polls a backend close future");
  const lostOwnerInstall = rustItem(storage, "fn db_io_park_lost_owner");
  if (!lostOwnerInstall.includes("Result<(), DbIoLostOwner>") || !lostOwnerInstall.includes("DB_IO_LOST_OWNER_QUARANTINE") || lostOwnerInstall.includes(".expect(")) failures.push("DB aggregate retirement lacks checked exact-owner overflow/quarantine return");
  const lostOwnerClose = rustItem(storage, "fn db_io_lost_owner_close_step");
  for (const required of ["db_io_lost_owner_close_opportunity", "quarantine[index].as_mut()", "overflow[index].as_mut()", "*slot = quarantine[index].take()", "*slot = overflow[index].take()"])
    if (!lostOwnerClose.includes(required)) failures.push(`DB overflow/quarantine reclaim does not retain the exact owner until a primary target exists: ${required}`);
  if (/let\s+owner\s*=\s*(?:quarantine|overflow)\[index\]\.take\(\)/.test(lostOwnerClose)) failures.push("DB overflow/quarantine reclaim takes an exact owner before durable target installation");
  const artifactOwner = rustItem(storage, "impl DbIoArtifactId");
  for (const required of ["DbIoText::try_from_str(source.as_str())", "DbIoDriverReservation::try_reserve", "ArtifactId(source.as_str().to_owned())", ".observe_capacity(capacity)?", "driver.0.into_bytes()", "DbIoExternalBytes::new", "pub fn as_artifact", "pub fn terminal_is_empty"])
    if (!artifactOwner.includes(required)) failures.push(`DB admitted artifact conversion lacks exact fixed/dynamic authority: ${required}`);
  const artifactDrop = rustItem(storage, "impl Drop for DbIoArtifactId");
  if (!artifactDrop.includes("DbIoLostOwner::ArtifactId(owner)")) failures.push("DB admitted artifact conversion Drop does not retain its exact dynamic owner");
  const observedBytes = rustItem(storage, "impl Future for DbIoObservedBytesWrite");
  if (!storage.includes("source: Option<DbIoExternalBytes>") || !observedBytes.includes("source.close_step()") || !observedBytes.includes("owner.output.seal_retained_step()") || observedBytes.includes("owner.output.finish()") || !storage.includes("fn terminal_is_empty(&self)") || !storage.includes("self.value.is_none() && self.phase == 2")) failures.push("DB observed driver allocation lacks an explicit resumable external owner");
  const observedText = rustItem(storage, "pub async fn db_io_copy_observed_text");
  for (const required of ["DbIoExternalBytes::new(source.into_bytes())", "reservation.observe_capacity(source.capacity()?)?", "while !source.terminal_is_empty()", "source.close_step()", "semio_framework_async::yield_once().await"])
    if (!observedText.includes(required)) failures.push(`DB observed driver text lacks retained external-owner close evidence: ${required}`);
  if (observedText.includes("drop(source)")) failures.push("DB observed driver text ordinarily drops its external allocation");
  const writerSeal = rustItem(storage, "impl Future for DbIoPageWriterSeal");
  const writerSealStep = rustItem(storage, "pub fn seal_retained_step");
  for (const required of ["self.seal_phase", "self.seal_page", "self.transition(current, next)?", "Ok(None)", "Ok(Some(owner))"])
    if (!writerSealStep.includes(required)) failures.push(`DB writer publication lacks persisted one-opportunity seal state: ${required}`);
  const writerTransition = rustItem(storage.slice(0, storage.indexOf("impl Future for DbIoPageWriterSeal")), "fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase)");
  if (!ordered(writerTransition, ["for page in self.pages.iter().take(self.reserved as usize).flatten()", "page.validate_phase(slot, expected)?", "for page in self.pages.iter().take(self.reserved as usize).flatten()", ".phase = next"])) failures.push("DB writer seal transition is not one bounded validate-all-then-commit phase change over its fixed page array");
  if (!writerSeal.includes("writer.seal_retained_step()") || writerSeal.includes("writer.finish()") || /while\b|\bloop\b/.test(writerSealStep)) failures.push("DB writer publication is not a persisted one-page transition cursor");
  for (const [name, source] of [["storage", storage], ["SQLite", sqlite], ["PostgreSQL", postgres], ["Neo4j", neo4j]] as const)
    if (/\b(?:writer|output)\.(?:finish|seal)\(\)/.test(source)) failures.push(`${name} production bypasses retained DbIoPageWriterSeal completion`);

  const memoryStart = storage.indexOf("struct MemoryDbIoExecutor");
  const memoryEnd = storage.indexOf("//#region 🔖️Fs", memoryStart);
  const memory = storage.slice(memoryStart, memoryEnd < 0 ? undefined : memoryEnd);
  for (const required of [
    "wal: std::sync::Mutex<Box<[Option<MemoryWalOwner>]>>",
    "payloads: std::sync::Mutex<Box<[Option<MemoryPayloadOwner>]>>",
    "leases: std::sync::Mutex<Box<[Option<MemoryLeaseOwner>]>>",
    "payloads: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS))",
    "leases: std::sync::Mutex::new(memory_fixed_none_box(DB_IO_MEMORY_OWNERS))",
    "fn bind_owner_operation",
    "fn owner_backing_bytes",
    "pages.transfer_to_backend(backend_operation)?",
    "impl DbIoTaskExecutor for MemoryDbIoExecutor",
    "MemoryDbIoCursor::LeaseRelease",
  ]) if (!memory.includes(required)) failures.push(`memory fixed backing/task authority missing: ${required}`);
  for (const forbidden of ["HashMap<", "BTreeMap<", "collect::<HashMap", "process_worker_pool("])
    if (memory.includes(forbidden)) failures.push(`memory typed operation retains uncensused dynamic owner: ${forbidden}`);
  const memoryFacade = storage.slice(storage.indexOf("pub struct MemoryStorage"), storage.indexOf("//#region 🔖️Fs"));
  if (!memoryFacade.includes("pub async fn new(pool: Arc<WorkerPool>)") || !memoryFacade.includes("register_db_io_backend_reserved_with_use(DbIoBackendKind::Memory, executor, pool.clone(), pool_use,") || !memoryFacade.includes("submit_db_io_task_admitted(task).await?.finish().await") || !memoryFacade.includes("memory_execute(DbIoTask::")) failures.push("MemoryStorage does not inject and use the shared typed WorkerPool");
  if (!memory.includes("output.seal_retained_step()")) failures.push("Memory production completion bypasses its retained one-opportunity page seal");

  for (const owner of ["DbIoPageWriter", "DbIoPages", "DbIoU64List", "DbIoLeaseResult", "DbIoFault", "DbIoResultLease", "DbIoArtifactId"] as const) {
    const dropBody = rustItem(storage, `impl Drop for ${owner}`);
    if (!dropBody.includes("db_io_park_lost_owner(")) failures.push(`${owner} Drop does not durably park its populated exact owner`);
    if (dropBody.includes("db_io_operation_return(") || dropBody.includes("db_io_result_handback(")) failures.push(`${owner} Drop directly returns aggregate credit or result handback`);
  }
  const cancelStart = storage.lastIndexOf("pub fn cancel(&self)");
  const cancel = storage.slice(cancelStart, storage.indexOf("pub async fn take_async_native", cancelStart));
  if (!ordered(cancel, ["owner.cancelled = true", "drop(owner)", "db_io_enqueue_close(self.handle)?"])) failures.push("cancel does not retain the exact task before mounted close enrollment");
  const completeStart = storage.lastIndexOf("pub fn complete(mut self");
  const complete = storage.slice(completeStart, storage.indexOf("impl Drop for DbIoAsyncTaskLease", completeStart));
  if (!ordered(complete, ["owner.async_detached = false", "DbIoTerminal::Cancelled(terminal.ok())", "drop(owner)", "db_io_operation_return(self.handle.operation, db_io_async_lease_credit())?"])) failures.push("async completion does not retain cancelled output before returning lease credit");
  const taskCloseStart = storage.lastIndexOf("pub fn db_io_task_close_step");
  const taskClose = storage.slice(taskCloseStart, storage.indexOf("fn db_io_rotate_close_head", taskCloseStart));
  if (!ordered(taskClose, ["owner.phase == DbIoTaskPhase::Executing", "db_io_rotate_close_head(handle)?"])) failures.push("mounted close does not rotate a detached/executing exact owner");

  for (const required of ["backend_close_cursor", "backend_terminal", "fn close_backend_step(&mut self,", "fn backend_terminal_is_empty(&self)", "close_db_io_backend(self.control).await"])
    if (!sqlite.includes(required)) failures.push(`SQLite backend close witness missing: ${required}`);
  if (!sqlite.includes("output.seal_retained_step()?")) failures.push("SQLite staged read bypasses retained one-opportunity page sealing");
  for (const [name, source, executor] of [
    ["PostgreSQL", postgres, "PostgresDbIoExecutor"],
    ["Neo4j", neo4j, "Neo4jDbIoExecutor"],
  ] as const) {
    for (const required of [
      `struct ${executor}`,
      `impl DbIoTaskExecutor for ${executor}`,
      "DbIoExecutorMode::AsyncNative",
      "register_db_io_backend",
      "submit_db_io_task",
      "operation.start_async_native_on_lane_io().await?",
      "fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask)",
      "with_admitted_artifact!",
      "db_io_transfer_list",
      "fn close_backend_step(&mut self,",
      "fn backend_terminal_is_empty(&self)",
      "DbIoDriverReservation",
      "close_db_io_backend(self.control).await",
    ]) if (!source.includes(required)) failures.push(`${name} typed async-native authority missing: ${required}`);
    if (source.includes("db_io_copy_pages")) failures.push(`${name} retains unobserved raw driver byte conversion`);
    const facadeStart = source.indexOf(name === "PostgreSQL" ? "pub struct PostgresStorage" : "pub struct Neo4jStorage");
    const facade = source.slice(facadeStart, source.indexOf("//#region 🔖️DbBackend", facadeStart));
    if (!facade.includes("worker_pool: Arc<WorkerPool>") || !facade.includes("self.execute(DbIoTask::")) failures.push(`${name} public facade bypasses typed shared-pool submission`);
    if (/\.execute\(&self\.(?:pool|graph)\)|\.fetch_(?:one|optional)\(&self\.(?:pool|graph)\)/.test(facade)) failures.push(`${name} public facade directly calls its external driver`);
    if (!ordered(facade, ["submit_db_io_task_admitted(task).await?", "operation.start_async_native_on_lane_io().await?", "operation.finish().await"])) failures.push(`${name} external driver execution escapes the generation-qualified Lane::Io worker order`);
    if (/drive_task\([^)]*\)\.await/.test(facade) || facade.includes("take_async_native().await")) failures.push(`${name} facade polls an actual external driver future outside WorkerPool authority`);
    if (/ArtifactId\([^)]*(?:as_str\(\)|document)[^)]*(?:to_string|to_owned)/.test(source)) failures.push(`${name} constructs an uncensused post-admission ArtifactId`);
    const artifactMacro = source.slice(source.indexOf("macro_rules! with_admitted_artifact"), source.indexOf("use ", source.indexOf("macro_rules! with_admitted_artifact")));
    for (const required of ["owner.as_artifact()?", "while owner.close_step()?", "semio_framework_async::yield_once().await"])
      if (!artifactMacro.includes(required)) failures.push(`${name} admitted artifact macro lacks retained conversion/close evidence: ${required}`);
    if (source.includes("LeaseInfo { resource:") || source.includes("holder.as_str().to_string(), fence:")) failures.push(`${name} constructs an uncensused raw LeaseInfo result`);
    if (source.includes("Waker::noop()")) failures.push(`${name} backend close manually polls an external future without the shared Lane::Io task context`);
    if (/poll_fn\([\s\S]{0,180}Poll::Ready\(\(\)\)/.test(source)) failures.push(`${name} retains a Ready-only external-driver pseudo-yield`);
  }
  for (const required of ["reserve_driver_output", "DbIoDriverReservation::try_reserve(self.active_operation", "db_io_write_observed_bytes(reservation, bytes, output).await", "db_io_copy_observed_text(reservation, holder)"])
    if (!postgres.includes(required)) failures.push(`PostgreSQL pre-driver capacity admission missing: ${required}`);
  for (const forbidden of ["reserve_driver_output(len as u64)", "reserve_driver_output(range.len)"])
    if (postgres.includes(forbidden)) failures.push(`PostgreSQL external Vec allocation is not precharged to its maximum observed capacity: ${forbidden}`);
  for (const required of ["reserve_driver_read", "BoltBytes", "db_io_write_observed_bytes_range(reservation, bytes.value", "DbIoExternalBytes::new(current.value.to_vec())", "current_reservation.observe_capacity(current.capacity()?)?", "while !current.terminal_is_empty()", "semio_framework_async::yield_once().await", "write_driver_bytes(reservation, bytes", "self.graph = Some(Graph::connect(config).await", "Neo4jDbIoExecutor::new(config"])
    if (!neo4j.includes(required)) failures.push(`Neo4j retained codec/connect authority missing: ${required}`);
  for (const forbidden of ["fn encode_bytes(", "let updated = apply_append", "let updated = apply_truncate", "connect_with_config_and_uri(config, uri.clone()).await", "BASE64", "base64::", "decode_bytes(&row.get::<String>"])
    if (neo4j.includes(forbidden)) failures.push(`Neo4j production retains a whole-buffer or pre-registration path: ${forbidden}`);
  for (const fixture of [
    "db_io_one_byte_high_capacity_candidate_is_rejected_with_exact_owner",
    "db_io_process_and_operation_ledger_return_to_exact_prior_witness",
    "db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity",
    "db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate",
    "db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger",
    "db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity",
    "db_io_real_queued_callback_rejects_a_reused_task_slot_aba",
    "db_io_retry_generation_max_publishes_a_lossless_terminal_fault",
    "db_io_postgres_and_neo4j_mock_drivers_use_supplied_writer_and_observed_capacity",
    "db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker",
    "db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback",
    "db_io_all_five_backend_controls_require_explicit_terminal_close_witness",
    "db_io_memory_backend_uses_actual_typed_submit_take_result_and_terminal_close",
    "db_io_async_native_lost_backend_uses_typed_lane_lease_and_mounted_terminal_witness",
    "db_io_cancellation_before_during_and_receiver_drop_retain_exact_terminal_owners",
    "db_io_panic_backend_fault_and_shutdown_close_reach_exact_prior_witness",
  ]) if (!storageSource.includes(fixture)) failures.push(`DB hostile B1-B6 fixture missing: ${fixture}`);
  const actualWorkerLaw = rustItem(storageSource, "async fn db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker");
  for (const evidence of ["worker_role.load", "assert_ne!", "close_worker_role.load", "close_thread.lock"])
    if (!actualWorkerLaw.includes(evidence)) failures.push(`DB exact WorkerPool Lane::Io law lacks production-path evidence: ${evidence}`);
  const saturationLaw = rustItem(storageSource, "fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate");
  for (const evidence of ["DB_IO_LOST_OWNER_SLOTS", "db_io_park_lost_owner(owner)", "DB_IO_LOST_OWNER_OVERFLOW", "DB_IO_LOST_OWNER_QUARANTINE", "retained-overflow-owner", "exact-plus-one-candidate", "exact-plus-two-candidate", "exact-all-tier-refusal", "db_io_park_lost_owner(refused)", "assert_eq!(exact", "db_io_lost_owner_close_step"])
    if (!saturationLaw.includes(evidence)) failures.push(`DB saturation identity/recovery law lacks production-path evidence: ${evidence}`);
  const closeInterruptionLaw = rustItem(storageSource, "async fn db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger");
  for (const evidence of ["db_io_close_platform", "Poll::Pending", "Poll::Ready(Ok(()))", "Poll::Ready(Err(DbError::LimitExceeded(", "drop(close)", "ledger_witness"])
    if (!closeInterruptionLaw.includes(evidence)) failures.push(`DB Ready/Pending/fault close-interruption law lacks production-path evidence: ${evidence}`);
  const pageSealLaw = rustItem(storageSource, "fn db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity");
  for (const evidence of ["seal_retained()", "Future::poll", "Poll::Pending", "drop(seal)", "DB_IO_OPERATION_PAGES + 1", "DbIoPagePhase::Queued", "Poll::Ready(Err(rejected))", "rejected.into_writer()", "ledger_witness()"])
    if (!pageSealLaw.includes(evidence)) failures.push(`DB page-seal hostile law lacks exact interrupted/max/fault owner evidence: ${evidence}`);
  if (!postgresSource.includes("lost_postgres_facade_drives_the_real_lazy_pool_to_closed") || !postgres.includes("self.pool.is_closed()") || !postgres.includes("db_storage_driver_runtime::detach_unit(Box::pin(async move { pool.close().await }))") || !postgres.includes("std::future::Future::poll(std::pin::Pin::new(receiver), context)")) failures.push("PostgreSQL lost-facade path lacks a real Lane::Io-polled async pool-close terminal witness");
  if (!neo4jSource.includes("lost_neo4j_facade_retires_the_real_owned_config_without_a_service")) failures.push("Neo4j lost-facade path lacks its real owned-control retirement law");
  if (!hub.includes("PostgresStorage::connect(pool.clone()") || !hub.includes("Neo4jStorage::connect(pool.clone()")) failures.push("Hub async-native storage callers bypass the shared WorkerPool");
  return failures;
}



export function interactivityP1qR4Failures(packSource: string, snapshotSource: string, indexSource: string, walSource: string, artifactSource: string, querySource: string, engineSource: string, compactSource: string, cliSource: string, stateSource: string): string[] {
  const pack = interactivityProductionSource(packSource);
  const snapshot = interactivityProductionSource(snapshotSource);
  const index = interactivityProductionSource(indexSource);
  const wal = interactivityProductionSource(walSource);
  const artifact = interactivityProductionSource(artifactSource);
  const query = interactivityProductionSource(querySource);
  const engine = interactivityProductionSource(engineSource);
  const compact = interactivityProductionSource(compactSource);
  const cli = interactivityProductionSource(cliSource);
  const state = interactivityProductionSource(stateSource);
  const failures: string[] = [];
  const rustItem = (source: string, marker: string): string => {
    const start = source.indexOf(marker);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    if (open < 0) return "";
    let depth = 0;
    for (let cursor = open; cursor < source.length; cursor += 1) {
      if (source[cursor] === "{") depth += 1;
      else if (source[cursor] === "}" && --depth === 0) return source.slice(start, cursor + 1);
    }
    return "";
  };
  const rustBlocks = (source: string, keyword: "for" | "loop"): string[] => {
    const blocks: string[] = [];
    const pattern = keyword === "for" ? /\bfor\b/g : /\bloop\b/g;
    for (let match = pattern.exec(source); match; match = pattern.exec(source)) {
      const open = source.indexOf("{", match.index);
      if (open < 0 || open - match.index > 240) continue;
      let depth = 0;
      for (let cursor = open; cursor < source.length; cursor += 1) {
        if (source[cursor] === "{") depth += 1;
        else if (source[cursor] === "}" && --depth === 0) {
          blocks.push(source.slice(match.index, cursor + 1));
          pattern.lastIndex = cursor + 1;
          break;
        }
      }
    }
    return blocks;
  };
  const hasTerminalCloseSweep = (source: string): boolean => {
    if (/while[^\n{]*close_(?:step|owner_step)/.test(source)) return true;
    for (const block of rustBlocks(source, "loop"))
      if (/match[\s\S]{0,160}close_(?:step|owner_step)/.test(block) && /Ok\(false\)[\s\S]{0,80}break/.test(block)) return true;
    for (const block of rustBlocks(source, "for"))
      if (/close_(?:step|owner_step)/.test(block) && !/\.grant\(\)/.test(block) && !/Poll::Pending/.test(block)) return true;
    return false;
  };
  for (const [name, source] of [["artifact", artifact], ["query", query], ["engine", engine], ["compact", compact]] as const)
    for (const forbidden of ["std::mem::forget", "std::mem::ManuallyDrop"])
      if (source.includes(forbidden)) failures.push(`${name} retirement abandons an exact owner through ${forbidden}`);
  for (const [name, source] of [["artifact", artifact], ["query", query], ["engine", engine], ["compact", compact], ["CLI", cli]] as const)
    if (/poll_fn\([\s\S]{0,180}Poll::Ready\(\(\)\)/.test(source)) failures.push(`${name} retains a Ready-only pseudo-yield instead of persisted Pending state`);
  for (const [name, source] of [["state", state], ["WAL", wal], ["index", index]] as const) {
    if (/poll_fn\([\s\S]{0,180}Poll::Ready\(\(\)\)/.test(source)) failures.push(`${name} retains a Ready-only page/external-owner pseudo-yield`);
    if (/\b(?:writer|output)\.(?:finish|seal)\(\)/.test(source)) failures.push(`${name} production bypasses retained DbIoPageWriterSeal completion`);
    if (source.includes("drop(source)")) failures.push(`${name} ordinarily drops a raw admitted external byte owner`);
  }
  for (const [name, source, marker] of [["state", state, "pub async fn try_admit"], ["WAL", wal, "pub async fn try_admit"], ["index", index, "pub async fn try_admit"]] as const) {
    const admission = rustItem(source, marker);
    for (const required of ["DbIoExternalBytes::new(source)", "reservation.observe_capacity(capacity)", "while !source.terminal_is_empty()", "source.close_step()", "semio_framework_async::yield_once().await", "seal_retained().await"])
      if (!admission.includes(required)) failures.push(`${name} admission lacks retained external-owner/page-seal evidence: ${required}`);
  }
  for (const [name, source, owner] of [["state", state, "StateEntryRejected"], ["WAL", wal, "WalBytesRejected"], ["index", index, "IndexBytesRejected"]] as const) {
    const drop = rustItem(source, `impl Drop for ${owner}`);
    if (!drop.includes("self.source.take()") || !drop.includes("DbIoExternalBytes::new(source)")) failures.push(`${name} rejected-source Drop does not retain its exact external allocation`);
  }
  for (const [name, source, marker, quarantine] of [
    ["artifact", artifact, "fn retire_artifact_state_owner", "ARTIFACT_STATE_RETIREMENT_QUARANTINE"],
    ["query", query, "fn retire_query_rows", "QUERY_RETIRED_ROWS_QUARANTINE"],
    ["engine", engine, "fn retire_engine_query_stream", "ENGINE_QUERY_RETIREMENT_QUARANTINE"],
    ["compact", compact, "fn retire_compaction_pages", "COMPACTION_PAGE_RETIREMENT_QUARANTINE"],
  ] as const) {
    const install = rustItem(source, marker);
    if (!install.includes("Result<(),") || !install.includes(quarantine) || install.includes(".expect(")) failures.push(`${name} retirement lacks checked exact-owner overflow/quarantine return`);
  }
  for (const [name, source, reservation, install] of [
    ["artifact", artifact, "ARTIFACT_STATE_RETIREMENT_RESERVATIONS", "install_reserved_artifact_state_owner"],
    ["query", query, "QUERY_RETIRED_ROWS_RESERVATIONS", "install_reserved_query_rows"],
    ["engine", engine, "ENGINE_QUERY_RETIREMENT_RESERVATIONS", "install_reserved_engine_query_stream"],
    ["compact", compact, "COMPACTION_RETIREMENT_RESERVATIONS", "install_reserved_compaction_pages"],
  ] as const) {
    if (!source.includes(reservation) || !source.includes(install) || !source.includes("fetch_or(bit") || !source.includes("fetch_and(!")) failures.push(`${name} leaf admission lacks an exact durable retirement reservation`);
    for (const forbidden of ["return_to_leaf_authorities", "_or_recover", "let pages = std::mem::replace", "let rows = std::mem::replace", "let results = std::mem::replace"])
      if (source.includes(forbidden)) failures.push(`${name} all-tier refusal destructures or ordinarily drops a leaf owner through ${forbidden}`);
  }
  const queryExecute = rustItem(query, "pub async fn execute");
  for (const required of ["projected.preflight_push()?", "matched.take(0)", "projected.push_preflighted"])
    if (!queryExecute.includes(required)) failures.push(`query projection lacks target retirement preflight before source-row extraction: ${required}`);
  if (queryExecute.indexOf("projected.preflight_push()?") > queryExecute.indexOf("matched.take(0)")) failures.push("query projection extracts its source row before target retirement preflight");
  const queryRefresh = rustItem(query, "pub async fn refresh");
  for (const required of ["diff.added.preflight_push()?", "diff.updated.preflight_push()?", "result.rows.take(0)", "push_preflighted(row)"])
    if (!queryRefresh.includes(required)) failures.push(`query refresh lacks branch-specific retirement preflight before source-row extraction: ${required}`);
  if (queryRefresh.indexOf("diff.added.preflight_push()?") > queryRefresh.indexOf("result.rows.take(0)") || queryRefresh.indexOf("diff.updated.preflight_push()?") > queryRefresh.indexOf("result.rows.take(0)")) failures.push("query refresh extracts its source row before branch-specific retirement preflight");
  const projectionSource = rustItem(query, "pub async fn from_value");
  for (const required of ["check_len(count as u64", "rows.preflight_push()?", "rows.push_preflighted"])
    if (!projectionSource.includes(required)) failures.push(`query projection source lacks fixed-count retirement preflight before value extraction: ${required}`);
  if (projectionSource.includes("rows.push(QueryRow")) failures.push("query projection source restores post-extraction row admission");
  const artifactQuerySource = rustItem(artifact, "impl<'a> db_query::QuerySource for StateQuerySource<'a>");
  for (const required of ["rows.preflight_push()?", "path_row_value", "rows.push_preflighted(row)"])
    if (!artifactQuerySource.includes(required)) failures.push(`artifact query source lacks retirement preflight before page-backed row construction: ${required}`);
  if (artifactQuerySource.indexOf("rows.preflight_push()?") > artifactQuerySource.indexOf("path_row_value")) failures.push("artifact query source constructs a page-backed row before target retirement preflight");
  const compactCollect = rustItem(compact, "async fn collect_chain_pages");
  for (const required of ["pages.preflight_push()?", "cursor.read_page(hash).await?", "pages.push_preflighted(page)"])
    if (!compactCollect.includes(required)) failures.push(`compaction chain collection lacks retirement preflight before page acquisition: ${required}`);
  if (compactCollect.indexOf("pages.preflight_push()?") > compactCollect.indexOf("cursor.read_page(hash).await?") || compactCollect.includes("rejected.try_push(page)")) failures.push("compaction chain collection acquires or destructures a page before target retirement preflight");
  for (const required of ["pub struct PackIdentityChunkCursor", "pub async fn read_fragment", "pub fn identity_chunk_cursor", "retained fragment cursor requires an identity chunk"])
    if (!pack.includes(required)) failures.push(`pack retained identity cursor missing: ${required}`);
  const snapshotRead = rustItem(snapshot, "pub async fn read_page");
  if (!snapshotRead.includes("identity_chunk_cursor") || !snapshotRead.includes("read_fragment(&mut fragment).await")) failures.push("snapshot page reader does not consume the retained pack fragment cursor");
  if (snapshotRead.includes("read_chunk(")) failures.push("snapshot page reader materializes a whole generic pack chunk");
  const indexDecode = rustItem(index, "async fn decode_run_pages(");
  if (!indexDecode.includes("let _ = pages.close_step()?;") || !indexDecode.includes("drop(pages)")) failures.push("index decode does not retire one page opportunity before retained handoff");
  if (/while[^\n{]*close_step/.test(indexDecode)) failures.push("index decode bulk-drains admitted pages");
  for (const [name, source] of [["WAL batch", rustItem(wal, "impl WalRecordBatch")], ["index entries", rustItem(index, "impl RunEntries")], ["index blob list", rustItem(index, "impl IndexBlobList")]] as const)
    if (hasTerminalCloseSweep(source)) failures.push(`${name} close cursor retains a loop/for/while owner drain`);
  for (const required of ["WAL batch close lost retained record", "index close lost retained entry", "index blob close lost retained owner"])
    if (!`${wal} ${index}`.includes(required)) failures.push(`fixed close cursor missing: ${required}`);
  const walNext = rustItem(wal, "pub async fn next_step");
  if (!walNext.includes("WalReplayStep::Yield") || !walNext.includes("if self.close_segment_step().await?")) failures.push("WAL replay segment transition lacks a resumable close yield");
  if (/while[^\n{]*close_segment_step/.test(walNext)) failures.push("WAL replay segment transition bulk-drains its retained segment");
  const queryDrop = rustItem(query, "impl Drop for QueryRows");
  for (const required of ["QUERY_RETIRED_ROWS", "QUERY_RETIRED_ROWS_OVERFLOW", "QUERY_RETIRED_ROWS_QUARANTINE", "QUERY_RETIREMENT_PRESSURE_FAULT", "query_rows_maintenance_step", "install_reserved_query_rows(std::mem::replace"])
    if (!query.includes(required)) failures.push(`query mounted retirement missing: ${required}`);
  if (queryDrop.includes("while") || queryDrop.includes("loop")) failures.push("QueryRows Drop bulk-drains its result graph");
  const engineDrop = rustItem(engine, "impl Drop for QueryStream");
  for (const required of ["ENGINE_QUERY_RETIREMENT", "ENGINE_QUERY_RETIREMENT_OVERFLOW", "ENGINE_QUERY_RETIREMENT_QUARANTINE", "ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT", "engine_query_maintenance_step", "install_reserved_engine_query_stream(std::mem::replace"])
    if (!engine.includes(required)) failures.push(`engine query mounted retirement missing: ${required}`);
  if (engineDrop.includes("while") || engineDrop.includes("loop")) failures.push("QueryStream Drop bulk-drains its result graph");
  for (const required of ["ArtifactStateRetirementCursor", "ARTIFACT_STATE_RETIREMENT", "ARTIFACT_STATE_RETIREMENT_OVERFLOW", "ARTIFACT_STATE_RETIREMENT_QUARANTINE", "ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT", "artifact_state_retirement_maintenance_step", "install_reserved_artifact_state_owner"])
    if (!artifact.includes(required)) failures.push(`artifact fixed staging retirement missing: ${required}`);
  for (const required of ["CompactionRetainedPages", "COMPACTION_PAGE_RETIREMENT", "COMPACTION_PAGE_RETIREMENT_OVERFLOW", "COMPACTION_PAGE_RETIREMENT_QUARANTINE", "COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT", "MountedCompactionPageClose", "publish_retained"])
    if (!compact.includes(required)) failures.push(`compaction fixed retained publication missing: ${required}`);
  if (/Vec\s*<\s*(?:db_state::)?Page\s*>/.test(compact)) failures.push("compaction retains a dynamic Vec<Page> owner graph");
  for (const required of ["MountedWalRecordCommandClose", "MountedWalBatchCommandClose", "MountedWalReplayCommandClose", "MountedSnapshotCommandClose", "CliCommandCloseWitness", "CliCommandCloseTerminal::Fault(error)"])
    if (!cli.includes(required)) failures.push(`CLI mounted command close missing: ${required}`);
  for (const marker of ["impl std::future::Future for MountedWalRecordCommandClose", "impl std::future::Future for MountedWalBatchCommandClose", "impl std::future::Future for MountedWalReplayCommandClose", "impl std::future::Future for MountedSnapshotCommandClose"]) {
    const close = rustItem(cli, marker);
    if (!close.includes("self.terminal") || close.includes(".expect(")) failures.push(`CLI mounted close lacks a deterministic repeat-poll terminal witness: ${marker}`);
  }
  const replayDecode = rustItem(artifact, "impl Future for ArtifactWalEnvelopeDecode");
  for (const required of ["ArtifactWalTextField", "ArtifactWalPageField", "dependency_count", "std::task::Poll::Pending", "owner.page.poll", "ARTIFACT_WAL_DEPENDENCIES"])
    if (!`${artifact} ${replayDecode}`.includes(required)) failures.push(`artifact WAL retained decoder missing: ${required}`);
  const replayDecodeBody = replayDecode.slice(replayDecode.indexOf("{") + 1);
  if (/Vec::with_capacity|decode_protocol_field|while\b|\bfor\b/.test(replayDecodeBody)) failures.push("artifact WAL retained decoder restores a synchronous or preallocated owner loop");
  const replayAdapter = rustItem(artifact, "impl Future for ArtifactWalEnvelopeAdapter");
  for (const required of ["owner.control.grant()", "retained.diff_payload.page(owner.page)", "retained.diff_payload.close_step()?", "retained.inverse_payload.page(owner.page)", "retained.inverse_payload.close_step()?", "std::task::Poll::Pending"])
    if (!replayAdapter.includes(required)) failures.push(`artifact WAL schema adapter lacks one-entry/page/close progress: ${required}`);
  const replayOpen = rustItem(artifact, "pub async fn open_retained");
  if (!replayOpen.includes("decode_retained_envelope(bytes, &mut control).await?") || !replayOpen.includes("adapt_retained_envelope(retained, &mut control).await?")) failures.push("artifact live WAL replay bypasses the retained decoder/schema adapter");
  for (const [name, source] of [["artifact", rustItem(artifact, "async fn apply_entries")], ["compact", compact], ["CLI wal-inspect", rustItem(cli, "async fn cmd_wal_inspect")], ["CLI snapshot-inspect", rustItem(cli, "async fn cmd_snapshot_inspect")], ["CLI replay", rustItem(cli, "async fn cmd_replay")], ["CLI migration", rustItem(cli, "async fn cmd_migrate")]] as const)
    if (hasTerminalCloseSweep(source)) failures.push(`${name} production retains a loop/for/while whole-owner close sweep`);
  for (const [name, raw, fixture] of [
    ["pack", packSource, "identity_chunk_cursor_retains_fragment_progress_and_terminal_verification"],
    ["WAL", walSource, "wal_replay_cancel_resume_close_and_fragment_crc_are_deterministic"],
    ["query", querySource, "interrupted_query_rows_drop_retains_one_resumable_close_owner"],
    ["engine", engineSource, "interrupted_query_stream_drop_retains_one_resumable_close_owner"],
    ["artifact", artifactSource, "artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless"],
    ["artifact WAL", artifactSource, "retained_wal_decoder_covers_pending_cancel_deadline_corrupt_max_and_max_plus_one"],
    ["compaction", compactSource, "compaction_fixed_pages_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_return_exact_credit"],
    ["CLI", cliSource, "cli_command_close_success_refusal_cancel_stale_fault_drop_interrupted_and_max_plus_one_have_exact_exit_witnesses"],
  ] as const) if (!raw.includes(fixture)) failures.push(`${name} hostile retained-close fixture missing: ${fixture}`);
  for (const [name, raw, fixture] of [
    ["state", stateSource, "retained_state_exact_backing_cancel_capacity_and_close_are_hostile"],
    ["WAL bytes", walSource, "wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner"],
    ["index bytes", indexSource, "exact_backing_handback_cancel_close_and_fragment_order_are_deterministic"],
  ] as const) {
    if (!raw.includes(fixture)) failures.push(`${name} hostile external-owner fixture missing: ${fixture}`);
    const law = rustItem(raw, `async fn ${fixture}`);
    for (const evidence of ["Vec::with_capacity", "as_ptr()", "capacity()", "cancelled.store", "std::time::Instant::now()", "cursor deadline reached", "into_source().unwrap()"])
      if (!law.includes(evidence)) failures.push(`${name} hostile external-owner law lacks exact capacity/cancel/deadline/handback evidence: ${evidence}`);
  }
  const artifactLaw = rustItem(artifactSource, "async fn artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless");
  for (const evidence of ["StateEntry::try_admit", "state cursor cancelled", "state cursor deadline reached", "ARTIFACT_STATE_RETIREMENT_OVERFLOW", "ARTIFACT_STATE_RETIREMENT_QUARANTINE", "ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT", "reserve_artifact_state_retirement", "Some(source)", "second_source", "exact-all-tier-state-refusal", "state.apply_entries"])
    if (!artifactLaw.includes(evidence)) failures.push(`artifact hostile law lacks exact production-path evidence: ${evidence}`);
  const compactLaw = rustItem(compactSource, "async fn compaction_fixed_pages_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_return_exact_credit");
  for (const evidence of ["unwrap_err", "exact_operation", "second_operation", "COMPACTION_PAGE_RETIREMENT_OVERFLOW", "COMPACTION_PAGE_RETIREMENT_QUARANTINE", "COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT", "retirement.map(|reservation| reservation.tier)", "Some(2)", "MountedCompactionPageClose", "exact-all-tier-compaction-refusal", "exact_refusal_operation"])
    if (!compactLaw.includes(evidence)) failures.push(`compaction hostile law lacks exact production-path evidence: ${evidence}`);
  const queryLaw = rustItem(querySource, "async fn interrupted_query_rows_drop_retains_one_resumable_close_owner");
  for (const evidence of ["QUERY_RETIRED_ROWS_OVERFLOW", "QUERY_RETIRED_ROWS_QUARANTINE", "QUERY_RETIREMENT_PRESSURE_FAULT", "retirement.map(|reservation| reservation.tier)", "Some(2)", "0x5155_4552_59", "0x5155_4552_5a", "0x5155_4552_5b", "retire_query_rows(exact)"])
    if (!queryLaw.includes(evidence)) failures.push(`query hostile law lacks saturation identity/recovery evidence: ${evidence}`);
  const engineLaw = rustItem(engineSource, "fn interrupted_query_stream_drop_retains_one_resumable_close_owner");
  for (const evidence of ["ENGINE_QUERY_RETIREMENT_OVERFLOW", "ENGINE_QUERY_RETIREMENT_QUARANTINE", "ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT", "retirement.map(|reservation| reservation.tier)", "Some(2)", "exact-overflow-stream", "second-overflow-stream", "exact-all-tier-stream-refusal", "retire_engine_query_stream(exact)"])
    if (!engineLaw.includes(evidence)) failures.push(`engine hostile law lacks saturation identity/recovery evidence: ${evidence}`);
  const cliLaw = rustItem(cliSource, "async fn cli_command_close_success_refusal_cancel_stale_fault_drop_interrupted_and_max_plus_one_have_exact_exit_witnesses");
  for (const evidence of ["Future::poll", "Poll::Pending", "Poll::Ready(Ok(CliCommandCloseWitness", "Poll::Ready(Err(error))", "error == fault", "CLI_COMMAND_CLOSE_INTERRUPTED", "terminal: Some(CliCommandCloseTerminal::Fault", "unwrap_err"])
    if (!cliLaw.includes(evidence)) failures.push(`CLI hostile law lacks Ready/Pending/refusal/drop evidence: ${evidence}`);
  const replayLaw = rustItem(artifactSource, "async fn retained_wal_decoder_covers_pending_cancel_deadline_corrupt_max_and_max_plus_one");
  for (const evidence of ["poll_artifact_wal_decode", "is_pending", "wal cursor cancelled", "wal cursor deadline reached", "WAL command envelope has trailing bytes", "ARTIFACT_WAL_DEPENDENCIES", "ARTIFACT_WAL_FIELD_BYTES", "max-plus-one", "db_io_maintenance_step"])
    if (!replayLaw.includes(evidence)) failures.push(`artifact WAL hostile decoder law lacks real-path evidence: ${evidence}`);
  for (const evidence of ["cli_full_cycle_succeeds_for_a_seeded_document", "cli_verify_fails_on_a_torn_wal_tail_and_repair_fixes_it", "cli_migrate_appends_a_migration_record_visible_to_wal_inspect"])
    if (!cliSource.includes(evidence)) failures.push(`CLI R4 command-path law missing: ${evidence}`);
  return failures;
}



export function interactivityDbIoCallerMigrationFailures(sources: readonly (readonly [string, string])[]): string[] {
  const failures: string[] = [];
  for (const [file, raw] of sources) {
    const source = interactivityProductionSource(raw);
    for (const forbidden of ["run_blocking_op", "DbIoRequest", "DbIoPages::try_new", "DbIoPages::try_range", ".into_vec()", "MemoryStorage::default()"])
      if (source.includes(forbidden)) failures.push(`DB caller ${file} retains removed raw/generic I/O boundary: ${forbidden}`);
  }
  return failures;
}



export function interactivityDbIoDirectWriterFailures(engineSource: string, snapshotSource: string, indexSource: string, walSource: string, sprSource: string, packSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const snapshot = interactivityProductionSource(snapshotSource);
  const index = interactivityProductionSource(indexSource);
  const wal = interactivityProductionSource(walSource);
  const spr = interactivityProductionSource(sprSource);
  const pack = interactivityProductionSource(packSource);
  const failures: string[] = [];
  for (const required of ["async fn encode_catalog_pages", "DbIoPageWriter::try_reserve", "catalog_write_json_string", "writer.seal_retained().await"])
    if (!engine.includes(required)) failures.push(`catalog direct retained writer missing: ${required}`);
  for (const forbidden of ["serde_json::to_vec", "db_io_copy_pages(&encoded", "db_io_copy_pages(&empty"])
    if (engine.includes(forbidden)) failures.push(`catalog production retains encode-then-copy path: ${forbidden}`);
  for (const required of ["struct SnapshotPageSink", "SnapshotPageSink::try_new()?", "pub async fn build_generation_pages", "writer.seal_retained().await", "descriptor.write_retained(&mut descriptor_segment).await?", "begin_identity_segment(pack::KIND_SNAPSHOT"])
    if (!snapshot.includes(required)) failures.push(`snapshot direct retained sink missing: ${required}`);
  for (const forbidden of ["PackWriter::begin(Vec", "db_io_copy_pages(&encoded", "db_io_copy_pages(&bytes", "let descriptor_bytes = descriptor.encode().await"])
    if (snapshot.includes(forbidden)) failures.push(`snapshot production retains encode-then-copy path: ${forbidden}`);
  for (const required of ["async fn encode_run_pages", "DbIoPageWriter::try_reserve", "Crc32cCursor", "run_write", "writer.seal_retained().await"])
    if (!index.includes(required)) failures.push(`index direct retained writer missing: ${required}`);
  for (const forbidden of ["db_io_copy_pages(&encoded", "db_io_copy_pages(&bytes"])
    if (index.includes(forbidden)) failures.push(`index production retains encode-then-copy path: ${forbidden}`);
  for (const required of ["struct SharedBuf(std::sync::Arc<std::sync::Mutex<db_storage::DbIoPageWriter>>)", "fn try_new() -> Result<Self, DbError>", "async fn copy_range", "async fn read_exact", "write_fragment(&bytes[cursor..])", "record.write_retained(&mut self.writer).await?"])
    if (!wal.includes(required)) failures.push(`WAL direct retained page sink missing: ${required}`);
  for (const forbidden of ["Mutex<Vec<u8>>", ".snapshot()", "db_io_copy_pages(&snapshot", "SharedBuf::new()"])
    if (wal.includes(forbidden)) failures.push(`WAL production retains whole-buffer path: ${forbidden}`);
  for (const required of ["async fn write_frame_retained", "Crc32cCursor::new()", "pending_chain_hasher: blake3::Hasher", "write_frame_retained(&mut self.sink"])
    if (!spr.includes(required)) failures.push(`WAL frame codec retained cursor missing: ${required}`);
  for (const forbidden of ["scratch: Vec<u8>", "pending_digests: Vec", "encode_frame_into(&mut self.scratch", "let mut concat = Vec::with_capacity"])
    if (spr.includes(forbidden)) failures.push(`WAL frame codec retains whole-frame/list allocation: ${forbidden}`);
  for (const required of ["pub struct PackIdentitySegment", "pub async fn begin_identity_segment", "let chunks = std::mem::take(&mut self.chunks)", "segment.write_fragment(retained_varint", "self.begin_identity_segment(semio_framework_replication_codec::KIND_CHUNK"])
    if (!pack.includes(required)) failures.push(`snapshot pack codec retained cursor missing: ${required}`);
  for (const forbidden of ["let table_bytes = encode_chunk_table", "encode_segment(semio_framework_replication_codec::KIND_CHUNK_TABLE", "encode_segment(semio_framework_replication_codec::KIND_MANIFEST"])
    if (pack.includes(forbidden)) failures.push(`snapshot pack codec retains whole-table/segment copy: ${forbidden}`);
  return failures;
}



export function interactivityDbIoCursorCallerFailures(syncSource: string): string[] {
  const sync = interactivityProductionSource(syncSource);
  const followUp = sync.slice(sync.indexOf("enum DatabaseSyncHelloFollowUp"), sync.indexOf("struct DatabaseSyncHelloPrepared"));
  const execute = sync.slice(sync.indexOf("async fn database_sync_hello_execute"), sync.indexOf("type DatabaseSyncHelloExecutionFuture"));
  const failures: string[] = [];
  if (sync.includes("db_io_prepare_platform(pages)")) failures.push("DB bootstrap retains a whole repository copy path: db_io_prepare_platform(pages)");
  for (const forbidden of [".chunks(snapshot_chunk_bytes)", "let chunks: Vec<&[u8]>", "chunk.to_vec()", "bytes.to_vec()", "while ", "for "])
    if (followUp.includes(forbidden)) failures.push(`DB bootstrap retains a whole repository copy path: ${forbidden}`);
  for (const required of [
    "enum DatabaseSyncHelloFollowUp",
    "Snapshot { pages: db_io_core::DbIoPages, chunk_bytes: usize, offset: usize, page: u8, page_offset: usize, seq: u32, chunk: Option<protocol::SnapshotChunkBytes>, done: bool }",
    "fn drive_one_with_grant(",
    "grant.check(cancelled, expired)?",
    "let unit_bytes = (*chunk_bytes).min(DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES)",
    "let fragment = pages.page(*page)",
    "let copied = remaining.min(fragment.len().saturating_sub(*page_offset))",
    "target.try_extend_from_slice(&fragment[*page_offset..*page_offset + copied])",
    "*page = page.checked_add(1)",
    "let bytes = chunk.take()",
    "return Ok(Some(Some(frame)))",
  ]) if (!followUp.includes(required)) failures.push(`DB bootstrap retained page cursor missing: ${required}`);
  if (!execute.includes("for page in 0..pages.page_count()") || !execute.includes("hash.update(fragment)") || !execute.includes("database_sync_hello_opportunity(&cancelled, &expired).await")) failures.push("DB bootstrap retained page hash cursor missing");
  return failures;
}



export function interactivityArtifactSubmitFailures(engineSource: string, artifactSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const artifact = interactivityProductionSource(artifactSource);
  const handle = engine.slice(engine.indexOf("//#region 🔖️ArtifactHandle"), engine.indexOf("//#endregion 🔖️ArtifactHandle"));
  const drive = handle.slice(handle.indexOf("fn drive_one(self: Arc<Self>, generation: u64)"), handle.indexOf("fn close_one", handle.indexOf("fn drive_one(self: Arc<Self>, generation: u64)")));
  const runner = artifact.slice(artifact.indexOf("type ArtifactBuildFuture"), artifact.indexOf("//#region 🧪️Tests", artifact.indexOf("type ArtifactBuildFuture")));
  const runnerCloseStep = runner.slice(runner.indexOf("pub fn close_step"), runner.indexOf("pub fn terminal_is_empty", runner.indexOf("pub fn close_step")));
  const runnerTerminalClose = runner.slice(runner.indexOf("pub fn close(mut self)"), runner.indexOf("impl Drop for ArtifactRunnerTerminalJob", runner.indexOf("pub fn close(mut self)")));
  const failures: string[] = [];
  for (const forbidden of ["block_on(", "ask_blocking", "submit_blocking", "thread::spawn", "WorkerPool::new("]) if (handle.includes(forbidden) || runner.includes(forbidden)) failures.push(`live ArtifactHandle submit route retains ${forbidden}`);
  if (!handle.includes("const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024") || !handle.includes("const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = 64") || !handle.includes("const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = 1024") || !handle.includes("const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64") || !handle.includes("ArtifactSubmitAdmission::try_claim(items, bytes)")) failures.push("ArtifactHandle submit lacks fixed item and byte admission before request transfer");
  if (!handle.includes("batch.envelopes.capacity()") || !handle.includes("envelope.dependencies.capacity()") || !handle.includes("envelope.diff.payload.capacity()") || !handle.includes("envelope.inverse.payload.capacity()") || !handle.includes("dependency.0.capacity()")) failures.push("ArtifactHandle submit does not preflight every nested owner byte");
  if (!drive.includes("if generation != self.generation") || !drive.includes("if self.authority.generation() != self.authority_generation") || drive.indexOf("if generation != self.generation") > drive.indexOf("self.scheduled.store")) failures.push("ArtifactHandle submit generation freshness follows mutable scheduling state");
  if (!drive.includes("ArtifactSubmitWorkOwner::Request") || !drive.includes("self.authority.submit_retained") || !drive.includes("std::pin::Pin::new(future).poll(&mut context)") || drive.includes("loop {") || drive.includes("while ")) failures.push("ArtifactHandle submit grant is not one retained request handoff or one future poll");
  if (!handle.includes("impl std::task::Wake for ArtifactSubmitWake") || !handle.includes("self.generation == state.generation") || !handle.includes("self.scheduled.compare_exchange(false, true") || !handle.includes("SubmitProgress::Waiting")) failures.push("ArtifactHandle late readiness or wake storm is not generation-keyed and one-shot");
  if (!handle.includes("self.pool.try_submit(Lane::Io, job)") || !handle.includes("error.into_job()") || !handle.includes("self.pool.callback_at") || !handle.includes("retry_generation") || !handle.includes("ARTIFACT_SUBMIT_RETRY_LIMIT")) failures.push("ArtifactHandle quiet saturation lacks exact closure handback and coalesced timer-wheel retry");
  for (const terminal of ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "pub fn resume(mut self)"]) if (!handle.includes(terminal)) failures.push(`ArtifactHandle terminal authority missing ${terminal}`);
  if (!runner.includes("ArtifactBuildFuture") || !runner.includes("ArtifactTurnFuture") || !runner.includes("future.as_mut().poll(&mut context)") || !runner.includes("Self::start_turn(engine, envelope.payload)") || !runner.includes("self.pool.try_submit(semio_framework_async::Lane::UserVisible, job)") || !runner.includes("self.pool.callback_at")) failures.push("ArtifactRunner does not retain one build/turn poll with exact pool retry");
  if (!runnerCloseStep.includes("let closed =") || !runnerCloseStep.includes("if closed") || !runnerCloseStep.includes("terminal.take()") || !runnerTerminalClose.includes("let closed =") || !runnerTerminalClose.includes("if !closed") || !runnerTerminalClose.includes("terminal_job")) failures.push("ArtifactRunner terminal close can drop the last runner owner before its retained cursor is released");
  if (!engine.includes("ArtifactEngine::create_retained") || !engine.includes("ArtifactEngine::open_retained") || engine.includes("move || db_artifact::ArtifactEngine::create(")) failures.push("live database authority construction reaches a synchronous ArtifactEngine wrapper");
  if (!artifact.includes("pub fn submit_retained") || !artifact.includes("self.address.ask(Priority::Command") || !artifact.includes("pub async fn query") || !artifact.includes("pub async fn frontier")) failures.push("ArtifactAuthority retains a blocking mailbox compatibility path on the live submit route");
  for (const fixture of [
    "artifact_submit_late_readiness_parks_then_one_shot_wake_reschedules",
    "artifact_submit_pool_saturation_without_later_ingress_retains_exact_job",
    "artifact_submit_cancel_before_during_after_preserves_exact_owner",
    "artifact_submit_stale_generation_and_slot_aba_cannot_consume_current_work",
    "artifact_submit_missing_handle_terminalizes_without_mailbox_mutation",
    "artifact_submit_terminal_job_work_result_take_resume_and_close_one_owner",
    "artifact_submit_item_cap_plus_one_and_nested_bytes_plus_one_return_owner",
    "artifact_runner_one_grant_polls_one_turn_and_never_blocks_on",
  ]) if (!engineSource.includes(fixture) && !artifactSource.includes(fixture)) failures.push(`ArtifactHandle retained-submit fixture missing: ${fixture}`);
  return failures;
}



export function interactivityDatabaseCapabilityOpenFailures(engineSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const capability = engine.slice(engine.indexOf("//#region 🔖️CapabilityOpen"), engine.indexOf("//#endregion 🔖️CapabilityOpen"));
  const openWith = engine.slice(engine.indexOf("async fn open_with"), engine.indexOf("pub async fn create_document", engine.indexOf("async fn open_with")));
  const waits = engine.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const failures: string[] = [];
  const expectedWaits = engine.includes("DatabaseSyncHelloFuture::try_submit") ? 0 : engine.includes("db_compact::DatabaseCompactionFuture::try_submit") ? 1 : 2;
  if (waits !== expectedWaits || engine.includes("db_actor::block_on(storage.capabilities())")) failures.push("DB retained open packets do not preserve the post-P1z production waits");
  if (!openWith.includes("Self::open_retained(pool.clone(), storage)") || !openWith.includes("capability_probe.await?.into_parts()") || !openWith.includes("Err(rejected) => return Err(rejected.close_and_take_error())") || openWith.includes("rejected.into_parts().0") || openWith.includes("storage.capabilities().await")) failures.push("Database open does not consume the retained probe or explicitly close the exact rejected storage owner");
  for (const required of [
    "const DATABASE_CAPABILITY_OPEN_SLOTS: usize = 64",
    "const DATABASE_CAPABILITY_OPEN_ITEMS: u64 = 8",
    "const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024",
    "DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS",
    "DATABASE_CAPABILITY_OPEN_TOTAL_BYTES",
    "DatabaseCapabilityOpenAdmission::try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES)",
    "database_capability_open_registry",
  ]) if (!capability.includes(required)) failures.push(`database capability-open fixed admission missing ${required}`);
  for (const phase of ["Handoff", "Poll", "RetainWork", "DrainWork", "ReleaseWork", "RetainResult", "Publish", "Terminal"])
    if (!capability.includes(phase)) failures.push(`database capability-open retained phase missing ${phase}`);
  const poll = capability.slice(capability.indexOf("fn poll_backend_once"), capability.indexOf("fn stage_terminal"));
  if (!poll.includes("work.poll(&mut context)") || !poll.includes("std::panic::catch_unwind") || poll.includes("loop {") || poll.includes("while ")) failures.push("database capability-open backend work is not one panic-safe poll per I/O-lane grant");
  const pendingPoll = poll.slice(poll.indexOf("Ok(std::task::Poll::Pending)"), poll.indexOf("Ok(std::task::Poll::Ready"));
  const readyPoll = poll.slice(poll.indexOf("Ok(std::task::Poll::Ready"), poll.indexOf("Err(_)"));
  const faultPoll = poll.slice(poll.indexOf("Err(_)"));
  const pollTransition = capability.slice(capability.indexOf("fn publish_poll_terminal"), capability.indexOf("fn poll_backend_once"));
  if (
    !pollTransition.includes("self.cancelled.store(true")
    || !pollTransition.includes("self.terminal_error.lock()")
    || !pollTransition.includes("self.set_phase(DatabaseCapabilityOpenPhase::RetainWork)")
    || !pollTransition.includes("fn release_terminal_poll")
    || pollTransition.indexOf("self.polling.store(false") > pollTransition.indexOf("self.schedule_cleanup()")
    || pendingPoll.indexOf("Some(work)") < 0
    || pendingPoll.indexOf("Some(work)") > pendingPoll.indexOf("self.poll_terminal_if_cancelled_or_stale()")
    || pendingPoll.indexOf("Some(work)") > pendingPoll.indexOf("self.polling.store(false")
    || readyPoll.indexOf("Some(work)") < 0
    || readyPoll.indexOf("Some(output)") < 0
    || readyPoll.indexOf("Some(work)") > readyPoll.indexOf("self.set_phase(DatabaseCapabilityOpenPhase::RetainWork)")
    || readyPoll.indexOf("Some(output)") > readyPoll.indexOf("self.set_phase(DatabaseCapabilityOpenPhase::RetainWork)")
    || readyPoll.indexOf("self.set_phase(DatabaseCapabilityOpenPhase::RetainWork)") > readyPoll.indexOf("self.polling.store(false")
    || faultPoll.indexOf("Some(work)") < 0
    || faultPoll.indexOf("Some(work)") > faultPoll.indexOf("self.publish_poll_terminal(")
    || faultPoll.indexOf("self.publish_poll_terminal(") > faultPoll.indexOf("self.release_terminal_poll()")
    || faultPoll.includes("self.polling.store(false")
  ) failures.push("database capability-open poll transition re-enables wake before exact work/result/fault/phase publication");
  const drive = capability.slice(capability.indexOf("fn drive_one"), capability.indexOf("fn poll_backend_once"));
  if (!drive.includes("match self.phase()") || !drive.includes("DatabaseCapabilityOpenPhase::Handoff") || !drive.includes("DatabaseCapabilityOpenPhase::Poll") || !drive.includes("DatabaseCapabilityOpenPhase::Publish")) failures.push("database capability-open callback does not persistently separate handoff, poll, retention, and publication");
  const handoff = drive.slice(drive.indexOf("DatabaseCapabilityOpenPhase::Handoff"), drive.indexOf("DatabaseCapabilityOpenPhase::Poll"));
  if (handoff.includes(".poll(")) failures.push("database capability-open handoff grant also polls backend work");
  const publish = capability.slice(capability.indexOf("fn publish_staged"), capability.indexOf("fn roots_are_empty"));
  if (!publish.includes("self.cancelled.load") || !publish.includes("!self.is_current()") || publish.indexOf("self.cancelled.load") > publish.indexOf("self.complete(") || publish.indexOf("!self.is_current()") > publish.indexOf("self.complete(")) failures.push("database capability-open publication does not revalidate cancellation and admission generation immediately before result exposure");
  const retryStart = capability.indexOf("fn arm_retry");
  const retryEnd = capability.indexOf("fn drive_one", retryStart);
  const retry = capability.slice(retryStart, retryEnd < 0 ? capability.length : retryEnd);
  if (!capability.includes("self.pool.try_submit(Lane::Io, job)") || !capability.includes("error.into_job()") || !retry.includes("self.pool.callback_at") || !retry.includes("advance_retry_generation_observed_once") || !retry.includes("compare_exchange(current, generation") || !retry.includes("observed == state.retry_generation.load") || retry.includes("loop {") || !capability.includes("generation != self.generation")) failures.push("database capability-open quiet saturation/retry lacks exact retained job, generation, or one bounded compare-exchange opportunity");
  for (const required of [
    "pub struct DatabaseCapabilityOpenRejected",
    "pub fn take_storage(&mut self) -> Option<Arc<db_storage::DbBackend>>",
    "pub fn retry(mut self, pool: Arc<WorkerPool>)",
    "pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep",
    "pub fn close_and_take_error(mut self) -> DbError",
    "pub struct DatabaseCapabilityOpenTerminalHandle",
    "pub struct DatabaseCapabilityOpenTerminalResult",
    "pub fn take_database_capability_open_terminal",
    "pub fn take_next_database_capability_open_terminal",
    "pub fn take_result(&self) -> Option<DatabaseCapabilityOpenTerminalResult>",
    "terminal_result_checked_out",
    "pub fn resume(self) -> Result<DatabaseCapabilityOpenFuture, Self>",
    "pub fn close_step(&self) -> DatabaseCapabilityOpenCloseStep",
    "pub fn terminal_is_empty(&self) -> bool",
  ]) if (!capability.includes(required)) failures.push(`database capability-open exact rejection/terminal authority missing ${required}`);
  if (!capability.includes("DatabaseCapabilityOpenResult { storage, capabilities }") || !capability.includes("terminal_work") || !capability.includes("terminal_result") || !capability.includes("terminal_completion") || !capability.includes("roots_are_empty")) failures.push("database capability-open does not retain exact backend/output roots through public terminal cleanup");
  for (const forbidden of ["db_actor::block_on", "semio_framework_async::block_on", "WorkerPool::new(", "thread::spawn", "open_inline"])
    if (capability.includes(forbidden)) failures.push(`database capability-open retained route contains ${forbidden}`);
  for (const fixture of [
    "database_capability_open_fixed_admission_cap_plus_one_and_generation_aba",
    "database_capability_open_success_returns_exact_storage_owner_and_scalar",
    "database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close",
    "database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal",
    "database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary",
    "database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result",
    "database_capability_open_rejection_take_retry_and_close_preserve_exact_storage",
    "database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback",
    "database_capability_open_retry_contention_is_one_compare_exchange_per_callback",
  ]) if (!engineSource.includes(fixture)) failures.push(`database capability-open fixture missing ${fixture}`);
  const wakeFixtureStart = engineSource.lastIndexOf("async fn database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary");
  const wakeFixtureEnd = engineSource.indexOf("#[semio_framework_async_macros::async_test]", wakeFixtureStart + 1);
  const wakeFixture = wakeFixtureStart < 0 ? "" : engineSource.slice(wakeFixtureStart, wakeFixtureEnd < 0 ? engineSource.length : wakeFixtureEnd);
  if (
    !engineSource.includes("slots: [Option<semio_framework_async::Job>; 8]")
    || !wakeFixture.includes("controlled_submit_hook")
    || !wakeFixture.includes("state.schedule();")
    || wakeFixture.includes("state.scheduled.store(true")
    || !wakeFixture.includes("let initial =")
    || !wakeFixture.includes("initial();")
    || !wakeFixture.includes("let successor =")
    || !wakeFixture.includes("successor();")
    || !wakeFixture.includes("wake-after-release cannot admit a duplicate successor")
    || !wakeFixture.includes("Pending is repolled only by its next governed successor")
    || !wakeFixture.includes("terminal Ready/panic successor advances cleanup without repolling")
    || !wakeFixture.includes("terminal_work")
    || !wakeFixture.includes("staged_result")
  ) failures.push("database capability-open controlled wake fixture masks scheduling or does not execute and inspect the exact coalesced successor");
  return failures;
}



export function interactivityDatabaseCatalogReadFailures(engineSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const catalog = engine.slice(engine.indexOf("//#region 🔖️CatalogRootRead"), engine.indexOf("//#endregion 🔖️CatalogRootRead"));
  const openWith = engine.slice(engine.indexOf("async fn open_with"), engine.indexOf("pub async fn create_document", engine.indexOf("async fn open_with")));
  const waits = engine.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const failures: string[] = [];
  const expectedWaits = engine.includes("DatabaseSyncHelloFuture::try_submit") ? 0 : engine.includes("db_compact::DatabaseCompactionFuture::try_submit") ? 1 : 2;
  if (waits !== expectedWaits || engine.includes("db_actor::block_on(async { storage.catalog().await.read_root().await })")) failures.push("DB catalog-root retained read does not preserve the post-P1z production waits");
  if (!openWith.includes("Self::open_catalog_read_retained(pool.clone(), storage)") || !openWith.includes("catalog_probe.await?.into_parts()") || !openWith.includes("Err(rejected) => return Err(rejected.close_and_take_error(pool.clone()))")) failures.push("Database open does not consume the retained catalog-root authority and mount the exact rejection on the process pool");
  for (const required of [
    "const DATABASE_CATALOG_READ_SLOTS: usize = 64",
    "const DATABASE_CATALOG_READ_ITEMS: u64 = 8",
    "const DATABASE_CATALOG_READ_BYTES: u64 = 64 * 1024",
    "DATABASE_CATALOG_READ_TOTAL_ITEMS",
    "DATABASE_CATALOG_READ_TOTAL_BYTES",
    "DatabaseCatalogReadAdmission::try_claim()",
    "database_catalog_read_registry",
    "DatabaseCatalogRootKey",
  ]) if (!catalog.includes(required)) failures.push(`database catalog-read admission missing ${required}`);
  for (const phase of ["Handoff", "Poll", "RetainWork", "DrainWork", "ReleaseWork", "RetainResult", "Publish", "Terminal"])
    if (!catalog.includes(phase)) failures.push(`database catalog-read phase missing ${phase}`);
  const drive = catalog.slice(catalog.indexOf("fn drive_one"), catalog.indexOf("fn poll_backend_once"));
  const handoff = drive.slice(drive.indexOf("DatabaseCatalogReadPhase::Handoff"), drive.indexOf("DatabaseCatalogReadPhase::Poll"));
  if (!drive.includes("match self.phase()") || handoff.includes(".poll(")) failures.push("catalog handoff combines a backend poll with owner transfer");
  const work = catalog.slice(catalog.indexOf("impl DatabaseCatalogReadWork"), catalog.indexOf("enum DatabaseCatalogReadPhase"));
  if (!work.includes("storage.catalog().await.read_root().await") || !work.includes("DatabaseCatalogReadResult { storage, key, root }") || work.includes("block_on")) failures.push("catalog backend future does not return exact storage/key/root owners");
  const poll = catalog.slice(catalog.indexOf("fn poll_backend_once"), catalog.indexOf("fn publish_terminal"));
  if (!poll.includes("std::panic::catch_unwind") || !poll.includes("work.poll(&mut context)") || poll.includes("loop {") || poll.includes("while ")) failures.push("catalog backend is not one panic-safe poll per I/O opportunity");
  for (const branch of ["Poll::Pending", "Poll::Ready", "Err(_)"]) if (!poll.includes(branch)) failures.push(`catalog backend poll branch missing ${branch}`);
  if (!poll.includes("Some(work)") || !poll.includes("Some(result)") || !poll.includes("DATABASE_CATALOG_READ_BYTES as usize") || !poll.includes("self.release_terminal_poll()")) failures.push("catalog poll does not retain work/result and byte-cap fault before wake release");
  if (!catalog.includes("impl std::task::Wake for DatabaseCatalogReadWake") || !catalog.includes("wake_requested") || !catalog.includes("self.scheduled.compare_exchange(false, true") || !catalog.includes("self.pool.try_submit(Lane::Io, job)") || !catalog.includes("error.into_job()") || !catalog.includes("self.pool.callback_at")) failures.push("catalog wake/saturation does not coalesce and retain the exact job");
  if (!catalog.includes("if generation != self.generation") || !catalog.includes("self.cancelled.load") || !catalog.includes("!self.is_current()") || catalog.indexOf("!self.is_current()", catalog.indexOf("fn publish_staged")) > catalog.indexOf("let result =", catalog.indexOf("fn publish_staged"))) failures.push("catalog cancellation/generation freshness does not precede publication");
  const publicPoll = catalog.slice(catalog.indexOf("impl Future for DatabaseCatalogReadFuture"), catalog.indexOf("impl Drop for DatabaseCatalogReadFuture"));
  const publicWaker = publicPoll.indexOf("*self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone())");
  const publicRecheck = publicPoll.indexOf("let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()", publicWaker + 1);
  if (publicWaker < 0 || publicRecheck < publicWaker || !catalog.includes("controlled_publication_before_waker_hook") || !catalog.includes("fn publish_public_completion")) failures.push("catalog public future can lose completion between its first check and consumer waker registration");
  if (/derive\([^\)]*(?:Clone|Copy|Default)[^\)]*\)\]\s*pub struct DatabaseCatalogRootKey/.test(catalog)) failures.push("catalog root key is cloneable/copyable/defaultable");
  for (const required of [
    "pub struct DatabaseCatalogReadRejected",
    "pub fn retry(mut self, pool: Arc<WorkerPool>)",
    "pub fn close_step(&mut self) -> DatabaseCatalogReadCloseStep",
    "pub fn close_and_take_error(self, pool: Arc<WorkerPool>) -> DbError",
    "pub struct DatabaseCatalogReadFuture",
    "pub struct DatabaseCatalogReadTerminalHandle",
    "pub struct DatabaseCatalogReadTerminalResult",
    "pub fn take_database_catalog_read_terminal",
    "pub fn take_result(&self) -> Option<DatabaseCatalogReadTerminalResult>",
    "pub fn resume(self) -> Result<DatabaseCatalogReadFuture, Self>",
    "terminal_result_checked_out",
    "roots_are_empty",
  ]) if (!catalog.includes(required)) failures.push(`catalog exact public terminal ownership missing ${required}`);
  const rejected = catalog.slice(catalog.indexOf("impl DatabaseCatalogReadRejected"), catalog.indexOf("type DatabaseCatalogReadBackendFuture"));
  const rejectedClose = rejected.slice(rejected.indexOf("pub fn close_step"), rejected.indexOf("pub fn terminal_is_empty"));
  const storageBranch = rejectedClose.slice(rejectedClose.indexOf("if let Some(storage)"), rejectedClose.indexOf("else if let Some(key)"));
  if (storageBranch.includes("key.take()") || !rejectedClose.includes("else if let Some(key) = self.key.take()") || !rejected.includes("DatabaseCatalogReadRejectedClose::mount(pool, self, schedule)") || !rejected.includes("self.pool.try_submit(Lane::Io, job)") || !rejected.includes("error.into_job()") || !rejected.includes("owner.close_step()")) failures.push("catalog rejected storage/key owners are not mounted and retired on distinct process-pool grants");
  for (const forbidden of ["db_actor::block_on", "semio_framework_async::block_on", "WorkerPool::new(", "thread::spawn", "open_inline"])
    if (catalog.includes(forbidden)) failures.push(`catalog retained region contains ${forbidden}`);
  for (const fixture of [
    "database_catalog_read_fixed_cap_plus_one_and_generation_aba",
    "database_catalog_read_success_returns_exact_storage_key_and_root",
    "database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls",
    "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
    "database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants",
    "database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key",
    "database_catalog_read_terminal_result_drop_hands_back_exact_result",
  ]) if (!engineSource.includes(fixture)) failures.push(`catalog retained fixture missing ${fixture}`);
  const controlledAt = engineSource.indexOf("async fn database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls");
  const controlled = controlledAt < 0 ? "" : engineSource.slice(controlledAt, engineSource.indexOf("#[semio_framework_async_macros::async_test]", controlledAt + 1));
  if (!controlled.includes("state.schedule();") || controlled.includes("state.scheduled.store(true") || !controlled.includes("initial();") || !controlled.includes("successor();") || !controlled.includes("one coalesced catalog-read successor") || !controlled.includes("late wake cannot overtake")) failures.push("catalog controlled wake fixture masks or bypasses the production successor path");
  return failures;
}
//#endregion 🔖️InteractivityAudit
