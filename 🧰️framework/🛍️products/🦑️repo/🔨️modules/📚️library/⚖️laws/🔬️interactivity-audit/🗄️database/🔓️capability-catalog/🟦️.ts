import { interactivityProductionSource, interactivityRegionSource } from "../🟦️.ts";

export function interactivityDatabaseCapabilityOpenFailures(engineSource: string): string[] {
  const engine = interactivityProductionSource(engineSource);
  const capability = interactivityRegionSource(engine, "🔖️CapabilityOpen") ?? "";
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
  const catalog = interactivityRegionSource(engine, "🔖️CatalogRootRead") ?? "";
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
