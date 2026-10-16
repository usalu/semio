/** 🧭️ Tool-job coverage: segmented drain, reserved routes and bounded queues.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { TOOL_JOB_PLUGIN_RESERVED_IDS, toolJobRustBlock } from "../📋️inventory/🟦️.ts";

export function toolJobSegmentedTerminalDrainExact(source: string): boolean {
  const start = source.indexOf("async fn take_segmented_download_chunk(&mut self, operation_id: u64)");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && block.body.includes("let chunk = output.chunks.take_chunk()?") && block.body.includes("if chunk.is_none()") && block.body.includes("self.segmented_downloads.remove(operation_id)") && !block.body.includes("chunks_remaining() == 0") && source.includes("segmented_download_remains_addressable_until_terminal_none_is_observed") && source.includes("take-segmented-download-chunk: async func(instance-id: u32, operation-id: u64) -> result<option<list<u8>>, plugin-error>") && source.includes("async fn take_segmented_download_chunk(instance_id: u32, operation_id: u64) -> Result<Option<Vec<u8>>") && source.includes("plugin_take_segmented_download_chunk(runtime, instance_id, operation_id)") && source.includes(".map_err($crate::component::component::plugin_error)") && !source.includes("plugin_take_segmented_download_chunk(runtime, instance_id, operation_id))).ok().flatten()");
}

export function toolJobPuzzleReservedRoutesExact(source: string, host: string, jobs: string): boolean {
  const implStart = source.indexOf("impl ArtifactEditor for Puzzle5dPlayApp");
  const implOpen = implStart < 0 ? -1 : source.indexOf("{", implStart);
  const implementation = implOpen < 0 ? undefined : toolJobRustBlock(source, implOpen);
  if (!implementation) return false;
  // 🧳️ The framework registers every reserved route's factory for every app and builds the job through
  // `build_reserved_tool_job`; an app-owned factory under the same key refuses app construction.
  const exactFactories = !/puzzle5d_reserved_factory!|Puzzle5d(?:Copy|Cut|Paste|Import)JobFactory/.test(source) && TOOL_JOB_PLUGIN_RESERVED_IDS.every((id) => implementation.body.includes(`"${id}" =>`));
  // 🧵️ Copy and cut share one clipboard job; each route job lends outcomes through the puzzle outbox (`step -> Result<Option<JobOutcomeBorrow>>` + `borrow_outcome`),
  // prepares its retained commit output BEFORE it completes, and closes through the shared outbox-then-owners ladder.
  const routeJobs = ["Puzzle5dClipboardJob", "Puzzle5dPasteJob", "Puzzle5dImportJob"];
  const resumableJobs = routeJobs.every((job) => {
    const start = source.indexOf(`struct ${job} {`);
    const end = source.indexOf(`impl ArtifactReservedJob for ${job} {}`);
    if (start < 0 || end < start) return false;
    const region = source.slice(start, end);
    return (
      region.includes("cx.is_cancelled()") &&
      region.includes("JobTurn::Cancelled") &&
      region.includes("JobTurn::Prepare(std::mem::take(&mut self.") &&
      region.indexOf("JobTurn::Prepare(") < region.indexOf("completion.complete(") &&
      region.includes("fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError>") &&
      region.includes("self.outbox.phase(cx)?") &&
      region.includes("self.outbox.settle(") &&
      region.includes("self.outbox.borrow_outcome(descriptor)") &&
      region.includes("fn begin_close(&mut self)") &&
      region.includes("fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> ") &&
      region.includes("InteractiveJobCloseStep") &&
      region.includes("crate::puzzle_job::job_close_step(") &&
      region.includes("fn terminal_is_empty(&self) -> bool")
    );
  });
  const fixedIngress =
    source.includes("const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096") &&
    (source.match(/raw_page: \[u8; PUZZLE5D_RESERVED_PAGE_BYTES\]/g) ?? []).length === 3 &&
    source.includes("fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)>") &&
    source.includes("if raw.len() > maximum_bytes") &&
    source.includes("Err((Fault::from(\"puzzle5d reserved wire exceeds its exact route cap before fixed-page copy\"), raw))") &&
    source.indexOf("let end = cursor.checked_add(units)") < source.indexOf("page[..units].copy_from_slice(&raw[*cursor..end])") &&
    implementation.body.includes("let raw = std::mem::take(&mut request.raw_wire)") &&
    implementation.body.includes("puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes)") &&
    implementation.body.includes("Err((fault, rejected))") &&
    implementation.body.includes("drop(rejected)");
  const retainedProtocol =
    source.includes("state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes())") &&
    source.includes("state[9..17].copy_from_slice(&progress.to_le_bytes())") &&
    source.includes("reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner") &&
    jobs.includes("RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput)") &&
    jobs.includes("Prepare(Vec<u8>)") &&
    jobs.includes("self.commit.append_original(cx, &self.source, &mut self.commit_cursor)") &&
    jobs.includes("self.commit.seal(cx)") &&
    jobs.includes("admit_complete(cx, None, self.commit.published())") &&
    jobs.includes("descriptor.complete(None, self.commit.published())") &&
    jobs.includes("self.commit.close_step_granted(grant)") &&
    jobs.includes("a_prepared_commit_output_rides_on_the_completion_and_closes_exactly");
  const hostProtocol =
    host.includes("async fn run_framework_reserved_job") &&
    host.includes("proof.admits::<A>(&admission)") &&
    host.includes("*applied_progress < checkpoint_progress") &&
    host.includes("retained_payload_eq_slice(output, raw)") &&
    host.includes("interactive-job.output-envelope") &&
    host.includes("session.resume()") &&
    host.includes("session.begin_close()") &&
    host.includes("Ok(semio_framework_job::WorkerJobCloseStep::Complete { .. }) if session.terminal_is_empty() => break") &&
    host.includes("self.validate_framework_reserved_commit(action, permit).await?") &&
    host.includes("semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation)") &&
    host.includes("lease.is_cancelled().await") &&
    host.includes("async fn ensure_reserved_emit_bounded(") &&
    host.includes("permit.finish()");
  return exactFactories && resumableJobs && fixedIngress && retainedProtocol && hostProtocol && implementation.body.includes("ArtifactReservedToolInput::Media") && implementation.body.includes("ArtifactReservedToolJob::new(Puzzle5dImportJob::new(");
}

/** 📋️ Puzzle 2d's half of the reserved-route policy. Its clipboard producer is the one-step
 * `Puzzle2dClipboardJob` (ported from puzzle3d), not puzzle5d's four fixed-page state machines, so the
 * fixed-page ingress clauses do not apply — what it must prove instead is that the app owns the route
 * at all (`copy`/`cut`/`paste` built here, never left as the framework's empty stub), that it owns a
 * fragment vocabulary (`clipboard_media_type`/`copy_fragment`/`cut_operations`/`paste_operations` over
 * `puzzle.2d.clipboard.v1`), that `import-media` keeps its own branch, and that the one step is
 * cancellable, prepares its retained commit output BEFORE it completes, and closes to terminal-empty. */
