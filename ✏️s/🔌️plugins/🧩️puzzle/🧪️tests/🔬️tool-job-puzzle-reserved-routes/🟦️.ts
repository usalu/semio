import { toolJobPuzzle2dReservedRoutesExact, toolJobPuzzleReservedRoutesExact } from "../../../../../📜️script.ts";

/** 🧪️ The puzzle outbox as the guards read it: the commit builder prepares the raw wire, completion lends the sealed pages. */
const PUZZLE_JOBS_FIXTURE = `
pub enum JobTurn { Yield, Cancelled, Complete, Prepare(Vec<u8>) }
pub struct JobOutbox { commit: RetainedPayloadBuilder }
impl Default for JobOutbox { fn default() -> Self { Self { commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput) } } }
impl JobOutbox {
  pub fn advance() { self.commit.append_original(cx, &self.source, &mut self.commit_cursor)?; self.commit.seal(cx)?; }
  pub fn settle() { JobTurn::Complete => return JobOutcomeBorrow::admit_complete(cx, None, self.commit.published()), JobTurn::Prepare(bytes) => {} }
  pub fn borrow_outcome() { JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()), }
  pub fn close_step(grant: RetainedCloneGrant) { self.commit.close_step_granted(grant)?; }
}
fn a_prepared_commit_output_rides_on_the_completion_and_closes_exactly() {}
`;

/** 🧪️ The framework host's reserved-route driver as the guards read it. */
const PUZZLE_RESERVED_HOST_FIXTURE = `async fn run_framework_reserved_job() { proof.admits::<A>(&admission); *applied_progress < checkpoint_progress; retained_payload_eq_slice(output, raw); protocol_fault = Some("interactive-job.output-envelope"); session.resume(); session.begin_close(); Ok(semio_framework_job::WorkerJobCloseStep::Complete { .. }) if session.terminal_is_empty() => break; self.validate_framework_reserved_commit(action, permit).await?; semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation); permit.lease.is_cancelled().await; permit.is_cancelled().await; async fn ensure_reserved_emit_bounded(action: &str) {} permit.finish(); }`;

const ROUTE_JOBS = ["Puzzle5dClipboardJob", "Puzzle5dPasteJob", "Puzzle5dImportJob"] as const;

const routeJob = (name: string, raw: string) => `struct ${name} { outbox: JobOutbox }
impl ${name} { fn turn(&mut self, cx: &StepContext<'_>) -> JobTurn { if self.closing || cx.is_cancelled() { return JobTurn::Cancelled; } return JobTurn::Prepare(std::mem::take(&mut self.${raw})); completion.complete(Ok(emit), EphemeralEmit::default()); JobTurn::Complete } }
impl InteractiveJob for ${name} { fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> { match self.outbox.phase(cx)? {} let turn = self.turn(cx); self.outbox.settle(turn, cx) } fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { self.outbox.borrow_outcome(descriptor) } fn begin_close(&mut self) { self.closing = true; } fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep { crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant) } fn terminal_is_empty(&self) -> bool { self.closing } }
impl ArtifactReservedJob for ${name} {}`;

