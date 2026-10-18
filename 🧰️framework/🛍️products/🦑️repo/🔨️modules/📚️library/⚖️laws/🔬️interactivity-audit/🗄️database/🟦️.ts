import { POLICY_RUST_TEST_EVIDENCE_BOUNDARY } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { interactivityCfgTestItemSpans, policyLineInTestMod, policyTestModSpans } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";

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

/** 🪓️ The text of one region, read as the one region the unsplit file held: from the first `//#region <label>` to the `//#endregion <label>` that no further `//#region <label>` continues, with every junction where a child module reopens the region removed (the closing marker, the skipped doc, `use super::*;` and outer marker lines, and the reopening marker); `null` when the region is absent. */
export function interactivityRegionSource(text: string, label: string): string | null {
  const open = `//#region ${label}`, close = `//#endregion ${label}`;
  const start = text.indexOf(open);
  if (start < 0) return null;
  let from = start, out = "";
  for (;;) {
    const end = text.indexOf(close, from);
    if (end < 0) return null;
    let offset = text.indexOf("\n", end), resume = -1;
    while (offset >= 0 && offset < text.length) {
      const next = text.indexOf("\n", offset + 1), lineEnd = next < 0 ? text.length : next, line = text.slice(offset + 1, lineEnd).trim();
      if (line === "" || line.startsWith("//!") || line === "use super::*;" || line.startsWith("//#endregion ") || line.startsWith("//#region ") && !line.startsWith(open)) {
        if (next < 0) break;
        offset = next;
        continue;
      }
      if (line.startsWith(open)) resume = lineEnd;
      break;
    }
    if (resume < 0) return out + text.slice(from, end);
    out += text.slice(from, end);
    from = resume;
  }
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
  const handle = interactivityRegionSource(engine, "🔖️ArtifactHandle") ?? "";
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
