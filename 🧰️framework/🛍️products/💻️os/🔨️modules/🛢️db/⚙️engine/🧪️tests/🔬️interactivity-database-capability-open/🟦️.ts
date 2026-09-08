import { interactivityDatabaseCapabilityOpenFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity database capability open policy assertions. */
export function interactivityDatabaseCapabilityOpenSelfTests(): void {
  const fixtures = `database_capability_open_fixed_admission_cap_plus_one_and_generation_aba database_capability_open_success_returns_exact_storage_owner_and_scalar database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result database_capability_open_rejection_take_retry_and_close_preserve_exact_storage database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback database_capability_open_retry_contention_is_one_compare_exchange_per_callback`;
  const wakeFixture = `struct ControlledCapabilitySubmitQueue { slots: [Option<semio_framework_async::Job>; 8] } async fn database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary { controlled_submit_hook state.schedule(); let initial = queue.pop(); initial(); wake-after-release cannot admit a duplicate successor let successor = queue.pop(); successor(); Pending is repolled only by its next governed successor terminal Ready/panic successor advances cleanup without repolling terminal_work staged_result }`;
  const waits = "db_actor::block_on(entry)\n".repeat(2);
  const capability = `//#region 🔖️CapabilityOpen const DATABASE_CAPABILITY_OPEN_SLOTS: usize = 64 const DATABASE_CAPABILITY_OPEN_ITEMS: u64 = 8 const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024 DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS DATABASE_CAPABILITY_OPEN_TOTAL_BYTES DatabaseCapabilityOpenAdmission::try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES) database_capability_open_registry enum DatabaseCapabilityOpenPhase { Handoff, Poll, RetainWork, DrainWork, ReleaseWork, RetainResult, Publish, Terminal } fn drive_one(self: Arc<Self>, generation: u64) { if generation != self.generation { return; } match self.phase() { DatabaseCapabilityOpenPhase::Handoff => {} DatabaseCapabilityOpenPhase::Poll => {} DatabaseCapabilityOpenPhase::Publish => {} } } fn publish_poll_terminal { self.cancelled.store(true); self.terminal_error.lock(); self.set_phase(DatabaseCapabilityOpenPhase::RetainWork); } fn poll_terminal_if_cancelled_or_stale {} fn release_terminal_poll { self.polling.store(false); self.wake_requested.store(false); self.schedule_cleanup(); } fn poll_backend_once { std::panic::catch_unwind(|| work.poll(&mut context)); match polled { Ok(std::task::Poll::Pending) => { slot = Some(work); self.poll_terminal_if_cancelled_or_stale(); self.polling.store(false); } Ok(std::task::Poll::Ready(output)) => { slot = Some(work); staged = Some(output); self.set_phase(DatabaseCapabilityOpenPhase::RetainWork); self.poll_terminal_if_cancelled_or_stale(); self.polling.store(false); } Err(_) => { slot = Some(work); self.publish_poll_terminal(fault); self.release_terminal_poll(); } } } fn arm_retry { advance_retry_generation_once(); } fn advance_retry_generation_once { advance_retry_generation_observed_once(current); } fn advance_retry_generation_observed_once { compare_exchange(current, generation); observed == state.retry_generation.load; self.pool.callback_at } fn publish_staged { self.cancelled.load(); !self.is_current(); self.complete(); } fn roots_are_empty self.pool.try_submit(Lane::Io, job) error.into_job() retry_generation pub struct DatabaseCapabilityOpenRejected pub fn take_storage(&mut self) -> Option<Arc<db_storage::DbBackend>> pub fn retry(mut self, pool: Arc<WorkerPool>) pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep pub fn close_and_take_error(mut self) -> DbError pub struct DatabaseCapabilityOpenTerminalHandle pub struct DatabaseCapabilityOpenTerminalResult terminal_result_checked_out pub fn take_database_capability_open_terminal pub fn take_next_database_capability_open_terminal pub fn take_result(&self) -> Option<DatabaseCapabilityOpenTerminalResult> pub fn resume(self) -> Result<DatabaseCapabilityOpenFuture, Self> pub fn close_step(&self) -> DatabaseCapabilityOpenCloseStep pub fn terminal_is_empty(&self) -> bool DatabaseCapabilityOpenResult { storage, capabilities } terminal_work terminal_result terminal_completion roots_are_empty //#endregion 🔖️CapabilityOpen`;
  const open = `async fn open_with { Self::open_retained(pool.clone(), storage); Err(rejected) => return Err(rejected.close_and_take_error()); capability_probe.await?.into_parts() } pub async fn create_document`;
  const good = `${waits}${capability}${open}${fixtures}${wakeFixture}`;
  const mutations = [
    ["reintroduced-capability-block", good.replace("Self::open_retained(pool.clone(), storage)", "db_actor::block_on(storage.capabilities())")],
    ["sixth-engine-wait", `${good} db_actor::block_on(extra)`],
    ["missing-byte-cap", good.replace("const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024", "const DATABASE_CAPABILITY_OPEN_BYTES: u64 = u64::MAX")],
    ["missing-aggregate-item-credit", good.replace("DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS", "UNBOUNDED_ITEMS")],
    ["combined-handoff-poll", good.replace("DatabaseCapabilityOpenPhase::Handoff => {}", "DatabaseCapabilityOpenPhase::Handoff => { work.poll(&mut context); }")],
    ["poll-loop", good.replace("work.poll(&mut context)", "loop { work.poll(&mut context) }")],
    ["missing-retain-work-phase", good.replaceAll("RetainWork", "SkipWork")],
    ["missing-release-work-phase", good.replaceAll("ReleaseWork", "DropWork")],
    ["quiet-saturation-drop", good.replace("error.into_job()", "drop(error)")],
    ["missing-generation-check", good.replace("if generation != self.generation { return; }", "")],
    ["missing-publication-cancel-check", good.replace("fn publish_staged { self.cancelled.load(); !self.is_current(); self.complete(); }", "fn publish_staged { !self.is_current(); self.complete(); }")],
    ["missing-publication-stale-check", good.replace("fn publish_staged { self.cancelled.load(); !self.is_current(); self.complete(); }", "fn publish_staged { self.cancelled.load(); self.complete(); }")],
    ["missing-exact-rejection", good.replace("pub fn take_storage(&mut self) -> Option<Arc<db_storage::DbBackend>>", "fn inspect_storage(&self)")],
    ["missing-public-terminal", good.replace("pub fn take_database_capability_open_terminal", "fn inspect_database_capability_open_terminal")],
    ["missing-cap-plus-one-fixture", good.replace("database_capability_open_fixed_admission_cap_plus_one_and_generation_aba", "")],
    ["wake-before-owner-publication", good.replaceAll("slot = Some(work);", "self.polling.store(false); slot = Some(work);")],
    [
      "ready-phase-after-wake-release",
      good.replace(
        "staged = Some(output); self.set_phase(DatabaseCapabilityOpenPhase::RetainWork); self.poll_terminal_if_cancelled_or_stale(); self.polling.store(false);",
        "staged = Some(output); self.polling.store(false); self.set_phase(DatabaseCapabilityOpenPhase::RetainWork); self.poll_terminal_if_cancelled_or_stale();",
      ),
    ],
    ["panic-fault-after-wake-release", good.replace("self.publish_poll_terminal(fault); self.release_terminal_poll();", "self.release_terminal_poll(); self.publish_poll_terminal(fault);")],
    ["missing-post-ready-cancel-stale-fixture", good.replace("database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result", "")],
    ["live-rejection-owner-drop", good.replace("Err(rejected) => return Err(rejected.close_and_take_error())", "Err(rejected) => return Err(rejected.into_parts().0)")],
    ["missing-terminal-result-handback", good.replace("pub fn take_result(&self) -> Option<DatabaseCapabilityOpenTerminalResult>", "fn inspect_terminal_result(&self)")],
    ["retry-generation-spin", good.replace("advance_retry_generation_observed_once(current);", "loop { compare_exchange(current, generation); }")],
    ["controlled-wake-scheduled-mask", good.replace("state.schedule();", "state.scheduled.store(true); state.poll_backend_once(state.generation);")],
    ["controlled-wake-successor-bypass", good.replace("successor();", "drop(successor);")],
  ] as const;
  for (const [name, source] of mutations) if (interactivityDatabaseCapabilityOpenFailures(source).length === 0) throw new Error(`[verify interactivity] database capability-open self-test ${name} was falsely accepted.`);
  const failures = interactivityDatabaseCapabilityOpenFailures(good);
  if (failures.length !== 0) throw new Error(`[verify interactivity] database capability-open self-test retained route was falsely rejected: ${failures.join("; ")}`);
}