export function toolJobPuzzle2dReservedRoutesExact(source: string, host: string, jobs: string): boolean {
  const implStart = source.indexOf("impl ArtifactEditor for Puzzle2dPlayApp");
  const implOpen = implStart < 0 ? -1 : source.indexOf("{", implStart);
  const implementation = implOpen < 0 ? undefined : toolJobRustBlock(source, implOpen);
  if (!implementation) return false;
  const jobStart = source.indexOf("impl InteractiveJob for Puzzle2dClipboardJob");
  const jobOpen = jobStart < 0 ? -1 : source.indexOf("{", jobStart);
  const job = jobOpen < 0 ? undefined : toolJobRustBlock(source, jobOpen);
  if (!job) return false;
  const exactFactories = !/puzzle2d_reserved_factory!|Puzzle2d(?:Copy|Cut|Paste)JobFactory/.test(source)
    && TOOL_JOB_PLUGIN_RESERVED_IDS.every((id) => id === "import-media" ? implementation.body.includes("PUZZLE2D_IMPORT_TOOL_ID") : implementation.body.includes(`"${id}"`));
  const ownedFragment = source.includes('const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1"')
    && implementation.body.includes("fn clipboard_media_type()")
    && implementation.body.includes("fn copy_fragment(")
    && implementation.body.includes("fn cut_operations(")
    && implementation.body.includes("fn paste_operations(")
    && source.includes("fn puzzle2d_copy_fragment_from(")
    && source.includes("fn puzzle2d_cut_operations_from(")
    && source.includes("fn puzzle2d_paste_operations_on(");
  const routedHere = implementation.body.includes('matches!(request.tool_id.as_str(), "copy" | "cut" | "paste")')
    && implementation.body.includes("ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request))")
    && implementation.body.includes("ArtifactReservedToolInput::Media")
    && implementation.body.includes("ArtifactReservedToolJob::new(Puzzle2dImportJob::new(");
  const clipboardStart = source.indexOf("struct Puzzle2dClipboardJob {");
  const clipboardEnd = source.indexOf("impl ArtifactReservedJob for Puzzle2dClipboardJob");
  const region = clipboardStart < 0 || clipboardEnd < clipboardStart ? "" : source.slice(clipboardStart, clipboardEnd);
  const resumableJob = region.includes("cx.is_cancelled()")
    && region.includes("JobTurn::Cancelled")
    && region.includes("JobTurn::Prepare(std::mem::take(&mut self.raw_wire))")
    && region.includes("completion.complete(")
    && region.indexOf("JobTurn::Prepare(") < region.indexOf("completion.complete(")
    && region.includes("JobTurn::Complete")
    && region.includes("self.outbox.phase(cx)?")
    && region.includes("self.outbox.settle(")
    && region.includes("self.outbox.borrow_outcome(descriptor)")
    && region.includes("fn begin_close(&mut self)")
    && region.includes("crate::puzzle_job::job_close_step(")
    && region.includes("fn terminal_is_empty(&self) -> bool")
    && source.includes("impl ArtifactReservedJob for Puzzle2dClipboardJob")
    && jobs.includes("JobTurn::Prepare(bytes)")
    && jobs.includes("admit_complete(cx, None, self.commit.published())");
  const hostProtocol = host.includes("async fn run_framework_reserved_job")
    && host.includes("self.validate_framework_reserved_commit(action, permit).await?")
    && host.includes("async fn ensure_reserved_emit_bounded(")
    && host.includes("permit.is_cancelled().await");
  return exactFactories && ownedFragment && routedHere && resumableJob && hostProtocol;
}



