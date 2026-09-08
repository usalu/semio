import { interactivityDatabaseCompactionFailures } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity database compaction policy assertions. */
export function interactivityDatabaseCompactionSelfTests(compact: string, engine: string, snapshot: string, index: string, contract: string): void {
  const baseline = interactivityDatabaseCompactionFailures(compact, engine, snapshot, index, contract);
  if (baseline.length !== 0) throw new Error(`[verify interactivity p1y] live source rejected before mutations: ${baseline.join("; ")}`);
  const mutations: readonly [string, "compact" | "engine" | "snapshot" | "index" | "contract", string, string][] = [
    ["slot-cap-removed", "compact", "const DATABASE_COMPACTION_SLOTS: usize = 32", "const DATABASE_COMPACTION_SLOTS: usize = usize::MAX"],
    ["capacity-ledger-uses-len", "compact", "document.0.capacity() > db_storage::DbIoText::maximum_capacity()", "document.0.len() > db_storage::DbIoText::maximum_capacity()"],
    ["descriptor-capacity-ledger-uses-len", "compact", "descriptor.roots.capacity()", "descriptor.roots.len()"],
    ["index-capacity-ledger-uses-len", "compact", "index_document.0.capacity()", "index_document.0.len()"],
    ["descriptor-retirement-removed", "compact", "retire_compaction_descriptor(descriptor).await", "drop(descriptor)"],
    ["unchecked-generation", "compact", "self.next_generation = generation.checked_add(1)", "self.next_generation = generation + 1"],
    ["dynamic-hash-set", "compact", "struct DatabaseCompactionHashOwners", "struct DatabaseCompactionHashSet(HashSet<pack::ContentHash>)"],
    ["opportunity-no-yield", "compact", "async fn compaction_opportunity(cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {\n    semio_framework_async::yield_once().await;", "async fn compaction_opportunity(cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {\n    std::thread::yield_now();"],
    ["wrong-lane", "compact", "self.pool.try_submit(Lane::Io, job)", "self.pool.try_submit(Lane::Maintenance, job)"],
    ["saturation-drops-job", "compact", "Some((error.into_job(), next))", "drop(error)"],
    ["retry-registration-removed", "compact", "self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry())", "drop(state)"],
    ["retry-limit-removed", "compact", "attempt >= DATABASE_COMPACTION_RETRY_LIMIT", "false"],
    ["driver-claim-removed", "compact", "DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Driving as u8", "DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Idle as u8"],
    ["callback-polls-live-backend", "compact", "fn drive_close_claimed(self: Arc<Self>) {\n        use std::sync::atomic::Ordering;", "fn drive_close_claimed(self: Arc<Self>) {\n        let _ = self.poll_one();\n        use std::sync::atomic::Ordering;"],
    ["terminal-schedule-bypasses-guard", "compact", "self.callback_close.load(Ordering::Acquire) && execution_terminal", "self.callback_close.load(Ordering::Acquire) || execution_terminal"],
    ["pending-owner-dropped", "compact", "core.future = Some(future)", "drop(future)"],
    ["ready-owner-dropped", "compact", "return false;\n                }\n                core.output = Some(output)", "return false;\n                }\n                drop(output)"],
    ["panic-quarantine-dropped", "compact", "core.quarantined = Some(future)", "drop(future)"],
    ["panic-release-future-removed", "compact", "core.panic_release = if", "core.panic_release = None; if false"],
    ["panic-public-completes-before-retire", "compact", "if state.panic_retired.load(std::sync::atomic::Ordering::Acquire)", "if state.panic_fault.load(std::sync::atomic::Ordering::Acquire)"],
    ["panic-quarantine-close-removed", "compact", "core.quarantined.take()", "core.quarantined.as_ref().map(|_| ())"],
    ["release-error-clears-fence", "compact", "if result.is_ok() {", "if true {"],
    ["release-error-marks-released", "compact", "recovery.releasing.store(false, std::sync::atomic::Ordering::Release)", "recovery.released.store(true, std::sync::atomic::Ordering::Release)"],
    ["release-error-dropped", "compact", "core.release_fault = Some(error)", "drop(error)"],
    ["release-error-retry-callback-removed", "compact", "self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.release_retry_callback())", "drop(state)"],
    ["release-waiting-terminal-guard-removed", "compact", "core.release_waiting.is_none()", "true"],
    ["lease-install-removed", "compact", "lease_recovery.install(fence)", "drop(fence)"],
    ["lease-admitted-epoch-removed", "compact", "lease.acquire(lease_recovery.resource.as_str(), holder.as_str(), DEFAULT_LEASE_TTL_MS, now_ms)", "lease.acquire(lease_recovery.resource.as_str(), holder.as_str(), DEFAULT_LEASE_TTL_MS, 0)"],
    ["index-child-private-control", "compact", "handle.retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL)", "handle.operation_control(65_536)"],
    ["index-child-cancel-bypasses-close", "compact", "if let Err(error) = compaction_opportunity(cancelled).await {\n                        break Err(error);\n                    }", "compaction_opportunity(cancelled).await?;"],
    ["index-control-detaches-parent", "index", "IndexCursorControl::new(cancelled, deadline, fuel)", "self.operation_control(fuel)"],
    ["snapshot-atomic-claim-removed", "snapshot", "let _claim = SnapshotPublicationClaim::try_claim(document)?", "let _claim = ()"],
    ["snapshot-atomic-claim-cannot-acquire", "snapshot", "compare_exchange(0, identity", "compare_exchange(identity, identity"],
    ["snapshot-prewrite-generation-check-removed", "snapshot", "observed != Some(expected_generation)", "false"],
    ["snapshot-hidden-hash-vec-restored", "snapshot", "async fn build_generation_retained_expected", "async fn build_generation_retained_expected /* Vec::with_capacity */"],
    ["snapshot-exact-body-recovery-removed", "snapshot", "Err(error) => Err(SnapshotRetainedPublicationRejected { error, body })", "Err(error) => { drop(body); panic!(\"{error:?}\") }"],
    ["cumulative-items-replaced", "compact", "let next_items = self.items.checked_add(items)", "let next_items = items.checked_add(0)"],
    ["cumulative-bytes-replaced", "compact", "let next_bytes = self.bytes.checked_add(bytes)", "let next_bytes = bytes.checked_add(0)"],
    ["page-ledger-release-removed", "compact", "ledger.release(page_items, page_bytes)?", "drop((page_items, page_bytes))"],
    ["deadline-registration-removed", "compact", "pool.callback_at(deadline_ms, move || deadline.deadline_callback())", "drop(deadline)"],
    ["future-drop-does-not-schedule", "compact", "if let Some(state) = self.state.take() {\n            state.abandoned.store(true, std::sync::atomic::Ordering::Release);\n            state.cancelled.store(true, std::sync::atomic::Ordering::Release);\n            state.callback_close.store(true, std::sync::atomic::Ordering::Release);\n            state.schedule();", "if let Some(state) = self.state.take() {\n            state.abandoned.store(true, std::sync::atomic::Ordering::Release);\n            state.cancelled.store(true, std::sync::atomic::Ordering::Release);\n            state.callback_close.store(true, std::sync::atomic::Ordering::Release);\n            drop(state);"],
    ["lost-wake-recheck-removed", "compact", "*state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());\n        if let Some(execution) = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take()", "*state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());\n        if let Some(execution) = None"],
    ["dynamic-report", "compact", "pub struct CompactionIndexReports", "pub struct CompactionIndexReports(Vec<IndexKindReport>)"],
    ["eager-compactor-live", "compact", "#[cfg(test)]\npub struct Compactor<'storage>", "pub struct Compactor<'storage>"],
    ["facade-blocking", "engine", "compaction.await?.close_and_take_report()", "db_actor::block_on(compaction).close_and_take_report()"],
    ["handoff-law-removed", "compact", "retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms", "removed_handoff_law"],
    ["deadline-law-removed", "compact", "retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once", "removed_deadline_law"],
    ["max-law-removed", "compact", "retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority", "removed_max_law"],
    ["aba-law-removed", "compact", "retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity", "removed_aba_law"],
    ["index-child-law-removed", "compact", "retained_compaction_index_child_uses_exact_parent_cancel_and_eight_ms_control", "removed_index_child_law"],
    ["atomic-publication-law-removed", "compact", "retained_compaction_expected_snapshot_publication_never_persists_stale_baseline", "removed_atomic_publication_law"],
    ["panic-release-law-removed", "compact", "retained_compaction_panic_after_lease_acquire_releases_once_before_public_fault_and_registry_drain", "removed_panic_release_law"],
    ["release-error-success-law-removed", "compact", "retained_compaction_release_error_retries_through_real_worker_loop_until_success_before_public_fault", "removed_release_error_success_law"],
    ["release-error-perpetual-law-removed", "compact", "retained_compaction_perpetual_release_error_keeps_fence_fault_admission_and_registry_discoverable", "removed_release_error_perpetual_law"],
    ["cumulative-ledger-law-removed", "compact", "retained_compaction_cumulative_observed_backing_rejects_individually_valid_combined_max_plus_one", "removed_cumulative_ledger_law"],
    ["contract-loses-selected-wait", "contract", "The P1y facade cut is `Database::compact_document`", "The P1y facade cut is `Database::other_wait`"],
    ["contract-allows-private-index-control", "contract", "private child token, thirty-second deadline, or 65,536-fuel control is outside the P1y contract", "private child control is permitted"],
    ["contract-allows-postwrite-revalidation", "contract", "refuses a mismatched generation before descriptor/page construction or storage write", "checks a mismatched generation after storage write"],
    ["contract-allows-panic-early-completion", "contract", "Public fault completion is forbidden until the release witness", "Public fault completion is permitted before the release witness"],
    ["contract-allows-release-error-fence-loss", "contract", "Only `Ok(())` from the backend release may consume the retained fence", "Any backend release result may consume the retained fence"],
    ["contract-allows-release-error-completion", "contract", "persistent release error blocks public terminal completion", "persistent release error permits public terminal completion"],
    ["contract-allows-per-object-backing", "contract", "Backing credit is cumulative across every simultaneously live descriptor", "Backing credit is checked independently for each descriptor"],
  ];
  for (const [name, target, from, to] of mutations) {
    const source = target === "compact" ? compact : target === "engine" ? engine : target === "snapshot" ? snapshot : target === "index" ? index : contract;
    if (!source.includes(from)) throw new Error(`[verify interactivity p1y] mutation ${name} did not bind live source`);
    const mutated = source.replace(from, to);
    const failures = interactivityDatabaseCompactionFailures(
      target === "compact" ? mutated : compact,
      target === "engine" ? mutated : engine,
      target === "snapshot" ? mutated : snapshot,
      target === "index" ? mutated : index,
      target === "contract" ? mutated : contract,
    );
    if (failures.length === 0) throw new Error(`[verify interactivity p1y] hostile mutation ${name} was falsely accepted`);
  }
}
