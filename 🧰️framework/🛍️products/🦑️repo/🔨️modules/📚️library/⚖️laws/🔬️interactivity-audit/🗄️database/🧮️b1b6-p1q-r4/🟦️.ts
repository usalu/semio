import { interactivityProductionSource } from "../🟦️.ts";

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
