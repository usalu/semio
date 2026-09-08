import { interactivityDatabaseCreateCatalogFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity database create catalog policy assertions. */
export function interactivityDatabaseCreateCatalogSelfTests(): void {
  const waits = "db_actor::block_on(entry)\n".repeat(2);
  const retained = `//#region 🔖️CreateDocumentCatalogCas
const DATABASE_CREATE_CATALOG_SLOTS: usize = 32; const DATABASE_CREATE_CATALOG_MAX_ENTRIES: usize = 4_096; DATABASE_CREATE_CATALOG_MAX_ENTRIES * 4; DATABASE_CREATE_CATALOG_MAX_ENTRIES * 2; DbIoText::maximum_capacity(); const DATABASE_CREATE_CATALOG_MAX_PAGES: usize = db_storage::DB_IO_OPERATION_PAGES; const DATABASE_CREATE_CATALOG_COPY_BYTES: usize = 256; DATABASE_CREATE_CATALOG_TOTAL_ITEMS DATABASE_CREATE_CATALOG_TOTAL_BYTES DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES
struct DatabaseCreateCatalogBackingLedger { items: u64, bytes: u64 } impl DatabaseCreateCatalogBackingLedger { fn new { u64::try_from(base_capacity); u64::try_from(bytes); } fn observe(&mut self, items: u64, bytes: usize) { self.items.checked_add(items); u64::try_from(bytes); self.bytes.checked_add(bytes); self.items > DATABASE_CREATE_CATALOG_ITEMS; self.bytes > DATABASE_CREATE_CATALOG_BYTES; } }
fn ledger { document.0.capacity() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES; generation.checked_add(1); self.items.checked_add(DATABASE_CREATE_CATALOG_ITEMS); self.bytes.checked_add(DATABASE_CREATE_CATALOG_BYTES); entry.document.0.capacity(); candidate.capacity(); text.capacity(); pages.checked_mul(db_storage::DB_IO_PAGE_BYTES); }
struct DatabaseCreateCatalogEncodeCursor { pending: [u8; 32] } enum DatabaseCreateCatalogPhase { Scan Reserve Clone Snapshot Encode Seal Claim Handoff Poll CloseWork Revalidate Retire Publish Terminal }
DatabaseCreateCatalogPhase::Scan DatabaseCreateCatalogPhase::Reserve DatabaseCreateCatalogPhase::Clone DatabaseCreateCatalogPhase::Snapshot DatabaseCreateCatalogPhase::Encode DatabaseCreateCatalogPhase::Seal DatabaseCreateCatalogPhase::Claim DatabaseCreateCatalogPhase::Handoff DatabaseCreateCatalogPhase::Poll DatabaseCreateCatalogPhase::CloseWork DatabaseCreateCatalogPhase::Revalidate DatabaseCreateCatalogPhase::Retire DatabaseCreateCatalogPhase::Publish DatabaseCreateCatalogPhase::Terminal
fn try_prepare(pool, catalog, storage, document) { DatabaseCreateCatalogAdmission::try_claim(&document); catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES; DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity()); Arc::clone(&catalog.entries); let state = Arc::new(DatabaseCreateCatalogState); database_create_catalog_registry; state.schedule(); }
fn schedule(self: &Arc<Self>) { self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8; self.wake_requested.store(true; self.pool.try_submit(Lane::Io, job); }
fn submit_exact { *self.retry_job.lock() = Some((error.into_job(), next_attempt)); self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8; let state = self.clone(); self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry()); }
fn retry(self: Arc<Self>) { !self.is_current(); self.cancelled.load; self.deadline_ms.load; attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT; self.terminal_job = Some(job); self.retry_closing.store(true; self.drive_callback_close_claimed(); self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8; } fn arm_callback_close { self.arm_callback_close(); } fn callback_close_one { self.arm_callback_close(); }
fn drive_one(self: Arc<Self>, generation: u64) { self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8; self.opportunities.fetch_add; self.drive_claimed(generation); self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8, DatabaseCreateCatalogDriverAuthority::Idle as u8; self.wake_requested.swap(false; }
fn drive_claimed { generation != self.generation || !self.is_current(); self.cancelled.load; self.deadline_ms.load; DatabaseCreateCatalogPhase::CloseWork DatabaseCreateCatalogPhase::Revalidate }
fn stage_error {}
fn scan_one { entry.document == *document; source_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES; cursor.scan_byte += 1; Self::add_encoded(&mut cursor, bytes); }
fn reserve_candidate_one { try_reserve_exact(capacity); observed = candidate.capacity(); cursor.candidate = Some(candidate); cursor.backing.observe(observed); }
fn clone_boundary { DATABASE_CREATE_CATALOG_COPY_BYTES }
fn clone_one { clone_text; observed = text.capacity(); cursor.clone_text = Some(text); cursor.backing.observe(observed); cursor.clone_byte = end; }
fn snapshot_one { cursor.snapshot = Some(Arc::new(candidate)); cursor.backing.observe(DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES); pages.checked_mul(db_storage::DB_IO_PAGE_BYTES); cursor.backing.observe(pages); }
fn encode_one { encode.step(snapshot.as_slice(), writer); }
fn seal_one { DbIoPageWriter::seal_retained_step; }
fn claim_one { self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention(); catalog.pending.is_some() && catalog.pending != Some(token); catalog.revision != cursor.base_revision; catalog.epoch != cursor.base_epoch; Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity; catalog.pending = Some(token); }
fn handoff_one { DatabaseCreateCatalogWork::new(storage, pages, expected); }
fn poll_backend_once { self.polling.compare_exchange(false, true); std::panic::catch_unwind(|| work.poll(&mut context)); Ok(std::task::Poll::Pending) => { self.poll_work = Some(work); self.polling.store(false); } Ok(std::task::Poll::Ready(actual)) => { self.terminal_work = Some(work); self.outcome = Some(actual); self.polling.store(false); } Err(_) => { self.terminal_work = Some(work); } }
fn close_work_one {}
fn revalidate_one { self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention(); catalog.pending != Some(token); catalog.revision != cursor.base_revision; catalog.epoch != cursor.base_epoch; Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity; cursor.base_epoch.epoch.checked_add(1); catalog.revision.checked_add(1); catalog.entries = snapshot; }
fn retire_intermediate_one { self.terminal_job; self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention(); self.pending_owned.store(false; pages.close_step(); writer.close_step(); candidate.pop(); cursor.clone_text.take(); cursor.base.take(); }
fn publish_one {}
impl Future for DatabaseCreateCatalogFuture { fn poll { self.state.completion.lock(); self.state.waker.lock(); self.state.completion.lock(); } }
impl Drop for DatabaseCreateCatalogFuture {}
impl Drop for DatabaseCreateCatalogResult { fn drop { state.terminal_completion.lock(); state.abandoned.store(true); state.begin_callback_close(); } }
struct DatabaseCreateCatalogRejectedOwner; struct DatabaseCreateCatalogRejectedClose { terminal_job } impl DatabaseCreateCatalogRejectedClose { fn submit_exact(self: &Arc<Self>, job, attempt) { *self.retry_job.lock() = Some((error.into_job(), next_attempt)); self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8; let state = self.clone(); self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry()); } fn retry(self: Arc<Self>) { attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT; self.pool.now_ms() >= self.deadline_ms; self.terminal_job = Some(job); self.callback_close.store(true; self.drive_close_claimed(); } fn drive_one {} fn callback_close_one {} } impl DatabaseCreateCatalogRejected { fn new { DatabaseCreateCatalogRejectedClose::prepare(pool); } fn into_parts { self.close.take_owner(); self.close.restore_owner(owner); } }
type DatabaseCreateCatalogBackendFuture = Future; storage.catalog().await.cas_root(expected, pages).await;
impl DatabaseCreateCatalogTerminalHandle { fn close_step { self.state.begin_callback_close(); } }
//#endregion 🔖️CreateDocumentCatalogCas`;
  const catalog = `struct CatalogState { revision: u64, entries: Arc<Vec<CatalogEntry>>, pending: Option<DatabaseCreateCatalogToken> }`;
  const caller = `pub async fn create_document(&self, spec: ArtifactSpec) { self.create_document_catalog_retained(spec.document); Err(rejected) => return Err(rejected.close_and_take_error()); let _published_epoch = actual?; self.spawn_authority_create(document.clone()).await?; db_engine.document_created; self.register_handle(document, authority); } pub async fn catalog(&self) { let entries = { let catalog = self.catalog.lock(); Arc::clone(&catalog.entries) }; CatalogView { artifacts: entries.as_ref().clone() } } pub async fn health(&self)`;
  const asyncRuntime = `fn worker_loop(inner) { inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN); select_and_pop(inner); } /// 🧵️ The native, multi-OS-thread work-stealing pool pub fn callback_at(&self) { self.inner.wheel.schedule_callback(deadline_ms, callback); self.inner.notify_idle(); } pub fn is_shutdown`;
  const contract = `The cancellation/deadline/exhaustion latency guarantee is conditional on shared-pool service: at least one native worker must return to the head of \`WorkerPool::worker_loop\`. A sole OS worker that permanently never returns is outside P1x's cancellation-latency guarantee. Keep the exact refused job, storage, document, cursor/backing, admission and generation registry discoverable; must not begin a backend poll, invent a timer thread, create a second pool, or require facade/caller execution for completion. Once service resumes close exactly once. Real worker loop service, never a test-task call to \`TimerWheel::fire_due\`. The two-worker reserved-capacity law must drive actual saturated P1x and rejection-close authorities into \`Retry\` through their exact \`callback_at(... state.retry())\` registrations.`;
  const laws = [
    `fn database_create_catalog_max_plus_one_document_and_entry_caps_return_exact_owners() { DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1; document.0.capacity(); DATABASE_CREATE_CATALOG_MAX_ENTRIES; Arc::as_ptr(&storage); into_parts(); }`,
    `fn database_create_catalog_observed_vec_and_string_overallocation_faults_retire_exact_backings() { controlled_capacity_overage; DATABASE_CREATE_CATALOG_ITEMS as usize + 1; candidate.is_some(); observed backing capacity; DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1; clone_text.is_some(); cloned string capacity; admission; }`,
    `fn database_create_catalog_large_tree_yields_scan_copy_encode_seal_and_publishes_exact_epoch() { 0..128; state.opportunities.load; epoch.next(); catalog.revision; catalog.pending.is_none(); }`,
    `fn database_create_catalog_duplicate_and_concurrent_same_base_are_deterministic() { AlreadyExists; first.is_ok(); second.is_ok(); DbError::Fenced; filter(|entry|); }`,
    `fn database_create_catalog_cancel_deadline_and_generation_aba_preserve_exact_identity() { probe.cancel(); deadline_ms.store(0; replacement; StaleGeneration; Arc::as_ptr(&storage); }`,
    `fn database_create_catalog_handoff_cancel_claim_prevents_backend_poll_and_retires_exact_pages() { catalog_bootstrap_pages(3); controlled_driver_hook; DatabaseCreateCatalogDriverAuthority::Driving; active_drivers; probe.cancel(); !state.polling.load; Some(operation); Err(DbError::Closed); }`,
    `fn database_create_catalog_pending_ready_and_panic_publish_work_before_driver_release() { ControlledCreateCatalogPoll::Pending; ControlledCreateCatalogPoll::Ready; ControlledCreateCatalogPoll::Panic; cancel_on_ready; Ok(epoch.next()); poll_worker_thread; poll_work; terminal_work; }`,
    `fn database_create_catalog_saturation_retains_exact_job_and_recovers() { WorkerSubmitErrorKind::Saturated; retry_job; Some(pointer); actual.is_ok(); }`,
    `fn database_create_catalog_real_worker_loop_services_finite_saturation_cancel_deadline_exhaustion_and_close() { replenishing_held_create_catalog_io_pool; release_held_create_catalog_worker; submission_refusals; DATABASE_CREATE_CATALOG_RETRY_LIMIT; backend_polls; callback_worker_thread; terminal_job_retirements; retry exhausted; retry_job; terminal_job; database_create_catalog_registry; close.terminal_is_empty(); pool.shutdown(); }`,
    `fn database_create_catalog_two_worker_reserved_capacity_services_timers_while_one_violator_is_held() { reserved_replenishing_create_catalog_io_pool; DatabaseCreateCatalogFuture::try_submit; DatabaseCreateCatalogDriverAuthority::Retry; retry_job; reserved-cancel; reserved-deadline; reserved-exhaust; release_held_create_catalog_worker(&service_gate); submission_refusals; DATABASE_CREATE_CATALOG_RETRY_LIMIT; backend_polls; callback_worker_thread; terminal_job_retirements; database_create_catalog_registry; admission; close.terminal_is_empty(); maintenance_gate; }`,
    `fn database_create_catalog_sole_permanently_nonreturning_worker_retains_discoverable_owners_without_latency_claim() { held_create_catalog_io_pool; probe.cancel(); drop(probe); DatabaseCreateCatalogDriverAuthority::Retry; retry_job; storage_pointer; admission; backend_polls; database_create_catalog_registry; take_database_create_catalog_terminal; DatabaseCreateCatalogCloseStep::Blocked; !terminal.terminal_is_empty(); }`,
    `fn database_create_catalog_drop_terminal_close_retires_one_owner_per_lane_grant() { take_database_create_catalog_terminal; terminal.close_step(); saturating_sub(current) <= 1; state.admission; }`,
    `fn database_create_catalog_one_production_opportunity_is_under_eight_ms_and_native_wasm_share_source() { drive_one(state.generation); from_millis(8); opportunities.load; include_str!; target_arch; db_actor::block_on; }`,
    `fn database_create_catalog_maximum_catalog_claim_revalidate_and_snapshot_clone_never_hold_worker() { DATABASE_CREATE_CATALOG_MAX_ENTRIES - 1; DatabaseCreateCatalogPhase::Claim; DatabaseCreateCatalogPhase::Revalidate; DatabaseCreateCatalogPhase::Retire; pending_owned; catalog_contention_armed; from_millis(8); Arc::clone(&catalog.entries); entries.as_ref().clone(); }`,
    `fn database_create_catalog_durable_publication_precedes_authority_spawn_emit_and_registration() { create_document_catalog_retained; actual.is_ok(); open_artifacts; catalog; document; }`,
    `fn database_create_catalog_publication_check_register_recheck_has_no_lost_wake() { controlled_publication_before_waker_hook; hook_state.schedule(); completion.lock(); published.load; state.waker; Ok(epoch.next()); }`,
  ].join("\n");
  const good = `${waits}${catalog}${retained}${caller}${laws}`;
  const retryRegistration = "self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry())";
  const rejectionRegistrationStart = good.indexOf(retryRegistration, good.indexOf("struct DatabaseCreateCatalogRejectedOwner"));
  const rejectionRegistrationRemoved = rejectionRegistrationStart < 0 ? good : `${good.slice(0, rejectionRegistrationStart)}drop(state)${good.slice(rejectionRegistrationStart + retryRegistration.length)}`;
  const mutations: readonly [string, string][] = [
    ["third-wait", `${good} db_actor::block_on(extra)`],
    ["caller-block-on", good.replace("self.create_document_catalog_retained(spec.document)", "db_actor::block_on(cas_root())")],
    ["mutable-catalog-vector", good.replace("entries: Arc<Vec<CatalogEntry>>", "entries: Vec<CatalogEntry>")],
    ["missing-revision", good.replace("revision: u64", "revision: usize")],
    ["len-ledger", good.replace("document.0.capacity() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES", "document.0.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES")],
    ["unchecked-generation", good.replace("generation.checked_add(1)", "generation + 1")],
    ["claim-after-clone", good.replace("DatabaseCreateCatalogAdmission::try_claim(&document); catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES; DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity()); Arc::clone(&catalog.entries)", "catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES; DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity()); Arc::clone(&catalog.entries); DatabaseCreateCatalogAdmission::try_claim(&document)")],
    ["missing-scan-cursor", good.replace("cursor.scan_byte += 1", "scan_all(source)")],
    ["missing-base-capacity", good.replace("source_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES", "source.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES")],
    ["infallible-candidate", good.replace("try_reserve_exact(capacity)", "Vec::with_capacity(capacity)")],
    ["whole-string-clone", good.replace("cursor.clone_byte = end", "clone_text = source.to_string()")],
    ["dynamic-encode-buffer", good.replace("pending: [u8; 32]", "pending: Vec<u8>")],
    ["bulk-seal", good.replace("DbIoPageWriter::seal_retained_step", "DbIoPageWriter::finish")],
    ["missing-snapshot-identity", good.replace("Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity", "false")],
    ["handoff-polls", good.replace("DatabaseCreateCatalogWork::new(storage, pages, expected);", "DatabaseCreateCatalogWork::new(storage, pages, expected).poll(&mut context);")],
    ["wrong-lane", good.replace("self.pool.try_submit(Lane::Io, job)", "self.pool.try_submit(Lane::Maintenance, job)")],
    ["saturation-drops-job", good.replace("Some((error.into_job(), next_attempt))", "drop(error)")],
    ["retry-exhaustion-removed", good.replace("attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT; self.terminal_job", "false; self.terminal_job")],
    ["retry-currentness-removed", good.replace("!self.is_current(); self.cancelled.load", "self.cancelled.load")],
    ["retry-cancel-removed", good.replace("self.cancelled.load; self.deadline_ms.load", "self.deadline_ms.load")],
    ["retry-deadline-removed", good.replace("self.deadline_ms.load; attempt >=", "attempt >=")],
    ["retry-job-not-handed-to-close", good.replace("self.terminal_job = Some(job); self.retry_closing.store(true", "drop(job); self.retry_closing.store(true")],
    ["retry-timer-registration-removed", good.replace("self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry())", "drop(state)")],
    ["rejection-retry-timer-registration-removed", rejectionRegistrationRemoved],
    ["retry-callback-close-bypassed", good.replace("self.drive_callback_close_claimed()", "drop(self)")],
    ["rejection-retry-exhaustion-removed", good.replace("attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT; self.pool.now_ms() >= self.deadline_ms", "self.pool.now_ms() >= self.deadline_ms")],
    ["rejection-retry-deadline-removed", good.replace("self.pool.now_ms() >= self.deadline_ms; self.terminal_job", "self.terminal_job")],
    ["missing-driver-claim", good.replace("DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8", "DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Idle as u8")],
    ["driver-release-before-body", good.replace("self.drive_claimed(generation); self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8", "self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8; self.drive_claimed(generation);")],
    ["ready-owner-after-release", good.replace("self.terminal_work = Some(work); self.outcome = Some(actual); self.polling.store(false)", "self.polling.store(false); self.terminal_work = Some(work); self.outcome = Some(actual)")],
    ["pending-drops-owner", good.replace("self.poll_work = Some(work); self.polling.store(false)", "drop(work); self.polling.store(false)")],
    ["panic-drops-owner", good.replace("Err(_) => { self.terminal_work = Some(work);", "Err(_) => { drop(work);")],
    ["late-cancel-discards-ready", good.replace("self.cancelled.load; self.deadline_ms.load; DatabaseCreateCatalogPhase::CloseWork DatabaseCreateCatalogPhase::Revalidate", "self.cancelled.load; self.deadline_ms.load; DatabaseCreateCatalogPhase::Retire DatabaseCreateCatalogPhase::Publish")],
    ["unchecked-publication-revision", good.replace("catalog.revision.checked_add(1)", "catalog.revision + 1")],
    ["publish-before-revalidate", good.replace("cursor.base_epoch.epoch.checked_add(1); catalog.revision.checked_add(1); catalog.entries = snapshot", "catalog.entries = snapshot; cursor.base_epoch.epoch.checked_add(1); catalog.revision.checked_add(1)")],
    ["bulk-retirement", good.replace("pages.close_step();", "while !pages.terminal_is_empty() { pages.close_step(); }")],
    ["lost-wake-recheck", good.replace("self.state.completion.lock(); self.state.waker.lock(); self.state.completion.lock();", "self.state.completion.lock(); self.state.waker.lock();")],
    ["result-drop", good.replace("state.terminal_completion.lock();", "drop(owner);")],
    ["rejection-destructure", good.replace("self.close.take_owner(); self.close.restore_owner(owner);", "drop(self.close);")],
    ["candidate-capacity-unobserved", good.replace("observed = candidate.capacity(); cursor.candidate = Some(candidate); cursor.backing.observe(observed);", "cursor.candidate = Some(candidate);")],
    ["candidate-capacity-drop-before-retain", good.replace("cursor.candidate = Some(candidate); cursor.backing.observe(observed);", "cursor.backing.observe(observed); cursor.candidate = Some(candidate);")],
    ["string-capacity-unobserved", good.replace("observed = text.capacity(); cursor.clone_text = Some(text); cursor.backing.observe(observed);", "cursor.clone_text = Some(text);")],
    ["string-capacity-drop-before-retain", good.replace("cursor.clone_text = Some(text); cursor.backing.observe(observed);", "cursor.backing.observe(observed); cursor.clone_text = Some(text);")],
    ["base-vec-capacity-unobserved", good.replace("DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity())", "DatabaseCreateCatalogBackingLedger::new(document.0.len(), catalog.entries.len())")],
    ["arc-page-capacity-unobserved", good.replace("cursor.backing.observe(DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES); pages.checked_mul(db_storage::DB_IO_PAGE_BYTES); cursor.backing.observe(pages);", "cursor.backing.observe(0);")],
    ["claim-blocking-lock", good.replace("fn claim_one { self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention();", "fn claim_one { self.catalog.lock();")],
    ["revalidate-blocking-lock", good.replace("fn revalidate_one { self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention();", "fn revalidate_one { self.catalog.lock();")],
    ["retire-blocking-lock", good.replace("self.terminal_job; self.catalog.try_lock(); std::sync::TryLockError::WouldBlock; self.defer_catalog_contention(); self.pending_owned.store(false", "self.terminal_job; self.catalog.lock(); self.pending_owned.store(false")],
    ["catalog-deep-clone-under-lock", good.replace("Arc::clone(&catalog.entries) }; CatalogView { artifacts: entries.as_ref().clone() }", "catalog.entries.as_ref().clone() }; CatalogView { artifacts: entries }")],
    ["spawn-before-durable", good.replace("let _published_epoch = actual?; self.spawn_authority_create", "self.spawn_authority_create; let _published_epoch = actual?;")],
    ["missing-max-law-owner", good.replace("document.0.capacity(); DATABASE_CREATE_CATALOG_MAX_ENTRIES; Arc::as_ptr(&storage); into_parts();", "assert!(true);")],
    ["capacity-overallocation-law-shallow", good.replace("controlled_capacity_overage; DATABASE_CREATE_CATALOG_ITEMS as usize + 1; candidate.is_some(); observed backing capacity; DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1; clone_text.is_some(); cloned string capacity; admission;", "assert!(true);")],
    ["large-law-shallow", good.replace("0..128; state.opportunities.load; epoch.next(); catalog.revision; catalog.pending.is_none();", "assert!(true);")],
    ["duplicate-law-shallow", good.replace("AlreadyExists; first.is_ok(); second.is_ok(); DbError::Fenced; filter(|entry|);", "assert!(true);")],
    ["aba-law-shallow", good.replace("probe.cancel(); deadline_ms.store(0; replacement; StaleGeneration; Arc::as_ptr(&storage);", "probe.cancel();")],
    ["handoff-cancel-law-shallow", good.replace("catalog_bootstrap_pages(3); controlled_driver_hook; DatabaseCreateCatalogDriverAuthority::Driving; active_drivers; probe.cancel(); !state.polling.load; Some(operation); Err(DbError::Closed);", "probe.cancel();")],
    ["poll-law-no-ready-cancel", good.replace("cancel_on_ready; Ok(epoch.next()); poll_worker_thread; poll_work; terminal_work;", "assert!(actual.is_err());")],
    ["saturation-law-no-recovery", good.replace("WorkerSubmitErrorKind::Saturated; retry_job; Some(pointer); actual.is_ok();", "WorkerSubmitErrorKind::Saturated;")],
    ["finite-saturation-law-shallow", good.replace("replenishing_held_create_catalog_io_pool; release_held_create_catalog_worker; submission_refusals; DATABASE_CREATE_CATALOG_RETRY_LIMIT; backend_polls; callback_worker_thread; terminal_job_retirements; retry exhausted; retry_job; terminal_job; database_create_catalog_registry; close.terminal_is_empty(); pool.shutdown();", "assert!(true);")],
    ["finite-saturation-law-manual-fire", good.replace("release_held_create_catalog_worker; submission_refusals", "pool.timer_wheel().fire_due(u64::MAX); submission_refusals")],
    ["finite-saturation-law-adds-caller-fire", good.replace("release_held_create_catalog_worker; submission_refusals", "release_held_create_catalog_worker; pool.timer_wheel().fire_due(u64::MAX); submission_refusals")],
    ["two-worker-p1x-retry-law-shallow", good.replace("reserved_replenishing_create_catalog_io_pool; DatabaseCreateCatalogFuture::try_submit; DatabaseCreateCatalogDriverAuthority::Retry; retry_job; reserved-cancel; reserved-deadline; reserved-exhaust; release_held_create_catalog_worker(&service_gate); submission_refusals; DATABASE_CREATE_CATALOG_RETRY_LIMIT; backend_polls; callback_worker_thread; terminal_job_retirements; database_create_catalog_registry; admission; close.terminal_is_empty(); maintenance_gate;", "assert!(true);")],
    ["sole-nonreturning-law-shallow", good.replace("held_create_catalog_io_pool; probe.cancel(); drop(probe); DatabaseCreateCatalogDriverAuthority::Retry; retry_job; storage_pointer; admission; backend_polls; database_create_catalog_registry; take_database_create_catalog_terminal; DatabaseCreateCatalogCloseStep::Blocked; !terminal.terminal_is_empty();", "assert!(true);")],
    ["drop-law-no-single-owner", good.replace("terminal.close_step(); saturating_sub(current) <= 1; state.admission;", "drop(terminal);")],
    ["timing-law-no-budget", good.replace("drive_one(state.generation); from_millis(8); opportunities.load;", "assert!(true);")],
    ["contention-law-shallow", good.replace("DATABASE_CREATE_CATALOG_MAX_ENTRIES - 1; DatabaseCreateCatalogPhase::Claim; DatabaseCreateCatalogPhase::Revalidate; DatabaseCreateCatalogPhase::Retire; pending_owned; catalog_contention_armed; from_millis(8); Arc::clone(&catalog.entries); entries.as_ref().clone();", "assert!(true);")],
    ["publication-law-shallow", good.replace("create_document_catalog_retained; actual.is_ok(); open_artifacts; catalog; document;", "assert!(true);")],
    ["lost-wake-law-shallow", good.replace("controlled_publication_before_waker_hook; hook_state.schedule(); completion.lock(); published.load; state.waker; Ok(epoch.next());", "assert!(true);")],
  ];
  for (const [name, source] of mutations) if (interactivityDatabaseCreateCatalogFailures(source, asyncRuntime, contract).length === 0) throw new Error(`[verify interactivity p1x] hostile mutation ${name} was falsely accepted.`);
  for (const [name, source] of [
    ["worker-loop-does-not-fire-timers", asyncRuntime.replace("inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN);", "")],
    ["worker-loop-fires-after-job-selection", asyncRuntime.replace("inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN); select_and_pop(inner);", "select_and_pop(inner); inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN);")],
    ["callback-does-not-wake-idle-worker", asyncRuntime.replace("self.inner.notify_idle();", "")],
  ] as const) if (interactivityDatabaseCreateCatalogFailures(good, source, contract).length === 0) throw new Error(`[verify interactivity p1x] async-runtime mutation ${name} was falsely accepted.`);
  for (const [name, source] of [
    ["contract-claims-permanent-worker-latency", contract.replace("outside P1x's cancellation-latency guarantee", "inside P1x's cancellation-latency guarantee")],
    ["contract-omits-discoverability", contract.replace("exact refused job, storage, document, cursor/backing, admission and generation registry discoverable", "operation remains safe")],
    ["contract-allows-caller-timer-driver", contract.replace("never a test-task call to `TimerWheel::fire_due`", "test task drives timers")],
    ["contract-detaches-two-worker-proof", contract.replace("two-worker reserved-capacity law must drive actual saturated P1x and rejection-close authorities into `Retry`", "two-worker law runs a generic callback")],
  ] as const) if (interactivityDatabaseCreateCatalogFailures(good, asyncRuntime, source).length === 0) throw new Error(`[verify interactivity p1x] liveness-contract mutation ${name} was falsely accepted.`);
  const failures = interactivityDatabaseCreateCatalogFailures(good, asyncRuntime, contract);
  if (failures.length !== 0) throw new Error(`[verify interactivity p1x] faithful source fixture was falsely rejected: ${failures.join("; ")}`);
}