/** 🧪️ Executes tool job puzzle reserved routes policy assertions. */
export function toolJobPuzzleReservedRoutesSelfTests(): number {
  const source = `
const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096;
fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)> { if raw.len() > maximum_bytes { return Err((Fault::from("puzzle5d reserved wire exceeds its exact route cap before fixed-page copy"), raw)); } Ok(raw) }
fn ingress(cursor: &mut usize, units: usize, raw: &[u8], page: &mut [u8]) { let end = cursor.checked_add(units); page[..units].copy_from_slice(&raw[*cursor..end]); }
fn checkpoint(cursor: usize, progress: u64) { state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes()); state[9..17].copy_from_slice(&progress.to_le_bytes()); }
struct Clipboard { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
struct Paste { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
struct Import { raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES] }
fn reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner() {}
impl ArtifactEditor for Puzzle5dPlayApp { fn routes() { registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id)); match id { "copy" => {}, "cut" => {}, "paste" => {}, "import-media" => { ArtifactReservedToolInput::Media; ArtifactReservedToolJob::new(Puzzle5dImportJob::new(; } } let raw = std::mem::take(&mut request.raw_wire); puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes); Err((fault, rejected)); drop(rejected); } }
${routeJob("Puzzle5dClipboardJob", "work.raw")}
${routeJob("Puzzle5dPasteJob", "raw")}
${routeJob("Puzzle5dImportJob", "raw")}
`;
  const host = PUZZLE_RESERVED_HOST_FIXTURE;
  const jobs = PUZZLE_JOBS_FIXTURE;
  const exact = (candidate = source, candidateHost = host, candidateJobs = jobs) => toolJobPuzzleReservedRoutesExact(candidate, candidateHost, candidateJobs);
  if (!exact()) throw new Error("[verify interactivity tool-jobs] self-test Puzzle5d retained reserved routes valid fixture was falsely rejected.");
  const mutations: readonly [string, () => boolean][] = [
    ["app-owned reserved factory restored", () => exact(`puzzle5d_reserved_factory!(Puzzle5dCopyJobFactory, "copy", "puzzle.5d.reserved.copy.v1");\n${source}`)],
    ["app-owned reserved registration restored", () => exact(source.replace("registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id));", "registry.register(Puzzle5dCutJobFactory::new(&controller_id));"))],
    ["route branch omitted", () => exact(source.replace('"paste" => {},', ""))],
    ["fixed-page ingress removed", () => exact(source.replace("raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES]", "raw_page: Vec<u8>"))],
    ["copy precedes ingress preflight", () => exact(source.replace("let end = cursor.checked_add(units); page[..units].copy_from_slice(&raw[*cursor..end]);", "page[..units].copy_from_slice(&raw[*cursor..end]); let end = cursor.checked_add(units);"))],
    ["max plus one owner law omitted", () => exact(source.replace("reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner", "reserved_wire_smoke"))],
    ["route cancellation omitted", () => exact(source.replace("if self.closing || cx.is_cancelled() { return JobTurn::Cancelled; }", ""))],
    ["commit output preparation omitted", () => exact(source.replace("return JobTurn::Prepare(std::mem::take(&mut self.work.raw));", ""))],
    ["completion precedes retained output preparation", () => exact(source.replace("return JobTurn::Prepare(std::mem::take(&mut self.raw)); completion.complete(Ok(emit), EphemeralEmit::default());", "completion.complete(Ok(emit), EphemeralEmit::default()); return JobTurn::Prepare(std::mem::take(&mut self.raw));"))],
    ["outbox settle omitted", () => exact(source.replace("self.outbox.settle(turn, cx)", "Ok(None)"))],
    ["outbox borrow omitted", () => exact(source.replace("self.outbox.borrow_outcome(descriptor)", "Err(ValueError::default())"))],
    ["outbox close ladder omitted", () => exact(source.replace("crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)", "InteractiveJobCloseStep::Complete { progress: Default::default() }"))],
    ["checkpoint progress omitted", () => exact(source.replace("state[9..17].copy_from_slice(&progress.to_le_bytes());", ""))],
    ["commit builder omitted", () => exact(source, host, jobs.replace("RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput)", "RetainedPayloadBuilder::new(JobPayloadStream::Preview)"))],
    ["completion without sealed commit output", () => exact(source, host, jobs.replace("admit_complete(cx, None, self.commit.published())", "admit_complete(cx, None, None)"))],
    ["commit pages never closed", () => exact(source, host, jobs.replace("self.commit.close_step_granted(grant)?;", ""))],
    ["prepared output law omitted", () => exact(source, host, jobs.replace("a_prepared_commit_output_rides_on_the_completion_and_closes_exactly", "commit_smoke"))],
    ["host commit freshness omitted", () => exact(source, host.replace("semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation);", ""))],
    ["host exact output ACK omitted", () => exact(source, host.replace("retained_payload_eq_slice(output, raw)", "false"))],
    ["host output envelope fault omitted", () => exact(source, host.replace("interactive-job.output-envelope", "interactive-job.other"))],
    ["host terminal close witness omitted", () => exact(source, host.replace("Ok(semio_framework_job::WorkerJobCloseStep::Complete { .. }) if session.terminal_is_empty() => break", "Ok(semio_framework_job::WorkerJobCloseStep::Complete { .. }) => break"))],
  ];
  for (const [name, mutation] of mutations) if (mutation()) throw new Error(`[verify interactivity tool-jobs] self-test Puzzle5d ${name} was falsely accepted.`);
  return mutations.length + 1 + toolJobPuzzle2dReservedRoutesSelfTests(host, jobs);
}

/** 🧪️ Executes the puzzle 2d half of the reserved-routes policy — its clipboard producer is the
 * one-step `Puzzle2dClipboardJob`, so the fixture proves route ownership, the app-owned fragment
 * vocabulary, and the cancellable prepare-then-complete step rather than 5d's fixed-page ingress. */
