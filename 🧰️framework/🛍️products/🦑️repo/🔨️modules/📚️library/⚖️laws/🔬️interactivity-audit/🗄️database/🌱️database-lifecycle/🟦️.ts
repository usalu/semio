/** 🌱️ Interactivity audit: database catalog bootstrap, document creation, compaction and sync-hello fixtures, artifact history, DB CLI and MCP HTTP transport sources.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { toolJobRustBlock } from "../../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { interactivityProductionSource, interactivityRegionSource } from "../🟦️.ts";

//#region 🔖️InteractivityAudit
//#region 🗄️P1wCatalogBootstrapCas
function interactivityDatabaseCatalogBootstrapFixtureBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}`);
  if (start < 0) return "";
  const open = source.indexOf("{", start);
  return open < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
}

/** 🔁️ The one loop a retained driver's `schedule` may hold is the lost-wake handshake: the Idle-to-Queued claim retried only after a
 * racing driver release left the authority Idle and this caller reclaimed its own wake, returning on every other observation. Blanks
 * that exact loop keyword so a region's no-loop census still sees every other loop. */
function interactivityDriverHandshakeBlanked(region: string, authority: string): string {
  const start = region.indexOf(`while self.driver_authority.compare_exchange(${authority}::Idle as u8, ${authority}::Queued as u8`);
  if (start < 0) return region;
  const exit = region.indexOf(`!= ${authority}::Idle as u8 || !self.wake_requested.swap(false`, start);
  if (exit < 0 || !region.slice(start, exit).includes("self.wake_requested.store(true") || !/^[^{};]*\{\s*return;\s*\}/u.test(region.slice(exit))) return region;
  return `${region.slice(0, start)}if${region.slice(start + "while".length)}`;
}

