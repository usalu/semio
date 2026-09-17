import { toolJobPuzzle2dReservedRoutesExact, toolJobPuzzleReservedRoutesExact } from "../../../../../📜️script.ts";

/** 🧪️ Executes tool job puzzle reserved routes policy assertions. */
export function toolJobPuzzleReservedRoutesSelfTests(): number {
  const source = `
const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096;
fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)> { if raw.len() > maximum_bytes { return Err((Fault::from("puzzle5d reserved wire exceeds its exact route cap before fixed-page copy"), raw)); } Ok(raw) }
fn ingress(cursor: &mut usize, units: usize, raw: &[u8], page: &mut [u8]) { let end = cursor.checked_add(units); page[..units].copy_from_slice(&raw[*cursor..end]); }
fn payload() { match admission { Err(rejected) => { drop(rejected.into_source()); } } }
fn checkpoint(cursor: usize, progress: u64) { state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes()); state[9..17].copy_from_slice(&progress.to_le_bytes()); }
struct Puzzle5dCommitEnvelope { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
impl Puzzle5dCommitEnvelope { fn new() { RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput); writer.write_slice_page(cx, raw, &mut self.cursor); match writer.finish() { Err(writer) => { *self.writer = Some(writer); } } } fn take_output(&mut self) -> Option<RetainedJobPayload> {} fn close() { self.commit.close_step(maximum_items, maximum_bytes); self.commit.terminal_is_empty(); } }
struct Clipboard { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
struct Paste { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
fn reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner() {}
impl ArtifactEditor for Puzzle5dPlayApp { fn routes() { registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id)); match id { "copy" => {}, "cut" => {}, "paste" => {}, "import-media" => { ArtifactReservedToolInput::Media; ArtifactReservedToolJob::new(Puzzle5dImportJob::new(; } } let raw = std::mem::take(&mut request.raw_wire); puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes); Err((fault, rejected)); drop(rejected); } }
${["Copy", "Cut", "Paste", "Import"].map((name) => `impl InteractiveJob for Puzzle5d${name}Job { fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome { if cx.is_cancelled() { return StepOutcome::Cancelled; } self.commit.prepare(&self.raw, cx); completion.complete(output); let output = self.commit.take_output(); CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output } } fn begin_close(&mut self) { self.commit.begin_close(); } fn terminal_is_empty(&self) -> bool { self.commit.terminal_is_empty() } }`).join("\n")}
`;
  const host = `async fn run_framework_reserved_job() { proof.admits::<A>(&admission); operation.base_revision != base_revision || operation.generation != generation; checkpoint.applied_progress < checkpoint_progress; !retained_payload_eq_slice(&candidate.output, raw); session.resume(); session.begin_close(); WorkerJobCloseStep::Complete if session.terminal_is_empty(); self.validate_framework_reserved_commit(action, permit).await?; permit.is_cancelled().await; completion.take_emit()?; Self::ensure_reserved_emit_bounded(action); permit.finish(); }`;
  const exact = (candidate = source, candidateHost = host) => toolJobPuzzleReservedRoutesExact(candidate, candidateHost);
  if (!exact()) throw new Error("[verify interactivity tool-jobs] self-test Puzzle5d retained reserved routes valid fixture was falsely rejected.");
  const mutations: readonly [string, () => boolean][] = [
    ["app-owned reserved factory restored", () => exact(`puzzle5d_reserved_factory!(Puzzle5dCopyJobFactory, "copy", "puzzle.5d.reserved.copy.v1");\n${source}`)],
    ["app-owned reserved registration restored", () => exact(source.replace("registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id));", "registry.register(Puzzle5dCutJobFactory::new(&controller_id));"))],
    ["route branch omitted", () => exact(source.replace('"paste" => {},', ""))],
    ["fixed-page ingress removed", () => exact(source.replace("raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES]", "raw_page: Vec<u8>"))],
    ["copy precedes ingress preflight", () => exact(source.replace("let end = cursor.checked_add(units); page[..units].copy_from_slice(&raw[*cursor..end]);", "page[..units].copy_from_slice(&raw[*cursor..end]); let end = cursor.checked_add(units);"))],
    ["max plus one owner law omitted", () => exact(source.replace("reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner", "reserved_wire_smoke"))],
    ["route cancellation omitted", () => exact(source.replace("if cx.is_cancelled() { return StepOutcome::Cancelled; }", ""))],
    ["commit output parity omitted", () => exact(source.replace("let output = self.commit.take_output();", "let output = RetainedJobPayload::empty(JobPayloadStream::CommitOutput);"))],
    ["completion precedes retained output preparation", () => exact(source.replace("self.commit.prepare(&self.raw, cx); completion.complete(output);", "completion.complete(output); self.commit.prepare(&self.raw, cx);"))],
    ["checkpoint progress omitted", () => exact(source.replace("state[9..17].copy_from_slice(&progress.to_le_bytes());", ""))],
    ["host freshness omitted", () => exact(source, host.replace("operation.base_revision != base_revision || operation.generation != generation", "false"))],
    ["host exact output ACK omitted", () => exact(source, host.replace("!retained_payload_eq_slice(&candidate.output, raw)", "false"))],
    ["host terminal close witness omitted", () => exact(source, host.replace("WorkerJobCloseStep::Complete if session.terminal_is_empty()", "WorkerJobCloseStep::Complete"))],
  ];
  for (const [name, mutation] of mutations) if (mutation()) throw new Error(`[verify interactivity tool-jobs] self-test Puzzle5d ${name} was falsely accepted.`);
  return mutations.length + 1 + toolJobPuzzle2dReservedRoutesSelfTests(host);
}