export function toolJobPuzzle2dReservedRoutesSelfTests(host: string, jobs: string): number {
  const source = `
const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";
fn puzzle2d_copy_fragment_from(fixture: &Value, node_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {}
fn puzzle2d_cut_operations_from(fixture: &Value, node_ids: &[String]) -> Result<Vec<Puzzle2dMutation>, ClipboardError> {}
fn puzzle2d_paste_operations_on(fixture: &Value, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<(Vec<Puzzle2dMutation>, Vec<String>), ClipboardError> {}
struct Puzzle2dClipboardJob { outbox: JobOutbox }
impl Puzzle2dClipboardJob { fn turn(&mut self, cx: &StepContext<'_>) -> JobTurn { if self.closing || cx.is_cancelled() { return JobTurn::Cancelled; } if !self.raw_wire.is_empty() { return JobTurn::Prepare(std::mem::take(&mut self.raw_wire)); } let emit = self.emit(); completion.complete(Ok(emit), EphemeralEmit::default()); JobTurn::Complete } }
impl InteractiveJob for Puzzle2dClipboardJob { fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> { match self.outbox.phase(cx)? {} let turn = self.turn(cx); self.outbox.settle(turn, cx) } fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { self.outbox.borrow_outcome(descriptor) } fn begin_close(&mut self) { self.closing = true; } fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep { crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant) } fn terminal_is_empty(&self) -> bool { self.closing } }
impl ArtifactReservedJob for Puzzle2dClipboardJob {}
impl ArtifactEditor for Puzzle2dPlayApp { fn clipboard_media_type() {} fn copy_fragment(doc: &ArtifactView<'_, S>) {} fn cut_operations(doc: &ArtifactView<'_, S>) {} fn paste_operations(doc: &ArtifactView<'_, S>) {} fn build_reserved_tool_job(request: R) { if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") { return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request)))); } if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID { return Ok(None); } let ArtifactReservedToolInput::Media { port, media } = &request.input else {}; Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media)))) } }
`;
  const exact = (candidate = source, candidateHost = host, candidateJobs = jobs) => toolJobPuzzle2dReservedRoutesExact(candidate, candidateHost, candidateJobs);
  if (!exact()) throw new Error("[verify interactivity tool-jobs] self-test Puzzle2d reserved routes valid fixture was falsely rejected.");
  const mutations: readonly [string, () => boolean][] = [
    ["app-owned reserved factory restored", () => exact(`puzzle2d_reserved_factory!(Puzzle2dCopyJobFactory, "copy", "puzzle.2d.reserved.copy.v1");\n${source}`)],
    ["clipboard route branch handed back to the framework stub", () => exact(source.replace('if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") { return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request)))); }', ""))],
    ["import-media branch omitted", () => exact(source.replace("ArtifactReservedToolJob::new(Puzzle2dImportJob::new(", "ArtifactReservedToolJob::new(OtherImportJob::new("))],
    ["owned fragment schema omitted", () => exact(source.replace('const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";', ""))],
    ["cut operations producer omitted", () => exact(source.replace("fn puzzle2d_cut_operations_from(", "fn unused_cut_operations_from("))],
    ["paste operations hook omitted", () => exact(source.replace("fn paste_operations(doc: &ArtifactView<'_, S>) {}", ""))],
    ["route cancellation omitted", () => exact(source.replace("if self.closing || cx.is_cancelled() { return JobTurn::Cancelled; }", ""))],
    ["retained output preparation omitted", () => exact(source.replace("if !self.raw_wire.is_empty() { return JobTurn::Prepare(std::mem::take(&mut self.raw_wire)); }", ""))],
    ["completion precedes retained output preparation", () => exact(source.replace("if !self.raw_wire.is_empty() { return JobTurn::Prepare(std::mem::take(&mut self.raw_wire)); } let emit = self.emit(); completion.complete(Ok(emit), EphemeralEmit::default());", "let emit = self.emit(); completion.complete(Ok(emit), EphemeralEmit::default()); if !self.raw_wire.is_empty() { return JobTurn::Prepare(std::mem::take(&mut self.raw_wire)); }"))],
    ["outbox settle omitted", () => exact(source.replace("self.outbox.settle(turn, cx)", "Ok(None)"))],
    ["terminal close witness omitted", () => exact(source.replace("fn terminal_is_empty(&self) -> bool { self.closing }", ""))],
    ["reserved job trait omitted", () => exact(source.replace("impl ArtifactReservedJob for Puzzle2dClipboardJob {}", ""))],
    ["sealed commit output not lent on completion", () => exact(source, host, jobs.replace("admit_complete(cx, None, self.commit.published())", "admit_complete(cx, None, None)"))],
    ["host bounded-emit guard omitted", () => exact(source, host.replace("async fn ensure_reserved_emit_bounded(action: &str) {}", ""))],
  ];
  for (const [name, mutation] of mutations) if (mutation()) throw new Error(`[verify interactivity tool-jobs] self-test Puzzle2d ${name} was falsely accepted.`);
  return mutations.length + 1;
}