export function toolJobSegmentedQueueHardBounded(source: string): boolean {
  const chunksStart = source.indexOf("impl ArtifactOutputChunks");
  const chunksOpen = chunksStart < 0 ? -1 : source.indexOf("{", chunksStart);
  const chunks = chunksOpen < 0 ? undefined : toolJobRustBlock(source, chunksOpen);
  const sealStart = source.indexOf("pub fn seal(&self)", chunksStart);
  const sealOpen = sealStart < 0 ? -1 : source.indexOf("{", sealStart);
  const seal = sealOpen < 0 ? undefined : toolJobRustBlock(source, sealOpen);
  return (
    !!chunks &&
    !!seal &&
    chunks.body.includes("checked_add(chunk.len())") &&
    chunks.body.includes("chunk.len() > ARTIFACT_OUTPUT_CHUNK_BYTES") &&
    chunks.body.includes("state.chunks.push(chunk)") &&
    chunks.body.includes("state.chunks.pop()") &&
    chunks.body.includes("self.inner.state.try_lock()") &&
    source.includes("struct ArtifactFixedQueue<T>") &&
    source.includes("items.try_reserve_exact(capacity)") &&
    source.includes("self.items.len() >= self.capacity") &&
    source.includes("self.items.push_back(value)") &&
    source.includes("self.items.pop_front()") &&
    source.includes("segmented_output_preallocates_exact_former_growth_boundary_and_drains_terminal_storage_to_zero") &&
    source.includes("segmented_output_seal_is_linearly_ordered_with_push") &&
    !source.includes("Box<[Option<Vec<u8>>]>") &&
    seal.body.indexOf("self.inner.state.try_lock()") >= 0 &&
    seal.body.indexOf("self.inner.state.try_lock()") < seal.body.indexOf("compare_exchange(false, true") &&
    seal.body.indexOf("compare_exchange(false, true") < seal.body.indexOf("self.inner.bytes.load")
  );
}