/** 🧪️ Executes the puzzle 2d half of the reserved-routes policy — its clipboard producer is the
 * one-step `Puzzle2dClipboardJob`, so the fixture proves route ownership, the app-owned fragment
 * vocabulary, and the cancellable complete-before-output step rather than 5d's fixed-page ingress. */
export function toolJobPuzzle2dReservedRoutesSelfTests(host: string): number {
  const source = `
const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";
fn puzzle2d_copy_fragment_from(fixture: &Value, node_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {}
fn puzzle2d_cut_operations_from(fixture: &Value, node_ids: &[String]) -> Result<Vec<Puzzle2dMutation>, ClipboardError> {}
fn puzzle2d_paste_operations_on(fixture: &Value, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<(Vec<Puzzle2dMutation>, Vec<String>), ClipboardError> {}
impl InteractiveJob for Puzzle2dClipboardJob { fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome { if cx.is_cancelled() { return StepOutcome::Cancelled; } let emit = self.emit(); completion.complete(Ok(emit), EphemeralEmit::default()); let output = puzzle2d_job_payload(cx, JobPayloadStream::CommitOutput, &self.raw_wire); StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output }) } fn begin_close(&mut self) { self.closing = true; } fn terminal_is_empty(&self) -> bool { self.closing } }
impl ArtifactReservedJob for Puzzle2dClipboardJob {}
impl ArtifactEditor for Puzzle2dPlayApp { fn clipboard_media_type() {} fn copy_fragment(doc: &ArtifactView<'_, S>) {} fn cut_operations(doc: &ArtifactView<'_, S>) {} fn paste_operations(doc: &ArtifactView<'_, S>) {} fn build_reserved_tool_job(request: R) { if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") { return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request)))); } if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID { return Ok(None); } let ArtifactReservedToolInput::Media { port, media } = &request.input else {}; Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media)))) } }
`;
  const exact = (candidate = source, candidateHost = host) => toolJobPuzzle2dReservedRoutesExact(candidate, candidateHost);
  if (!exact()) throw new Error("[verify interactivity tool-jobs] self-test Puzzle2d reserved routes valid fixture was falsely rejected.");
  const mutations: readonly [string, () => boolean][] = [
    ["app-owned reserved factory restored", () => exact(`puzzle2d_reserved_factory!(Puzzle2dCopyJobFactory, "copy", "puzzle.2d.reserved.copy.v1");\n${source}`)],
    ["clipboard route branch handed back to the framework stub", () => exact(source.replace('if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") { return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request)))); }', ""))],
    ["import-media branch omitted", () => exact(source.replace("ArtifactReservedToolJob::new(Puzzle2dImportJob::new(", "ArtifactReservedToolJob::new(OtherImportJob::new("))],
    ["owned fragment schema omitted", () => exact(source.replace('const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";', ""))],
    ["cut operations producer omitted", () => exact(source.replace("fn puzzle2d_cut_operations_from(", "fn unused_cut_operations_from("))],
    ["paste operations hook omitted", () => exact(source.replace("fn paste_operations(doc: &ArtifactView<'_, S>) {}", ""))],
    ["route cancellation omitted", () => exact(source.replace("if cx.is_cancelled() { return StepOutcome::Cancelled; }", ""))],
    ["retained output prepared before completion", () => exact(source.replace("completion.complete(Ok(emit), EphemeralEmit::default()); let output = puzzle2d_job_payload(cx, JobPayloadStream::CommitOutput, &self.raw_wire);", "let output = puzzle2d_job_payload(cx, JobPayloadStream::CommitOutput, &self.raw_wire); completion.complete(Ok(emit), EphemeralEmit::default());"))],
    ["terminal close witness omitted", () => exact(source.replace("fn terminal_is_empty(&self) -> bool { self.closing }", ""))],
    ["reserved job trait omitted", () => exact(source.replace("impl ArtifactReservedJob for Puzzle2dClipboardJob {}", ""))],
    ["host bounded-emit guard omitted", () => exact(source, host.replace("Self::ensure_reserved_emit_bounded(action);", ""))],
  ];
  for (const [name, mutation] of mutations) if (mutation()) throw new Error(`[verify interactivity tool-jobs] self-test Puzzle2d ${name} was falsely accepted.`);
  return mutations.length + 1;
}
