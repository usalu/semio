import { interactivityVcsBridgeFailures } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity vcs bridge policy assertions. */
export function interactivityVcsBridgeSelfTests(): void {
  const fixtures = `vcs_retained_item_cap_plus_one_and_nested_bytes_plus_one_return_without_mutation vcs_record_derived_owner_credit_cap_plus_one_preserves_exact_input vcs_checkpoint_derived_owner_credit_cap_plus_one_preserves_exact_input vcs_checkpoint_derived_item_boundary_admits_31_rejects_32_and_preserves_exact_owners vcs_derived_owner_process_aggregate_plus_one_rejects_without_consuming_input vcs_retained_pending_wake_is_fifo_one_shot_and_quiet_without_release vcs_retained_cancel_clears_waiter_and_slot_aba_stays_stale vcs_retained_live_source_has_no_nested_executor_or_guarded_await`;
  const good = `pub mod vcs_integration { const VCS_OPERATION_ITEMS: usize = 64 const VCS_OPERATION_PAGE_BYTES: u64 = 16 * 1024 const VCS_OPERATION_PAGES: u64 = 4 const VCS_TOTAL_PAGES: u64 = 256 VcsOperationAdmission::try_claim(items, bytes) fn record_credit { document.0.capacity() change.parent.as_ref() change.author.0.capacity() change.message.capacity() std::mem::size_of::<HashMutation>() } fn checkpoint_credit { let derived_author_items = request.authors.len(); .and_then(|value| value.checked_add(request.authors.len())) .and_then(|value| value.checked_add(derived_author_items)) request.change_ids.capacity() request.authors.capacity() request.change_ids.iter().map(String::capacity) request.authors.iter().map let derived_author_owner_bytes; let derived_author_id_bytes; let fixed = [author_owner_bytes, derived_author_owner_bytes, derived_author_id_bytes]; vcs_credit(items, fixed.into_iter() } fn relation_credit protocol::ActorId(author.0) let mutations = Vec::from([operation]); Vec::with_capacity(source_authors.capacity()) waiters: [Option<VcsStoreWaiter>; VCS_OPERATION_ITEMS] min_by_key(|(_, generation)| *generation) state.waiters[self.slot] = None state.busy_generation = Some(*generation) impl Future for VcsStoreAcquire { VcsOperationAdmission::is_current(self.slot, self.generation) state.busy_generation Poll::Pending } impl Drop for VcsStoreAcquire impl Drop for VcsStoreBuildPermit impl Drop for VcsStoreLease { self.cell.release(self.generation, self.store.take()) } ${fixtures} //#endregion 🔖️VersionGraph`;
  const productionCliWaits = "db::actor::block_on(work)\n".repeat(18);
  const goodCli = `${productionCliWaits}#[cfg(test)]\nfn seed_document() {\n  db::actor::block_on(work)\n}`;
  const mutations = [
    ["nested-block-on", good.replace("state.busy_generation", "block_on(work); state.busy_generation")],
    ["unbounded-waiters", good.replace("waiters: [Option<VcsStoreWaiter>; VCS_OPERATION_ITEMS]", "waiters: Vec<VcsStoreWaiter>")],
    ["missing-nested-bytes", good.replace("request.change_ids.iter().map(String::capacity)", "request.change_ids.len()")],
    ["uncredited-record-author-clone", good.replace("protocol::ActorId(author.0)", "protocol::ActorId(change.author.0.clone())")],
    ["uncredited-record-mutation-vec", good.replace("std::mem::size_of::<HashMutation>()", "0").replace("let mutations = Vec::from([operation]);", "mutations: vec![operation]")],
    ["uncredited-checkpoint-author-id", good.replaceAll("derived_author_id_bytes", "unreserved_author_id_bytes")],
    ["uncredited-checkpoint-author-vec", good.replaceAll("derived_author_owner_bytes", "unreserved_author_owner_bytes")],
    ["uncredited-checkpoint-derived-id-item", good.replace(".and_then(|value| value.checked_add(derived_author_items))", "")],
    ["checkpoint-derived-vec-growth", good.replace("Vec::with_capacity(source_authors.capacity())", "source_authors.into_iter().collect::<Vec<_>>()")],
    ["stale-after-mutation", good.replace("VcsOperationAdmission::is_current(self.slot, self.generation) state.busy_generation", "state.busy_generation VcsOperationAdmission::is_current(self.slot, self.generation)")],
    ["wake-all", good.replace("//#endregion 🔖️VersionGraph", "wake_all //#endregion 🔖️VersionGraph")],
    ["unreserved-fifo-wake", good.replace("state.busy_generation = Some(*generation)", "drop(generation)")],
    ["missing-lease-handback", good.replace("self.cell.release(self.generation, self.store.take())", "drop(self.store.take())")],
    ["poll-loop", good.replace("Poll::Pending", "loop { Poll::Pending }")],
    ["missing-fixture", good.replace("vcs_retained_cancel_clears_waiter_and_slot_aba_stays_stale", "")],
    ["missing-derived-owner-fixture", good.replace("vcs_checkpoint_derived_owner_credit_cap_plus_one_preserves_exact_input", "")],
  ] as const;
  for (const [name, source] of mutations) if (interactivityVcsBridgeFailures(source, goodCli).length === 0) throw new Error(`[verify interactivity] VCS bridge self-test ${name} was falsely accepted.`);
  const productionSeedCli = `${productionCliWaits}fn seed_document() {\n  db::actor::block_on(work)\n}`;
  if (interactivityVcsBridgeFailures(good, productionSeedCli).length === 0) throw new Error("[verify interactivity] VCS bridge self-test production seed_document was falsely accepted.");
  const missingSeedWaitCli = `${productionCliWaits}#[cfg(test)]\nfn seed_document() {\n  drop(work)\n}`;
  if (interactivityVcsBridgeFailures(good, missingSeedWaitCli).length === 0) throw new Error("[verify interactivity] VCS bridge self-test missing test-only seed_document wait was falsely accepted.");
  if (interactivityVcsBridgeFailures(good, goodCli).length !== 0) throw new Error("[verify interactivity] VCS bridge self-test retained authority was falsely rejected.");
}