export function interactivityDatabaseCatalogBootstrapFailures(engineSource: string): string[] {
  const production = interactivityProductionSource(engineSource);
  const bootstrap = interactivityRegionSource(production, "🔖️CatalogBootstrapCas") ?? "";
  const openStart = production.indexOf("async fn open_with");
  const openEnd = production.indexOf("pub async fn create_document", openStart);
  const openWith = openStart < 0 || openEnd < 0 ? "" : production.slice(openStart, openEnd);
  const failures: string[] = [];
  const waits = production.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const expectedWaits = production.includes("DatabaseSyncHelloFuture::try_submit") ? 0 : production.includes("db_compact::DatabaseCompactionFuture::try_submit") ? 1 : 2;
  if (waits !== expectedWaits || openWith.includes("db_actor::block_on") || openWith.includes("storage.catalog().await.cas_root") || openWith.includes("cas_root(EpochFence::INITIAL")) failures.push("P1w retained bootstrap does not preserve the exact post-P1y production-wait/direct-CAS cut");
  if (!openWith.includes("DatabaseCatalogBootstrapFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, pages, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL") || !openWith.includes("Err(rejected) => return Err(rejected.close_and_take_error())") || !openWith.includes("let result = bootstrap.await?") || !openWith.includes("result.into_parts()") || !openWith.includes("storage = retained_storage") && !openWith.includes("(retained_storage, epoch, Vec::new())")) failures.push("fresh Database::open_with does not consume the retained P1w result and exact storage handback");
  for (const caller of ["pub async fn open(", "pub async fn open_at(", "pub async fn open_with_emit"])
    if (!production.includes(caller)) failures.push(`P1w caller census lost ${caller}`);
  for (const required of [
    "const DATABASE_CATALOG_BOOTSTRAP_SLOTS: usize = 64",
    "const DATABASE_CATALOG_BOOTSTRAP_ITEMS: u64 = 8",
    "const DATABASE_CATALOG_BOOTSTRAP_PAGES: u8 = 8",
    "const DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES: u64 = 16 * 1024",
    "DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS",
    "DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES",
    "DatabaseCatalogBootstrapAdmission::try_claim(pages.page_count())",
    "generation.checked_add(1)",
    "self.items.checked_add(DATABASE_CATALOG_BOOTSTRAP_ITEMS)",
    "self.bytes.checked_add(bytes)",
    "bytes.checked_add(DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES)",
    "pub struct DatabaseCatalogBootstrapKey(pub [u8; 16])",
    "pages: Option<db_io_core::DbIoPages>",
    "expected: EpochFence",
    "actual: Option<Result<EpochFence, DbError>>",
    "pub struct DatabaseCatalogBootstrapResult",
    "enum DatabaseCatalogBootstrapDriverAuthority",
    "driver_authority: std::sync::atomic::AtomicU8",
  ]) if (!bootstrap.includes(required)) failures.push(`P1w fixed credited authority missing ${required}`);
  const prepare = bootstrap.slice(bootstrap.indexOf("fn try_prepare_with_key"), bootstrap.indexOf("pub fn generation(&self)", bootstrap.indexOf("fn try_prepare_with_key")));
  if (prepare.indexOf("DatabaseCatalogBootstrapAdmission::try_claim(pages.page_count())") < 0 || prepare.indexOf("DatabaseCatalogBootstrapAdmission::try_claim(pages.page_count())") > prepare.indexOf("let state = Arc::new(DatabaseCatalogBootstrapState") || prepare.indexOf("database_catalog_bootstrap_registry") > prepare.lastIndexOf("state.schedule()")) failures.push("P1w pre-admission/registry guard ordering can expose or destructure owners before durable authority installation");
  for (const phase of ["Handoff", "Poll", "RetainWork", "CloseWork", "RetireInput", "Validate", "Publish", "Terminal"])
    if (!bootstrap.includes(`DatabaseCatalogBootstrapPhase::${phase}`)) failures.push(`P1w retained phase missing ${phase}`);
  for (const authority of ["Idle", "Queued", "Driving", "Retry"])
    if (!bootstrap.includes(`DatabaseCatalogBootstrapDriverAuthority::${authority}`)) failures.push(`P1w atomic driver authority missing ${authority}`);
  const stateImplStart = bootstrap.indexOf("impl DatabaseCatalogBootstrapState");
  const scheduleStart = bootstrap.indexOf("fn schedule(self: &Arc<Self>)", stateImplStart);
  const submitStart = bootstrap.indexOf("fn submit_exact", scheduleStart);
  const retryStart = bootstrap.indexOf("fn arm_retry", submitStart);
  const driveStart = bootstrap.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", retryStart);
  const schedule = scheduleStart < 0 || submitStart < 0 ? "" : bootstrap.slice(scheduleStart, submitStart);
  const submit = submitStart < 0 || retryStart < 0 ? "" : bootstrap.slice(submitStart, retryStart);
  const retry = retryStart < 0 || driveStart < 0 ? "" : bootstrap.slice(retryStart, driveStart);
  const idleToQueued = "self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Idle as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8";
  const queuedToRetry = "self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Retry as u8";
  const retryToQueued = "state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Retry as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8";
  if (schedule.indexOf(idleToQueued) < 0 || schedule.indexOf(idleToQueued) > schedule.indexOf("self.scheduled.store(true") || !schedule.includes("self.wake_requested.store(true") || schedule.includes("self.scheduled.compare_exchange(false, true")) failures.push("P1w scheduling is not linearized by Idle-to-Queued driver authority before its observation mirror");
  if (submit.indexOf("Some((error.into_job(), next_attempt))") < 0 || submit.indexOf("Some((error.into_job(), next_attempt))") > submit.indexOf(queuedToRetry) || !submit.includes(queuedToRetry)) failures.push("P1w queue refusal does not publish the exact job before Queued-to-Retry authority release");
  if (!retry.includes(retryToQueued) || retry.indexOf(retryToQueued) > retry.indexOf("state.scheduled.store(true")) failures.push("P1w retry callback can submit without first claiming Retry-to-Queued authority");
  const driveEnd = bootstrap.indexOf("fn handoff_one", driveStart);
  const drive = driveStart < 0 || driveEnd < 0 ? "" : bootstrap.slice(driveStart, driveEnd);
  const queuedToDriving = "self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Driving as u8";
  const drivingToIdle = "self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Driving as u8, DatabaseCatalogBootstrapDriverAuthority::Idle as u8";
  if (!drive.includes("if generation != self.generation") || !drive.includes("if !self.is_current()") || !drive.includes("match self.phase()") || !drive.includes("self.retire_one()")) failures.push("P1w callback does not bind exact generation, phase, freshness, and close authority");
  if (drive.indexOf(queuedToDriving) < 0 || drive.indexOf(queuedToDriving) > drive.indexOf("self.scheduled.store(false") || drive.indexOf("self.drive_claimed(generation)") < drive.indexOf("self.scheduled.store(false") || drive.indexOf(drivingToIdle) < drive.indexOf("self.drive_claimed(generation)") || drive.indexOf("self.wake_requested.swap(false") < drive.indexOf(drivingToIdle)) failures.push("P1w driver claim/release order leaves a scheduled-false or wake-consumption concurrency window");
  const handoff = bootstrap.slice(bootstrap.indexOf("fn handoff_one"), bootstrap.indexOf("fn poll_backend_once"));
  if (!handoff.includes("self.expected != EpochFence::INITIAL") || !handoff.includes("self.pages.lock()") || !handoff.includes("DatabaseCatalogBootstrapWork::new(storage, pages, self.expected)") || handoff.includes(".poll(")) failures.push("P1w handoff is not one guarded pages transfer distinct from backend polling");
  const work = bootstrap.slice(bootstrap.indexOf("struct DatabaseCatalogBootstrapWork"), bootstrap.indexOf("enum DatabaseCatalogBootstrapPhase"));
  const workNew = work.slice(work.indexOf("fn new("), work.indexOf("fn poll("));
  const workPoll = work.slice(work.indexOf("fn poll("), work.indexOf("fn close_one"));
  const workClose = work.slice(work.indexOf("fn close_one"), work.indexOf("fn terminal_is_empty"));
  if (!workNew.includes("future: None") || !workNew.includes("storage: Some(storage)") || !workNew.includes("pages: Some(pages)") || workNew.includes("Box::pin") || !workPoll.includes("self.storage.take()") || !workPoll.includes("self.pages.take()") || !workPoll.includes("self.future = Some(Box::pin") || !workClose.includes("pages.close_step()?") || workClose.includes("loop {") || workClose.includes("while ")) failures.push("P1w unpolled backend work can own/drop raw pages before its first Lane::Io poll or close more than one page per opportunity");
  const poll = bootstrap.slice(bootstrap.indexOf("fn poll_backend_once"), bootstrap.indexOf("fn publish_poll_error"));
  const pending = poll.slice(poll.indexOf("Poll::Pending"), poll.indexOf("Poll::Ready"));
  const ready = poll.slice(poll.indexOf("Poll::Ready"), poll.indexOf("Err(_)", poll.indexOf("Poll::Ready")));
  const pollClaim = "self.polling.compare_exchange(false, true";
  const cancellationCheck = "self.cancelled.load(Ordering::Acquire)";
  const cancellationBranch = poll.slice(poll.indexOf(cancellationCheck), poll.indexOf("self.set_progress", poll.indexOf(cancellationCheck)));
  if (!poll.includes("std::panic::catch_unwind") || !poll.includes("work.poll(&mut context)") || pending.indexOf("Some(work)") < 0 || pending.indexOf("Some(work)") > pending.indexOf("self.release_poll()") || ready.indexOf("Some(work)") < 0 || ready.indexOf("Some(actual)") < 0 || ready.indexOf("Some(work)") > ready.indexOf("self.release_poll()") || ready.indexOf("Some(actual)") > ready.indexOf("self.release_poll()")) failures.push("P1w Pending/Ready/panic polling does not publish exact owners before releasing the polling gate");
  if (poll.indexOf(pollClaim) < 0 || poll.indexOf(cancellationCheck) < poll.indexOf(pollClaim) || poll.indexOf("work.poll(&mut context)") < poll.indexOf(cancellationCheck) || !cancellationBranch.includes("Some(work)") || cancellationBranch.indexOf("Some(work)") > cancellationBranch.indexOf("self.release_poll()")) failures.push("P1w backend poll is not linearized before cancellation recheck with exact work republished before gate release");
  const releasePoll = bootstrap.slice(bootstrap.indexOf("fn release_poll"), bootstrap.indexOf("fn stage_error", bootstrap.indexOf("fn release_poll")));
  if (!releasePoll.includes("self.polling.store(false") || releasePoll.includes("wake_requested.swap") || releasePoll.includes("self.schedule()")) failures.push("P1w polling release consumes a wake or schedules outside the unique Driving authority");
  const validate = bootstrap.slice(bootstrap.indexOf("fn validate_one"), bootstrap.indexOf("fn publish_one"));
  if (!validate.includes("self.expected.epoch.checked_add(1)") || !validate.includes("actual.epoch == expected") || !validate.includes("Err(error) => Err(error)") || !validate.includes("DbError::Fenced { expected, actual: actual.epoch }")) failures.push("P1w expected/actual revision validation is wrapping, lossy, or retries a CAS mismatch");
  const publish = bootstrap.slice(bootstrap.indexOf("fn publish_one"), bootstrap.indexOf("fn retained_owner_count"));
  if (!publish.includes("!self.is_current()") || !publish.includes("self.abandoned.load") || !publish.includes("terminal_completion") || !publish.includes("completion") || publish.indexOf("!self.is_current()") > publish.indexOf("let completion =")) failures.push("P1w freshness/lost-handle checks do not precede result exposure");
  if (!bootstrap.includes("self.pool.try_submit(Lane::Io, job)") || !bootstrap.includes("error.into_job()") || !bootstrap.includes("self.pool.callback_at") || !bootstrap.includes("retry_pressure") || !bootstrap.includes("DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT")) failures.push("P1w queue pressure does not retain and retry the exact Lane::Io job under a fixed pressure witness");
  const publicFuture = bootstrap.slice(bootstrap.indexOf("impl Future for DatabaseCatalogBootstrapFuture"), bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapFuture"));
  const firstCheck = publicFuture.indexOf("self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()");
  const register = publicFuture.indexOf("Some(context.waker().clone())", firstCheck);
  const recheck = publicFuture.indexOf("self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()", register);
  if (firstCheck < 0 || register < firstCheck || recheck < register || !bootstrap.includes("controlled_publication_before_waker_hook")) failures.push("P1w public future lacks check-register-recheck lost-wakeup closure");
  const cancel = bootstrap.slice(bootstrap.indexOf("pub fn cancel(&self)"), bootstrap.indexOf("impl Future for DatabaseCatalogBootstrapFuture"));
  const futureDrop = bootstrap.slice(bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapFuture"), bootstrap.indexOf("pub struct DatabaseCatalogBootstrapTerminalHandle"));
  const cancelStore = cancel.indexOf("self.state.cancelled.store(true");
  const cancelWake = cancel.indexOf("self.state.wake_requested.store(true", cancelStore);
  const cancelSchedule = cancel.indexOf("self.state.schedule()", cancelWake);
  const dropCancel = futureDrop.indexOf("self.state.cancelled.store(true");
  const dropWake = futureDrop.indexOf("self.state.wake_requested.store(true", dropCancel);
  const dropSchedule = futureDrop.indexOf("self.state.schedule()", dropWake);
  if (cancelStore < 0 || cancelWake < cancelStore || cancelSchedule < cancelWake || dropCancel < 0 || dropWake < dropCancel || dropSchedule < dropWake || cancel.includes("self.state.polling.load") || futureDrop.includes("self.state.polling.load")) failures.push("P1w cancel/Drop does not publish cancellation and wake to the unified driver authority in exact order");
  const terminal = bootstrap.slice(bootstrap.indexOf("impl DatabaseCatalogBootstrapTerminalHandle"), bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapTerminalHandle"));
  const terminalClose = terminal.slice(terminal.indexOf("pub fn close_step"), terminal.indexOf("pub fn terminal_is_empty"));
  for (const required of ["pub struct DatabaseCatalogBootstrapRejected", "impl Drop for DatabaseCatalogBootstrapRejected", "DatabaseCatalogBootstrapRejectedClose::prepare", "impl Drop for DatabaseCatalogBootstrapResult", "state.terminal_completion.lock()", "state.closing.store(true", "state.release_success()", "pub struct DatabaseCatalogBootstrapTerminalHandle", "pub struct DatabaseCatalogBootstrapTerminalResult", "pub fn take_database_catalog_bootstrap_terminal", "pub fn take_result", "pub fn resume", "pub fn witness", "retained_owner_count", "roots_are_empty"])
    if (!bootstrap.includes(required)) failures.push(`P1w exact take/resume/close/witness authority missing ${required}`);
  const resultDrop = bootstrap.slice(bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapResult"), bootstrap.indexOf("struct DatabaseCatalogBootstrapRejectedCloseOwner"));
  if (!resultDrop.includes("let Some(state) = self.state.take()") || !resultDrop.includes("state: None") || !resultDrop.includes("state.terminal_completion.lock()") || !resultDrop.includes("state.schedule()") || resultDrop.includes("Arc::new") || resultDrop.includes("ResultClose::mount")) failures.push("P1w result Drop allocates post-admission retirement state or bypasses its generation registry handback");
  const rejectedImpl = bootstrap.slice(bootstrap.indexOf("impl DatabaseCatalogBootstrapRejected {"), bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapRejected"));
  const rejectedDropStart = bootstrap.indexOf("impl Drop for DatabaseCatalogBootstrapRejected");
  const rejectedDrop = bootstrap.slice(rejectedDropStart, bootstrap.indexOf("struct DatabaseCatalogBootstrapRejectedClose {", rejectedDropStart));
  if (!rejectedImpl.includes("DatabaseCatalogBootstrapRejectedClose::prepare(pool, owner)") || !rejectedImpl.includes("self.close.take_owner()") || !rejectedImpl.includes("self.close.restore_owner(owner)") || !rejectedDrop.includes("self.close.has_owner()") || !rejectedDrop.includes("self.close.schedule()") || rejectedDrop.includes("Arc::new") || rejectedDrop.includes("storage.take()") || rejectedDrop.includes("pages.take()") || rejectedDrop.includes("key.take()")) failures.push("P1w refusal destructures or allocates its retirement authority during Drop instead of preserving its prepared owner ledger");
  if (!terminalClose.includes("self.state.schedule()") || !terminalClose.includes("DatabaseCatalogBootstrapDriverAuthority::Driving") || !terminalClose.includes("DatabaseCatalogBootstrapDriverAuthority::Retry") || terminalClose.includes("self.state.polling.load") || terminalClose.includes("retire_one(") || terminalClose.includes("close_one(") || terminalClose.includes("drop(")) failures.push("P1w public close bypasses atomic driver authority or executes retirement on the facade caller");
  const retired = bootstrap.slice(bootstrap.indexOf("fn retire_one"), bootstrap.indexOf("fn release_success"));
  if (!retired.includes("owner.close_step()") || !retired.includes("DatabaseCatalogBootstrapWork::close_one") || !retired.includes("owner.close_one()") || retired.includes("while ") || retired.includes("loop {")) failures.push("P1w retirement does not advance one page/future/result/control owner per opportunity");
  const bootstrapLoops = interactivityDriverHandshakeBlanked(bootstrap, "DatabaseCatalogBootstrapDriverAuthority");
  for (const forbidden of ["db_actor::block_on", "semio_framework_async::block_on", "WorkerPool::new(", "thread::spawn", "loop {", "while ", ".unwrap()", ".expect(", ".to_string()", "Vec<", "String>"])
    if (bootstrapLoops.includes(forbidden)) failures.push(`P1w production region contains prohibited ${forbidden}`);

  const fixtureRequirements: readonly [string, readonly string[]][] = [
    ["database_catalog_bootstrap_max_plus_one_and_aba_preserve_exact_credit_identity", ["DATABASE_CATALOG_BOOTSTRAP_PAGES + 1", "replacement.1", "!state.release(slot, generation", "state.bytes"]],
    ["database_catalog_bootstrap_real_max_plus_one_refusal_returns_pages_storage_key_and_fence", ["storage_pointer", "operation = pages.operation()", "page_count()", "mount_close_and_take_error", "terminal_is_empty"]],
    ["database_catalog_bootstrap_success_runs_on_pool_io_worker_and_returns_exact_storage_epoch", ["poll_worker_thread", "Arc::as_ptr(&storage)", "EpochFence::INITIAL.next()"]],
    ["database_catalog_bootstrap_cas_mismatch_returns_identical_storage_and_exact_fenced_error_without_retry", ["Arc::as_ptr(&storage)", "Err(DbError::Fenced { expected: 1, actual: 0 })"]],
    ["database_catalog_bootstrap_ready_and_pending_interruption_publish_once_and_retain_owner", ["ControlledCatalogBootstrapPoll::Ready", "ControlledCatalogBootstrapPoll::Pending", "probe.cancel()", "polls.load", "Err(DbError::Closed)"]],
    ["database_catalog_bootstrap_handoff_interruption_retires_unpolled_pages_one_lane_opportunity_at_a_time", ["catalog_bootstrap_pages(3)", "DatabaseCatalogBootstrapPhase::Poll", "work.future.is_none()", "work.pages", "work.storage", "probe.cancel()", "Err(DbError::Closed)", "terminal_work"]],
    ["database_catalog_bootstrap_atomic_driver_claim_closes_first_poll_pending_ready_panic_and_retirement_races", ["controlled_driver_claim_hook", "DatabaseCatalogBootstrapDriverAuthority::Driving", "assert_ne!(driver_thread, std::thread::current().id())", "page_identity", "storage_identity", "polls.load", "0", "active_drivers", "max_active_drivers", "ControlledCatalogBootstrapPoll::Pending", "ControlledCatalogBootstrapPoll::Ready", "ControlledCatalogBootstrapPoll::Panic", "controlled_driver_release_hook", "completion.lock()", "terminal_completion.lock()", "state.admission", "database_catalog_bootstrap_registry", "release(slot, generation, bytes)", "drop(result)", "DatabaseCatalogBootstrapCloseStep::Blocked", "terminal.terminal_is_empty()", "DatabaseCatalogBootstrapDriverAuthority::Idle", "Arc::as_ptr(&storage)"]],
    ["database_catalog_bootstrap_lost_handle_take_resume_close_and_terminal_witness_are_exact", ["take_database_catalog_bootstrap_terminal", "terminal.resume()", "terminal.close_step()", "retained_owners", "driver_grants", "initial.saturating_sub(current) as u64 <= granted - initial_grants + grant_in_flight"]],
    ["database_catalog_bootstrap_backend_no_service_close_retires_only_on_io_lane", ["ControlledCatalogBootstrapPoll::NoService", "poll_worker_thread", "terminal.close_step()", "terminal_empty"]],
    ["database_catalog_bootstrap_stale_generation_fault_preserves_storage_pages_and_current_slot", ["replacement_generation", "DbError::StaleGeneration", "admission.release(state.slot, replacement_generation", "Arc::as_ptr(&storage)"]],
    ["database_catalog_bootstrap_real_queue_saturation_retains_exact_job_and_recovers_identity", ["WorkerSubmitErrorKind::Saturated", "retry_job", "Lane::Io", "Some(pointer)", "EpochFence::INITIAL.next()"]],
    ["database_catalog_bootstrap_replay_is_deterministic_and_never_reuses_initial_after_a_winner", ["for _ in 0..2", "EpochFence::INITIAL.next()", "Err(DbError::Fenced { expected: 1, actual: 0 })"]],
    ["database_catalog_bootstrap_publication_race_and_queue_pressure_keep_exact_successor", ["initial()", "successor()", "polls.load", "Ready is never repolled", "controlled_publication_before_waker_hook", "Poll::Ready(Ok(_))"]],
    ["database_catalog_bootstrap_public_result_drop_hands_back_exact_owner_without_post_admission_allocation", ["drop(result)", "terminal_completion", "owner.state.is_none()", "Some(pointer)", "take_database_catalog_bootstrap_terminal", "terminal.take_result()", "retained.into_parts()", "state.admission"]],
  ];
  for (const [name, tokens] of fixtureRequirements) {
    const body = interactivityDatabaseCatalogBootstrapFixtureBody(engineSource, name);
    if (body.length === 0 || tokens.some((token) => !body.includes(token))) failures.push(`P1w hostile law body is missing production-path evidence: ${name}`);
  }
  return failures;
}


//#endregion 🗄️P1wCatalogBootstrapCas

//#region 🌱️P1xCreateDocumentCatalogCas
function interactivityDatabaseCreateCatalogFixtureBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}`);
  if (start < 0) return "";
  const open = source.indexOf("{", start);
  return open < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
}

export function interactivityDatabaseCreateCatalogFailures(engineSource: string, asyncSource: string, contractSource: string): string[] {
  const production = interactivityProductionSource(engineSource);
  const asyncProduction = interactivityProductionSource(asyncSource);
  const start = production.indexOf("//#region 🔖️CreateDocumentCatalogCas");
  const end = production.indexOf("//#endregion 🔖️CreateDocumentCatalogCas", start);
  const retained = start < 0 || end < 0 ? "" : production.slice(start, end);
  const failures: string[] = [];
  const waits = production.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const createStart = production.indexOf("pub async fn create_document(");
  const createOpen = production.indexOf("{", createStart);
  const createDocument = createStart < 0 || createOpen < 0 ? "" : toolJobRustBlock(production, createOpen)?.body ?? "";
  const prepare = retained.slice(retained.indexOf("fn try_prepare("), retained.indexOf("pub fn generation(&self)"));
  const stateImpl = retained.indexOf("impl DatabaseCreateCatalogState");
  const scheduleStart = retained.indexOf("fn schedule(self: &Arc<Self>)", stateImpl);
  const submitStart = retained.indexOf("fn submit_exact", scheduleStart);
  const retryStart = retained.indexOf("fn retry(self: Arc<Self>)", submitStart);
  const driveStart = retained.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", retryStart);
  const schedule = retained.slice(scheduleStart, submitStart);
  const submit = retained.slice(submitStart, retryStart);
  const retry = retained.slice(retryStart, driveStart);
  const drive = retained.slice(driveStart, retained.indexOf("fn drive_claimed", driveStart));
  const claimed = retained.slice(retained.indexOf("fn drive_claimed"), retained.indexOf("fn stage_error", retained.indexOf("fn drive_claimed")));
  const scan = retained.slice(retained.indexOf("fn scan_one"), retained.indexOf("fn reserve_candidate_one"));
  const reserve = retained.slice(retained.indexOf("fn reserve_candidate_one"), retained.indexOf("fn clone_boundary"));
  const clone = retained.slice(retained.indexOf("fn clone_one"), retained.indexOf("fn snapshot_one"));
  const snapshot = retained.slice(retained.indexOf("fn snapshot_one"), retained.indexOf("fn encode_one"));
  const encode = retained.slice(retained.indexOf("fn encode_one"), retained.indexOf("fn seal_one"));
  const seal = retained.slice(retained.indexOf("fn seal_one"), retained.indexOf("fn claim_one"));
  const claim = retained.slice(retained.indexOf("fn claim_one"), retained.indexOf("fn handoff_one"));
  const handoff = retained.slice(retained.indexOf("fn handoff_one"), retained.indexOf("fn poll_backend_once"));
  const poll = retained.slice(retained.indexOf("fn poll_backend_once"), retained.indexOf("fn close_work_one"));
  const revalidate = retained.slice(retained.indexOf("fn revalidate_one"), retained.indexOf("fn retire_intermediate_one"));
  const retire = retained.slice(retained.indexOf("fn retire_intermediate_one"), retained.indexOf("fn publish_one"));
  const futurePoll = retained.slice(retained.indexOf("impl Future for DatabaseCreateCatalogFuture"), retained.indexOf("impl Drop for DatabaseCreateCatalogFuture"));
  const resultDrop = retained.slice(retained.indexOf("impl Drop for DatabaseCreateCatalogResult"), retained.indexOf("struct DatabaseCreateCatalogRejectedOwner"));
  const rejection = retained.slice(retained.indexOf("struct DatabaseCreateCatalogRejectedOwner"), retained.indexOf("type DatabaseCreateCatalogBackendFuture"));
  const terminal = retained.slice(retained.indexOf("impl DatabaseCreateCatalogTerminalHandle"), retained.indexOf("impl Drop for DatabaseCreateCatalogTerminalHandle"));
  const backing = retained.slice(retained.indexOf("struct DatabaseCreateCatalogBackingLedger"), retained.indexOf("struct DatabaseCreateCatalogAdmissionSlot"));
  const rejectionSubmitStart = rejection.indexOf("fn submit_exact(self: &Arc<Self>");
  const rejectionRetryStart = rejection.indexOf("fn retry(self: Arc<Self>)");
  const rejectionSubmit = rejection.slice(rejectionSubmitStart, rejectionRetryStart);
  const rejectionRetry = rejection.slice(rejectionRetryStart, rejection.indexOf("fn drive_one", rejectionRetryStart));
  const catalogStart = production.indexOf("pub async fn catalog(&self)");
  const catalogEnd = production.indexOf("pub async fn health(&self)", catalogStart);
  const catalogMethod = catalogStart < 0 || catalogEnd < 0 ? "" : production.slice(catalogStart, catalogEnd);
  const workerLoopStart = asyncProduction.indexOf("fn worker_loop(");
  const workerLoopEnd = asyncProduction.indexOf("/// 🧵️ The native, multi-OS-thread work-stealing pool", workerLoopStart);
  const workerLoop = workerLoopStart < 0 || workerLoopEnd < 0 ? "" : asyncProduction.slice(workerLoopStart, workerLoopEnd);
  const callbackAtStart = asyncProduction.indexOf("pub fn callback_at(");
  const callbackAtEnd = asyncProduction.indexOf("pub fn is_shutdown", callbackAtStart);
  const callbackAt = callbackAtStart < 0 || callbackAtEnd < 0 ? "" : asyncProduction.slice(callbackAtStart, callbackAtEnd);
  const scheduleCallbackStart = asyncProduction.indexOf("fn schedule_callback(");
  const scheduleCallback = scheduleCallbackStart < 0 ? "" : asyncProduction.slice(scheduleCallbackStart, asyncProduction.indexOf("fn is_fired", scheduleCallbackStart));
  const earlierDeadlineStart = asyncProduction.indexOf("inner.wheel.install_earlier_deadline_hook(");
  const earlierDeadline = earlierDeadlineStart < 0 ? "" : asyncProduction.slice(earlierDeadlineStart, asyncProduction.indexOf("}));", earlierDeadlineStart));

  const expectedWaits = production.includes("DatabaseSyncHelloFuture::try_submit") ? 0 : production.includes("db_compact::DatabaseCompactionFuture::try_submit") ? 1 : 2;
  if (waits !== expectedWaits || createDocument.includes("db_actor::block_on") || createDocument.includes("cas_root(") || createDocument.includes("target_arch = \"wasm32\"")) failures.push("P1x does not preserve the exact post-P1y native/Wasm caller cut");
  if (!workerLoop.includes("inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN)") || workerLoop.indexOf("inner.wheel.fire_due_batch") > workerLoop.indexOf("select_and_pop") || !callbackAt.includes("self.inner.wheel.schedule_callback") || !scheduleCallback.includes("self.announce_earlier_deadline(earliest)") || !earlierDeadline.includes("parking.signal_timer")) failures.push("P1x liveness evidence is not bound to real WorkerPool worker-loop timer service");
  for (const required of [
    "cancellation/deadline/exhaustion latency guarantee is conditional on shared-pool service",
    "at least one native worker must return to the head of `WorkerPool::worker_loop`",
    "sole OS worker that permanently never returns",
    "outside P1x's cancellation-latency guarantee",
    "exact refused job, storage, document, cursor/backing, admission and generation registry discoverable",
    "must not begin a backend poll",
    "invent a timer thread",
    "create a second pool",
    "facade/caller execution for completion",
    "close exactly once",
    "never a test-task call to `TimerWheel::fire_due`",
    "two-worker reserved-capacity law must drive actual saturated P1x and rejection-close authorities into `Retry`",
    "exact `callback_at(... state.retry())` registrations",
  ]) if (!contractSource.includes(required)) failures.push(`P1x liveness contract is missing narrowed shared-pool truth: ${required}`);
  if (!production.includes("entries: Arc<Vec<CatalogEntry>>") || !production.includes("revision: u64") || !production.includes("pending: Option<DatabaseCreateCatalogToken>")) failures.push("P1x catalog base lacks immutable Arc identity, revision, or pending CAS token");
  for (const required of [
    "const DATABASE_CREATE_CATALOG_SLOTS: usize = 32",
    "const DATABASE_CREATE_CATALOG_MAX_ENTRIES: usize = 4_096",
    "DbIoText::maximum_capacity()",
    "const DATABASE_CREATE_CATALOG_MAX_PAGES: usize = db_io_core::DB_IO_OPERATION_PAGES",
    "const DATABASE_CREATE_CATALOG_COPY_BYTES: usize = 256",
    "DATABASE_CREATE_CATALOG_TOTAL_ITEMS",
    "DATABASE_CREATE_CATALOG_TOTAL_BYTES",
    "DatabaseCreateCatalogAdmission::try_claim(&document)",
    "document.0.capacity() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES",
    "generation.checked_add(1)",
    "self.items.checked_add(DATABASE_CREATE_CATALOG_ITEMS)",
    "self.bytes.checked_add(DATABASE_CREATE_CATALOG_BYTES)",
  ]) if (!retained.includes(required)) failures.push(`P1x fixed credited pre-admission missing ${required}`);
  for (const required of [
    "DATABASE_CREATE_CATALOG_MAX_ENTRIES * 4",
    "DATABASE_CREATE_CATALOG_MAX_ENTRIES * 2",
    "struct DatabaseCreateCatalogBackingLedger",
    "fn observe(&mut self, items: u64, bytes: usize)",
    "self.items.checked_add(items)",
    "self.bytes.checked_add(bytes)",
    "self.items > DATABASE_CREATE_CATALOG_ITEMS",
    "self.bytes > DATABASE_CREATE_CATALOG_BYTES",
    "DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity())",
    "entry.document.0.capacity()",
    "candidate.capacity()",
    "text.capacity()",
    "DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES",
    "pages.checked_mul(db_io_core::DB_IO_PAGE_BYTES)",
  ]) if (!retained.includes(required)) failures.push(`P1x observed backing ledger missing ${required}`);
  if (!backing.includes("u64::try_from(base_capacity)") || !backing.includes("u64::try_from(bytes)") || backing.includes(" as u64")) failures.push("P1x observed backing ledger truncates allocated capacities");
  if (prepare.indexOf("DatabaseCreateCatalogAdmission::try_claim(&document)") < 0 || prepare.indexOf("DatabaseCreateCatalogAdmission::try_claim(&document)") > prepare.indexOf("Arc::clone(&catalog.entries)") || prepare.indexOf("catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES") > prepare.indexOf("let state = Arc::new(DatabaseCreateCatalogState") || prepare.indexOf("database_create_catalog_registry") > prepare.lastIndexOf("state.schedule()")) failures.push("P1x admission/base capture/registry publication ordering is not lossless");
  for (const phase of ["Scan", "Reserve", "Clone", "Snapshot", "Encode", "Seal", "Claim", "Handoff", "Poll", "CloseWork", "Revalidate", "Retire", "Publish", "Terminal"])
    if (!retained.includes(`DatabaseCreateCatalogPhase::${phase}`)) failures.push(`P1x retained phase missing ${phase}`);
  if (!scan.includes("entry.document == *document") || !scan.includes("cursor.scan_byte += 1") || !scan.includes("source_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES") || !scan.includes("Self::add_encoded(&mut cursor, bytes)")) failures.push("P1x duplicate/byte-cap/encoded-length scan is not one retained byte opportunity");
  if (!reserve.includes("try_reserve_exact(capacity)") || !clone.includes("DATABASE_CREATE_CATALOG_COPY_BYTES") && !retained.includes("DATABASE_CREATE_CATALOG_COPY_BYTES") || !clone.includes("clone_text") || !clone.includes("cursor.clone_byte = end") || !retained.includes("cursor.snapshot = Some(Arc::new(candidate))")) failures.push("P1x catalog copy/snapshot allocation is not fixed and incrementally cursor-owned");
  if (reserve.indexOf("cursor.candidate = Some(candidate)") < 0 || reserve.indexOf("cursor.candidate = Some(candidate)") > reserve.indexOf("cursor.backing.observe") || clone.indexOf("cursor.clone_text = Some(text)") < 0 || clone.indexOf("cursor.clone_text = Some(text)") > clone.indexOf("cursor.backing.observe")) failures.push("P1x post-allocation capacity refusal can drop candidate/String backing before retained retirement");
  if (!snapshot.includes("cursor.snapshot = Some(Arc::new(candidate))") || !snapshot.includes("DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES") || snapshot.indexOf("cursor.snapshot = Some(Arc::new(candidate))") > snapshot.indexOf("cursor.backing.observe") || !snapshot.includes("pages.checked_mul(db_io_core::DB_IO_PAGE_BYTES)") || snapshot.lastIndexOf("cursor.backing.observe") < snapshot.indexOf("pages.checked_mul(db_io_core::DB_IO_PAGE_BYTES)")) failures.push("P1x snapshot Arc/page backing is not retained before exact observed-capacity refusal");
  if (!retained.includes("struct DatabaseCreateCatalogEncodeCursor") || !retained.includes("pending: [u8; 32]") || !encode.includes("encode.step(snapshot.as_slice(), writer)") || !seal.includes("DbIoPageWriter::seal_retained_step") || retained.includes("encode_catalog_pages(&candidate)")) failures.push("P1x encoding/page transfer is not one fragment/page transition");
  if (!claim.includes("self.catalog.try_lock()") || !claim.includes("std::sync::TryLockError::WouldBlock") || !claim.includes("self.defer_catalog_contention()") || claim.includes("self.catalog.lock()") || !claim.includes("catalog.pending.is_some() && catalog.pending != Some(token)") || !claim.includes("catalog.revision != cursor.base_revision") || !claim.includes("catalog.epoch != cursor.base_epoch") || !claim.includes("Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity") || !claim.includes("catalog.pending = Some(token)")) failures.push("P1x CAS claim can block or does not bind revision, epoch, snapshot identity, and exact pending owner");
  if (!handoff.includes("DatabaseCreateCatalogWork::new(storage, pages, expected)") || handoff.includes(".poll(") || !retained.includes("storage.catalog().await.cas_root(expected, pages).await")) failures.push("P1x backend handoff combines owner transfer with a poll or bypasses retained CAS work");
  const idleToQueued = "compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8";
  const queuedToDriving = "compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8";
  const queuedToRetry = "compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8";
  const retryToQueued = "compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8";
  const drivingToIdle = "compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8, DatabaseCreateCatalogDriverAuthority::Idle as u8";
  const retryCallback = "self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry())";
  if (!schedule.includes(idleToQueued) || !schedule.includes("wake_requested.store(true") || !retained.includes("self.pool.try_submit(Lane::Io, job)")) failures.push("P1x driver admission is not uniquely claimed on shared Lane::Io");
  if (!submit.includes("Some((error.into_job(), next_attempt))") || submit.indexOf("Some((error.into_job(), next_attempt))") > submit.indexOf(queuedToRetry) || !submit.includes(retryCallback) || submit.indexOf(retryCallback) < submit.indexOf(queuedToRetry) || !retry.includes(retryToQueued) || !retry.includes("attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT") || !retry.includes("!self.is_current()") || !retry.includes("self.cancelled.load") || !retry.includes("self.deadline_ms.load") || !retry.includes("Some(job)") || !retry.includes("self.retry_closing.store(true") || !retry.includes("self.drive_callback_close_claimed(true)") || !retry.includes("fn callback_close_one") || !retry.includes("self.arm_callback_close()")) failures.push("P1x saturation/retry lacks its exact refusal-branch callback_at registration or bounded stale/cancel/deadline/exhaustion close handoff");
  if (!rejectionSubmit.includes("Some((error.into_job(), next_attempt))") || !rejectionSubmit.includes(retryCallback) || rejectionSubmit.indexOf(retryCallback) < rejectionSubmit.indexOf(queuedToRetry) || !rejectionRetry.includes("attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT") || !rejectionRetry.includes("self.pool.now_ms() >= self.deadline_ms") || !rejectionRetry.includes("Some(job)") || !rejectionRetry.includes("self.callback_close.store(true") || !rejectionRetry.includes("self.drive_close_claimed()") || !rejection.includes("fn callback_close_one") || !rejection.includes("terminal_job")) failures.push("P1x rejection-close saturation retry lacks its exact refusal-branch callback registration or exact job/owner close handoff");
  if (!drive.includes(queuedToDriving) || drive.indexOf(queuedToDriving) > drive.indexOf("self.opportunities.fetch_add") || drive.indexOf("self.drive_claimed(generation)") > drive.indexOf(drivingToIdle) || drive.indexOf(drivingToIdle) > drive.indexOf("wake_requested.swap(false")) failures.push("P1x driver claim/release/wake ordering permits a competing grant or lost wake");
  const cancellationCheck = claimed.indexOf("self.cancelled.load");
  const deadlineCheck = claimed.indexOf("self.deadline_ms.load");
  if (!claimed.includes("generation != self.generation || !self.is_current()") || cancellationCheck < 0 || claimed.indexOf("DatabaseCreateCatalogPhase::CloseWork", cancellationCheck) < cancellationCheck || claimed.indexOf("DatabaseCreateCatalogPhase::Revalidate", cancellationCheck) < cancellationCheck || deadlineCheck < 0 || claimed.indexOf("DatabaseCreateCatalogPhase::CloseWork", deadlineCheck) < deadlineCheck || claimed.indexOf("DatabaseCreateCatalogPhase::Revalidate", deadlineCheck) < deadlineCheck) failures.push("P1x freshness/cancel/deadline checks can retire a durable Ready CAS or admit ABA work");
  const pending = poll.slice(poll.indexOf("Ok(std::task::Poll::Pending)"), poll.indexOf("Ok(std::task::Poll::Ready"));
  const ready = poll.slice(poll.indexOf("Ok(std::task::Poll::Ready"), poll.indexOf("Err(_)"));
  const panic = poll.slice(poll.indexOf("Err(_)"));
  if (!poll.includes("std::panic::catch_unwind") || !poll.includes("work.poll(&mut context)") || !pending.includes("Some(work)") || pending.indexOf("Some(work)") > pending.indexOf("self.polling.store(false") || !ready.includes("Some(work)") || !ready.includes("Some(actual)") || ready.indexOf("Some(work)") > ready.indexOf("self.polling.store(false") || ready.indexOf("Some(actual)") > ready.indexOf("self.polling.store(false") || !panic.includes("Some(work)")) failures.push("P1x Pending/Ready/panic does not publish every exact owner before poll-claim release");
  const checkedRevision = revalidate.indexOf("catalog.revision.checked_add(1)");
  if (!revalidate.includes("self.catalog.try_lock()") || !revalidate.includes("std::sync::TryLockError::WouldBlock") || !revalidate.includes("self.defer_catalog_contention()") || revalidate.includes("self.catalog.lock()") || !revalidate.includes("catalog.pending != Some(token)") || !revalidate.includes("catalog.revision != cursor.base_revision") || !revalidate.includes("catalog.epoch != cursor.base_epoch") || !revalidate.includes("Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity") || !revalidate.includes("cursor.base_epoch.epoch.checked_add(1)") || checkedRevision < 0 || revalidate.indexOf("catalog.entries = snapshot") < revalidate.indexOf("catalog.pending != Some(token)") || checkedRevision > revalidate.indexOf("catalog.entries = snapshot")) failures.push("P1x nonblocking revalidation/publication can overwrite a newer catalog or wrap revision");
  if (!retire.includes("self.catalog.try_lock()") || !retire.includes("std::sync::TryLockError::WouldBlock") || !retire.includes("self.defer_catalog_contention()") || retire.includes("self.catalog.lock()") || !retire.includes("self.pending_owned.store(false")) failures.push("P1x pending-token retirement can block a worker or release ownership before catalog cleanup");
  const catalogArc = catalogMethod.indexOf("Arc::clone(&catalog.entries)");
  const catalogDeepClone = catalogMethod.indexOf("entries.as_ref().clone()");
  if (catalogArc < 0 || catalogDeepClone < catalogArc || catalogMethod.indexOf("};", catalogArc) > catalogDeepClone || catalogMethod.includes("catalog.entries.as_ref().clone()")) failures.push("Database::catalog still deep-clones the dynamic catalog while holding the transaction mutex");
  if (!retire.includes("pages.close_step()") || !retire.includes("writer.close_step()") || !retire.includes("candidate.pop()") || !retire.includes("cursor.clone_text.take()") || !retire.includes("cursor.base.take()") || retire.includes("loop {") || retire.includes("while ")) failures.push("P1x close is not one admitted page/backing/owner per Lane::Io opportunity");
  if (!futurePoll.includes("completion.lock()") || futurePoll.indexOf("waker.lock()") < futurePoll.indexOf("completion.lock()") || futurePoll.lastIndexOf("completion.lock()") < futurePoll.indexOf("waker.lock()")) failures.push("P1x completion lacks check-register-recheck lost-wakeup closure");
  if (!resultDrop.includes("state.terminal_completion.lock()") || !resultDrop.includes("state.abandoned.store(true") || !resultDrop.includes("state.begin_callback_close()") || !rejection.includes("DatabaseCreateCatalogRejectedClose::prepare(pool") || !rejection.includes("self.close.take_owner()") || !rejection.includes("self.close.restore_owner(owner)") || !terminal.includes("self.state.begin_callback_close()") || !retire.includes("self.terminal_job") ) failures.push("P1x result/rejection/lost-handle paths do not hand exact owners to retained close authority");
  const mountRunStart = production.indexOf("async fn run_document_mount(");
  const mountRun = mountRunStart < 0 ? "" : production.slice(mountRunStart, production.indexOf("async fn mount_document(", mountRunStart));
  const publishMountStart = production.indexOf("async fn publish_mount_catalog(");
  const publishMount = publishMountStart < 0 ? "" : production.slice(publishMountStart, production.indexOf("async fn run_open_document_mount(", publishMountStart));
  const mountComplete = production.slice(production.indexOf("fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>)"), production.indexOf("//#region 🔖️Database"));
  const published = mountRun.indexOf("Self::publish_mount_catalog(");
  const spawn = mountRun.indexOf("db_artifact::ArtifactAuthority::spawn_with_pool_use(");
  const emit = mountRun.indexOf("db_engine.document_created");
  const reply = mountRun.indexOf("Ok(DatabaseDocumentMountReply");
  if (!createDocument.includes("self.mount_document(spec.document, DatabaseDocumentMountPolicy::Create)") || !publishMount.includes("DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use, catalog, storage, document, true)") || !publishMount.includes("Err(rejected) => return Err(rejected.close_and_take_error())") || !publishMount.includes("let result = transaction.await?") || !publishMount.includes("actual.map(|_| ())") || published < 0 || spawn < published || emit < spawn || reply < emit || !mountComplete.includes("DatabaseDocumentMountSlot::Ready(")) failures.push("P1x caller spawns/emits/registers before durable catalog publication or loses rejected owners");
  const retainedLoops = interactivityDriverHandshakeBlanked(retained, "DatabaseCreateCatalogDriverAuthority");
  for (const forbidden of ["db_actor::block_on", "thread::spawn", "WorkerPool::new(", "loop {", "while ", "#[cfg(target_arch", ".unwrap()", ".expect("])
    if (retainedLoops.includes(forbidden)) failures.push(`P1x retained production region contains prohibited ${forbidden}`);
  const laws: readonly [string, readonly string[]][] = [
    ["database_create_catalog_max_plus_one_document_and_entry_caps_return_exact_owners", ["DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1", "document.0.capacity()", "DATABASE_CREATE_CATALOG_MAX_ENTRIES", "Arc::as_ptr(&storage)", "into_parts()"]],
    ["database_create_catalog_observed_vec_and_string_overallocation_faults_retire_exact_backings", ["controlled_capacity_overage", "DATABASE_CREATE_CATALOG_ITEMS as usize + 1", "candidate.is_some()", "observed backing capacity", "DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1", "clone_text.is_some()", "cloned string capacity", "admission"]],
    ["database_create_catalog_large_tree_yields_scan_copy_encode_seal_and_publishes_exact_epoch", ["0..128", "state.opportunities.load", "epoch.next()", "catalog.revision", "catalog.pending.is_none()"]],
    ["database_create_catalog_duplicate_and_concurrent_same_base_are_deterministic", ["AlreadyExists", "first.is_ok()", "second.is_ok()", "DbError::Fenced", "filter(|entry|"]],
    ["database_create_catalog_cancel_deadline_and_generation_aba_preserve_exact_identity", ["probe.cancel()", "deadline_ms.store(0", "replacement", "StaleGeneration", "Arc::as_ptr(&storage)"]],
    ["database_create_catalog_handoff_cancel_claim_prevents_backend_poll_and_retires_exact_pages", ["catalog_bootstrap_pages(3)", "controlled_driver_hook", "DatabaseCreateCatalogDriverAuthority::Driving", "active_drivers", "probe.cancel()", "!state.polling.load", "Some(operation)", "Err(DbError::Closed)"]],
    ["database_create_catalog_pending_ready_and_panic_publish_work_before_driver_release", ["ControlledCreateCatalogPoll::Pending", "ControlledCreateCatalogPoll::Ready", "ControlledCreateCatalogPoll::Panic", "cancel_on_ready", "Ok(epoch.next())", "poll_worker_thread", "poll_work", "terminal_work"]],
    ["database_create_catalog_saturation_retains_exact_job_and_recovers", ["WorkerSubmitErrorKind::Saturated", "retry_job", "Some(pointer)", "actual.is_ok()"]],
    ["database_create_catalog_real_worker_loop_services_finite_saturation_cancel_deadline_exhaustion_and_close", ["replenishing_held_create_catalog_io_pool", "release_held_create_catalog_worker", "spent_submission_attempts", "DATABASE_CREATE_CATALOG_RETRY_LIMIT", "backend_polls", "callback_worker_thread", "terminal_job_retirements", "retry exhausted", "retry_job", "terminal_job", "database_create_catalog_registry", "close.terminal_is_empty()", "pool.shutdown()"]],
    ["database_create_catalog_two_worker_reserved_capacity_services_timers_while_one_violator_is_held", ["reserved_replenishing_create_catalog_io_pool", "DatabaseCreateCatalogFuture::try_submit", "DatabaseCreateCatalogDriverAuthority::Retry", "retry_job", "reserved-cancel", "reserved-deadline", "reserved-exhaust", "release_held_create_catalog_worker(&service_gate)", "spent_submission_attempts", "DATABASE_CREATE_CATALOG_RETRY_LIMIT", "backend_polls", "callback_worker_thread", "terminal_job_retirements", "database_create_catalog_registry", "admission", "close.terminal_is_empty()", "maintenance_gate"]],
    ["database_create_catalog_sole_permanently_nonreturning_worker_retains_discoverable_owners_without_latency_claim", ["held_create_catalog_io_pool", "probe.cancel()", "drop(probe)", "DatabaseCreateCatalogDriverAuthority::Retry", "retry_job", "storage_pointer", "admission", "backend_polls", "database_create_catalog_registry", "take_database_create_catalog_terminal", "DatabaseCreateCatalogCloseStep::Blocked", "!terminal.terminal_is_empty()"]],
    ["database_create_catalog_drop_terminal_close_retires_one_owner_per_lane_grant", ["take_database_create_catalog_terminal", "terminal.close_step()", "state.opportunities", "initial.saturating_sub(current) as u64 <= granted - initial_grants", "state.admission"]],
    ["database_create_catalog_one_production_opportunity_is_under_eight_ms_and_native_wasm_share_source", ["drive_one(state.generation)", "from_millis(8)", "opportunities.load", "include_str!", "target_arch", "db_actor::block_on"]],
    ["database_create_catalog_maximum_catalog_claim_revalidate_and_snapshot_clone_never_hold_worker", ["DATABASE_CREATE_CATALOG_MAX_ENTRIES - 1", "DatabaseCreateCatalogPhase::Claim", "DatabaseCreateCatalogPhase::Revalidate", "DatabaseCreateCatalogPhase::Retire", "pending_owned", "catalog_contention_armed", "from_millis(8)", "Arc::clone(&catalog.entries)", "entries.as_ref().clone()"]],
    ["database_create_catalog_durable_publication_precedes_authority_spawn_emit_and_registration", ["create_document_catalog_retained", "actual.is_ok()", "open_artifacts", "catalog", "document"]],
    ["database_create_catalog_publication_check_register_recheck_has_no_lost_wake", ["controlled_publication_before_waker_hook", "hook_state.schedule()", "completion.lock()", "published.load", "state.waker", "Ok(epoch.next())"]],
  ];
  for (const [name, tokens] of laws) {
    const body = interactivityDatabaseCreateCatalogFixtureBody(engineSource, name);
    if (body.length === 0 || tokens.some((token) => !body.includes(token))) failures.push(`P1x hostile law body is missing production-path evidence: ${name}`);
    if (body.includes("timer_wheel().fire_due")) failures.push(`P1x hostile law uses test-task timer progress instead of WorkerPool worker_loop: ${name}`);
  }
  return failures;
}


//#endregion 🌱️P1xCreateDocumentCatalogCas

//#region 🧹P1yDatabaseCompaction
function interactivityDatabaseCompactionFixtureBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}`);
  if (start < 0) return "";
  const open = source.indexOf("{", start);
  return open < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
}

export function interactivityDatabaseCompactionFailures(compactSource: string, engineSource: string, snapshotSource: string, indexSource: string, contractSource: string): string[] {
  const compact = interactivityProductionSource(compactSource);
  const engine = interactivityProductionSource(engineSource);
  const snapshot = interactivityProductionSource(snapshotSource);
  const index = interactivityProductionSource(indexSource);
  const start = compact.indexOf("//#region 🧵️RetainedCompactionJob");
  const end = compact.indexOf("//#endregion 🧵️RetainedCompactionJob", start);
  const retained = start < 0 || end < 0 ? "" : compact.slice(start, end);
  const facadeStart = engine.indexOf("fn compact_document_retained(");
  const facadeEnd = engine.indexOf("pub fn hello_retained", facadeStart);
  const facade = facadeStart < 0 || facadeEnd < 0 ? "" : engine.slice(facadeStart, facadeEnd);
  const failures: string[] = [];
  const waits = engine.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const expectedWaits = engine.includes("DatabaseSyncHelloFuture::try_submit") ? 0 : 1;
  if (facade.includes(".compact_retained(holder, consolidate_snapshots,")) failures.push("P1y facade compacts through the document actor (`retained_compaction_with_wal`): a lease release error there completes publicly and consumes the fence, with no retained release witness, while the retained `DatabaseCompactionFuture` that carries the witness has no production caller");
  else if (waits !== expectedWaits || facade.includes("block_on(") || !facade.includes("DatabaseCompactionFuture::try_submit") || !facade.includes("let requested_at_ms = now_ms().await") || !facade.includes("compaction.await?.close_and_take_report()") || !facade.includes("Err(rejected) => return Err(rejected.close_and_take_error())")) failures.push("P1y selected facade wait was not cut over to the retained terminal witness with the exact post-P1z wait census and admitted epoch");
  for (const required of [
    "const DATABASE_COMPACTION_SLOTS: usize = 32",
    "const DATABASE_COMPACTION_MAX_SEGMENTS: usize = 64",
    "DATABASE_COMPACTION_OPERATION_ITEMS",
    "DATABASE_COMPACTION_OPERATION_BYTES",
    "DATABASE_COMPACTION_TOTAL_ITEMS",
    "DATABASE_COMPACTION_TOTAL_BYTES",
    "document.0.capacity() > db_io_core::DbIoText::maximum_capacity()",
    "struct DatabaseCompactionBackingLedger",
    "fn database_compaction_observe_backing",
    "descriptor.roots.capacity()",
    "descriptor.new_pages.capacity()",
    "index_document.0.capacity()",
    "async fn close_compaction_descriptor",
    "database_compaction_admit_descriptor",
    "generation.checked_add(1)",
    "DatabaseCompactionAdmission::try_claim(&document)",
    "database_compaction_registry()",
    "struct DatabaseCompactionSegmentOwners",
    "CompactionRetainedPages",
    "type DatabaseCompactionExecutionFuture",
    "quarantined: Option<DatabaseCompactionExecutionFuture>",
    "pub struct DatabaseCompactionResult",
    "pub struct DatabaseCompactionRejected",
  ]) if (!retained.includes(required)) failures.push(`P1y fixed retained authority missing ${required}`);
  if (retained.includes("struct DatabaseCompactionHashOwners") || retained.includes("DATABASE_COMPACTION_MAX_HASHES") || !compactSource.includes("fn document_compaction_retains_shared_and_private_cas_without_global_reference_authority(")) failures.push("P1y production compaction retains a payload-hash working set instead of deferring payload reclamation to the global reference authority");
  if (!compact.includes("pub struct CompactionIndexReports") || !compact.includes("slots: [Option<IndexKindReport>; COMPACTION_INDEX_REPORTS]") || compact.includes("pub struct CompactionIndexReports(Vec")) failures.push("P1y fixed index report authority is missing");
  if ((retained.match(/close_compaction_descriptor\(/g)?.length ?? 0) < 6) failures.push("P1y admitted snapshot descriptor owners do not all use incremental backing retirement");
  const descriptorClose = retained.slice(retained.indexOf("async fn close_compaction_descriptor"), retained.indexOf("async fn retire_compaction_snapshot_body"));
  if (!descriptorClose.includes("retire_compaction_descriptor(descriptor).await") || !descriptorClose.includes("ledger.release(items, bytes)") || descriptorClose.indexOf("retire_compaction_descriptor(descriptor).await") > descriptorClose.indexOf("ledger.release(items, bytes)")) failures.push("P1y descriptor close does not retire the exact owner before returning its cumulative backing credit");
  if (!retained.includes("self.next_generation = generation.checked_add(1)") || !snapshot.includes("expected_generation.checked_add(1)")) failures.push("P1y admission and atomic snapshot publication generations are not both overflow-checked");
  if (/\b(Vec|HashMap|HashSet|BTreeMap|BTreeSet)\s*</.test(retained)) failures.push("P1y retained production region contains a dynamic working-set collection");
  for (const authority of ["Idle", "Queued", "Driving", "Retry"])
    if (!retained.includes(`DatabaseCompactionDriverAuthority::${authority}`)) failures.push(`P1y atomic driver authority missing ${authority}`);
  const schedule = retained.slice(retained.indexOf("fn schedule(self: &Arc<Self>)"), retained.indexOf("fn submit_exact"));
  const submit = retained.slice(retained.indexOf("fn submit_exact"), retained.indexOf("fn retry("));
  const retry = retained.slice(retained.indexOf("fn retry("), retained.indexOf("fn drive_one"));
  const drive = retained.slice(retained.indexOf("fn drive_one"), retained.indexOf("fn poll_one"));
  const poll = retained.slice(retained.indexOf("fn poll_one"), retained.indexOf("fn move_output_to_terminal"));
  const releaseRecovery = retained.slice(retained.indexOf("impl DatabaseCompactionLeaseRecovery"), retained.indexOf("async fn retained_compaction_execute"));
  const releaseRetry = retained.slice(retained.indexOf("fn arm_release_retry"), retained.indexOf("fn drive_one"));
  const callbackClose = retained.slice(retained.indexOf("fn drive_close_claimed"), retained.indexOf("fn close_terminal_one"));
  if (!schedule.includes("compare_exchange(DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Queued as u8") || !schedule.includes("self.submit_exact") || !schedule.includes("callback_close.load(Ordering::Acquire) && execution_terminal") || !schedule.includes("core.release_waiting.is_none()") || !schedule.includes("core.release_fault.is_none()") || !schedule.includes("!self.release_retry_armed.load")) failures.push("P1y scheduling lacks the unique Idle-to-Queued owner claim or permits callback cleanup to overtake retained lease release");
  if (callbackClose.includes("poll_one(")) failures.push("P1y callback/drop cleanup can poll the live backend future outside typed Lane::Io");
  if (!submit.includes("self.pool.try_submit(Lane::Io, job)") || !submit.includes("Some((error.into_job(), next))") || !submit.includes("DatabaseCompactionDriverAuthority::Retry") || !submit.includes("self.pool.callback_at")) failures.push("P1y Lane::Io saturation does not retain and register the exact retry closure");
  if (!retry.includes("DATABASE_COMPACTION_RETRY_LIMIT") || !retry.includes("self.deadline_ms.load") || !retry.includes("self.cancelled.load") || !retry.includes("DatabaseCompactionDriverAuthority::Retry as u8, DatabaseCompactionDriverAuthority::Queued as u8") || !retry.includes("self.submit_exact(job, attempt)")) failures.push("P1y retry exhaustion/cancel/deadline does not retain the exact closure for eventual Lane::Io cleanup");
  if (!drive.includes("DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Driving as u8") || drive.indexOf("self.poll_one()") > drive.indexOf("self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8") || drive.indexOf("self.wake_requested.swap(false") > drive.indexOf("self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8")) failures.push("P1y driver can release its claim before polling and consuming the published wake");
  const readyPublication = poll.indexOf("core.output = Some(output)");
  if (!poll.includes("core.future = Some(future)") || readyPublication < 0 || !poll.includes("core.quarantined = Some(future)") || !poll.includes("core.panic_release = if") || !poll.includes("self.lease_recovery.retry_release_after_panic()") || !poll.includes("self.lease_recovery.release_future()") || !poll.includes("std::panic::catch_unwind") || poll.indexOf("drop(core)", readyPublication) < readyPublication) failures.push("P1y Pending/Ready/panic does not publish every exact owner and panic-release future before driver release");
  if (!releaseRecovery.includes("if result.is_ok()") || !releaseRecovery.includes("recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()") || !releaseRecovery.includes("recovery.released.store(true") || !releaseRecovery.includes("recovery.releasing.store(false") || releaseRecovery.indexOf("if result.is_ok()") > releaseRecovery.indexOf("recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()")) failures.push("P1y lease release consumes the retained fence or publishes released on backend Err");
  if (!poll.includes("Ok(std::task::Poll::Ready(Err(error)))") || !poll.includes("core.release_fault = Some(error)") || !poll.includes("core.release_retry_fault = Some(error)") || !poll.includes("self.arm_release_retry()") || !poll.includes("core.release_waiting = Some(output)")) failures.push("P1y release Err/output is not retained before retry or can publish before the successful lease witness");
  if (!releaseRetry.includes("self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.release_retry_callback())") || !releaseRetry.includes("self.lease_recovery.released.load") || !releaseRetry.includes("core.release_retry_fault.take().or_else(|| core.release_fault.take())") || !releaseRetry.includes("core.release_waiting.take()") || !releaseRetry.includes("core.panic_release = self.lease_recovery.release_future()") || !releaseRetry.includes("self.schedule()")) failures.push("P1y retained lease release fault does not back off through the real worker loop, close exactly, and resume only typed Lane::Io polling");
  const opportunity = retained.slice(retained.indexOf("async fn compaction_opportunity"), retained.indexOf("fn compaction_resource"));
  if (!opportunity.includes("semio_framework_async::yield_once().await") || !opportunity.includes("cancelled.load")) failures.push("P1y opportunity does not yield and check cancellation once");
  for (const body of ["contains", "retained_compaction_under_lease", "retained_compaction_snapshot"])
    if (retained.indexOf(`${body}(`) < 0 || retained.indexOf("compaction_opportunity", retained.indexOf(`${body}(`)) < 0) failures.push(`P1y cursor ${body} lacks bounded opportunity control`);
  if (!retained.includes("lease_recovery.install(fence)") || !retained.includes("lease_recovery.release_future()") || !retained.includes("match (run, release)") || !retained.includes("(Err(error), _) => Err(error)") || !retained.includes("DatabaseCompactionProgress::LeaseRelease")) failures.push("P1y cancellation/fault/panic does not prioritize the retained exact lease release with earlier-run error precedence");
  if (!retained.includes("lease.acquire(lease_recovery.resource.as_str(), holder.as_str(), DEFAULT_LEASE_TTL_MS, now_ms)") || !retained.includes("publish_retained_expected(document, latest_generation") || retained.includes("manager.publish_retained(document, db_snapshot::SnapshotOrigin::FullBaseline")) failures.push("P1y lease/publication is not qualified by the admitted epoch and atomic expected snapshot generation");
  const indexChild = retained.slice(retained.indexOf("progress.store(DatabaseCompactionProgress::IndexMerge"), retained.indexOf("if consolidate_snapshots"));
  const indexControl = index.slice(index.indexOf("pub fn retained_operation_control"), index.indexOf("pub fn cancel", index.indexOf("pub fn retained_operation_control")));
  const indexGrant = index.slice(index.indexOf("pub fn grant(&mut self)"), index.indexOf("pub struct IndexBytes"));
  if (!indexChild.includes("compaction_opportunity(cancelled).await?") || !indexChild.includes("retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL)") || !indexChild.includes("DATABASE_COMPACTION_TURN_MS") || indexChild.includes("operation_control(65_536)") || !indexChild.includes("handle.compact(&mut control).await") || indexChild.indexOf("drop(handle)") < 0 || indexChild.indexOf("drop(handle)") > indexChild.indexOf("let stats = match stats") || !indexChild.includes('if message == "index cursor cancelled" => return Err(DbError::Closed)') || !indexControl.includes("IndexCursorControl::retained(cancelled, deadline, fuel)") || !index.includes("cooperative: Some((duration, fuel))")) failures.push("P1y index child does not use the exact parent cancel, eight-ms deadline and bounded resumable fuel authority with lossless debit return");
  if (indexGrant.includes("std::thread::yield_now()")) failures.push("P1y retained index budget refill yields the OS thread (std::thread::yield_now) instead of the owned compaction future at its eight-ms boundary");
  const expectedPublish = snapshot.slice(snapshot.indexOf("pub async fn publish_retained_expected"), snapshot.indexOf("async fn publish_page_source", snapshot.indexOf("pub async fn publish_retained_expected")));
  const expectedBuild = snapshot.slice(snapshot.indexOf("async fn build_generation_retained_expected"), snapshot.indexOf("pub async fn build_generation_pages", snapshot.indexOf("async fn build_generation_retained_expected")));
  const publicationClaim = snapshot.slice(snapshot.indexOf("const SNAPSHOT_PUBLICATION_CLAIMS"), snapshot.indexOf("impl SnapshotPageSource for OptionalSnapshotPages"));
  if (!expectedPublish.includes("let _claim = SnapshotPublicationClaim::try_claim(self.storage.publication_scope(), document)?") || !publicationClaim.includes("[u64; SNAPSHOT_PUBLICATION_CLAIMS]") || !publicationClaim.includes("if claims.contains(&identity) {\n            return Err(DbError::Conflict(") || !publicationClaim.includes("impl Drop for SnapshotPublicationClaim") || !publicationClaim.includes("if claims[self.slot] == self.identity") || publicationClaim.includes(".await") || !expectedPublish.includes("observed != Some(expected_generation)") || expectedPublish.indexOf("observed != Some(expected_generation)") > expectedPublish.indexOf("write_generation") || !expectedPublish.includes("SnapshotRetainedPublicationRejected { error, body }") || expectedBuild.includes("Vec::with_capacity") || !expectedBuild.includes("write_retained_publication_descriptor")) failures.push("P1y expected snapshot publication lacks one nonblocking atomic claim, pre-write stale refusal, exact body recovery or allocation-free retained hash path");
  if (!retained.includes("next_items = self.items.checked_add(items)") || !retained.includes("next_bytes = self.bytes.checked_add(bytes)") || !retained.includes("self.items.checked_sub") || !retained.includes("self.bytes.checked_sub") || (retained.match(/ledger\.release\(page_items, page_bytes\)\?/g)?.length ?? 0) !== 6 || !retained.includes("close_compaction_descriptor(owner, ledger).await?")) failures.push("P1y observed backing is not cumulative/debit-returned across every retained descriptor/page/error owner");
  if (!retained.includes("pool.callback_at(deadline_ms, move || deadline.deadline_callback())") || !retained.includes("expired.store(true") || !retained.includes("DbError::Timeout(\"database compaction deadline\"")) failures.push("P1y deadline does not use the mounted production callback and typed terminal result");
  const resultDrop = retained.slice(retained.indexOf("impl Drop for DatabaseCompactionResult"), retained.indexOf("pub struct DatabaseCompactionFuture"));
  const futureDrop = retained.slice(retained.indexOf("impl Drop for DatabaseCompactionFuture"), retained.indexOf("struct DatabaseCompactionRejectedClose"));
  if (!resultDrop.includes("DatabaseCompactionTerminalOwners::from_execution") || !resultDrop.includes("state.arm_callback_close()") || !futureDrop.includes("state.abandoned.store(true") || !futureDrop.includes("state.cancelled.store(true") || !futureDrop.includes("state.schedule()")) failures.push("P1y result/future Drop can destroy or orphan retained owners");
  const publicPoll = retained.slice(retained.indexOf("impl Future for DatabaseCompactionFuture"), retained.indexOf("impl Drop for DatabaseCompactionFuture"));
  const firstOutputCheck = publicPoll.indexOf("core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take()");
  const registerWaker = publicPoll.indexOf("= Some(context.waker().clone())");
  const secondOutputCheck = publicPoll.indexOf("core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take()", firstOutputCheck + 1);
  if (firstOutputCheck < 0 || registerWaker < firstOutputCheck || secondOutputCheck < registerWaker || publicPoll.indexOf("state.schedule()", secondOutputCheck) < secondOutputCheck) failures.push("P1y public completion lacks check-register-recheck lost-wake closure");
  if (!publicPoll.includes("state.panic_retired.load") || publicPoll.indexOf("state.panic_retired.load") > publicPoll.indexOf("if !state.current()") || publicPoll.includes("quarantined.is_some() {\n            self.completed = true")) failures.push("P1y public panic completion can precede lease release, quarantine retirement and registry drain");
  const close = retained.slice(retained.indexOf("impl DatabaseCompactionTerminalOwners"), retained.indexOf("struct DatabaseCompactionCore"));
  if (!close.includes("report.index_reports.close_step()") || !close.includes("holder.as_mut().is_some_and") || !close.includes("self.document.take().is_some() || self.storage.take().is_some()") || close.includes("while ") || close.includes("loop {")) failures.push("P1y terminal close is not exactly one report/holder/document/storage owner per opportunity");
  const stateClose = retained.slice(retained.indexOf("fn close_terminal_one(&self)"), retained.indexOf("fn arm_callback_close", retained.indexOf("fn close_terminal_one(&self)")));
  if (!stateClose.includes("lease_recovery.released.load") || !stateClose.includes("core.release_quarantined.take()") || !stateClose.includes("core.quarantined.take()") || !stateClose.includes("self.release_terminal()") || !stateClose.includes("self.panic_retired.store(true")) failures.push("P1y panic close does not retire every release/execution quarantine and release admission/registry after the exact lease witness");
  for (const eager of ["segment_horizons", "plan_wal_retention", "apply_wal_retention", "sweep_payloads", "compact_all_indexes", "collect_chain_pages", "SnapshotConsolidator", "build_cold_archive", "Compactor<'storage>"])
    if (!compactSource.includes(`#[cfg(test)]\n${eager === "SnapshotConsolidator" || eager === "Compactor<'storage>" ? "pub struct " : eager === "plan_wal_retention" ? "pub fn " : ""}${eager}`) && !compactSource.includes(`#[cfg(test)]\nasync fn ${eager}`) && !compactSource.includes(`#[cfg(test)]\npub async fn ${eager}`)) failures.push(`P1y eager compatibility path remains live: ${eager}`);
  const laws: readonly [string, readonly string[]][] = [
    ["retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms", ["held_compaction_worker_pool", "DatabaseCompactionDriverAuthority::Queued", "future.cancel()", "Arc::as_ptr(&storage)", "document.0.as_ptr()", "from_millis(8)", "DbError::Closed", "database_compaction_registry"]],
    ["retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once", ["deadline_ms.store(0", "state.deadline_callback", "Wake::wake_by_ref", "DbError::Timeout", "admission", "DatabaseCompactionDriverAuthority::Idle"]],
    ["retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority", ["DATABASE_COMPACTION_SLOTS", "p1y-slot-max-plus-one", "database compaction admission slots", "maximum_capacity() + 1", "document.0.as_ptr()", "DatabaseCompactionHashOwners", "DATABASE_COMPACTION_MAX_HASHES", "SnapshotDescriptor", "Vec::with_capacity(DATABASE_COMPACTION_OPERATION_ITEMS as usize)", "database_compaction_admit_descriptor", "database compaction snapshot backing"]],
    ["retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity", ["replacement", "DbError::StaleGeneration", "database_compaction_registry", "DatabaseCompactionTerminalOwners", "before - after, 1", "terminal_is_empty"]],
    ["retained_compaction_index_child_uses_exact_parent_cancel_and_eight_ms_control", ["retained_operation_control", "cancelled.clone()", "DATABASE_COMPACTION_TURN_MS", "DATABASE_COMPACTION_INDEX_FUEL", "index cursor cancelled", "65_536"]],
    ["retained_compaction_expected_snapshot_publication_never_persists_stale_baseline", ["publish_retained_expected", "expected", "StaleGeneration", "generations.as_slice(), &[0, 1]", "retire_compaction_snapshot_body"]],
    ["retained_compaction_panic_after_lease_acquire_releases_once_before_public_fault_and_registry_drain", ["lease_recovery.install", "DatabaseCompactionExecutionFuture", "panic!", "panic_retired", "admission", "database_compaction_registry"]],
    ["retained_compaction_release_error_retries_through_real_worker_loop_until_success_before_public_fault", ["fail_release_attempts(1)", "release_attempts", "core.future.is_none()", "release_fault.is_none()", "lease_recovery.released", "lease_storage.lease().await.current", "admission", "database_compaction_registry"]],
    ["retained_compaction_perpetual_release_error_keeps_fence_fault_admission_and_registry_discoverable", ["fail_release_attempts(usize::MAX)", "release_attempts", "core.future.is_none()", "!state.lease_recovery.released", "Some(fence)", "release_fault.is_some()", "panic_release.is_some()", "release_retry_armed", "panic_retired", "admission", "database_compaction_registry"]],
    ["retained_compaction_cumulative_observed_backing_rejects_individually_valid_combined_max_plus_one", ["DatabaseCompactionBackingLedger", "DATABASE_COMPACTION_OPERATION_BYTES", "cumulative max plus one", "ledger.release", "ledger.items, ledger.bytes"]],
  ];
  for (const [law, evidence] of laws) {
    const body = interactivityDatabaseCompactionFixtureBody(compactSource, law);
    if (!body || evidence.some((token) => !body.includes(token))) failures.push(`P1y hostile law ${law} is missing production-path evidence`);
  }
  for (const required of ["Selected Production Wait", "Database::compact_document", "run_from_latest_snapshot", "explicit resumable state owner", "generation-tagged slots", "lease", "cancellation", "timing validation"])
    if (!contractSource.includes(required)) failures.push(`P1y governing census lost ${required}`);
  if (!contractSource.includes("The P1y facade cut is `Database::compact_document`")) failures.push("P1y governing census lost the exact selected facade cut");
  for (const required of ["private child token, thirty-second deadline, or 65,536-fuel control is outside the P1y contract", "refuses a mismatched generation before descriptor/page construction or storage write", "Public fault completion is forbidden until the release witness", "Only `Ok(())` from the backend release may consume the retained fence", "persistent release error blocks public terminal completion", "Backing credit is cumulative across every simultaneously live descriptor"])
    if (!contractSource.includes(required)) failures.push(`P1y independent RED remediation contract lost ${required}`);
  return failures;
}


//#endregion 🧹P1yDatabaseCompaction

//#region 👋️P1zDatabaseSyncHello
function interactivityDatabaseSyncHelloFixtureBody(source: string, name: string): string {
  const start = source.indexOf("fn " + name);
  if (start < 0) return "";
  const open = source.indexOf("{", start);
  return open < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
}

export function interactivityDatabaseSyncHelloFailures(syncSource: string, engineSource: string, hubSource: string, walSource: string, protocolSource: string, contractSource: string): string[] {
  const sync = interactivityProductionSource(syncSource);
  const engine = interactivityProductionSource(engineSource);
  const hub = interactivityProductionSource(hubSource);
  const wal = interactivityProductionSource(walSource);
  const protocol = interactivityProductionSource(protocolSource);
  const retained = interactivityRegionSource(sync, "👋️RetainedHello") ?? "";
  const facadeStart = engine.indexOf("pub fn hello_retained");
  const facadeEnd = engine.indexOf("\n}\n", facadeStart);
  const facade = facadeStart < 0 || facadeEnd < 0 ? "" : engine.slice(facadeStart, facadeEnd);
  const admissionReadyStart = sync.indexOf("impl std::future::Future for DatabaseSyncHelloAdmissionReady");
  const admissionReady = admissionReadyStart < 0 ? "" : sync.slice(admissionReadyStart, sync.indexOf("impl Drop for DatabaseSyncHelloAdmissionReady", admissionReadyStart));
  const failures: string[] = [];
  const waits = engine.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  if (waits !== 0 || facade.includes("block_on(") || !facade.includes("DatabaseSyncHelloFuture::try_submit") || !facade.includes("hello.await?.close_and_take_session()") || !facade.includes("let error = rejected.close_and_take_error();") || !facade.includes("db_sync::DatabaseSyncHelloAdmissionReady::new(self.pool.clone(), deadline_ms).await?") || !admissionReady.includes("state.waiters.register(&mut this.waiter, context.waker())") || !admissionReady.includes("this.pool.callback_at(this.deadline_ms") || admissionReady.includes("block_on(")) failures.push("P1z final selected engine wait is not the retained terminal witness");
  const hubHello = hub.slice(hub.indexOf("let mut hello_session = match state.db.hello"), hub.indexOf("state.release_color(&space_id, &actor.0);", hub.indexOf("Ok(None) => break")));
  if (!hubHello.includes("hello_session.take_welcome()") || !hubHello.includes("hello_session.next_frame().await") || !hubHello.includes("Ok(Some(frame))") || !hubHello.includes("Ok(None) => break") || !hubHello.includes("welcome.frame()") || !hubHello.includes("frame.frame()") || !hubHello.includes("welcome.acknowledge()") || !hubHello.includes("frame.acknowledge()") || hubHello.indexOf("sender.send(welcome_bytes)") < 0 || hubHello.indexOf("welcome.acknowledge()") < hubHello.indexOf("sender.send(welcome_bytes)") || hubHello.indexOf("sender.send(frame_bytes)") < 0 || hubHello.indexOf("frame.acknowledge()") < hubHello.indexOf("sender.send(frame_bytes)") || hub.includes("welcome_response.follow_up")) failures.push("P1z hub caller is not an explicit post-send returned-frame acknowledgement consumer");
  for (const required of ["const DATABASE_SYNC_HELLO_SLOTS: usize = 8", "const DATABASE_SYNC_HELLO_MAX_ITEMS: usize = 65_536", "const DATABASE_SYNC_HELLO_MAX_BYTES: usize = 256 * 1024 * 1024", "const DATABASE_SYNC_HELLO_TURN_MS: u64 = 8", "const DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES: usize = 4 * 1024", "DatabaseSyncHelloBackingLedger", "database_sync_hello_input_credit", "database_sync_hello_envelope_credit", "DatabaseSyncHelloAdmission::try_claim", "DatabaseSyncHelloFollowUp", "DatabaseSyncHelloExecutionFuture", "DatabaseSyncHelloRejectedClose", "database_sync_hello_registry"])
    if (!retained.includes(required)) failures.push("P1z fixed retained authority missing " + required);
  const backingLedger = retained.slice(retained.indexOf("impl DatabaseSyncHelloBackingLedger"), retained.indexOf("fn database_sync_hello_allocate_vec"));
  if (!backingLedger.includes("let next_items = self.items.checked_add(items)") || !backingLedger.includes("let next_bytes = self.bytes.checked_add(bytes)") || !backingLedger.includes("self.items == 0 && self.bytes == 0") || !backingLedger.includes("self.bytes -= self.bytes.min(db_io_core::DB_IO_PAGE_BYTES)") || !retained.includes("owners.session_id.capacity()") || retained.includes("owners.session_id.len()")) failures.push("P1z cumulative observed backing ledger is incomplete or lacks bounded byte/item retirement");
  const snapshotAllocation = retained.slice(retained.indexOf("struct DatabaseSyncHelloSnapshotBackingReservation"), retained.indexOf("fn database_sync_hello_clone_string"));
  const snapshotBacking = protocol.slice(protocol.indexOf("pub const SNAPSHOT_CHUNK_BACKING_BYTES"), protocol.indexOf("pub enum ServerFrame"));
  const snapshotDecode = protocol.slice(protocol.indexOf("fn read_snapshot_chunk_bytes"), protocol.indexOf("pub async fn decode_server_frame"));
  if (!snapshotAllocation.includes('ledger.observe(1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES, "database sync hello fixed snapshot chunk backing")?') || !snapshotAllocation.includes("Ok(DatabaseSyncHelloSnapshotBackingReservation { bytes: DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES })") || snapshotAllocation.indexOf("ledger.observe(1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES") > snapshotAllocation.indexOf("protocol::SnapshotChunkBytes::allocate_fixed()") || !snapshotAllocation.includes("fn allocate(self, ledger: &mut DatabaseSyncHelloBackingLedger)") || !snapshotAllocation.includes("let actual = owner.backing_bytes()") || !snapshotAllocation.includes("if actual != self.bytes") || !snapshotAllocation.includes("owner.close_one()") || !snapshotAllocation.includes("ledger.release(1, self.bytes)?") || !snapshotAllocation.includes("return Err(DbError::LimitExceeded") || !snapshotAllocation.includes("        }\n        Ok(owner)") || snapshotAllocation.includes("reserve_allocation") || snapshotAllocation.includes("try_reserve_exact")) failures.push("P1z snapshot backing is not exact-unit pre-debited, observed, retained on refusal, and held through transfer");
  if (!snapshotBacking.includes("pub const SNAPSHOT_CHUNK_BACKING_BYTES: usize = 4 * 1024") || !snapshotBacking.includes("Option<Box<[u8; SNAPSHOT_CHUNK_BACKING_BYTES]>>") || !snapshotBacking.includes("Box::new([0; SNAPSHOT_CHUNK_BACKING_BYTES])") || !snapshotBacking.includes("filter(|end| *end <= SNAPSHOT_CHUNK_BACKING_BYTES)") || !snapshotBacking.includes("size_of_val(backing.as_ref())") || !snapshotBacking.includes("self.backing.take()") || snapshotBacking.includes("Vec<u8>") || snapshotBacking.includes("try_reserve_exact") || !snapshotDecode.includes("if len > SNAPSHOT_CHUNK_BACKING_BYTES as u64") || !snapshotDecode.includes("SnapshotChunkBytes::try_from_slice(source)")) failures.push("P1z protocol snapshot owner or decoder can exceed the fixed 4 KiB backing");
  const vecAllocation = retained.slice(retained.indexOf("fn database_sync_hello_allocate_vec<T>"), retained.indexOf("struct DatabaseSyncHelloSnapshotBackingReservation"));
  const tailReaderStart = retained.indexOf("async fn database_sync_hello_tail_reader(");
  const tailReader = tailReaderStart < 0 ? "" : retained.slice(tailReaderStart, retained.indexOf("\n}\n", tailReaderStart));
  if (!vecAllocation.includes("let reserved = ledger.reserve_allocation(1, requested, label)?") || vecAllocation.indexOf("ledger.reserve_allocation(1, requested, label)") > vecAllocation.indexOf("owner.try_reserve_exact(count)") || !vecAllocation.includes("ledger.settle_allocation(reserved, actual, label)?") || !retained.includes("database_sync_hello_allocate_vec::<u8>(ledger, source.len(), label)")) failures.push("P1z admitted owners are not debited against the cumulative ledger before they allocate");
  if (tailReaderStart < 0 || !/reserve_allocation|database_sync_hello_allocate_vec/.test(tailReader)) failures.push("P1z WAL envelope backings are not cumulatively debited before every allocation: the tail reader decodes each command (decode_wal_command) into an unaccounted envelope and Vec");
  if (/\bowners\.(?:document(?:\.0)?|session_id|origin(?:\.0)?)\.clone\s*\(/.test(retained) || retained.includes("server_frontier.clone()") || !retained.includes("ArtifactId(std::mem::take(&mut owners.document.0))") || !retained.includes("std::mem::take(&mut owners.session_id)") || !retained.includes("std::mem::take(&mut owners.origin.0)")) failures.push("P1z admitted input or tail identity is cloned additively or instead of moved");
  if (!retained.includes("state.next_generation = generation.checked_add(1)") || !retained.includes("entry.generation == self.generation") || !retained.includes("if self.current()") || !retained.includes("StaleGeneration")) failures.push("P1z generation admission/publication is not checked end to end");
  const schedule = retained.slice(retained.indexOf("fn schedule("), retained.indexOf("fn submit_exact("));
  const submit = retained.slice(retained.indexOf("fn submit_exact("), retained.indexOf("fn retry("));
  const retry = retained.slice(retained.indexOf("fn retry("), retained.indexOf("fn drive_one(self: std::sync::Arc<Self>)"));
  const drive = retained.slice(retained.indexOf("fn drive_one(self: std::sync::Arc<Self>)"), retained.indexOf("fn poll_one("));
  const poll = retained.slice(retained.indexOf("fn poll_one("), retained.indexOf("fn arm_close("));
  if (!schedule.includes("compare_exchange(DatabaseSyncHelloDriverAuthority::Idle as u8, DatabaseSyncHelloDriverAuthority::Queued as u8") || !schedule.includes("self.submit_exact")) failures.push("P1z schedule lacks the unique driver authority claim");
  if (!submit.includes("try_submit(semio_framework_async::Lane::Io, job)") || !submit.includes("Some((error.into_job(), next))") || !submit.includes("callback_at")) failures.push("P1z saturated I/O submission loses its exact job or retry registration");
  if (!retry.includes("DATABASE_SYNC_HELLO_RETRY_LIMIT") || !retry.includes("self.expired.store(true") || !retry.includes("self.cancelled.store(true") || !retry.includes("self.submit_exact(job, attempt)")) failures.push("P1z retry exhaustion/deadline cannot hand off to retained cancellation close");
  const driveClaim = drive.indexOf("DatabaseSyncHelloDriverAuthority::Queued as u8, DatabaseSyncHelloDriverAuthority::Driving as u8");
  const driveWakeClear = drive.indexOf("self.wake_requested.store(false", driveClaim);
  const drivePoll = drive.indexOf("self.poll_one()", driveWakeClear);
  const driveRelease = drive.indexOf("self.driver.store(DatabaseSyncHelloDriverAuthority::Idle as u8", drivePoll);
  const driveWakeRecheck = drive.indexOf("self.wake_requested.load(", driveRelease);
  if (driveClaim < 0 || driveWakeClear < 0 || drivePoll < 0 || driveRelease < 0 || driveWakeRecheck < 0 || drive.indexOf("self.schedule()", driveWakeRecheck) < 0) failures.push("P1z driver releases authority before exact owner and wake publication");
  if (!poll.includes("std::panic::catch_unwind") || !poll.includes("core.future = Some(future)") || !poll.includes("core.execution = Some(execution)") || !poll.includes("core.quarantined = Some(DatabaseSyncHelloQuarantineClose") || !poll.includes("follow_up.drive_one(ledger, &self.cancelled, &self.expired, &mut context)") || poll.indexOf("database_sync_hello_control(&self.cancelled, &self.expired)") > poll.indexOf("follow_up.drive_one(ledger, &self.cancelled, &self.expired, &mut context)")) failures.push("P1z Pending Ready panic or cancel-before-stream owners are not retained by the typed I/O driver");
  const execute = retained.slice(retained.indexOf("async fn database_sync_hello_execute"), retained.indexOf("type DatabaseSyncHelloExecutionFuture"));
  const snapshotReadStart = execute.indexOf("let page_reservation = database_sync_hello_reserve_snapshot_pages");
  const snapshotRead = snapshotReadStart < 0 ? "" : execute.slice(snapshotReadStart, execute.indexOf("let pack_hash", snapshotReadStart));
  if (!retained.includes("const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS: usize = db_io_core::DB_IO_OPERATION_PAGES") || !retained.includes("const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES: usize = DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS * db_io_core::DB_IO_PAGE_BYTES") || !snapshotAllocation.includes("struct DatabaseSyncHelloSnapshotPageReservation") || !snapshotAllocation.includes('ledger.observe(DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES, "database sync hello fixed snapshot page backing")?') || !snapshotAllocation.includes("let bytes = items.checked_mul(db_io_core::DB_IO_PAGE_BYTES)") || !snapshotAllocation.includes("items > self.items || bytes > self.bytes || pages.len() > bytes") || !snapshotAllocation.includes("ledger.release(self.items - items, self.bytes - bytes)") || !snapshotAllocation.includes("ledger.release(self.items, self.bytes)") || !snapshotRead.includes("database_sync_hello_reserve_snapshot_pages(&mut ledger)?") || snapshotRead.indexOf("database_sync_hello_reserve_snapshot_pages(&mut ledger)?") > snapshotRead.indexOf("snapshots.read_generation") || snapshotRead.includes("reserve_allocation") || !snapshotRead.includes("page_reservation.observed(&pages)") || !snapshotRead.includes("page_reservation.settle(&mut ledger, page_items, page_bytes)?") || (snapshotRead.match(/page_reservation\.release\(&mut ledger\)\?/g)?.length ?? 0) < 3 || (snapshotRead.match(/ledger\.release\(page_items, page_bytes\)\?/g)?.length ?? 0) < 4 || (snapshotRead.match(/database_sync_hello_close_pages\(&mut pages, &cancelled, &expired\)\.await\?/g)?.length ?? 0) < 6) failures.push("P1z snapshot read-generation pages lack exact fixed predebit, observed backing settlement, or close/error credit return");
  const opportunity = retained.slice(retained.indexOf("async fn database_sync_hello_opportunity"), retained.indexOf("fn database_sync_hello_control"));
  const control = retained.slice(retained.indexOf("fn database_sync_hello_control"), retained.indexOf("fn database_sync_hello_allocate_envelope_vec"));
  if ((opportunity.match(/database_sync_hello_control\(cancelled, expired\)/g)?.length ?? 0) !== 2 || opportunity.indexOf("semio_framework_async::yield_once().await") < opportunity.indexOf("database_sync_hello_control(cancelled, expired)") || opportunity.lastIndexOf("database_sync_hello_control(cancelled, expired)") < opportunity.indexOf("semio_framework_async::yield_once().await") || !execute.includes("database_sync_hello_opportunity(&cancelled, &expired).await?") || execute.indexOf("database_sync_hello_opportunity(&cancelled, &expired).await?") > execute.indexOf("replay_sync_state_retained") || !retained.includes("records.replenish") || !retained.includes("decode_control.renew_step()?") || !retained.includes("WalCursorControl::stall_bounded(cancelled.clone(), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)")) failures.push("P1z cancellation or deadline is not rechecked after yield and before the next backend operation");
  if (!control.includes("if expired.load(std::sync::atomic::Ordering::Acquire)") || !control.includes("if cancelled.load(std::sync::atomic::Ordering::Acquire)") || control.indexOf("if expired.load") > control.indexOf("if cancelled.load") || !control.includes('DbError::Timeout("database sync hello deadline".to_string())')) failures.push("P1z independently expired deadline does not control every operation");
  const grant = retained.slice(retained.indexOf("struct DatabaseSyncHelloGrant"), retained.indexOf("impl DatabaseSyncHelloFollowUp"));
  const stream = retained.slice(retained.indexOf("impl DatabaseSyncHelloFollowUp"), retained.indexOf("struct DatabaseSyncHelloPrepared"));
  const streamDrive = stream.slice(stream.indexOf("fn drive_one"), stream.indexOf("fn close_one"));
  const tailDriveStart = retained.indexOf("fn drive(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger");
  const tailDrive = tailDriveStart < 0 ? "" : retained.slice(tailDriveStart, retained.indexOf("fn close_one", tailDriveStart));
  if (!grant.includes("std::time::Instant::now().checked_add(std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS))") || !grant.includes("database_sync_hello_control(cancelled, expired)?") || !grant.includes("std::time::Instant::now() >= self.deadline") || !streamDrive.includes("let mut grant = DatabaseSyncHelloGrant::fresh()?") || !streamDrive.includes("self.drive_one_with_grant(ledger, cancelled, expired, &mut grant, context)") || !streamDrive.includes("let unit_bytes = (*chunk_bytes).min(DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES)") || !streamDrive.includes("let len = unit_bytes.min(pages.len() - *offset)") || !streamDrive.includes("database_sync_hello_reserve_snapshot_chunk(ledger, len)?") || !streamDrive.includes("let owner = reservation.allocate(ledger)?") || !streamDrive.includes("let remaining = unit_bytes.saturating_sub(target.len())") || (streamDrive.match(/grant\.check\(cancelled, expired\)\?/g)?.length ?? 0) < 6 || !streamDrive.includes("pages.page(*page)") || !streamDrive.includes("target.try_extend_from_slice") || !streamDrive.includes("let next_seq = seq.checked_add(1)") || streamDrive.indexOf("let next_seq = seq.checked_add(1)") > streamDrive.indexOf("let bytes = chunk.take()") || !streamDrive.includes("Ok(DatabaseSyncHelloFollowUpStep::Progress)") || streamDrive.includes("while ") || streamDrive.includes("loop {") || !streamDrive.includes("SnapshotDone") || !streamDrive.includes("Self::Tail(tail) => tail.drive(ledger, context)") || !tailDrive.includes("self.take_page()") || !tailDrive.includes('ledger.observe(items, bytes, "database sync hello tail frame backing")?') || tailDrive.indexOf('ledger.observe(items, bytes, "database sync hello tail frame backing")?') > tailDrive.indexOf("Ok(DatabaseSyncHelloFollowUpStep::Frame(frame))") || tailDrive.includes("while ") || tailDrive.includes("loop {")) failures.push("P1z snapshot or tail output lacks a fresh real 8 ms grant and fixed one-page allocation/copy/publication unit");
  if (!streamDrive.includes("if chunk.is_none() {\n                    grant.check(cancelled, expired)?") || !streamDrive.includes("grant.check(cancelled, expired)?;\n                let fragment = pages.page(*page)") || !streamDrive.includes("grant.check(cancelled, expired)?;\n                if !target.try_extend_from_slice") || !streamDrive.includes("if target.len() == unit_bytes || *offset == pages.len() {\n                    grant.check(cancelled, expired)?")) failures.push("P1z snapshot allocation, copy, or publication is not immediately deadline/cancel gated");
  if (retained.includes("let _ = database_sync_hello_control") || retained.includes("database_sync_hello_control(cancelled, expired).ok()")) failures.push("P1z cleanup or stream path discards cancellation/deadline control");
  const nextPoll = retained.slice(retained.indexOf("impl std::future::Future for DatabaseSyncHelloNextFuture"), retained.indexOf("impl Drop for DatabaseSyncHelloNextFuture"));
  const returnedClose = retained.slice(retained.indexOf("fn close_returned_frame_one"), retained.indexOf("fn poll_one"));
  const returnedLease = retained.slice(retained.indexOf("struct DatabaseSyncHelloReturnedFrameLease"), retained.indexOf("pub struct DatabaseSyncHelloSession"));
  if (!retained.includes("returned_generation: std::sync::atomic::AtomicU64") || !retained.includes("returned_fallback: Option<DatabaseSyncHelloReturnedFrameLease>") || !returnedLease.includes("state.returned_generation.try_update") || !returnedLease.includes("generation.checked_add(1).filter(|next| *next != 0)") || !returnedLease.includes("pub struct DatabaseSyncHelloReturnedFrame") || !returnedLease.includes("pub fn acknowledge") || !returnedLease.includes("core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation, items, bytes, close: None })") || (returnedLease.match(/core\.returned_fallback = Some\(DatabaseSyncHelloReturnedFrameLease/g)?.length ?? 0) < 2 || !returnedLease.includes("match self.mount_close()") || returnedLease.includes("let _ = self.mount_close()") || !returnedLease.includes("lease.close = Some(DatabaseSyncHelloFrameClose") || !returnedClose.includes("core.returned_fallback.as_mut()") || !returnedClose.includes("close.close_one()") || !returnedClose.includes("close.terminal_is_empty()") || !returnedClose.includes("ledger.release(items, bytes)") || !returnedClose.includes("owner.generation == generation") || retained.includes("outstanding_bytes") || (nextPoll.match(/core\.returned_frame\.is_none\(\)/g)?.length ?? 0) < 2) failures.push("P1z returned output credit can escape its generation-qualified acknowledgement close");
  const close = retained.slice(retained.indexOf("fn close_one_claimed"), retained.indexOf("fn deadline_callback"));
  const deadlineCallback = retained.slice(retained.indexOf("fn deadline_callback"), retained.indexOf("fn progress", retained.indexOf("fn deadline_callback")));
  if (!deadlineCallback.includes("self.expired.store(true") || !deadlineCallback.includes("self.cancelled.store(true") || !deadlineCallback.includes("self.schedule()")) failures.push("P1z deadline callback does not publish both controlling state and real-loop wake");
  const followUpClose = stream.slice(stream.indexOf("fn close_one"), stream.indexOf("fn terminal_is_empty"));
  if (!retained.includes("struct DatabaseSyncHelloQuarantineClose") || !retained.includes("struct DatabaseSyncHelloFollowUpCloseFault") || !retained.includes("fn database_sync_hello_apply_follow_up_close_result") || !retained.includes("*fault = Some(retained)") || !retained.includes("self.future.is_none() && self.items == 0 && self.bytes == 0") || !followUpClose.includes("Ok(pages.close_step()?.is_some())") || followUpClose.includes(".ok().flatten()") || !close.includes("quarantine.close_one()") || !close.includes("quarantine.terminal_is_empty()") || !close.includes("ledger.close_one_credit()") || !close.includes("ledger.terminal_is_empty()") || retained.includes("core.quarantined = None") || !close.includes("let result = prepared.follow_up.close_one()") || !close.includes("database_sync_hello_apply_follow_up_close_result(&mut execution.close_fault") || !close.includes("prepared.follow_up.terminal_is_empty()") || !close.includes("execution.close_fault.is_some()") || !close.includes("owners.close_one()") || !close.includes("owners.terminal_is_empty()") || !close.includes("registry[self.slot] = None") || close.indexOf("registry[self.slot] = None") < close.indexOf("core.execution = None")) failures.push("P1z terminal close can suppress page-close faults or release before follow-up, quarantine, owner, item, and byte ledgers are empty");
  const publicPoll = retained.slice(retained.indexOf("impl std::future::Future for DatabaseSyncHelloFuture"), retained.indexOf("impl Drop for DatabaseSyncHelloFuture"));
  if ((publicPoll.match(/execution\.is_some\(\)/g)?.length ?? 0) !== 2 || !publicPoll.includes("context.waker().clone()") || !publicPoll.includes("state.schedule()") || !publicPoll.includes("if self.completed")) failures.push("P1z public future lacks check-register-recheck or typed repeat poll");
  if ((nextPoll.match(/frame\.take\(\)/g)?.length ?? 0) !== 2 || !nextPoll.includes("*state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());\n        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);") || !nextPoll.includes("state.demand.store(true") || !nextPoll.includes("state.schedule()") || !nextPoll.includes("if self.completed")) failures.push("P1z frame stream lacks lost-wake closure or deterministic repeat polling");
  const rejected = retained.slice(retained.indexOf("struct DatabaseSyncHelloRejectedClose"), retained.indexOf("//#endregion 👋️RetainedHello"));
  const terminalRetry = rejected.slice(rejected.indexOf("fn retry_terminal_once"), rejected.indexOf("fn terminal_witness"));
  const registryRelease = rejected.slice(rejected.indexOf("fn database_sync_hello_release_rejected_registry"), rejected.indexOf("pub struct DatabaseSyncHelloRejectedTerminalWitness"));
  if (!rejected.includes("try_submit(semio_framework_async::Lane::Io") || !rejected.includes("retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>") || (rejected.match(/self\.retain_retry\(error\.into_job\(\)/g)?.length ?? 0) !== 2 || !rejected.includes("attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT.saturating_sub(1)") || !rejected.includes("self.pool.now_ms() >= self.deadline_ms") || !rejected.includes("self.cancelled.load") || !rejected.includes("fn claim_submission") || !rejected.includes("*next <= DATABASE_SYNC_HELLO_RETRY_LIMIT") || !rejected.includes("database_sync_hello_install_rejected_registry(&close)") || !rejected.includes("pub fn database_sync_hello_rejected_terminal_witness") || !rejected.includes("registry_generation") || !rejected.includes("owners_retained") || (rejected.match(/callback_at/g)?.length ?? 0) !== 3 || terminalRetry.includes("callback_at") || !registryRelease.includes("registry_released.load") || !registryRelease.includes("database_sync_hello_release_rejected_registry(&next)") || !rejected.includes("owner.close_one()") || !rejected.includes("owner.terminal_is_empty()") || !rejected.includes("database_sync_hello_release_rejected_registry(&self)")) failures.push("P1z refusal close lacks bounded no-ninth-submit recovery with generation-qualified discoverable exact job/input ownership");
  if (!wal.includes("pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize)") || !wal.includes("self.control.replenish(deadline, fuel)")) failures.push("P1z WAL cursor cannot preserve identity across bounded opportunities");
  if (!syncSource.includes("#[cfg(test)]\npub async fn handle_hello") || !syncSource.includes("#[cfg(test)]\npub async fn build_welcome") || !syncSource.includes("#[cfg(test)]\nasync fn lower_bootstrap_plan")) failures.push("P1z eager batch hello path remains production reachable");
  const laws: readonly [string, readonly string[]][] = [
    ["retained_sync_hello_handoff_first_poll_cancel_preserves_exact_owner_and_io_lane", ["held_sync_hello_pool", "DatabaseSyncHelloFuture::try_submit", "future.cancel()", "DatabaseSyncHelloDriverAuthority::Queued", "Arc::as_ptr", "document_identity", "Err(DbError::Closed)"]],
    ["retained_sync_hello_max_plus_one_refusal_keeps_storage_document_frontier_session_origin_identity", ["DATABASE_SYNC_HELLO_SLOTS", "DatabaseSyncHelloFuture::try_submit", "unwrap_err", "document.0.as_ptr()", "retained-session", "retained-origin"]],
    ["retained_sync_hello_tail_stream_publishes_welcome_then_one_backpressured_frame", ["DatabaseSyncHelloFuture::try_submit", "take_welcome", "next_frame().await", "ServerFrame::Commands", "envelopes.len() == 3", "is_none()"]],
    ["retained_sync_hello_snapshot_cursor_copies_at_most_one_page_fragment_per_driver_opportunity", ["DatabaseSyncHelloFuture::try_submit", "SnapshotStorage::write_generation", "p1z-snapshot", "ServerFrame::SnapshotChunk", "actual == seq", "SnapshotDone", "next_frame().await"]],
    ["retained_sync_hello_returned_snapshot_credit_waits_for_exact_generation_ack", ["DatabaseSyncHelloFuture::try_submit", "returned_frame", "lease.generation", "ledger_before", "Poll::Pending", "first.acknowledge()", "next.await", "SnapshotChunk { seq: 1"]],
    ["retained_sync_hello_maximum_snapshot_request_stays_page_unit_bounded", ["database_sync_hello_reserve_snapshot_pages", "page_ledger.items", "page_ledger.bytes", "DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS", "DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES", "page_reservation.observed", "db_io_core::DB_IO_PAGE_BYTES", "page_reservation.settle", "page_owner.close_step", "page_owner.terminal_is_empty()", "page_ledger.terminal_is_empty()", "database_sync_hello_reserve_snapshot_chunk", "fixed_ledger.items", "fixed_ledger.bytes", "fixed_reservation.bytes", "fixed_reservation.allocate", "fixed.backing_bytes()", "DatabaseSyncHelloReturnedFrameLease", "fixed_lease.items", "fixed_lease.bytes", "fixed_lease.close", "fixed_ledger.release", "fixed_ledger.terminal_is_empty()", "DATABASE_SYNC_HELLO_MAX_BYTES", "DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES * 2 + 1", "DatabaseSyncHelloFuture::try_submit", "bytes.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES", "frame.acknowledge()", "SnapshotDone { seq_count: 3 }"]],
    ["retained_sync_hello_grant_deadline_between_allocation_copy_and_publication_retains_credit", ["DatabaseSyncHelloGrant::expiring_at(3)", "drive_one_with_grant", "chunk: Some(chunk), offset: 0", "DatabaseSyncHelloGrant::expiring_at(5)", "chunk.len() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES", "ledger.items", "close_one", "ledger.close_one_credit", "independently_expired", "DbError::Timeout"]],
    ["retained_sync_hello_cancel_before_stream_demand_publishes_no_new_frame", ["DatabaseSyncHelloFuture::try_submit", "take_welcome", "session.cancel()", "session.next_frame().await", "Err(DbError::Closed)", "state.demand", "core.frame.as_ref()"]],
    ["retained_sync_hello_cumulative_actual_backing_rejects_max_plus_one_without_mutation", ["DatabaseSyncHelloBackingLedger", "DATABASE_SYNC_HELLO_MAX_ITEMS", "DATABASE_SYNC_HELLO_MAX_BYTES", "cumulative max plus one", "assert_eq!"]],
    ["retained_sync_hello_predebits_envelope_clone_and_overallocation_before_owner_construction", ["database_sync_hello_allocate_vec::<u8>", "database_sync_hello_clone_string", "source.as_ptr()", "owner.as_ptr()", "DATABASE_SYNC_HELLO_MAX_BYTES", "assert_eq!"]],
    ["retained_sync_hello_cancel_between_yield_and_resume_prevents_next_wal_backend_operation", ["database_sync_hello_opportunity", "Poll::Pending", "cancelled.store(true", "Poll::Ready(Err(DbError::Closed))", "database_sync_hello_control"]],
    ["retained_sync_hello_quarantine_cursor_and_byte_item_ledger_reach_zero_before_release", ["DatabaseSyncHelloExecutionFuture", "DatabaseSyncHelloQuarantineClose", "quarantine.close_one()", "quarantine.terminal_is_empty()", "ledger.close_one_credit()", "ledger.terminal_is_empty()"]],
    ["retained_sync_hello_page_close_error_is_typed_and_blocks_terminal_release", ["database_sync_hello_apply_follow_up_close_result", "Err(DbError::Internal", "p1z retained page-close fault", "retained.attempts", "retained.error", "terminal_is_empty", "close_one"]],
    ["retained_sync_hello_refusal_retry_saturation_is_bounded_and_retains_terminal_job_owner", ["replenishing_held_sync_hello_pool", "DatabaseSyncHelloRejected::new", "retirement_generation", "close_and_take_error", "database_sync_hello_rejected_terminal_witness", "owners_retained", "job_retained", "witness.submissions", "DATABASE_SYNC_HELLO_RETRY_LIMIT"]],
    ["retained_sync_hello_forever_stuck_sole_worker_guarantees_discoverable_ownership_only", ["held_sync_hello_pool", "Lane::Io", "DatabaseSyncHelloRejected::new", "p1z forever-stuck sole worker", "retirement_generation", "database_sync_hello_rejected_terminal_witness", "owners_retained", "job_retained", "witness.submissions"]],
    ["retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service", ["held_sync_hello_pool", "deadline_callback", "drop(future)", "expired.load", "cancelled.load", "database_sync_hello_control", "DbError::Timeout", "admission", "database_sync_hello_registry"]],
    ["retained_sync_hello_ready_pending_panic_and_repeat_poll_have_typed_terminal_states", ["Poll::Pending", "Poll::Ready(execution)", "core.quarantined = Some(DatabaseSyncHelloQuarantineClose", "if self.completed", "DbError::Closed"]],
    ["retained_sync_hello_production_census_has_zero_blocking_waits_and_no_eager_follow_up", ["db_actor::block_on(db_sync::handle_hello", "DatabaseSyncHelloFuture::try_submit", "#[cfg(test)]\\npub async fn handle_hello", "DatabaseSyncHelloNextFuture"]],
  ];
  for (const [law, evidence] of laws) {
    const body = interactivityDatabaseSyncHelloFixtureBody(syncSource, law);
    if (!body || evidence.some((token) => !body.includes(token))) failures.push("P1z hostile law " + law + " is missing production-path evidence");
  }
  for (const required of ["Selected Production Wait", "Database::hello", "Retained Job Contract", "One shared WorkerPool", "Lane::Io", "8 ms", "fresh monotonic", "fixed 4 KiB frame unit", "Only that fixed unit is pre-debited", "independently of cancellation", "Pre-admission", "Snapshot output", "Generation", "Cancellation", "Caller Migration", "generation-qualified lease", "explicitly acknowledges", "permanently non-returning sole OS worker", "discoverable retained ownership and no owner loss only", "no cancellation-latency or completion claim", "real `worker_loop`", "No timer thread, second pool, or caller-driven completion"])
    if (!contractSource.includes(required)) failures.push("P1z governing census lost " + required);
  if (!contractSource.includes("The selected P1z production wait is `Database::hello`")) failures.push("P1z governing census lost the exact selected facade cut");
  return failures;
}


//#endregion 👋️P1zDatabaseSyncHello

export function interactivityArtifactHistoryFailures(engineSource: string, artifactSource: string, walSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const artifact = interactivityProductionSource(artifactSource);
  const wal = interactivityProductionSource(walSource);
  const history = engine.slice(engine.indexOf("const ARTIFACT_HISTORY_OPERATION_SLOTS"), engine.indexOf("pub struct ArtifactHandle"));
  const replay = interactivityRegionSource(artifact, "🔖️HistoryReplay") ?? "";
  const replayNext = replay.slice(replay.indexOf("fn next("), replay.indexOf("impl Future for HistoryReplayFuture"));
  const reservationBuild = replay.slice(replay.indexOf("impl HistoryReplayReservation"), replay.indexOf("struct HistoryReplayReservationCloseCursor"));
  const actor = interactivityRegionSource(artifact, "🔖️Actor") ?? "";
  const terminalHandle = history.slice(history.indexOf("impl ArtifactHistoryTerminalHandle"), history.indexOf("impl Future for HistoryFuture"));
  const terminalCloseStart = terminalHandle.indexOf("pub fn close_step(&self)");
  const terminalClose = terminalHandle.slice(terminalCloseStart, terminalHandle.indexOf("pub fn terminal_is_empty(&self)", terminalCloseStart));
  const productionWaits = engine.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const failures: string[] = [];
  if (productionWaits > 2 || engine.includes("async fn replay_history") || engine.includes("db_wal::replay_document(&storage.wal")) failures.push("DB engine retained history route exceeds the two post-P1x residual waits or revives whole history replay");
  for (const forbidden of ["block_on(", "ask_blocking", "submit_blocking", "thread::spawn", "WorkerPool::new(", "loop {", "while "])
    if (history.includes(forbidden) || replay.includes(forbidden)) failures.push(`retained history route contains ${forbidden}`);
  if (!history.includes("const ARTIFACT_HISTORY_OPERATION_SLOTS: usize = 8") || !history.includes("const ARTIFACT_HISTORY_PAGE_BYTES: u64 = 16 * 1024") || !history.includes("const ARTIFACT_HISTORY_OPERATION_PAGES: u64 = 2_048") || !history.includes("const ARTIFACT_HISTORY_OPERATION_ITEMS: usize = 20_481") || !history.includes("ArtifactHistoryAdmission::try_claim()")) failures.push("history operation lacks fixed item/page/operation/process admission");
  if (!replay.includes("pub struct HistoryReplayReservation") || !replay.includes(".try_reserve_exact(HISTORY_REPLAY_SEGMENT_PAGES") || !replay.includes(".try_reserve_exact(HISTORY_REPLAY_RESULT_PAGES)") || !replay.includes(".try_reserve_exact(HISTORY_REPLAY_MAX_OPERATION_IDS)") || !replay.includes(".try_reserve_exact(HISTORY_REPLAY_MAX_ENTRIES)") || !replay.includes("result_len.checked_add(len)") || !replay.includes("retained_bytes") || !history.includes("HistoryView::new(view, Some(admission)") || !history.includes("reservation: Option<db_artifact_history::HistoryReplayReservation>")) failures.push("history source and derived byte/item owners are not jointly credited before publication");
  if (!replay.includes("HistoryEnvelopeField::Dependency") || !replay.includes("HistoryEnvelopeField::DiffPayload") || !replay.includes("HistoryEnvelopeField::InversePayload") || !replay.includes("self.dependencies -= 1") || !replay.includes("self.pos != self.end")) failures.push("history envelope cursor is not retained one field/item with exact range validation");
  if (!replay.includes("PageRead") || !replay.includes("requested =") || !replay.includes("requested.min(HISTORY_REPLAY_PAGE_BYTES)") && !replay.includes(".min(HISTORY_REPLAY_PAGE_BYTES)") || !replay.includes("pack::ByteRange { offset, len: requested }") || !replay.includes("page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES") || replay.includes("pack::ByteRange { offset: 0, len }") || replay.includes("read(document, index, range_all)")) failures.push("history backend read is not retained to one admitted 16KiB page");
  if (!replay.includes("protocol::codec::Crc32cCursor") || !replay.includes("self.crc.update_page(page)") || !replay.includes("let maximum = self.payload_remaining.min(HISTORY_REPLAY_PAGE_BYTES)") || replay.includes("protocol::codec::crc32c(whole_frame)") || replay.includes("decode_history_token")) failures.push("history frame CRC/tokenization is not retained to one admitted page");
  if (!replay.includes("Command { offset: u64, len: u64 }") || replay.includes("Command(Vec<u8>)") || wal.includes("pub async fn decode_history_token")) failures.push("history command token duplicates raw pages or revives whole-frame WAL decoding");
  if (!replay.includes("HistoryReplayPhase::Retire") || !replay.includes("HistoryReplayTransition::FaultRetire") || !replay.includes("pub struct HistoryReplayReservationCloseCursor") || !replay.includes("self.page_count -= 1") || !replay.includes("self.source_page_count -= 1") || !replay.includes("self.operation_ids.as_mut().is_some_and") || !replay.includes("self.entries.as_mut().is_some_and") || !replay.includes("self.result_pages.as_mut().and_then(Vec::pop)") || !replay.includes("assert!(self.terminal_is_empty()")) failures.push("history normal/cancel close does not retire one exact page or nested owner per actor grant");
  if (!history.includes("terminal_reservation: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationCloseCursor>>") || !history.includes("pub struct ArtifactHistoryTerminalReservation") || !history.includes("begin_unhanded_reservation_close") || !history.includes("terminalize_unhanded_request") || !history.includes("pub fn take_terminal_reservation") || !history.includes("assert!(self.reservation.is_none()")) failures.push("history pre-handoff reservation lacks public retained retirement and exact admission handback");
  const constructionBuilder = replay.slice(replay.indexOf("struct HistoryReplayReservationConstructionBuilder"), replay.indexOf("pub struct HistoryReplayReservationConstructionFault"));
  const constructionFault = replay.slice(replay.indexOf("pub struct HistoryReplayReservationConstructionFault"), replay.indexOf("impl Drop for HistoryReplayReservationCloseCursor"));
  const copyableConstructionToken = /#\[derive\([^\)]*(?:Clone|Copy)[^\)]*\)\]\s*struct HistoryReplayReservationConstructionToken/.test(replay);
  const constructionClaimOwner = replay.slice(replay.indexOf("fn claim_history_replay_reservation_construction"), replay.indexOf("fn handback_history_replay_reservation_construction"));
  const constructionHandback = replay.slice(replay.indexOf("fn handback_history_replay_reservation_construction"), replay.indexOf("fn release_history_replay_reservation_construction"));
  const constructionClaim = reservationBuild.indexOf("HistoryReplayReservationConstructionBuilder::new()");
  const firstConstructionAllocation = reservationBuild.indexOf(".try_reserve_exact(HISTORY_REPLAY_SEGMENT_PAGES");
  if (copyableConstructionToken || !constructionClaimOwner.includes("cursor: Some(HistoryReplayReservationCloseCursor"))
    failures.push("history construction token is copyable or partial owners are not registry-resident before allocation");
  if (
    !constructionHandback.includes("registry.slots.get_mut(token.slot)") ||
    !constructionHandback.includes("!slot.occupied") ||
    !constructionHandback.includes("!slot.checked_out") ||
    !constructionHandback.includes("slot.generation != token.generation") ||
    !constructionHandback.includes("slot.cursor.is_none()") ||
    !constructionHandback.includes("HistoryReplayReservationConstructionHandbackRejection") ||
    constructionHandback.includes("slot.generation = token.generation") ||
    constructionHandback.includes("slot.error =") ||
    constructionHandback.includes("slot.cursor =")
  )
    failures.push("history construction handback is not a checked linear ABA-safe compare that leaves registry owners resident");
  if (
    !history.includes("terminal_construction: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationConstructionFault>>") ||
    !history.includes("pub struct ArtifactHistoryTerminalConstructionFault") ||
    !history.includes("pub fn take_terminal_construction_fault") ||
    !history.includes("impl ArtifactHistoryTerminalConstructionFault") ||
    !history.includes("pub fn resume(mut self) -> Result<(), Self>") ||
    !history.includes("construction_checked_out") ||
    !replay.includes("const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64") ||
    !replay.includes("pub fn try_new()") ||
    replay.includes("pub fn try_new() -> Result<Self, HistoryReplayReservationConstructionFault>") ||
    !replay.includes("pub struct HistoryReplayReservationConstructionFault") ||
    replay.includes("pub struct HistoryReplayReservationConstructionFaultCursor") ||
    replay.includes("pub fn into_parts") ||
    reservationBuild.includes("?") ||
    !reservationBuild.includes("scratch.try_reserve_exact(HISTORY_REPLAY_MAX_FIELD_BYTES)") ||
    constructionClaim < 0 ||
    firstConstructionAllocation < constructionClaim ||
    !constructionBuilder.includes("impl Drop for HistoryReplayReservationConstructionBuilder") ||
    !constructionBuilder.includes("handback_history_replay_reservation_construction(&token)") ||
    !constructionFault.includes("impl Drop for HistoryReplayReservationConstructionFault") ||
    !constructionFault.includes("handback_history_replay_reservation_construction(&token)") ||
    !constructionFault.includes("release_history_replay_reservation_construction(token)") ||
    !replay.includes("take_history_replay_reservation_construction_fault") ||
    constructionFault.includes("assert!(self.terminal_is_empty()")
  )
    failures.push("history reservation construction fault can escape or bulk-drop partial fixed owners before registered public retirement");
  const closeOwner = terminalClose.indexOf("if self.state.close_one()");
  const closeAuthority = terminalClose.indexOf("if self.state.authority.close_step()");
  const closeAdmission = terminalClose.indexOf("self.state.finish_if_terminal_empty()");
  if (!history.includes("fn terminal_roots_are_empty(&self) -> bool") || !history.includes("self.finished.load(std::sync::atomic::Ordering::Acquire)") || !history.includes("self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()") || terminalCloseStart < 0 || closeOwner < 0 || closeAuthority < closeOwner || closeAdmission < closeAuthority || !terminalClose.includes("return true")) failures.push("history public terminal witness can release or report empty before admission and registry ownership are terminal");
  if (!replay.includes("enum HistoryReplayTransition") || !replay.includes("phase: Option<HistoryReplayPhase>") || !replay.includes("self.transition = HistoryReplayTransition::FaultRetire") || !replay.includes("if this.phase.take().is_some()") || replay.includes("std::mem::replace(&mut self.phase, HistoryReplayPhase::Complete")) failures.push("history phase transition can publish Complete before panic-safe retained retirement");
  for (const scan of [".rposition(", "result_pages.iter()", "source_pages.iter().all", "pages.iter().all", "pages.iter().filter"])
    if (replay.includes(scan)) failures.push(`history fixed owner accounting contains full-capacity scan ${scan}`);
  if (!replay.includes("source_page_count") || !replay.includes("retained_operation_bytes") || !replay.includes("retained_result_bytes")) failures.push("history fixed owner accounting lacks retained O(1) counters");
  if (!actor.includes("ArtifactMessage::History") || !actor.includes("ArtifactTurn::History") || !actor.includes("engine.history_replay(operation_generation, cancelled, reservation)") || !actor.includes("Pin::new(&mut *replay).poll(&mut context)") || !actor.includes("replay.request_close") || !actor.includes("!replay.terminal_is_empty()")) failures.push("history replay is not polled through the retained artifact actor turn");
  const panicMessage = actor.indexOf("history replay cursor panicked");
  const panicRequest = actor.lastIndexOf("replay.request_close", panicMessage);
  const panicSchedule = actor.indexOf("self.schedule()", panicMessage);
  if (panicMessage < 0 || panicRequest < 0 || panicSchedule < panicMessage) failures.push("history runner panic does not retain and reschedule fault cleanup");
  if (!history.includes("if generation != self.generation") || !history.includes("if self.authority.generation() != self.authority_generation") || history.indexOf("if self.authority.generation() != self.authority_generation") > history.indexOf("self.authority.history_retained")) failures.push("history generation freshness follows mailbox mutation");
  if (!history.includes("impl std::task::Wake for ArtifactHistoryWake") || !history.includes("self.generation == state.generation") || !history.includes("self.scheduled.compare_exchange(false, true") || !history.includes("self.pool.callback_at") || !history.includes("retry_generation") || !history.includes("error.into_job()")) failures.push("history late readiness or quiet saturation lacks one-shot wake and exact timer retry");
  if (!history.includes("ArtifactHistoryWorkOwner::Request") || !history.includes("ArtifactHistoryWorkOwner::Actor") || history.includes("ArtifactHistoryWorkOwner::Map") || !history.includes("std::pin::Pin::new(future).poll(&mut context)") || !history.includes("self.complete(Ok(HistoryView::new")) failures.push("history grant is not one request, actor poll, or result item opportunity");
  if (replayNext.includes("?") || replayNext.includes("try_reserve") || replayNext.includes("Vec::with_capacity") || replayNext.includes("String::from_utf8") || replayNext.includes("std::mem::take(&mut this.entries)") || history.includes("drop(self.owner.take") || history.includes("drop(result)")) failures.push("history live cursor can bypass retained fault/result retirement or allocate after admission");
  for (const terminal of ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "impl ArtifactHistoryTerminalJob", "impl ArtifactHistoryTerminalWork"])
    if (!history.includes(terminal)) failures.push(`history terminal authority missing ${terminal}`);
  for (const fixture of [
    "artifact_history_empty_one_cap_plus_one_admission_returns_exact_request",
    "artifact_history_empty_and_two_batch_replay_are_deterministic",
    "artifact_history_nested_derived_item_and_byte_caps_precede_materialization",
    "artifact_history_segment_cap_plus_one_reads_only_one_admitted_page",
    "artifact_history_crc_and_frame_tokenizer_advance_one_page_per_grant",
    "artifact_history_cancel_retires_one_page_or_nested_owner_per_actor_grant",
    "artifact_history_quiet_late_wake_and_retry_are_generation_coalesced",
    "artifact_history_cancel_before_during_after_retains_actor_and_result_owners",
    "artifact_history_stale_generation_and_slot_aba_precede_mailbox_mutation",
    "artifact_history_replay_ordering_is_segment_frame_then_result_fifo",
    "artifact_history_terminal_job_work_result_take_resume_and_close_one_owner",
    "artifact_history_one_grant_advances_one_retained_phase_without_blocking",
    "artifact_history_drop_after_complete_moves_result_to_public_terminal_registry",
    "artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each",
    "artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner",
    "artifact_history_runner_close_mid_turn_retains_replay_until_terminal_empty",
    "artifact_history_future_handle_drop_and_terminal_take_resume_are_exact",
    "artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release",
    "artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty",
    "artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant",
    "artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner",
    "artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages",
    "artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop",
    "artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation",
    "artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite",
    "artifact_history_panic_at_each_phase_transition_retains_then_fault_retires",
    "artifact_history_fixed_owner_accounting_has_no_capacity_scan",
  ]) if (!engineSource.includes(fixture) && !artifactSource.includes(fixture) && !walSource.includes(fixture)) failures.push(`retained history fixture missing: ${fixture}`);
  return failures;
}



/** ⌨️ The DB CLI's process-entry waits: at most 18 production `db::actor::block_on` edges and exactly one more, in a test-only
 * `seed_document`, never in production code. */
export function interactivityDbCliFailures(cliSource: string): string[] {
  const productionCli = interactivityProductionSource(cliSource);
  const productionCliWaits = productionCli.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const wholeCliWaits = cliSource.match(/\bdb_actor::block_on\s*\(/g)?.length ?? 0;
  const seedDocument = cliSource.slice(cliSource.indexOf("fn seed_document"));
  if (productionCliWaits > 18 || wholeCliWaits !== productionCliWaits + 1 || productionCli.includes("fn seed_document") || !seedDocument.includes("db_actor::block_on(")) return ["DB CLI census exceeds 18 process-entry waits or does not isolate exactly one test-only seed_document wait"];
  return [];
}



export function interactivityMcpHttpTransportFailures(transportSource: string, bridgeSource: string, rootSource: string): string[] {
  const transport = interactivityProductionSource(transportSource);
  const bridge = interactivityProductionSource(bridgeSource);
  const failures: string[] = [];
  for (const forbidden of ["tokio::runtime::Runtime", "tokio::runtime::Builder", "tokio::spawn", "tokio::sync", "axum::serve", "block_on("]) {
    if (transport.includes(forbidden) || bridge.includes(forbidden)) failures.push(`live MCP transport retains ${forbidden}`);
  }
  if (!transport.includes("process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative") || !transport.includes("self.pool.try_submit(Lane::Io, job)") || !transport.includes("state.drive_one(self.pool.now_ms())")) failures.push("MCP HTTP is not one retained process-pool I/O-lane turn");
  if (!rootSource.includes("transport.start(server)?.wait()") || rootSource.includes("Runtime::new()") || rootSource.includes("block_on(")) failures.push("MCP process entry does not own the retained HTTP completion boundary");
  if (!transport.includes("listener.set_nonblocking(true)") || !transport.includes("stream.set_nonblocking(true)") || !transport.includes("fn accept_one(") || !transport.includes("fn read_one_page(") || !transport.includes("fn parse_one_http_token(") || !transport.includes("fn write_one_page(")) failures.push("MCP accept/read/parse/write authorities are not retained nonblocking steps");
  if (!transport.includes("HTTP_CONNECTION_CAPACITY") || !transport.includes("FixedOwnerRing<HttpTerminalConnection, HTTP_CONNECTION_CAPACITY>") || !transport.includes("request_credits: FixedByteCredits") || !transport.includes("response_credits: FixedByteCredits") || !transport.includes("try_acquire(bytes")) failures.push("MCP connection/request/response ownership lacks fixed item and byte preflight");
  if (!transport.includes("pub fn take_terminal_connection") || !transport.includes("pub fn take_terminal_job") || !transport.includes("error.into_job()") || !transport.includes("WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned") || !transport.includes("owner.close()")) failures.push("MCP rejected jobs or terminal connections lack exact retrieval and close");
  if (!transport.includes("callback_at(") || !transport.includes("retry_generation") || !transport.includes("readiness_armed.compare_exchange(false, true") || !transport.includes("generation_is_current(current, run_generation)")) failures.push("MCP retry/readiness ownership lacks coalesced generation-keyed re-arm");
  if (!transport.includes("HTTP_IO_PAGE_BYTES") || !transport.includes("connection.stream.read(&mut page[..page_bytes])") || !transport.includes("connection.stream.write(&connection.egress[connection.written..end])") || transport.includes("read_to_end(") || transport.includes("write_all(")) failures.push("MCP socket I/O is not bounded to one page per grant");
  if (!transport.includes("find_crlf_bounded(&connection.ingress, connection.parser.cursor, search_end)") || !transport.includes("connection.ingress.len().min(line_bound)") || !transport.includes("HTTP_REQUEST_LINE_BYTES") || !transport.includes("HTTP_HEADER_CAPACITY") || !transport.includes("HTTP_HEADER_NAME_BYTES") || !transport.includes("HTTP_HEADER_VALUE_BYTES") || !transport.includes("HTTP_PATH_BYTES") || !transport.includes("HTTP_SLOWLORIS_MS")) failures.push("MCP HTTP parsing lacks bounded one-token request-line/header and slowloris limits");
  const dispatch = transport.slice(transport.indexOf("fn dispatch_owned_http("));
  if (!dispatch.includes("origin_allowed(") || !dispatch.includes("owned_bearer_matches(") || !dispatch.includes("state.server.lock()") || dispatch.indexOf("origin_allowed(") > dispatch.indexOf("state.server.lock()") || dispatch.indexOf("owned_bearer_matches(") > dispatch.indexOf("state.server.lock()")) failures.push("MCP authentication/origin checks can follow bridge or protocol mutation");
  if (!transport.includes("websocket_accept(") || !transport.includes("websocket_key_nonce(key).ok_or(HttpTerminalReason::Malformed)?") || !transport.includes('head.header_occurrences("sec-websocket-key") != 1') || !transport.includes("bytes.len() != 24") || !transport.includes('bytes[22..] != *b"=="') || !transport.includes("b & 0x0f != 0") || !transport.includes("let masked = bytes[1] & 0x80 != 0") || !transport.includes("!fin || rsv != 0 || !masked") || !transport.includes("payload_len > WEBSOCKET_FRAME_BYTES") || !transport.includes("opcode & 0x8 != 0 && payload_len > 125")) failures.push("MCP WebSocket handshake key/frame limits or client-mask validation are incomplete");
  const openedBridge = transport.slice(transport.indexOf("fn parse_one_websocket_frame"), transport.indexOf("fn consume_websocket_ingress"));
  if (openedBridge.includes("ShellToGateway::decode") || !openedBridge.includes("ShellToGatewayDecodeCursor::new(frame.payload_len)") || !openedBridge.includes("decoder.step(|index| frame.payload_byte(&connection.ingress, index))") || !openedBridge.includes("ShellToGatewayMaterializeCursor::new(frame)") || !openedBridge.includes("materializer.step()") || !openedBridge.includes("inbound.generation != connection.key.generation") || !openedBridge.includes("ConnectionTurn::Terminal(HttpTerminalReason::Malformed)") || !openedBridge.includes("ConnectionTurn::Terminal(HttpTerminalReason::Unsupported)") || openedBridge.indexOf("self.consume_websocket_ingress(connection, consumed)") < openedBridge.indexOf("ShellMaterializeStep::Complete(message)")) failures.push("opened bridge bypasses retained validation/materialization or consumes exact raw ingress early");
  if (!transport.includes("payload_start: usize") || !transport.includes("payload_byte(self, bytes: &[u8], index: usize)") || transport.includes("payload: Vec<u8>") || !transport.includes("copy_control_payload")) failures.push("untrusted WebSocket ingress is allocated before bounded bridge validation");
  const boundedDecoder = bridge.slice(bridge.indexOf("pub(crate) struct ShellToGatewayDecodeCursor"), bridge.indexOf("//#endregion 🔖️BoundedShellDecode"));
  if (!boundedDecoder.includes("ranges: [Option<ShellRange>; BRIDGE_INBOUND_MAX_RANGES]") || !boundedDecoder.includes("BRIDGE_INBOUND_MAX_ITEMS.saturating_sub(self.items)") || !boundedDecoder.includes("count.checked_mul(minimum_item_bytes)") || !boundedDecoder.includes("self.payload_len.saturating_sub(self.cursor)") || !boundedDecoder.includes("BRIDGE_INBOUND_MAX_FIELD_BYTES") || !boundedDecoder.includes("owned_bytes.checked_add(len)") || !boundedDecoder.includes("ValidateStrings") || !boundedDecoder.includes("BRIDGE_INBOUND_PAGE_BYTES.min") || !boundedDecoder.includes("try_reserve_exact") || !boundedDecoder.includes("ShellToGatewayMaterializeCursor") || boundedDecoder.includes("ShellToGateway::decode")) failures.push("ShellToGateway untrusted counts/ranges/UTF-8 or retained owner construction are not completely bounded");
  const outboxSend = bridge.slice(bridge.indexOf("fn try_send(&self, frame: GatewayToShell)"), bridge.indexOf("fn claim(&self, bytes: usize)"));
  if (bridge.includes("tokio::sync") || bridge.includes("mpsc::") || bridge.includes("VecDeque<GatewayToShell>") || !bridge.includes("slots: [Option<BridgeOutboxItem>; BRIDGE_OUTBOX_MAX_ITEMS]") || !bridge.includes("BRIDGE_OUTBOX_MAX_BYTES.saturating_sub(self.bytes).saturating_sub(self.reserved_bytes)") || !bridge.includes("pub fn try_send_to") || !bridge.includes("Err(frame)") || !bridge.includes("BridgeEncodedFrame") || !bridge.includes("BRIDGE_OUTBOX_PAGE_BYTES") || !transport.includes("try_recv_encoded()") || !transport.includes("fn write_one_bridge_page") || !outboxSend.includes("frame.encoded_len()") || !outboxSend.includes("self.claim(bytes)") || outboxSend.includes("frame.encode()") || outboxSend.indexOf("self.claim(bytes)") > outboxSend.indexOf("BridgeEncodedFrame::encode(&frame, bytes)")) failures.push("live bridge outbox lacks preflight-first fixed-page encoding and exact handback");
  const broadcast = bridge.slice(bridge.indexOf("pub fn broadcast(&self, frame: GatewayToShell)"), bridge.indexOf("pub fn take_broadcast_completion"));
  const broadcastCursor = bridge.slice(bridge.indexOf("struct BridgeBroadcastCursor"), bridge.indexOf("struct BridgeAsyncState"));
  const broadcastStep = broadcastCursor.slice(broadcastCursor.indexOf("fn step("), broadcastCursor.indexOf("fn close_one_claim"));
  if (!broadcast.includes("-> Result<usize, GatewayToShell>") || !broadcast.includes("frame.encoded_len()") || !broadcast.includes("bytes.checked_mul(recipients)") || !broadcast.includes("reserve_broadcast()") || !broadcast.includes("reserve_retirement()") || !broadcast.includes("[Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS]") || !broadcast.includes("recipient_ids[..recipients].sort_unstable()") || !broadcast.includes("cancel_broadcast_reservation()") || !broadcast.includes("return Err(frame)") || broadcast.includes("frame.clone()") || broadcast.includes("BridgeEncodedFrame::encode")) failures.push("bridge broadcast clones, reorders, or encodes before atomic aggregate recipient admission");
  if (!broadcast.includes("BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&self.inner.asynchronous), retirement)))") || !broadcastCursor.includes("BridgeRecipientState::Published") || !broadcastCursor.includes("BridgeRecipientState::RecipientClosed") || !bridge.includes("struct BridgeRejectedPublish") || !broadcastCursor.includes("grant: rejected.grant") || !broadcastCursor.includes("self.recipient_cursor += 1") || !broadcastCursor.includes("BridgeBroadcastCompletion::Undelivered") || !broadcastCursor.includes("recipient_closed: self.recipient_closed") || !bridge.includes("pub fn take_broadcast_completion") || !bridge.includes("fn close_one_terminal_broadcast_claim") || !bridge.includes("let step = cursor.step();") || !bridge.includes("copy_encoded_page(self.offset") || !bridge.includes("BridgeBroadcastStep::Pending") || !bridge.includes("generation: grant.generation") || !bridge.includes("impl Drop for BridgeEncodedFrame") || !bridge.includes("authority.publish_retirement(grant, pages)") || !bridge.includes("fn close_one_terminal_retired_page") || !bridge.includes("fn take_terminal_job") || !bridge.includes("scheduled.compare_exchange(false, true") || !bridge.includes("retry_generation") || !bridge.includes("callback_at(") || /unreachable!|panic!/.test(broadcastCursor) || broadcastStep.includes("while ")) failures.push("broadcast close race lacks deterministic one-recipient results, exact completion, or retained shared-page retirement");
  const terminalTake = transport.slice(transport.indexOf("pub fn take_terminal_connection"), transport.indexOf("pub fn take_terminal_job"));
  const terminalDrive = transport.slice(transport.indexOf("fn drive_one(&mut self, now_ms"), transport.indexOf("fn next_non_io_connection"));
  if (!transport.includes("terminal_policy: HttpTerminalPolicy") || !transport.includes("HttpTerminalPolicy::Handback") || !transport.includes("state.terminal_policy = HttpTerminalPolicy::Close") || !transport.includes("HttpTurn::Parked") || !terminalDrive.includes("if self.terminal_policy == HttpTerminalPolicy::Close") || !terminalTake.includes("self.inner.clear_readiness()") || !terminalTake.includes("self.inner.request_schedule()") || !transport.includes("readiness_generation: AtomicU64") || !transport.includes("generation_is_current(authority.readiness_generation.load")) failures.push("terminal connection FIFO handback can race close or leave stale readiness armed");
  for (const fixture of [
    "connection_cap_plus_one_returns_the_exact_owner_without_mutating_fifo",
    "request_and_response_byte_cap_plus_one_return_the_exact_owner",
    "stale_readiness_and_retry_generations_cannot_rearm_an_aba_run",
    "websocket_rejects_unmasked_fragmented_and_oversize_frames_before_consumption",
    "one_terminal_close_grant_drains_exactly_one_fifo_owner",
    "cancellation_and_shutdown_drain_one_connection_authority_per_grant",
    "bridge_outbox_item_cap_plus_one_returns_the_exact_frame_and_rearms_after_one_receive",
    "bridge_outbox_byte_cap_plus_one_returns_the_exact_frame_before_queue_mutation",
    "bridge_outbox_terminal_close_rejects_the_exact_late_frame",
    "websocket_key_rejects_duplicate_invalid_alphabet_padding_whitespace_width_and_noncanonical_bits",
    "request_line_and_header_delimiter_search_faults_at_cap_without_scanning_late_crlf",
    "malformed_and_unsupported_open_bridge_binary_frames_retain_exact_raw_ingress",
    "bridge_outbox_page_boundary_matches_the_canonical_encoder",
    "terminal_public_fifo_preserves_generation_aba_and_process_close_is_one_owner_per_grant",
    "bounded_shell_decoder_rejects_ffffffff_counts_and_truncated_ranges_before_owner_allocation",
    "bounded_shell_decoder_cap_plus_one_and_every_variant_match_the_canonical_fixture",
    "bounded_shell_decoder_and_materializer_advance_incrementally",
    "incremental_bridge_decode_cancellation_and_stale_generation_retain_exact_raw_owner",
    "broadcast_partial_saturation_rolls_back_every_claim_and_returns_the_exact_uncloned_message",
    "broadcast_many_recipient_and_oversize_preflight_reject_before_encode",
    "shared_broadcast_leases_are_generation_keyed_and_close_rejects_aba_publish",
    "broadcast_close_before_first_publish_delivers_survivors_in_stable_admitted_order",
    "broadcast_close_mid_recipient_list_reports_partial_counts_and_fifo_delivery",
    "broadcast_all_close_returns_the_exact_original_completion_after_every_claim",
    "broadcast_reopen_same_slot_aba_cannot_consume_the_stale_recipient_claim",
    "broadcast_shutdown_cancel_poison_closes_each_remaining_claim_one_grant_then_reports_partial_delivery",
    "last_shared_lease_transfers_pages_to_one_page_terminal_retirement_grants",
    "terminal_broadcast_close_returns_one_exact_original_and_cancels_recipient_credit",
  ]) if (!transportSource.includes(fixture) && !bridgeSource.includes(fixture)) failures.push(`MCP owned-transport fixture missing: ${fixture}`);
  return failures;
}


//#endregion 🔖️InteractivityAudit
